use iced::Element;
use crate::document::{DocumentKey, DocumentKind};
use crate::document::image::ImageDocumentMessage;
use crate::document::text::TextDocumentMessage;
use crate::tabs::Tab;

pub struct DocumentTab {
    key: DocumentKey,
    document_kind: DocumentKind,
}

impl DocumentTab {
    pub fn new(key: DocumentKey, document_kind: DocumentKind) -> Self {
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
}

#[derive(Debug)]
pub enum DocumentTabAction {
    None
}

impl Tab for DocumentTab {
    type Message = DocumentTabMessage;
    type Action = DocumentTabAction;

    fn view(&self) -> Element<'_, Self::Message> {

        let view = match &self.document_kind {
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
        match self.document_kind {
            DocumentKind::TextDocument(ref document) => document.path.to_str().unwrap().to_string(),
            DocumentKind::ImageDocument(ref document) => document.path.to_str().unwrap().to_string(),
        }
    }

    fn update(&mut self, message: Self::Message) -> Self::Action {
        match (&mut self.document_kind, message) {
            (DocumentKind::TextDocument(document), DocumentTabMessage::TextDocumentMessage(message)) => {
                let _action = document.update(message);
                DocumentTabAction::None
            },
            (DocumentKind::ImageDocument(document), DocumentTabMessage::ImageDocumentMessage(message)) => {
                let _action = document.update(message);
                DocumentTabAction::None
            },
            _ => unreachable!()
        }
    }
}
