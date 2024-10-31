use std::sync::Arc;
use slotmap::new_key_type;
use iced::{Alignment, Background, Color, Element, Length};
use iced::widget::{container, horizontal_space, Space};
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
                iced::widget::text("Information"),
                // TODO add sidebar item elements from `self`
                container(Space::new(
                    Length::Fill,
                    Length::Fill
                ))
            ]
                .spacing(0)
                .padding(0)
                .width(200)
                .align_x(Alignment::Start),
        )
            .height(Length::Fill)
            .width(200)
            .style(|_theme|container::dark(_theme));

        sidebar
            .into()
    }
}