use iced_fonts::{Nerd, NERD_FONT};
use iced_fonts::nerd::icon_to_char;
use iced::Element;
use iced::widget::{text, column, checkbox};
use crate::{Tab, TabMessage};

#[derive(Default)]
pub struct HomeTab {
    show_on_startup: bool
}

impl Tab for HomeTab {
    fn view(&self) -> Element<'static, TabMessage> {
        // NOTE: this don't work, likely the font doesn't contain the glyph for 'House'.
        // let text = text("🏠")
        //     .font(NERD_FONT);
        let text = text(format!("{}", icon_to_char(Nerd::Home)).to_string())
            .font(NERD_FONT);

        let show_on_startup_checkbox = checkbox("Show on startup", self.show_on_startup);
            // FIXME fail to update config with new value
            // .on_toggle(|value|{
            //     // THERE ARE NO SUITABLE MESSAGES WE CAN SEND, TabMessage should NOT know about the HomeTab.
            // });

        column![
            text,
            show_on_startup_checkbox,
        ]
            .into()
    }

    fn label(&self) -> String {
        "Home".to_string()
    }
}
