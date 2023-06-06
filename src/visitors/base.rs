use std::path::Path;
use std::error;
use crate::file_system::files;

pub trait FileVisitor {
    fn visit_file(&self, path: &impl AsRef<Path>) -> Result<(), Box<dyn error::Error>>;
}

pub trait FileContentsEditor {
    fn edit(&self, contents: &str) -> Result<String, Box<dyn error::Error>>;
}

impl<E: FileContentsEditor> FileVisitor for E {
    fn visit_file(&self, path: &impl AsRef<Path>) -> Result<(), Box<dyn error::Error>> {
        files::edit_file(path, |contents| self.edit(contents))
    }
}
