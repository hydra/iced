use std::path::PathBuf;
use iced::{ContentFit, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{image, column, row, container};
use crate::document::Sidebar;

pub struct ImageDocument {
    pub path: PathBuf,
    handle: image::Handle,

    sidebar: Sidebar,
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
            handle,
            sidebar: Sidebar::default()
        }
    }

    pub fn view(&self) -> Element<'_, ImageDocumentMessage> {

        let sidebar = self.sidebar.view()
            .map(|message|ImageDocumentMessage::None);

        // FIXME the image should be:
        //       * top-left justified
        //       * maintain it's aspect ratio
        //       * fill the available space on the shortest edge of the container
        //       * have no whitespace to the left or of the image
        //       * have no whitespace on the right of the image
        //       However, no amount of fiddling with the .width/height/align methods makes it work.

        let image = image(&self.handle)
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(ContentFit::Contain);

        let image_container = container(image)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Left)
            .align_y(Vertical::Top)
            // TODO why do these take a 'Length' argument?
            .align_left(Length::Fill)
            .align_top(Length::Fill);

        let ui = row![sidebar, image_container];

        ui
            .into()
    }
}
