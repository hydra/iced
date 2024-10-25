use iced::Element;
use iced::widget::text;
use crate::{Tab, TabMessage};

#[derive(Default)]
pub struct HomeTab {}

impl Tab for HomeTab {
    fn view(&self) -> Element<'static, TabMessage> {
        let text = text("tab content area");

        text.into()
    }

    fn label(&self) -> String {
        "Home".to_string()
    }
}
