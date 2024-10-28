#[derive(Debug, Clone)]
pub enum ToolbarMessage {
    AddHome,
    CloseAllTabs,
}

#[derive(Debug)]
pub enum ToolbarAction {
    AddHomeTab,
    CloseAllTabs,
}

#[derive(Default)]
pub struct Toolbar {}

impl Toolbar {
    pub fn update(&mut self, message: ToolbarMessage) -> ToolbarAction {
        match message {
            ToolbarMessage::AddHome => {
                ToolbarAction::AddHomeTab
            }
            ToolbarMessage::CloseAllTabs => {
                ToolbarAction::CloseAllTabs
            }
        }
    }
}
