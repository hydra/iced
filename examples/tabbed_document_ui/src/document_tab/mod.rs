use slotmap::SlotMap;
use iced::Element;
use crate::document::{DocumentKey, DocumentKind};
use crate::document::image::ImageDocumentMessage;
use crate::document::text::TextDocumentMessage;

pub struct DocumentTab {
    key: DocumentKey,
}

impl DocumentTab {
    pub fn new(key: DocumentKey) -> Self {
        Self {
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

impl DocumentTab {
    pub fn view<'tab, 'document>(&'tab self, documents: &'document SlotMap<DocumentKey, DocumentKind>) -> Element<'document, DocumentTabMessage> {

        let document = documents.get(self.key).unwrap();

        let view = match document {
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

    pub fn label(&self, documents: &SlotMap<DocumentKey, DocumentKind>) -> String {
        let document = documents.get(self.key).unwrap();

        match document {
            DocumentKind::TextDocument(ref document) => document.path.file_name().unwrap().to_str().unwrap().to_string(),
            DocumentKind::ImageDocument(ref document) => document.path.file_name().unwrap().to_str().unwrap().to_string(),
        }
    }

    pub fn update(&mut self, message: DocumentTabMessage, documents: &mut SlotMap<DocumentKey, DocumentKind>) -> DocumentTabAction {
        let document = documents.get_mut(self.key).unwrap();

        match (document, message) {
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
