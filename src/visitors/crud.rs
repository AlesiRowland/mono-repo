use std::fs;
use std::error::Error;
use std::path::Path;
use crate::file_system::files::FileVisitor;

pub struct FileRemover;

impl FileVisitor for FileRemover {
    fn visit_file(&self, path: &impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        fs::remove_file(path).map_err(|err| err.into())
    }
}
