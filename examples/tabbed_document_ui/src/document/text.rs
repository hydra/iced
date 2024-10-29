use std::fs;
use std::path::PathBuf;

pub struct TextDocument {
    pub path: PathBuf,
    content: String,
}

impl TextDocument {
    pub fn new(path: PathBuf) -> Self {
        println!("creating text document. path: {:?}", path);

        let content = fs::read_to_string(&path).unwrap();

        Self {
            path,
            content,
        }
    }
}
