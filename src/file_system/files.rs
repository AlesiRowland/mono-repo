use std::error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Write};
use std::path::Path;

#[cfg(test)]
mod tests {
    use crate::file_system::files::edit_file;
    use std::io::Write;
    use std::path::Path;
    use std::{fs, io};
    use tempdir::TempDir;

    fn get_test_file<P: AsRef<Path>>(path: P) -> io::Result<fs::File> {
        let file = fs::File::options()
            .create(true)
            .read(true)
            .write(true)
            .truncate(true)
            .open(&path)?;

        Ok(file)
    }

    #[test]
    fn test_edit_file() -> io::Result<()> {
        // Test setup
        let temp = TempDir::new("temp").unwrap();
        let file_path = temp.path().join("temp_file.txt").into_boxed_path();
        let mut file = get_test_file(&file_path).unwrap();
        file.write_all(b"mock package\n").unwrap();

        // Act
        edit_file(&file_path, |_| Ok("hello world".to_string()))
            .map_err(|err| Box::new(err))
            .unwrap();

        // Collect
        let left = fs::read_to_string(&file_path)?;
        let right = "hello world";

        // Assert
        assert_eq!(left, right);
        Ok(())
    }
}

pub const TMP: &str = ".tmp";

fn convert_path_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    path.as_ref()
        .to_str()
        .ok_or(io::Error::new(
            ErrorKind::InvalidInput,
            "Filename contained Invalid UTF-8.",
        ))
        .and_then(|string| Ok(String::from(string)))
}

/// This function edits a given file using the callback.
/// We have several failure cases:
///     - When the file cannot be opened.
///     - When the file cannot be read to string.
///     - When the file cannot be written to.
/// We know these all will not edit the file.
pub fn edit_file<P, F>(path: P, func: F) -> Result<(), Box<dyn error::Error>>
where
    P: AsRef<Path>,
    F: FnOnce(&str) -> Result<String, Box<dyn error::Error>>,
{
    // We read first because we want to fail fast and not make a tmp file otherwise.
    let file_contents = fs::read_to_string(&path)?;
    let edited_file_contents = func(&file_contents)?;

    let mut tmp_file_name = path.as_ref().parent().unwrap().to_path_buf();
    tmp_file_name.push(TMP);
    let mut tmp = File::create(&tmp_file_name).map_err(|err| Box::new(err))?;
    tmp.write_all(edited_file_contents.as_bytes())
        .map_err(|err| Box::new(err))?;

    let to = convert_path_to_string(path).map_err(|err| Box::new(err))?;

    if let Err(_) = fs::rename(&tmp_file_name, &to) {
        println!("Error: Unable to overwrite file {}", &to);
        if let Err(_) = fs::remove_file(&tmp_file_name) {
            println!("Warning: Unable to remove tmp file when editing {}", &to)
        }
    }

    Ok(())
}

pub trait FileVisitor {
    fn visit_file(&self, path: &impl AsRef<Path>) -> Result<(), Box<dyn error::Error>>;
}

pub trait FileContentsEditor {
    fn edit(&self, contents: &str) -> Result<String, Box<dyn error::Error>>;
}

impl<E: FileContentsEditor> FileVisitor for E {
    fn visit_file(&self, path: &impl AsRef<Path>) -> Result<(), Box<dyn error::Error>> {
        edit_file(path, |contents| self.edit(contents))
    }
}
