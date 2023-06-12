use std::fs::{DirEntry, ReadDir};
use std::io::Read;
use std::iter::Filter;
use std::path::Path;
use std::{env, fs, io};

type DirResult = io::Result<DirEntry>;

fn is_dir(dir_result: &DirResult) -> bool {
    dir_result
        .as_ref()
        .map(|dir_entry| dir_entry.path().is_dir())
        .unwrap_or(true)
}

fn is_public(dir_result: &DirResult) -> bool {
    dir_result
        .as_ref()
        .map(|dir_entry| !dir_entry.file_name().to_string_lossy().starts_with("."))
        .unwrap_or(true)
}


pub fn create_service_iterator<P>(path: P) -> io::Result<impl Iterator<Item=DirResult>>
    where
        P: AsRef<Path>
{
    let read_dir = fs::read_dir(path) ?;
    Ok(read_dir.filter(is_dir).filter(is_public))
}

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
    fn test_create_service_iterator() -> Result<(), io::Error> {
        let temp_dir = fake_services_dir()?;
        env::set_current_dir(temp_dir.path())?;
        let directory_iterator = create_service_iterator(&temp_dir)?;
        let left = directory_iterator
            .map(|dir| dir.unwrap().file_name())
            .collect::<HashSet<OsString>>();
        let public_services = ["db_service", "config_service", "settings_service"];
        let right = HashSet::from(public_services.map(|s| s.into()));

        assert_eq!(left, right);
        Ok(())
    }
}