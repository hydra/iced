use crate::document_tab::{DocumentTab, DocumentTabAction, DocumentTabMessage};
use crate::home_tab::{HomeTab, HomeTabAction, HomeTabMessage};

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
