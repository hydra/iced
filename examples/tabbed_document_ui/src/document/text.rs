use std::fs;
use std::path::PathBuf;
use iced::Element;
use iced::widget::{row, text_editor};
use crate::document::{Sidebar, SidebarItem};

pub struct TextDocument {
    pub path: PathBuf,

    state: TextDocumentState,
}

#[derive(Default)]
pub struct TextDocumentState {
    content: text_editor::Content,
    sidebar: Sidebar,
}

#[derive(Debug, Clone)]
pub enum TextDocumentMessage {
    None,
    Edit(text_editor::Action),
}

#[derive(Debug)]
pub enum TextDocumentAction {
    None,
}

const SIDEBAR_ITEM_PATH: &str = "PATH";

impl TextDocument {
    pub fn new(path: PathBuf) -> Self {
        println!("creating text document. path: {:?}", path);

        let mut sidebar = Sidebar::default();
        sidebar.add_item(SIDEBAR_ITEM_PATH, SidebarItem::Text(
            "Path".to_string(),
            path.to_str().unwrap().to_string()
        ));

        let text = fs::read_to_string(&path).unwrap();
        let content = text_editor::Content::with_text(&text);
        let state = TextDocumentState {
            content,
            sidebar,
        };

        Self {
            path,
            state,
        }
    }

    pub fn view(&self) -> Element<'_, TextDocumentMessage> {

        let sidebar = self.state.sidebar.view()
            .map(|_message|TextDocumentMessage::None);

        println!("view. content, selection: {:?}", self.state.content.selection());
        // FIXME every time the view is re-created, the state is is lost, e.g. when switching tabs.
        //       lost state includes:
        //       * caret position
        //       * text selection
        let text_editor = text_editor(&self.state.content)
            .on_action(TextDocumentMessage::Edit);

        let ui = row![
            sidebar,
            text_editor,
        ];

        ui
            .into()
    }

    pub fn update(&mut self, message: TextDocumentMessage) -> TextDocumentAction {
        match message {
            TextDocumentMessage::Edit(action) => {
                self.state.content.perform(action);
                println!("update. content, selection: {:?}", self.state.content.selection());
                TextDocumentAction::None
            }
            TextDocumentMessage::None => unreachable!()
        }
    }
}

