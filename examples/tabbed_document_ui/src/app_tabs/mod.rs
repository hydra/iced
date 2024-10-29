use iced::Element;
use crate::document_tab::{DocumentTab, DocumentTabAction, DocumentTabMessage};
use crate::home_tab::{HomeTab, HomeTabAction, HomeTabMessage};
use crate::tabs::{AppTabs, Tab, TabKey};

pub enum TabKind {
    Home(HomeTab),
    Document(DocumentTab),
}

#[derive(Debug, Clone)]
pub enum TabKindMessage {
    HomeTabMessage(HomeTabMessage),
    DocumentTabMessage(DocumentTabMessage),
}

pub enum TabKindAction {
    HomeTabAction(HomeTabAction),
    DocumentTabAction(DocumentTabAction),
}

impl AppTabs<TabKindMessage, TabKindAction> for TabKind {
    fn view<'a>(&self, _key: TabKey) -> Element<'a, TabKindMessage> {
        match self {
            TabKind::Home(tab) => tab
                .view()
                .map(|message|{
                    TabKindMessage::HomeTabMessage(message)
                })
                .into(),
            TabKind::Document(tab) => tab
                .view()
                .map(|message|{
                    TabKindMessage::DocumentTabMessage(message)
                })
                .into()
        }
    }

    fn label(&self, _key: TabKey) -> String {
        match self {
            TabKind::Home(tab) => tab.label(),
            TabKind::Document(tab) => tab.label(),
        }
    }

    fn update(&mut self, message: TabKindMessage) -> TabKindAction {
        match (self, message) {
            (TabKind::Home(tab), TabKindMessage::HomeTabMessage(message)) => {
                TabKindAction::HomeTabAction(tab.update(message))
            }
            (TabKind::Document(tab), TabKindMessage::DocumentTabMessage(message)) => {
                TabKindAction::DocumentTabAction(tab.update(message))
            }
            _ => unreachable!()
        }
    }
}