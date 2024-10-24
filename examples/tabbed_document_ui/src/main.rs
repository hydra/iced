//! Tabbed document UI example

use iced_aw::style::tab_bar;
use iced_aw::Tabs;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Task };

/// entry point
pub fn main() -> iced::Result {
    iced::run("Tabbed document UI", TabbedDocumentUI::update, TabbedDocumentUI::view)
}

#[derive(Debug, Clone)]
enum Message {
    None
}

#[derive(PartialEq, Eq, Clone)]
struct TabId(pub usize);

#[derive(Default)]
struct TabbedDocumentUI {
}

impl TabbedDocumentUI {
    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {

        let home_button = button("home");
        let new_button = button("new");
        let open_button = button("open");
        let close_all_button = button("close all");


        let toolbar: Element<'_, Message> =
            row![home_button, new_button, open_button, close_all_button]
                .into();

        let tab_bar = Tabs::<Message, TabId>::new(|tab_id|{
            Message::None
        })
            .tab_icon_position(iced_aw::tabs::Position::Bottom)
            .on_close(|tab_id|{
                Message::None
            })
            .set_active_tab(&TabId(0))
            .tab_bar_style(Box::new(tab_bar::primary));

        let tab_bar: Element<'_, Message > = tab_bar
            .into();

        let text = text("content area");

        let ui: Element<'_, Message > =
            column![
                toolbar,
                tab_bar,
                text
            ]
                .into();

        container(ui).into()
    }
}
