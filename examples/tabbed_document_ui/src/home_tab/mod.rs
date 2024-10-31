use std::sync::{Arc, Mutex};
use iced_fonts::{Nerd, NERD_FONT};
use iced_fonts::nerd::icon_to_char;
use iced::{padding, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{text, column, checkbox, container, row, horizontal_space};
use crate::config::Config;
use crate::tabs::Tab;

pub struct HomeTab {
    config: Arc<Mutex<Config>>
}

impl HomeTab {
    pub fn new(config: Arc<Mutex<Config>>) -> Self {
        Self {
            config
        }
    }
}

#[derive(Debug, Clone)]
pub enum HomeTabMessage {
    ShowOnStartupChanged(bool)
}

#[derive(Debug)]
pub enum HomeTabAction {
    ShowOnStartupChanged
}

impl Tab for HomeTab {

    type Message = HomeTabMessage;
    type Action = HomeTabAction;

    fn view(&self) -> Element<'static, HomeTabMessage> {
        // NOTE: this don't work, likely the font doesn't contain the glyph for 'House'.
        // let text = text("🏠")
        //     .font(NERD_FONT);
        let icon = text(format!("{}", icon_to_char(Nerd::Home)).to_string())
            .font(NERD_FONT);
        let icon_container = container(icon)
            .padding(padding::right(20));

        let text = text("Home");

        let text_container = container(row![icon_container, text])
            .padding(20)
            .style(container::rounded_box);

        let show_on_startup_checkbox = checkbox(
            "Show on startup",
            self.config
                .lock()
                .unwrap()
                .show_home_on_startup
            )
            .on_toggle(|value|{
                HomeTabMessage::ShowOnStartupChanged(value)
            });

        let content = column![
            text_container,
            horizontal_space()
                .height(20),
            show_on_startup_checkbox,
        ]
            .align_x(Horizontal::Center);

        container(content)
            .center(Length::Fill)
            .into()
    }

    fn label(&self) -> String {
        "Home".to_string()
    }

    fn update(&mut self, message: HomeTabMessage) -> HomeTabAction {
        println!("message: {:?}", message);

        match message {
            HomeTabMessage::ShowOnStartupChanged(value) => {
                let mut config = self.config.lock().unwrap();
                config.show_home_on_startup = value;

                HomeTabAction::ShowOnStartupChanged
            }
        }
    }
}
