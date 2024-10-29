use std::path::PathBuf;
use iced::{Element, Length};
use iced::widget::Space;

pub struct ImageDocument {
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum ImageDocumentMessage {
    None
}

impl ImageDocument {
    pub fn new(path: PathBuf) -> Self {
        println!("creating image document. path: {:?}", path);
        Self {
            path,
        }
    }

    pub fn view(&self) -> Element<'_, ImageDocumentMessage> {
        let view = Space::new(
            Length::Fill,
            Length::Fill,
        );

        view
            .into()
    }
}
