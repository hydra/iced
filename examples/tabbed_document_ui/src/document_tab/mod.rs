use std::path::PathBuf;
use iced::{Element, Length};
use iced::widget::Space;
use crate::tabs::Tab;

pub struct DocumentTab {
    path: PathBuf
}

impl DocumentTab {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path
        }
    }
}

#[derive(Debug, Clone)]
pub enum DocumentTabMessage {
    None
}

#[derive(Debug)]
pub enum DocumentTabAction {
    None
}

impl Tab for DocumentTab {
    type Message = DocumentTabMessage;
    type Action = DocumentTabAction;

    fn view(&self) -> Element<'static, Self::Message> {
        Space::new(
            Length::Fill,
            Length::Fill
        )
            .into()
    }

    fn label(&self) -> String {
        self.path.to_str().unwrap().to_string()
    }

    fn update(&mut self, message: Self::Message) -> Self::Action {
        match message {
            DocumentTabMessage::None => DocumentTabAction::None
        }
    }
}
