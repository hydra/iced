use crate::document_tab::{DocumentTab, DocumentTabAction, DocumentTabMessage};
use crate::home_tab::{HomeTab, HomeTabAction, HomeTabMessage};
use crate::new_tab::{NewTab, NewTabAction, NewTabMessage};
use crate::tabs::TabKey;

pub enum TabKind {
    Home(HomeTab),
    Document(DocumentTab),
    New(NewTab)
}

#[derive(Debug, Clone)]
pub enum TabKindMessage {
    HomeTabMessage(HomeTabMessage),
    DocumentTabMessage(DocumentTabMessage),
    NewTabMessage(NewTabMessage),
}

pub enum TabKindAction {
    HomeTabAction(TabKey, HomeTabAction),
    DocumentTabAction(TabKey, DocumentTabAction),
    NewTabAction(TabKey, NewTabAction),
}
