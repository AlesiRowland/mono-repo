use crate::file_system::files;
use std::error;
use std::path::Path;

pub trait FileVisitor {
    fn visit_file(&self, path: &impl AsRef<Path>) -> Result<(), Box<dyn error::Error>>;
}

pub trait FileContentsEditor {
    fn edit(&self, contents: &str) -> Result<String, Box<dyn error::Error>>;
}

impl<E: FileContentsEditor> FileVisitor for E {
    fn visit_file(&self, path: &impl AsRef<Path>) -> Result<(), Box<dyn error::Error>> {
        files::overwrite_file(path, |contents| self.edit(contents))
    }
}
