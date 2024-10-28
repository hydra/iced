//! Tabbed document UI example

use iced_aw::style::tab_bar;
use iced_aw::{TabLabel, Tabs};
use iced_fonts::NERD_FONT_BYTES;
use slotmap::{new_key_type, SlotMap};
use iced::widget::{button, column, container, row, text};
use iced::{Element, Task};
use crate::home::{HomeTab, HomeTabMessage};

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
    AddHome,
    TabKindMessage(TabMessage<TabKindMessage>)
}

new_key_type! {
    /// A key for a tab
    pub struct TabKey;
}

#[derive(Debug, Clone)]
enum TabMessage<TKM> {
    TabSelected(TabKey),
    TabClosed(TabKey),
    TabKindMessage(TabKey, TKM),
}


trait Tab {
    type Message;

    fn view(&self) -> Element<'static, Self::Message>;
    fn label(&self) -> String;

    fn update(&mut self, message: Self::Message) -> ();
}

enum TabKind {
    Home(HomeTab),
}

#[derive(Debug, Clone)]
enum TabKindMessage {
    HomeTabMessage(HomeTabMessage),
}

impl TabKind {
    pub fn view(&self) -> Element<'static, TabKindMessage> {
        match self {
            TabKind::Home(tab) => tab
                .view()
                .map(|message|{
                    TabKindMessage::HomeTabMessage(message)
                })
                .into()
        }
    }

    pub fn label(&self) -> String {
        match self {
            TabKind::Home(tab) => tab.label()
        }
    }

    pub fn update(&mut self, message: TabKindMessage) {
        match (self, message) {
            (TabKind::Home(tab), TabKindMessage::HomeTabMessage(message)) => tab.update(message)
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
            Message::AddHome => {
                self.add_home()
            }
            Message::TabKindMessage(tab_kind_message) => {
                match tab_kind_message {
                    TabMessage::TabSelected(_key) => {}
                    TabMessage::TabClosed(_key) => {}
                    TabMessage::TabKindMessage(key, message) => {
                        match message {
                            TabKindMessage::HomeTabMessage(ref home_tab_message) => {
                                // find the tab in `self.tabs` and delegate to the `update` method on the tab instance
                                println!("home tab message: {:?}", home_tab_message);
                                let tab = self.tabs.get_mut(key).unwrap();
                                tab.update(message)
                            }
                        }
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
                Tabs::<TabMessage<TabKindMessage>, TabKey>::new(|tab_key|{
                    TabMessage::TabSelected(tab_key)
                })
                    .tab_icon_position(iced_aw::tabs::Position::Bottom)
                    .on_close(|tab_key|{
                        TabMessage::TabClosed(tab_key)
                    })
                    .tab_bar_style(Box::new(tab_bar::primary))
                ,
                |tab_bar, (key, tab)| {
                    let view = tab
                        .view()
                        .map(move |message|{
                            TabMessage::TabKindMessage(key, message)
                        });
                    tab_bar.push(key, TabLabel::Text(tab.label()), view)
                }
            );

        let tab_bar: Element<'_, TabMessage<TabKindMessage>> = tab_bar
            .into();

        let mapped_tab_bar: Element<'_, Message> = tab_bar
            .map(|tab_message|{
                Message::TabKindMessage(tab_message)
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
        // TODO somehow, get the config state into the HomeTab
        let home_tab = HomeTab::default();
        let _key = self.tabs.insert(TabKind::Home(home_tab));
    }
}
