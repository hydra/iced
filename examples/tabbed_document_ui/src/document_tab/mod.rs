use std::sync::Arc;
use iced::{Element, Length};
use iced::widget::Space;
use crate::document::DocumentKind;
use crate::tabs::Tab;

pub struct DocumentTab {
    document_kind: Arc<DocumentKind>
}

impl DocumentTab {
    pub fn new(document_kind: Arc<DocumentKind>) -> Self {
        Self {
            document_kind
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
        let path = match *self.document_kind {
            DocumentKind::TextDocument(ref document) => &document.path,
            DocumentKind::ImageDocument(ref document) => &document.path,
        };
        path.to_str().unwrap().to_string()
    }

    fn update(&mut self, message: Self::Message) -> Self::Action {
        match message {
            DocumentTabMessage::None => DocumentTabAction::None
        }
    }
}
