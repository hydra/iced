use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use iced::{widget, Element};
use iced::widget::{container, row, text_editor};
use crate::document::{Sidebar, SidebarItem};

pub struct TextDocument {
    pub path: PathBuf,

    state: TextDocumentState,
}

#[derive(Default)]
pub struct TextDocumentState {
    content: Option<text_editor::Content>,
    sidebar: Sidebar,
}

#[derive(Debug, Clone)]
pub enum TextDocumentMessage {
    None,
    Edit(text_editor::Action),
    Load,
    Loaded(String),
}

#[derive(Debug)]
pub enum TextDocumentAction {
    None,
    Load
}

const SIDEBAR_ITEM_PATH: &str = "PATH";

impl TextDocument {
    pub fn from_path(path: PathBuf) -> (Self, TextDocumentMessage) {
        println!("loading text document. path: {:?}", path);

        let mut sidebar = Sidebar::default();
        sidebar.add_item(SIDEBAR_ITEM_PATH, SidebarItem::Text(
            "Path".to_string(),
            path.to_str().unwrap().to_string()
        ));

        let state = TextDocumentState {
            content: None,
            sidebar,
        };

        (
            Self {
                path,
                state,
            },
            TextDocumentMessage::Load
        )
    }

    pub fn new(path: PathBuf) -> (Self, TextDocumentMessage) {
        println!("creating text document. path: {:?}", path);

        // FUTURE the file could be created asynchronously instead of directly here
        let _result = fs::write(&path, "New text file").ok();

        Self::from_path(path)
    }


    pub async fn load(path: PathBuf) -> String {
        let text = fs::read_to_string(&path).unwrap();

        // Simulate slow loading
        async_std::task::sleep(Duration::from_millis(500)).await;

        text
    }

    pub fn view(&self) -> Element<'_, TextDocumentMessage> {

        let sidebar = self.state.sidebar.view()
            .map(|_message|TextDocumentMessage::None);

        let content_container = match &self.state.content {
            Some(content) => {
                println!("view. content, selection: {:?}", content.selection());
                // FIXME every time the view is re-created, the state is is lost, e.g. when switching tabs.
                //       lost state includes:
                //       * caret position
                //       * text selection
                let text_editor = text_editor(&content)
                    .on_action(TextDocumentMessage::Edit);

                container(text_editor)
            },
            None => {
                container(widget::text("Loading..."))
            }
        };

        let ui = row![
            sidebar,
            content_container,
        ];

        ui
            .into()
    }

    pub fn update(&mut self, message: TextDocumentMessage) -> TextDocumentAction {
        println!("text document update, message: {:?}", message);
        match message {
            TextDocumentMessage::Edit(action) => {
                match &mut self.state.content {
                    Some(content) => {
                        content.perform(action);
                        println!("update. content, selection: {:?}", content.selection());
                    },
                    None => ()
                }
                TextDocumentAction::None
            },
            TextDocumentMessage::Load => {
                TextDocumentAction::Load
            },
            TextDocumentMessage::Loaded(text) => {
                let content = text_editor::Content::with_text(&text);

                self.state.content.replace(content);

                TextDocumentAction::None
            }
            TextDocumentMessage::None => unreachable!()
        }
    }
}

