use std::fs;
use std::path::PathBuf;
use iced::Element;
use iced::widget::{row, text};
use crate::document::{Sidebar, SidebarItem};
use crate::document::image::ImageDocumentMessage;

pub struct TextDocument {
    pub path: PathBuf,
    content: String,

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

        let content = fs::read_to_string(&path).unwrap();

        Self {
            path,
            content,
            sidebar,
        }
    }

    pub fn view(&self) -> Element<'_, TextDocumentMessage> {

        let sidebar = self.sidebar.view()
            .map(|_message|TextDocumentMessage::None);

        let text_content = text(&self.content);

        let ui = row![
            sidebar,
            text_content
        ];

        ui
            .into()

    }
}

