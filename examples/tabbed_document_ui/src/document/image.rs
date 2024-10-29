use std::path::PathBuf;
use iced::{ContentFit, Element, Length};
use iced::widget::{image, Space};

pub struct ImageDocument {
    pub path: PathBuf,
    handle: image::Handle
}

#[derive(Debug, Clone)]
pub enum ImageDocumentMessage {
    None
}

impl ImageDocument {
    pub fn new(path: PathBuf) -> Self {
        println!("creating image document. path: {:?}", path);

        let handle = image::Handle::from_path(&path);

        Self {
            path,
            handle
        }
    }

    pub fn view(&self) -> Element<'_, ImageDocumentMessage> {

        let image = image(&self.handle)
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(ContentFit::Contain);

        image
            .into()
    }
}
