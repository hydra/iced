//! Tabbed document UI example

use iced_aw::style::tab_bar;
use iced_aw::{TabLabel, Tabs};
use slotmap::{new_key_type, SlotMap};
use iced::widget::{button, column, container, row, text};
use iced::{Element, Task };
use crate::home::HomeTab;

/// entry point
pub fn main() -> iced::Result {
    iced::run("Tabbed document UI", TabbedDocumentUI::update, TabbedDocumentUI::view)
}

#[derive(Debug, Clone)]
enum Message {
    None,
    AddHome,
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
}

#[derive(Default)]
struct TabbedDocumentUI {
    tabs: SlotMap<TabKey, Box<dyn Tab>>,
}

mod home {
    use iced::Element;
    use iced::widget::text;
    use crate::{Tab, TabMessage};

    #[derive(Default)]
    pub struct HomeTab {}

    impl Tab for HomeTab {
        fn view(&self) -> Element<'static, TabMessage> {
            let text = text("content area");

            text.into()
        }
    }
}

impl TabbedDocumentUI {
    fn update(&mut self, _message: Message) -> Task<Message> {
        match _message {
            Message::None => {}
            Message::AddHome => {
                self.add_home()
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
            .fold(Tabs::<TabMessage, TabKey>::new(|tab_key|{
                TabMessage::TabSelected(tab_key)
            })
                .tab_icon_position(iced_aw::tabs::Position::Bottom)
                .on_close(|tab_key|{
                    TabMessage::TabClosed(tab_key)
                })
                //.set_active_tab(&TabId(0))
                .tab_bar_style(Box::new(tab_bar::primary))
            , |tab_bar, (key, tab)| {
                    tab_bar.push(key, TabLabel::Text("Home".to_string()), tab.view())
                });

        let tab_bar: Element<'_, TabMessage> = tab_bar
            .into();

        let mapped_tab_bar: Element<'_, Message> = tab_bar
            .into();

        let text = text("content area");

        let ui: Element<'_, Message> =
            column![
                toolbar,
                mapped_tab_bar,
                text
            ]
                .into();

        container(ui).into()
    }

    fn add_home(&mut self) {
        let home_tab = HomeTab::default();
        let _key = self.tabs.insert(Box::new(home_tab));
    }
}
