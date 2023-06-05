use std::path::Path;
use std::{env, fs, io};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::ffi::OsString;
    use std::io::Write;
    use std::{env, fs, io};
    use tempdir::TempDir;

    const FAKE_SERVICES: [&str; 4] = ["db_service", "config_service", "settings_service", ".git"];

    const FAKE_FILE: &str = "file.txt";

    fn fake_services_dir() -> Result<TempDir, io::Error> {
        let temp = TempDir::new("services")?;
        for service in FAKE_SERVICES {
            let path = temp.path().join(service);
            fs::create_dir(&path)?;
            let file_path = path.join("pyproject.toml");
            let mut file = fs::File::create(file_path)?;
            file.write_all(b"mock package = \"0.0.0\"\nother_package = \"1.1.1\"\n")?;
        }

        fs::File::create(temp.path().join(FAKE_FILE))?;
        Ok(temp)
    }

    #[test]
    fn test_dir_iterator() -> Result<(), io::Error> {
        let temp_dir = fake_services_dir()?;
        env::set_current_dir(temp_dir.path())?;
        let directory_iterator = DirIterator::for_cwd()?;
        let left = directory_iterator
            .map(|dir| dir.unwrap().file_name())
            .collect::<HashSet<OsString>>();
        let right = HashSet::from(FAKE_SERVICES.map(|s| s.into()));

        assert_eq!(left, right);
        Ok(())
    }

    #[test]
    fn test_public_dir_iterator() -> Result<(), io::Error> {
        let temp_dir = fake_services_dir()?;
        env::set_current_dir(temp_dir.path())?;
        let directory_iterator = PublicDirIterator::for_cwd()?;
        let left = directory_iterator
            .map(|dir| dir.unwrap().file_name())
            .collect::<HashSet<OsString>>();
        let public_services = ["db_service", "config_service", "settings_service"];
        let right = HashSet::from(public_services.map(|s| s.into()));

        assert_eq!(left, right);
        Ok(())
    }
}

/// Struct for iterating through directories.
pub struct DirIterator(fs::ReadDir);

/// Struct for iterating through public directories.
pub struct PublicDirIterator(DirIterator);

impl From<fs::ReadDir> for PublicDirIterator {
    fn from(value: fs::ReadDir) -> Self {
        let dir_iterator = DirIterator(value);
        PublicDirIterator(dir_iterator)
    }
}

impl From<fs::ReadDir> for DirIterator {
    fn from(value: fs::ReadDir) -> Self {
        DirIterator(value)
    }
}

/// # Implementation
/// Iterates through the directories of inner ReadDir, skipping files.
/// Each iteration returns an Option containing a Result
impl Iterator for DirIterator {
    type Item = Result<fs::DirEntry, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(dir_result) = self.0.next() {
            let dir_entry = match dir_result {
                Ok(dir_entry) => dir_entry,
                Err(err) => return Some(Err(err)),
            };

            let metadata = match dir_entry.metadata() {
                Ok(md) => md,
                Err(err) => return Some(Err(err)),
            };

            if metadata.is_dir() {
                return Some(Ok(dir_entry));
            }
        }
        None
    }
}

/// # Implementation
/// Iterates through the directories of inner ReadDir, skipping files and private
/// directories.
/// Each iteration returns an Option containing a Result
impl Iterator for PublicDirIterator {
    type Item = Result<fs::DirEntry, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(dir_result) = self.0.next() {
            match dir_result {
                Ok(dir) if dir.file_name().to_str().unwrap().starts_with(".") => continue,
                Ok(dir) => return Some(Ok(dir)),
                Err(err) => return Some(Err(err)),
            }
        };
        None
    }
}

pub trait DirEntryIterator:
From<fs::ReadDir> + Iterator<Item=Result<fs::DirEntry, io::Error>>
{
    fn for_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let read_dir = fs::read_dir(path.as_ref())?;
        Ok(Self::from(read_dir))
    }
    fn for_cwd() -> Result<Self, io::Error> {
        let cwd = env::current_dir()?;
        Self::for_path(cwd)
    }
}

impl DirEntryIterator for DirIterator {}

impl DirEntryIterator for PublicDirIterator {}
