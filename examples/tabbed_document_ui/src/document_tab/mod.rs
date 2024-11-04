use std::sync::Arc;
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
            DocumentKind::NewDocument(ref _document) => "New".to_string()
        }
    }

    fn update(&mut self, message: Self::Message) -> Self::Action {
        match (&*self.document_kind, message) {
            (DocumentKind::TextDocument(_document), DocumentTabMessage::TextDocumentMessage(_message)) => DocumentTabAction::None,
            (DocumentKind::ImageDocument(document), DocumentTabMessage::ImageDocumentMessage(message)) => {
                let _action = document.update(message);
                DocumentTabAction::None
            },
            (DocumentKind::NewDocument(document), DocumentTabMessage::NewDocumentMessage(message)) => {
                let _action = document.update(message);
                DocumentTabAction::None
            },
            _ => unreachable!()
        }
    }
}
