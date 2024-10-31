use iced::Element;
use iced::widget::horizontal_space;

#[derive(Default)]
pub struct NewDocument {}

#[derive(Debug, Clone)]
pub enum NewDocumentMessage {
    None
}

impl NewDocument {
    pub fn view(&self) -> Element<'_, NewDocumentMessage> {
        horizontal_space()
            .into()
    }
}