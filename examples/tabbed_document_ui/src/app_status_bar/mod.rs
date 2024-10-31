use iced::{alignment, Color, Element, Length};
use iced::widget::{container, text};

#[derive(Default)]
pub struct StatusBar {}

#[derive(Debug, Clone)]
pub enum StatusBarMessage {
    None
}

impl StatusBar {
    pub fn view(&self) -> Element<'_, StatusBarMessage> {
        let status_bar = container(text("status bar area"))
            .height(32)
            .width(Length::Fill)
            .align_y(alignment::Vertical::Center)
            .style(|_|container::background(Color::parse("#555").unwrap()));

        status_bar
            .into()
    }
}
