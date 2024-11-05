//! Tabbed document UI example
//!
//! Example `config.json` file
//! ```json
//! {"show_home_on_startup":true,"open_document_paths":["examples/tabbed_document_ui/assets/text_file_1.txt","examples/tabbed_document_ui/assets/text_file_2.txt","examples/tabbed_document_ui/assets/image_file_1.bmp"]}
//! ```

use std::path;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use iced_fonts::NERD_FONT_BYTES;
use rfd::{MessageDialog, MessageLevel};
use slotmap::SlotMap;
use thiserror::Error;
use iced::widget::{button, column, container, row};
use iced::{event, window, Element, Event, Subscription, Task};
use crate::app_status_bar::{StatusBar, StatusBarMessage};
use crate::app_tabs::{TabKind, TabKindAction, TabKindMessage};
use crate::app_toolbar::{ToolbarAction, ToolbarMessage};
use crate::config::Config;
use crate::document::{DocumentKey, DocumentKind};
use crate::document::image::ImageDocument;
use crate::document::text::TextDocument;
use crate::document_tab::DocumentTab;
use crate::home_tab::{HomeTab, HomeTabAction};
use crate::tabs::{TabAction, TabKey, TabMessage};

mod home_tab;
mod document_tab;
mod config;
mod document;

mod tabs;
mod app_tabs;
mod app_toolbar;
mod app_status_bar;

/// entry point
pub fn main() -> iced::Result {

    let config = Arc::new(Mutex::new(config::load()));

    let result = iced::application("Tabbed document UI", TabbedDocumentUI::update, TabbedDocumentUI::view)
        .font(NERD_FONT_BYTES)
        .exit_on_close_request(false)
        .subscription(TabbedDocumentUI::window_subscription)
        .run_with({
            let config = config.clone();
            move ||{
                let mut ui = TabbedDocumentUI::new(config.clone());

                let config = config.lock().unwrap();

                if config.show_home_on_startup {
                    ui.show_home();
                }

                let documents_to_open = config.open_document_paths.clone();
                for document_path in documents_to_open {
                    let _ = ui.open_document(&document_path);
                }

                (ui, Task::none())
            }
        });

    let config = config.lock().unwrap();
    config::save(&config);

    result
}

#[derive(Debug, Clone)]
enum Message {
    TabMessage(TabMessage<TabKindMessage>),
    ToolbarMessage(ToolbarMessage),
    CloseRequested,
    StatusBarMessage(StatusBarMessage),
    None,
}

struct TabbedDocumentUI {
    tabs: tabs::Tabs<TabKind, TabKindMessage, TabKindAction>,
    toolbar: app_toolbar::Toolbar,
    config: Arc<Mutex<Config>>,

    documents: SlotMap<DocumentKey, DocumentKind>,
    status_bar: StatusBar
}

const SUPPORTED_IMAGE_EXTENSIONS: [&'static str; 5] = ["bmp", "png", "jpg", "jpeg", "svg"];
const SUPPORTED_TEXT_EXTENSIONS: [&'static str; 1] = ["txt"];

impl TabbedDocumentUI {

    pub fn new(config: Arc<Mutex<Config>>) -> Self {
        Self {
            tabs: Default::default(),
            toolbar: Default::default(),
            config,
            documents: Default::default(),
            status_bar: Default::default(),
        }
    }

    fn window_subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _, _| {
            if let Event::Window(window::Event::CloseRequested) = event {
                Some(Message::CloseRequested)
            } else {
                None
            }
        })
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToolbarMessage(message) => self.on_toolbar_message(message),
            Message::TabMessage(message) => self.on_tab_message(message),
            Message::StatusBarMessage(message) => self.on_status_bar_message(message),
            Message::CloseRequested => self.on_close_requested(),
            Message::None => { Task::none() }
        }
    }

    fn on_toolbar_message(&mut self, toolbar_message: ToolbarMessage) -> Task<Message> {
        let action = self.toolbar.update(toolbar_message);
        match action {
            ToolbarAction::CloseAllTabs => {
                let closed_tabs = self.tabs.close_all();
                let tasks: Vec<_> = closed_tabs.into_iter().map(|(key, kind)|self.on_tab_closed(key, kind)).collect();

                Task::batch(tasks)
            }
            ToolbarAction::ShowHome => {
                self.show_home();

                Task::none()
            }
            ToolbarAction::OpenDocument => self.on_toolbar_open_document(),
            ToolbarAction::NewDocument => self.on_toolbar_new_document(),
        }
    }

    fn on_tab_message(&mut self, message: TabMessage<TabKindMessage>) -> Task<Message> {
        let action = self.tabs.update(message, |tab, message|{
            match (tab, message) {
                (TabKind::Home(tab), TabKindMessage::HomeTabMessage(message)) => {
                    TabKindAction::HomeTabAction(tab.update(message))
                }
                (TabKind::Document(tab), TabKindMessage::DocumentTabMessage(message)) => {
                    TabKindAction::DocumentTabAction(tab.update(message, &mut self.documents))
                }
                _ => unreachable!()
            }
        });

        match action {
            TabAction::TabSelected(key) => self.on_tab_selected(key),
            TabAction::TabClosed(key, kind) => self.on_tab_closed(key, kind),
            TabAction::TabAction(tab_kind_action) => self.on_tab_action(tab_kind_action),
        }
    }

    fn on_status_bar_message(&self, message: StatusBarMessage) -> Task<Message> {
        match message {
            StatusBarMessage::None => {}
        }
        Task::none()
    }


    fn on_close_requested(&mut self) -> Task<Message> {
        self.update_open_documents();

        window::get_latest().and_then(window::close)
    }

    fn on_toolbar_open_document(&mut self) -> Task<Message> {
        let all_extensions: Vec<_> = SUPPORTED_TEXT_EXTENSIONS.iter().chain(&SUPPORTED_IMAGE_EXTENSIONS).collect();

        let current_directory = std::env::current_dir().unwrap();

        let result = rfd::FileDialog::new()
            .add_filter("All", &all_extensions)
            .add_filter("Text", &SUPPORTED_TEXT_EXTENSIONS)
            .add_filter("Image", &SUPPORTED_IMAGE_EXTENSIONS)
            .set_directory(current_directory)
            .pick_file()
            .map(|path| {
                self.open_document(&path)
            });

        println!("open file result: {:?}", result);

        async fn show_error(error: OpenDocumentError) {
            // TODO set the parent window handle correctly, see `MessageDialog::set_parent`
            let _ = MessageDialog::new()
                .set_level(MessageLevel::Error)
                .set_title("Open document error")
                .set_description(format!("Opening document failed. reason: {}", error.to_string()))
                .set_buttons(rfd::MessageButtons::Ok)
                .show();
        }

        if let Some(Err(error)) = result {
            // Note: a Task is required due to a focus issue.
            //       on windows the task the dialog is not displayed until 'alt' is pressed (yes, really)
            //       observed if MessageDialog::show is called now instead of using Task::perform
            Task::perform(show_error(error), |_result| Message::None)
        } else {
            Task::none()
        }
    }

    fn on_toolbar_new_document(&mut self) -> Task<Message> {

        todo!()
    }

    fn on_tab_selected(&mut self, key: TabKey) -> Task<Message> {
        println!("tab selected. key: {:?}", key);

        Task::none()
    }

    fn on_tab_action(&mut self, tab_kind_action: TabKindAction) -> Task<Message> {
        match tab_kind_action {
            TabKindAction::HomeTabAction(home_tab_action) => {
                println!("home tab action: {:?}", home_tab_action);
                match home_tab_action {
                    HomeTabAction::ShowOnStartupChanged => {
                        // TODO something...
                    }
                }
            }
            TabKindAction::DocumentTabAction(document_tab_action) => {
                println!("document tab action: {:?}", document_tab_action);
            }
        }
        Task::none()
    }

    fn on_tab_closed(&mut self, key: TabKey, tab_kind: TabKind) -> Task<Message> {
        println!("tab closed. key: {:?}", key);

        match tab_kind {
            TabKind::Document(document_tab) => {
                let document_key = document_tab.key();
                let _document = self.documents.remove(document_key);
            },
            _ => ()
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {

        let home_button = button("home")
            .on_press(ToolbarMessage::ShowHome);
        let new_button = button("new")
            .on_press(ToolbarMessage::NewDocument);
        let open_button = button("open")
            .on_press(ToolbarMessage::OpenDocument);
        let close_all_button = button("close all")
            .on_press(ToolbarMessage::CloseAllTabs);


        let toolbar: Element<'_, ToolbarMessage> =
            row![home_button, new_button, open_button, close_all_button]
                .spacing(2)
                .into();

        let mapped_toolbar: Element<'_, Message> = toolbar
            .map(|toolbar_message| Message::ToolbarMessage(toolbar_message))
            .into();


        let tab_bar = self.tabs.view(|_key, tab|{
            match tab {
                TabKind::Home(tab) => tab
                    .view()
                    .map(|message|{
                        TabKindMessage::HomeTabMessage(message)
                    })
                    .into(),
                TabKind::Document(tab) => tab
                    .view(&self.documents)
                    .map(|message|{
                        TabKindMessage::DocumentTabMessage(message)
                    })
                    .into()
            }
        }, |_key, tab|{
            match tab {
                TabKind::Home(tab) => tab.label(),
                TabKind::Document(tab) => tab.label(&self.documents),
            }

        });

        let mapped_tab_bar: Element<'_, Message> = tab_bar
            .map(|tab_message|{
                Message::TabMessage(tab_message)
            })
            .into();

        let mapped_status_bar: Element<'_, Message> = self.status_bar.view()
            .map(|status_bar_message|{
                Message::StatusBarMessage(status_bar_message)
            })
            .into();

        let ui: Element<'_, Message> =
            column![
                // item              desired layout
                mapped_toolbar,   // height: auto
                mapped_tab_bar,   // height: fill
                mapped_status_bar // height: auto
            ]
                .into();

        container(ui).into()
    }

    fn show_home(&mut self) {
        let home_tab = self.tabs.iter().find(|(_key, value)|
            match value {
                TabKind::Home(_) => true,
                _ => false
            }
        );

        if let Some((key, _tab)) = home_tab {
            self.tabs.activate(key);
        } else {
            let home_tab = HomeTab::new(self.config.clone());
            let _key = self.tabs.push(TabKind::Home(home_tab));

            println!("added home tab");
        }
    }

    fn open_document(&mut self, path: &PathBuf) -> Result<(), OpenDocumentError>{

        let path = path::absolute(path)
            .or_else(|cause|Err(OpenDocumentError::IoError {cause}))?;

        let extension = path.extension().unwrap().to_str().unwrap();

        let document = if SUPPORTED_TEXT_EXTENSIONS.contains(&extension) {
            let text_document = TextDocument::new(path.clone());

            DocumentKind::TextDocument(text_document)
        } else if SUPPORTED_IMAGE_EXTENSIONS.contains(&extension) {
            let image_document = ImageDocument::new(path.clone());

            DocumentKind::ImageDocument(image_document)
        } else {
            return Err(OpenDocumentError::UnsupportedFileExtension { extension: extension.to_string() });
        };


        let document_key = self.documents.insert(document);

        let document_tab = DocumentTab::new(document_key);
        let key = self.tabs.push(TabKind::Document(document_tab));
        self.tabs.activate(key);

        Ok(())
    }

    /// Update the config with the currently open documents
    fn update_open_documents(&mut self) {
        let open_documents: Vec<PathBuf> = self.documents.iter()
            .map(|(_key, document)| {
                match document {
                    DocumentKind::TextDocument(document) => document.path.clone(),
                    DocumentKind::ImageDocument(document) => document.path.clone(),
                }
            })
            .collect();
        println!("open_documents: {:?}", open_documents);

        self.config.lock().unwrap().open_document_paths = open_documents;
    }
}

#[derive(Error, Debug)]
enum OpenDocumentError {
    #[error("Unsupported file type. extension: {extension}")]
    UnsupportedFileExtension{extension: String},
    #[error("IO error, cause: {cause}")]
    IoError{cause: std::io::Error},
}
