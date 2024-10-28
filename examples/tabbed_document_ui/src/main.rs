//! Tabbed document UI example

use std::sync::Arc;
use iced_fonts::NERD_FONT_BYTES;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Task};
use crate::app_tabs::{TabKind, TabKindAction, TabKindMessage};
use crate::config::Config;
use crate::home::{HomeTab, HomeTabAction};
use crate::tabs::{TabAction, TabKey, TabMessage};

mod home;
mod config;

mod tabs;
mod app_tabs;

/// entry point
pub fn main() -> iced::Result {

    let config = Arc::new(config::load());

    let result = iced::application("Tabbed document UI", TabbedDocumentUI::update, TabbedDocumentUI::view)
        .font(NERD_FONT_BYTES)
        .run_with({
            let config = config.clone();
            move ||{
                let mut ui = TabbedDocumentUI::new(config.clone());

                if config.show_home_on_startup {
                    ui.add_home();
                }

                (ui, Task::none())
            }
        });

    // TODO how do we get the value of the `show_on_startup` in the HomeTab instance back into the config?

    config::save(&config);

    result
}

#[derive(Debug, Clone)]
enum Message {
    TabKindMessage(TabMessage<TabKindMessage>),
    ToolbarMessage(ToolbarMessage),
}

#[derive(Debug, Clone)]
enum ToolbarMessage {
    AddHome,
    CloseAllTabs,
}

#[derive(Debug)]
enum ToolbarAction {
    AddedHomeTab,
    ClosedAllTabs(Vec<TabKey>),
}

struct TabbedDocumentUI {
    tabs: tabs::Tabs<TabKind, TabKindMessage, TabKindAction>,
    config: Arc<Config>
}

impl TabbedDocumentUI {

    pub fn new(config: Arc<Config>) -> Self {
        Self {
            tabs: Default::default(),
            config,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToolbarMessage(toolbar_message) => {
                let action = self.process_toolbar_message(toolbar_message);
                match action {
                    ToolbarAction::ClosedAllTabs(closed_tabs) => {
                        for tab_key in closed_tabs {
                            Self::on_tab_closed(tab_key)
                        }
                    }
                    ToolbarAction::AddedHomeTab => {
                        println!("added home tab");
                    }
                }
            }
            Message::TabKindMessage(message) => {
                let action = self.tabs.update(message);

                match action {
                    TabAction::TabSelected(key) => {
                        println!("tab selected. key: {:?}", key);
                    }
                    TabAction::TabClosed(key) => {
                        Self::on_tab_closed(key);
                    }
                    TabAction::TabAction(tab_kind_action) => {
                        match tab_kind_action {
                            TabKindAction::HomeTabAction(home_tab_action) => {
                                println!("home tab action: {:?}", home_tab_action);
                                match home_tab_action {
                                    HomeTabAction::ShowOnStartupChanged => {
                                        // TODO something...
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Task::none()
    }

    fn on_tab_closed(key: TabKey) {
        println!("tab closed. key: {:?}", key);
    }

    fn view(&self) -> Element<'_, Message> {

        let home_button = button("home")
            .on_press(ToolbarMessage::AddHome);
        let new_button = button("new");
        let open_button = button("open");
        let close_all_button = button("close all")
            .on_press(ToolbarMessage::CloseAllTabs);


        let toolbar: Element<'_, ToolbarMessage> =
            row![home_button, new_button, open_button, close_all_button]
                .into();

        let mapped_toolbar: Element<'_, Message> = toolbar
            .map(|toolbar_message| Message::ToolbarMessage(toolbar_message))
            .into();


        let tab_bar = self.tabs.view();

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
                mapped_toolbar,   // height: auto
                mapped_tab_bar,   // height: fill
                status_bar        // height: auto
            ]
                .into();

        container(ui).into()
    }

    fn add_home(&mut self) {
        let home_tab = HomeTab::new(self.config.show_home_on_startup);
        let _key = self.tabs.push(TabKind::Home(home_tab));
    }

    fn process_toolbar_message(&mut self, message: ToolbarMessage) -> ToolbarAction {
        match message {
            ToolbarMessage::AddHome => {
                self.add_home();
                ToolbarAction::AddedHomeTab
            }
            ToolbarMessage::CloseAllTabs => {
                let closed_tabs = self.tabs.close_all();
                ToolbarAction::ClosedAllTabs(closed_tabs)
            }
        }
    }
}

