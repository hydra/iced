#[derive(Debug, Clone)]
pub enum ToolbarMessage {
    ShowHome,
    CloseAllTabs,
    OpenDocument,
}

#[derive(Debug)]
pub enum ToolbarAction {
    ShowHome,
    CloseAllTabs,
    OpenDocument,
}

#[derive(Default)]
pub struct Toolbar {}

impl Toolbar {
    pub fn update(&mut self, message: ToolbarMessage) -> ToolbarAction {
        match message {
            ToolbarMessage::ShowHome => {
                ToolbarAction::ShowHome
            }
            ToolbarMessage::CloseAllTabs => {
                ToolbarAction::CloseAllTabs
            }
            ToolbarMessage::OpenDocument => {
                ToolbarAction::OpenDocument
            }
        }
    }
}
