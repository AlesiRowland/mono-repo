use std::fs;
use std::error::Error;
use std::path::Path;
use crate::file_system::files::FileVisitor;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io;
    use tempdir::TempDir;
    use crate::file_system::files::FileVisitor;
    use crate::visitors::crud::FileRemover;

    #[test]
    fn test_file_remover_visit_file() -> Result<(), io::Error>{
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
