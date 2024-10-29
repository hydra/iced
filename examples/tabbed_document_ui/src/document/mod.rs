use std::sync::Arc;
use slotmap::new_key_type;
use crate::document::image::ImageDocument;
use crate::document::text::TextDocument;

pub mod text;
pub mod image;

new_key_type! {
    /// A key for a document
    pub struct DocumentKey;
}

pub enum DocumentKind {
    TextDocument(Arc<TextDocument>),
    ImageDocument(Arc<ImageDocument>),
}