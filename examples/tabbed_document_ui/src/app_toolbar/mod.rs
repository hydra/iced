#[derive(Debug, Clone)]
pub enum ToolbarMessage {
    ShowHome,
    CloseAllTabs,
}

#[derive(Debug)]
pub enum ToolbarAction {
    ShowHome,
    CloseAllTabs,
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
        }
    }
}
