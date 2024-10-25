//! Tabbed document UI example

use std::any::Any;
use iced_aw::style::tab_bar;
use iced_aw::{TabLabel, Tabs};
use iced_fonts::NERD_FONT_BYTES;
use slotmap::{new_key_type, SlotMap};
use iced::widget::{button, column, container, row, text};
use iced::{Element, Task};
use crate::home::HomeTab;

mod home;
mod config;

/// entry point
pub fn main() -> iced::Result {

    let config = config::load();

    let result = iced::application("Tabbed document UI", TabbedDocumentUI::update, TabbedDocumentUI::view)
        .font(NERD_FONT_BYTES)
        .run_with(move ||{
            let mut ui = TabbedDocumentUI::default();
            if config.show_home_on_startup {
                ui.add_home();
            }

            (ui, Task::none())
        });

    // TODO how do we get the value of the `show_on_startup` in the HomeTab instance back into the config?

    config::save(&config);

    result
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
    ChildMessage(Box<dyn Any>)
}

trait Tab {
    type Message;

    fn view(&self) -> Element<'static, Self::Message>;
    fn label(&self) -> String;

    fn update(&mut self, message: Box<dyn Any>) -> ();
}

enum TabKind {
    Home(HomeTab),
}

impl TabKind {
    pub fn view(&self) -> Element<'static, TabMessage> {
        match self {
            TabKind::Home(tab) => tab
                .view()
                .map(|message|{
                    // TODO somehow put message in ChildMessage
                    TabMessage::ChildMessage(Box::new(message))
                })
                .into()
        }
    }

    pub fn label(&self) -> String {
        match self {
            TabKind::Home(tab) => tab.label()
        }
    }

    pub fn update(&mut self, message: Box<dyn Any>) {
        match self {
            TabKind::Home(tab) => tab.update(message)
        }
    }
}


#[derive(Default)]
struct TabbedDocumentUI {
    tabs: SlotMap<TabKey, TabKind>,
}

impl TabbedDocumentUI {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::None => {}
            Message::AddHome => {
                self.add_home()
            }
            Message::TabMessage(message) => {
                println!("message: {:?}", message);

                let tab_key= TabKey::default();
                let tab = self.tabs.get_mut(tab_key).unwrap();

                match message {
                    TabMessage::TabSelected(_) => {}
                    TabMessage::TabClosed(_) => {}
                    TabMessage::ChildMessage(child_message) => {
                        tab.update(child_message);

                    }
                }
            }
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
        let _key = self.tabs.insert(TabKind::Home(home_tab));
    }
}
