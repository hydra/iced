//! Tabbed document UI example

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use iced_fonts::NERD_FONT_BYTES;
use slotmap::SlotMap;
use iced::widget::{button, column, container, row, text};
use iced::{event, window, Element, Event, Subscription, Task};
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
                    ui.add_home();
                }

                let documents_to_open = config.open_document_paths.clone();
                for document_path in documents_to_open {
                    ui.open_document(document_path)
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
    TabKindMessage(TabMessage<TabKindMessage>),
    ToolbarMessage(ToolbarMessage),
    CloseRequested,
}

struct TabbedDocumentUI {
    tabs: tabs::Tabs<TabKind, TabKindMessage, TabKindAction>,
    toolbar: app_toolbar::Toolbar,
    config: Arc<Mutex<Config>>,
    documents: SlotMap<DocumentKey, Arc<DocumentKind>>,
}

impl TabbedDocumentUI {

    pub fn new(config: Arc<Mutex<Config>>) -> Self {
        Self {
            tabs: Default::default(),
            toolbar: Default::default(),
            config,
            documents: Default::default(),
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
            Message::ToolbarMessage(toolbar_message) => {
                let action = self.toolbar.update(toolbar_message);
                match action {
                    ToolbarAction::CloseAllTabs => {
                        let closed_tabs = self.tabs.close_all();
                        for tab_key in closed_tabs {
                            Self::on_tab_closed(tab_key)
                        }
                    }
                    ToolbarAction::AddHomeTab => {
                        self.add_home();
                        println!("added home tab");
                    }
                }
            }
            Message::TabKindMessage(message) => {
                let action = self.tabs.update(message);

                match action {
                    TabAction::TabSelected(key) => {
                        println!("tab selected. key: {:?}", key);
                    }
                    TabAction::TabClosed(key) => {
                        Self::on_tab_closed(key);
                    }
                    TabAction::TabAction(tab_kind_action) => {
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
                    }
                }
            }
            Message::CloseRequested => {
                self.update_open_documents();

                return window::get_latest().and_then(window::close);
            }
        }
        Task::none()
    }

    /// Update the config with the currently open documents
    fn update_open_documents(&mut self) {
        let open_documents: Vec<PathBuf> = self.documents.iter().map(|(_key, document)| {
            match &**document {
                DocumentKind::TextDocument(document) => document.path.clone(),
                DocumentKind::ImageDocument(document) => document.path.clone(),
            }
        }).collect();
        println!("open_documents: {:?}", open_documents);

        self.config.lock().unwrap().open_document_paths = open_documents;
    }

    fn on_tab_closed(key: TabKey) {
        println!("tab closed. key: {:?}", key);
    }

    fn view(&self) -> Element<'_, Message> {

        let home_button = button("home")
            .on_press(ToolbarMessage::AddHome);
        let new_button = button("new");
        let open_button = button("open");
        let close_all_button = button("close all")
            .on_press(ToolbarMessage::CloseAllTabs);


        let toolbar: Element<'_, ToolbarMessage> =
            row![home_button, new_button, open_button, close_all_button]
                .into();

        let mapped_toolbar: Element<'_, Message> = toolbar
            .map(|toolbar_message| Message::ToolbarMessage(toolbar_message))
            .into();


        let tab_bar = self.tabs.view();

        let mapped_tab_bar: Element<'_, Message> = tab_bar
            .map(|tab_message|{
                Message::TabKindMessage(tab_message)
            })
            .into();

        // FIXME not displayed when no tabs are present.
        let status_bar = text("status bar area");

        let ui: Element<'_, Message> =
            column![
                // item              desired layout
                mapped_toolbar,   // height: auto
                mapped_tab_bar,   // height: fill
                status_bar        // height: auto
            ]
                .into();

        container(ui).into()
    }

    fn add_home(&mut self) {
        let home_tab = HomeTab::new(self.config.clone());
        let _key = self.tabs.push(TabKind::Home(home_tab));
    }

    fn open_document(&mut self, path: PathBuf) {


        let document = match path.extension().unwrap().to_str().unwrap() {
            "txt" => {
                let text_document = TextDocument::new(path.clone());

                DocumentKind::TextDocument(Arc::new(text_document))
            },
            "bmp" | "png" | "jpg" | "jpeg" | "svg" => {
                let image_document = ImageDocument::new(path.clone());

                DocumentKind::ImageDocument(Arc::new(image_document))
            },
            _ => unreachable!()
        };

        let document_arc = Arc::new(document);

        let _document_key = self.documents.insert(document_arc.clone());

        let document_tab = DocumentTab::new(document_arc);
        let _key = self.tabs.push(TabKind::Document(document_tab));
    }
}


