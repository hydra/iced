use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use iced::Element;
use crate::document::{DocumentKey, DocumentKind};
use crate::document::image::ImageDocumentMessage;
use crate::document::new::NewDocumentMessage;
use crate::document::text::TextDocumentMessage;
use crate::tabs::Tab;

pub struct DocumentTab {
    key: DocumentKey,
    document_kind: Arc<DocumentKind>,
}

impl DocumentTab {
    pub fn new(key: DocumentKey, document_kind: Arc<DocumentKind>) -> Self {
        Self {
            key,
            document_kind,
        }
    }

    pub fn key(&self) -> DocumentKey {
        self.key.clone()
    }
}

#[derive(Debug, Clone)]
pub enum DocumentTabMessage {
    None,
    TextDocumentMessage(TextDocumentMessage),
    ImageDocumentMessage(ImageDocumentMessage),
    NewDocumentMessage(NewDocumentMessage),
}

#[derive(Debug)]
pub enum DocumentTabAction {
    None
}

impl Tab for DocumentTab {
    type Message = DocumentTabMessage;
    type Action = DocumentTabAction;

    fn view(&self) -> Element<'_, Self::Message> {

        let view = match &*self.document_kind {
            DocumentKind::TextDocument(text_document) => text_document
                .view()
                .map(DocumentTabMessage::TextDocumentMessage),
            DocumentKind::ImageDocument(image_document) => image_document
                .view()
                .map(DocumentTabMessage::ImageDocumentMessage),
            DocumentKind::NewDocument(new_document) => {
                new_document
                    .view()
                    .map(DocumentTabMessage::NewDocumentMessage)
            },
        };

        view
            .into()
    }

    fn label(&self) -> String {
        match *self.document_kind {
            DocumentKind::TextDocument(ref document) => document.path.to_str().unwrap().to_string(),
            DocumentKind::ImageDocument(ref document) => document.path.to_str().unwrap().to_string(),
            DocumentKind::NewDocument(ref document) => "New".to_string()
        }
    }

    fn update(&mut self, message: Self::Message) -> Self::Action {
        match message {
            DocumentTabMessage::None => DocumentTabAction::None,
            DocumentTabMessage::TextDocumentMessage(_) => DocumentTabAction::None,
            DocumentTabMessage::ImageDocumentMessage(_) => DocumentTabAction::None,
            DocumentTabMessage::NewDocumentMessage(message) => {
                match &*self.document_kind {
                    DocumentKind::NewDocument(document) => {
                        let _action = document.update(message);

                        DocumentTabAction::None
                    }
                    _ => unreachable!()
                }
            },
        }
    }
}
