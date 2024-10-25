//! Tabbed document UI example

use iced_aw::style::tab_bar;
use iced_aw::{TabLabel, Tabs};
use iced_fonts::NERD_FONT_BYTES;
use slotmap::{new_key_type, SlotMap};
use iced::widget::{button, column, container, row, text};
use iced::{Element, Task};
use crate::home::HomeTab;

mod home;

/// entry point
pub fn main() -> iced::Result {
    iced::application("Tabbed document UI", TabbedDocumentUI::update, TabbedDocumentUI::view)
        .font(NERD_FONT_BYTES)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    None,
    AddHome,
    TabMessage(TabMessage),

}

new_key_type! {
    /// A key for a tab
    pub struct TabKey;
}

#[derive(Debug, Clone)]
enum TabMessage {
    TabSelected(TabKey),
    TabClosed(TabKey),
}

trait Tab {
    fn view(&self) -> Element<'static, TabMessage>;
    fn label(&self) -> String;
}

#[derive(Default)]
struct TabbedDocumentUI {
    tabs: SlotMap<TabKey, Box<dyn Tab>>,
}

impl TabbedDocumentUI {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::None => {}
            Message::AddHome => {
                self.add_home()
            }
            Message::TabMessage(_) => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {

        let home_button = button("home")
            .on_press(Message::AddHome);
        let new_button = button("new");
        let open_button = button("open");
        let close_all_button = button("close all");


        let toolbar: Element<'_, Message> =
            row![home_button, new_button, open_button, close_all_button]
                .into();

        let tab_bar = self.tabs
            .iter()
            .fold(
                Tabs::<TabMessage, TabKey>::new(|tab_key|{
                    TabMessage::TabSelected(tab_key)
                })
                    .tab_icon_position(iced_aw::tabs::Position::Bottom)
                    .on_close(|tab_key|{
                        TabMessage::TabClosed(tab_key)
                    })
                    .tab_bar_style(Box::new(tab_bar::primary))
                ,
             |tab_bar, (key, tab)| {
                    tab_bar.push(key, TabLabel::Text(tab.label()), tab.view())
                }
            );

        let tab_bar: Element<'_, TabMessage> = tab_bar
            .into();

        let mapped_tab_bar: Element<'_, Message> = tab_bar
            .map(|tab_message|{
                Message::TabMessage(tab_message)
            })
            .into();

        // FIXME not displayed when no tabs are present.
        let status_bar = text("status bar area");

        let ui: Element<'_, Message> =
            column![
                // item              desired layout
                toolbar,          // height: auto
                mapped_tab_bar,   // height: fill
                status_bar        // height: auto
            ]
                .into();

        container(ui).into()
    }

    fn add_home(&mut self) {
        let home_tab = HomeTab::default();
        let _key = self.tabs.insert(Box::new(home_tab));
    }
}
