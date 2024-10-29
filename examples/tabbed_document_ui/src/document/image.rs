use std::path::PathBuf;

pub struct ImageDocument {
    pub path: PathBuf,
}

impl ImageDocument {
    pub fn new(path: PathBuf) -> Self {
        println!("creating image document. path: {:?}", path);
        Self {
            path,
        }
    }
}
