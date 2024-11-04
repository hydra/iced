use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use iced::Element;
use iced::widget::{row, text_editor};
use crate::document::{Sidebar, SidebarItem};

pub struct TextDocument {
    pub path: PathBuf,

    state: TextDocumentState,
}

#[derive(Default)]
pub struct TextDocumentState {
    content: text_editor::Content,
    sidebar: Sidebar,
}

#[derive(Debug, Clone)]
pub enum TextDocumentMessage {
    None
}

const SIDEBAR_ITEM_PATH: &str = "PATH";

impl TextDocument {
    pub fn new(path: PathBuf) -> Self {
        println!("creating text document. path: {:?}", path);


        let mut sidebar = Sidebar::default();
        sidebar.add_item(SidebarItem::Text(
            SIDEBAR_ITEM_PATH,
            "Path".to_string(),
            path.to_str().unwrap().to_string()
        ));

        let text = fs::read_to_string(&path).unwrap();
        let content = text_editor::Content::with_text(&text);
        let state = TextDocumentState {
            content,
            sidebar,
        };

        Self {
            path,
            state,
        }
    }

    pub fn view(&self) -> Element<'_, TextDocumentMessage> {

        let sidebar = self.state.sidebar.view()
            .map(|_message|TextDocumentMessage::None);


        let text_editor = text_editor(&self.state.content);

        let ui = row![
            sidebar,
            text_editor,
        ];

        ui
            .into()

    }
}

