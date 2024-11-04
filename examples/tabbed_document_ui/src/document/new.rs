use std::sync::Mutex;
use iced::Element;
use iced::widget::{button, row, text_input, column, text_editor};

pub struct NewDocument {
    state: Mutex<NewDocumentState>
}

impl Default for NewDocument {
    fn default() -> Self {
        Self {
            state: Mutex::new(NewDocumentState::default())
        }
    }
}

#[derive(Default)]
pub struct NewDocumentState {
    name: String,
    content: text_editor::Content
}

#[derive(Debug, Clone)]
pub enum NewDocumentMessage {
    None,
    NameChanged(String),
}

pub enum NewDocumentAction {
    None
}

impl NewDocument {
    pub fn view(&self) -> Element<'_, NewDocumentMessage> {

        let state_guard = self.state.lock().unwrap();

        let name_row = row![
            "Name",
            text_input("Name", &state_guard.name)
                .on_input(NewDocumentMessage::NameChanged)
        ];

        let ok_button = button("Ok");

        column![
            name_row,
            ok_button,
        ]
            .into()
    }

    pub fn update(&self, message: NewDocumentMessage) -> NewDocumentAction {
        match message {
            NewDocumentMessage::None => {}
            NewDocumentMessage::NameChanged(name) => { self.state.lock().unwrap().name = name }
        }

        NewDocumentAction::None
    }
}