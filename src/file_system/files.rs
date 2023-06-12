use std::error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use crate::file_system::directories::create_service_iterator;

pub const TMP: &str = ".tmp";

//! Converts a path to a String if possible.
//!
//! # Errors:
//!     - on UTF-8 paths.
fn convert_path_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    path.as_ref()
        .to_str()
        .ok_or(io::Error::new(
            ErrorKind::InvalidInput,
            "Filename contained Invalid UTF-8.",
        ))
        .and_then(|string| Ok(String::from(string)))
}

//! Creates a temp filename based off an actual filename
//! This is the equivalent of creating a path with the same parent as the input,
//! but with a actual filename of .tmp.
fn create_tmp_filename(actual_filename: &impl AsRef<Path>) -> Option<PathBuf>{
    let mut filename = actual_filename.as_ref().parent()?.to_path_buf();
    filename.push(TMP);
    Some(filename)
}
/// This function edits a given file using the callback.
/// We have several failure cases:
///     - When the file cannot be opened.
///     - When the file cannot be read to string.
///     - When the file cannot be written to.
/// We know these all will not edit the file.
pub fn overwrite_file<P, F>(path: P, func: F) -> Result<(), Box<dyn error::Error>>
where
    P: AsRef<Path>,
    F: FnOnce(&str) -> Result<String, Box<dyn error::Error>>,
{
    // We read first because we want to fail fast and not make a tmp file otherwise.
    let file_contents = fs::read_to_string(&path)?;
    let edited_file_contents = func(&file_contents)?;

    let mut tmp_file_name = create_tmp_filename(&path).unwrap();
    let mut tmp = File::create(&tmp_file_name)?;
    tmp.write_all(edited_file_contents.as_bytes())?;

    let to = convert_path_to_string(path).map_err(|err| Box::new(err))?;

    if let Err(_) = fs::rename(&tmp_file_name, &to) {
        println!("Error: Unable to overwrite file {}", &to);
        if let Err(_) = fs::remove_file(&tmp_file_name) {
            println!("Warning: Unable to remove tmp file when editing {}", &to)
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::file_system::files::overwrite_file;
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
        overwrite_file(&file_path, |_| Ok("hello world".to_string()))
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