use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use iced::{widget, Element, Task};
use iced::widget::row;

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

        let elements = row![
            widget::text("Name"),
            widget::text_input("Name", &self.name)
                .on_input(NewTabMessage::NameChanged),
            widget::text("Directory"),
            widget::text_input("Directory", &self.directory.to_str().unwrap()),
            widget::button("...")
                .on_press(NewTabMessage::ChooseDirectory),
            widget::pick_list(kinds, self.kind, NewTabMessage::KindSelected),
            widget::button("Ok")
                // TODO validation!
                .on_press(NewTabMessage::CreateDocument)
        ];

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
            NewTabMessage::CreateDocument => { NewTabAction::CreateDocument(self.name.clone(), self.directory.clone(), self.kind.unwrap()) }
        }
    }

    async fn choose_directory() -> Option<PathBuf> {
        let result = rfd::FileDialog::new()
            .pick_folder();

        result
    }
}