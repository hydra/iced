use std::any::Any;
use iced_fonts::{Nerd, NERD_FONT};
use iced_fonts::nerd::icon_to_char;
use iced::Element;
use iced::widget::{text, column, checkbox};
use crate::{Tab, TabMessage};

#[derive(Default)]
pub struct HomeTab {
    show_on_startup: bool
}

#[derive(Debug, Clone)]
pub enum HomeTabMessage {
    ShowOnStartupChanged(bool)
}

impl Tab for HomeTab {

    type Message = HomeTabMessage;

    fn view(&self) -> Element<'static, HomeTabMessage> {
        // NOTE: this don't work, likely the font doesn't contain the glyph for 'House'.
        // let text = text("🏠")
        //     .font(NERD_FONT);
        let text = text(format!("{}", icon_to_char(Nerd::Home)).to_string())
            .font(NERD_FONT);

        let show_on_startup_checkbox = checkbox("Show on startup", self.show_on_startup)
            .on_toggle(|value|{
                HomeTabMessage::ShowOnStartupChanged(value)
            });

        column![
            text,
            show_on_startup_checkbox,
        ]
            .into()
    }

    fn label(&self) -> String {
        "Home".to_string()
    }

    fn update(&mut self, message: Box<dyn Any>) -> () {
        let message: &HomeTabMessage = &message.downcast_ref().unwrap();

        match message {
            HomeTabMessage::ShowOnStartupChanged(value) => {
                self.show_on_startup = *value;
            }
        }

        println!("message: {:?}", message);
    }
}
