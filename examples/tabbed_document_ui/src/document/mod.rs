use std::sync::Arc;
use slotmap::new_key_type;
use iced::{Alignment, Element, Length};
use iced::widget::{container, horizontal_space};
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

#[derive(Default)]
pub struct Sidebar {}

#[derive(Debug, Clone)]
pub enum SidebarMessage {
    None
}

impl Sidebar {
    pub fn view(&self) -> Element<'_, SidebarMessage> {
        let sidebar = container(
            iced::widget::column![
                "Information",
                // TODO add sidebar item elements from `self`
                horizontal_space()
            ]
                .spacing(0)
                .padding(0)
                .width(200)
                .align_x(Alignment::Start),
        )
            .style(container::rounded_box)
            .center_y(Length::Fill);

        sidebar
            .into()
    }
}