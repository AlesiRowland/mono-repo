use crate::file_system::files::FileVisitor;
use std::error::Error;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests {
    use crate::file_system::files::FileVisitor;
    use crate::visitors::crud::FileRemover;
    use std::fs::File;
    use std::io;
    use tempdir::TempDir;

    #[test]
    fn test_file_remover_visit_file() -> Result<(), io::Error> {
        let tempdir = TempDir::new("test")?;
        let path = tempdir.path().join("tmp");
        File::create(&path)?;
        FileRemover.visit_file(&path).unwrap();
        let left = path.exists();
        let right = false;
        assert_eq!(left, right);
        Ok(())
    }
}
pub struct FileRemover;

impl FileVisitor for FileRemover {
    fn visit_file(&self, path: &impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        fs::remove_file(path).map_err(|err| err.into())
    }
}
