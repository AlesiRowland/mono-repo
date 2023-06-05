use crate::file_system::directories::{DirEntryIterator, PublicDirIterator};
use crate::file_system::files::FileVisitor;
use std::io;
use std::path::Path;

pub struct ServiceEditor<P: AsRef<Path>> {
    service_root: P,
    file_name: String,
}

impl<P: AsRef<Path>> ServiceEditor<P> {
    pub fn new(service_root: P, file_name: String) -> Self {
        ServiceEditor {
            service_root,
            file_name,
        }
    }

    fn create_iterator(&self) -> io::Result<PublicDirIterator> {
        Ok(PublicDirIterator::for_path(&self.service_root)?)
    }

    pub fn accept_command(&self, visitor: &impl FileVisitor) {
        let dir_iterator = self.create_iterator().unwrap();

        for dir_result in dir_iterator {
            let dir_entry = dir_result.unwrap();
            let file_path = dir_entry.path().join(&self.file_name);
            match visitor.visit_file(&file_path) {
                Ok(()) => continue,
                Err(err) => println!(
                    "Error mutating {}, {} Skipping...",
                    &file_path.to_string_lossy(),
                    err
                ),
            }
        }
    }
}
