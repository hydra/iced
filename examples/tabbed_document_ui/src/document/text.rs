use std::fs;
use std::path::PathBuf;
use iced::Element;
use iced::widget::text;

pub struct TextDocument {
    pub path: PathBuf,
    content: String,
}

#[derive(Debug, Clone)]
pub enum TextDocumentMessage {
    None
}

impl TextDocument {
    pub fn new(path: PathBuf) -> Self {
        println!("creating text document. path: {:?}", path);

        let content = fs::read_to_string(&path).unwrap();

        Self {
            path,
            content,
        }
    }

    pub fn view(&self) -> Element<'_, TextDocumentMessage> {
        let text = text(&self.content);

        text.into()
    }
}

