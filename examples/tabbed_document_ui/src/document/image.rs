use std::path::PathBuf;
use std::sync::Mutex;
use iced::{ContentFit, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{image, row, container};
use iced::widget::image::viewer;
use crate::document::{Sidebar, SidebarItem};


#[derive(Debug, Clone, Default)]
pub struct Coordinate {
    x: u32,
    y: u32,
}

pub struct ImageDocument {
    pub path: PathBuf,
    handle: image::Handle,

    state: Mutex<ImageDocumentState>,


}

#[derive(Default)]
pub struct ImageDocumentState {
    last_clicked: Option<Coordinate>,
    sidebar: Sidebar,
}

#[derive(Debug, Clone)]
pub enum ImageDocumentMessage {
    None,
    ImageClicked(Coordinate),
}

pub enum ImageDocumentAction {
    None
}

const SIDEBAR_ITEM_PATH: &str = "PATH";
const SIDEBAR_ITEM_LAST_CLICKED_COORDINATE: &str = "LAST_CLICKED_COORDINATE";

impl ImageDocument {
    pub fn new(path: PathBuf) -> Self {
        println!("creating image document. path: {:?}", path);

        let handle = image::Handle::from_path(&path);

        let mut sidebar = Sidebar::default();

        sidebar.add_item(SidebarItem::Text(
            SIDEBAR_ITEM_PATH,
            "Path".to_string(),
            path.to_str().unwrap().to_string()
        ));

        sidebar.add_item(SidebarItem::Text(
            SIDEBAR_ITEM_LAST_CLICKED_COORDINATE,
            "Last clicked coordinate".to_string(),
            "None".to_string()
        ));
        Self {
            path,
            handle,
            state: Mutex::new(Default::default()),
        }
    }

    pub fn view(&self) -> Element<'_, ImageDocumentMessage> {

        let sidebar_guard = self.state.lock().unwrap();
        let sidebar_element = sidebar_guard.sidebar.view()
            .map(|_message|ImageDocumentMessage::None);

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
            sidebar_element,
            image_container
        ];

        ui
            .into()
    }

    pub fn update(&self, message: ImageDocumentMessage) -> ImageDocumentAction {
        match message {
            ImageDocumentMessage::None => (),
            ImageDocumentMessage::ImageClicked(coordinate) => {

                let mut state = self.state.lock().unwrap();

                state.last_clicked = Some(coordinate);

                state.sidebar.update_item(SIDEBAR_ITEM_LAST_CLICKED_COORDINATE,|item: &mut SidebarItem|{
                    let SidebarItem::Text(_key, _label, value) = item;
                    *value = "foo".to_string();
                });
            }
        }
        ImageDocumentAction::None
    }
}
