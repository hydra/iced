use iced::Element;
use crate::home::{HomeTab, HomeTabAction, HomeTabMessage};
use crate::tabs::{AppTabs, Tab, TabKey};

pub enum TabKind {
    Home(HomeTab),
}

#[derive(Debug, Clone)]
pub enum TabKindMessage {
    HomeTabMessage(HomeTabMessage),
}

pub enum TabKindAction {
    HomeTabAction(HomeTabAction)
}

impl AppTabs<TabKindMessage, TabKindAction> for TabKind {
    fn view<'a>(&self, _key: TabKey) -> Element<'a, TabKindMessage> {
        match self {
            TabKind::Home(tab) => tab
                .view()
                .map(|message|{
                    TabKindMessage::HomeTabMessage(message)
                })
                .into()
        }
    }

    fn label(&self, _key: TabKey) -> String {
        match self {
            TabKind::Home(tab) => tab.label()
        }
    }

    fn update(&mut self, message: TabKindMessage) -> TabKindAction {
        match (self, message) {
            (TabKind::Home(tab), TabKindMessage::HomeTabMessage(message)) => {
                TabKindAction::HomeTabAction(tab.update(message))
            }
        }
    }
}