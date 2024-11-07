use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use iced::{widget, Element, Length, Task};
use iced::widget::{row, column, horizontal_space};

#[derive(Debug)]
pub struct NewTab {
    directory: PathBuf,
    name: String,
    kind: Option<NewDocumentKind>
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NewDocumentKind {
    Text,
    Image,
}

impl Display for NewDocumentKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            NewDocumentKind::Text => "Text",
            NewDocumentKind::Image => "Image",
        })
    }
}

#[derive(Debug, Clone)]
pub enum NewTabMessage {
    None,
    NameChanged(String),
    ChooseDirectory,
    KindSelected(NewDocumentKind),
    DirectoryChosen(PathBuf),
    CreateDocument,
}


pub enum NewTabAction {
    None,
    CreateDocument(String, PathBuf, NewDocumentKind),
    Task(Task<NewTabMessage>),
}

impl NewTab {
    pub fn new() -> Self {
        Self {
            directory: Default::default(),
            name: "".to_string(),
            kind: None,
        }
    }

    pub fn label(&self) -> String {
        "New".to_string()
    }

    pub fn view(&self) -> Element<'_, NewTabMessage> {

        let kinds = [
            NewDocumentKind::Text,
            NewDocumentKind::Image,
        ];

        let message_to_emit_when_valid = if
                !self.name.is_empty() &&
                self.kind.is_some() &&
                self.directory.exists()
        {
            Some(NewTabMessage::CreateDocument)
        } else {
            None
        };

        let form = column![
            column![
                widget::text("Name"),
                widget::text_input("Name", &self.name)
                    .on_input(NewTabMessage::NameChanged)
                    .width(Length::Fill),
            ]
                .spacing(5),
            column![
                widget::text("Directory"),
                row![
                    widget::text_input("Directory", &self.directory.to_str().unwrap()),
                    widget::button("...")
                        .on_press(NewTabMessage::ChooseDirectory)
                ]
                    .spacing(5)
                    .width(Length::Fill),
            ]
                .spacing(5),
            column![
                widget::text("Type"),
                widget::pick_list(kinds, self.kind, NewTabMessage::KindSelected)
                    .width(Length::Fill),
            ]
                .spacing(5),
            row![
                horizontal_space()
                    .width(Length::Fill),
                widget::button("Ok")
                    .on_press_maybe(message_to_emit_when_valid)
            ]
        ]
            .spacing(10);

        let elements = row![
            horizontal_space()
                .width(Length::FillPortion(2)),
            form
                .width(Length::FillPortion(6)),
            horizontal_space()
                .width(Length::FillPortion(2)),
        ]
            .padding(50);

        // FIXME when the window is resized such that the form elements are too small or not visible
        //       the form should be scrollable to allow it to be used.
        //       * attempted to wrap elements in a scrollable with horizontal and vertical scrolling
        //         but that resulted in a panic
        //       * attempted to wrap elements in a container with width+max_width and height+max height
        //         but that still allowed the form to be made to small.
        //       Notably there appears to be no 'min_width' or 'min_height'.

        elements
            .into()
    }

    pub fn update(&mut self, message: NewTabMessage) -> NewTabAction {
        match message {
            NewTabMessage::None => { NewTabAction::None },
            NewTabMessage::NameChanged(name) => { self.name = name; NewTabAction::None },
            NewTabMessage::ChooseDirectory => {
                let task = Task::perform(Self::choose_directory(), |result| {
                    match result {
                        Some(path) => NewTabMessage::DirectoryChosen(path),
                        None => NewTabMessage::None
                    }
                });
                NewTabAction::Task(task)
            }
            NewTabMessage::KindSelected(kind) => { self.kind.replace(kind); NewTabAction::None },
            NewTabMessage::DirectoryChosen(directory) => { self.directory = directory; NewTabAction::None },
            NewTabMessage::CreateDocument => {
                NewTabAction::CreateDocument(self.name.clone(), self.directory.clone(), self.kind.unwrap())
            }
        }
    }

    async fn choose_directory() -> Option<PathBuf> {
        let result = rfd::FileDialog::new()
            .pick_folder();

        result
    }
}
