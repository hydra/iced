use std::path::PathBuf;
use iced::{ContentFit, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{image, column, row, container};
use iced::widget::image::viewer;
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
        //       * have no whitespace to the left of the image
        //       * have no whitespace above the image
        //       However, no amount of fiddling with the .width/height/align methods makes it work.
        //       As soon as you specify either a width or height for the image, or a viewer you get
        //       padding either on either left AND right or top AND bottom.

        let image_viewer = viewer(self.handle.clone())
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(ContentFit::Contain);

        let image_container = container(image_viewer)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Left)
            .align_y(Vertical::Top);

        let ui = row![
            sidebar,
            image_container
        ];

        ui
            .into()
    }
}
