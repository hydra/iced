use std::sync::Arc;
use iced::Element;
use crate::document::{DocumentKey, DocumentKind};
use crate::document::image::ImageDocumentMessage;
use crate::document::text::TextDocumentMessage;
use crate::tabs::Tab;

pub struct DocumentTab {
    key: DocumentKey,
    document_kind: Arc<DocumentKind>,
}

impl DocumentTab {
    pub fn new(key: DocumentKey, document_kind: Arc<DocumentKind>) -> Self {
        Self {
            document_kind,
            key
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
        };

        view
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
            DocumentTabMessage::None => DocumentTabAction::None,
            DocumentTabMessage::TextDocumentMessage(_) => DocumentTabAction::None,
            DocumentTabMessage::ImageDocumentMessage(_) => DocumentTabAction::None,
        }
    }
}
