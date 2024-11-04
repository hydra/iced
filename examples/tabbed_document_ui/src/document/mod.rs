use slotmap::new_key_type;
use iced::{widget, Alignment, Element, Length};
use iced::widget::{column, container, row, Space};
use crate::document::image::ImageDocument;
use crate::document::text::TextDocument;

pub mod text;
pub mod image;

new_key_type! {
    /// A key for a document
    pub struct DocumentKey;
}

pub enum DocumentKind {
    TextDocument(TextDocument),
    ImageDocument(ImageDocument),
}

#[derive(Default)]
pub struct Sidebar {
    items: Vec<(&'static str, SidebarItem)>
}

#[derive(Debug, Clone)]
pub enum SidebarMessage {
    None
}

impl Sidebar {

    pub fn add_item(&mut self, key: &'static str, item: SidebarItem) {
        self.items.push((key, item));
    }

    pub fn update_item<F>(&mut self, key: &str, mut update_fn: F)
    where
        F: FnMut(&mut SidebarItem) -> ()
    {
        if let Some(item) = self.items.iter_mut()
            .find_map(|(ref candidate_key, item)|{
                //println!("candidate_key: {}", candidate_key);
                if (*candidate_key).eq(key) {
                    Some(item)
                } else {
                    None
                }
            }) {
            update_fn(item)
        }
    }

    pub fn view(&self) -> Element<'_, SidebarMessage> {

        let items = column(self.items.iter().map(|(_key, item)|SidebarItem::view(item)));

        let sidebar = container(
            widget::column![
                iced::widget::text("Information")
                    .width(Length::Fill)
                    .center(),
                items,
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

pub enum SidebarItem {
    Text(String, String)
}


impl SidebarItem {
    pub fn view(&self) -> Element<'_, SidebarMessage> {
        match self {
            SidebarItem::Text(title, value) => {
                row![
                    iced::widget::text(title)
                        .width(Length::FillPortion(1)),
                    iced::widget::text(value)
                        .width(Length::FillPortion(1)),

                ]
                    .width(Length::Fill)
                    .into()
            }
        }
    }
}