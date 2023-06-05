use crate::file_system::files::FileContentsEditor;
use regex::Regex;
use std::error::Error;

pub const PYPROJECT_TOML: &str = "pyproject.toml";

fn create_regex(package_name: &str) -> Result<Regex, regex::Error> {
    let reg_string = format!(r#"(?m)^{} = ".+"$"#, package_name);
    Regex::new(reg_string.as_str())
}

#[derive(Debug)]
pub struct StringReplacer {
    regex: Regex,
    repl: String,
}

impl StringReplacer {
    pub fn package_version_editor(package_name: &str, version: &str) -> Result<Self, regex::Error> {
        let regex = create_regex(&package_name)?;
        let repl = format!("{} = \"{}\"", package_name, version);
        Ok(Self { regex, repl })
    }

    pub fn package_remover(package_name: &str) -> Result<Self, regex::Error> {
        let regex = create_regex(&package_name)?;
        let repl = "".to_string();
        Ok(StringReplacer { regex, repl })
    }
}

impl FileContentsEditor for StringReplacer {
    fn edit(&self, contents: &str) -> Result<String, Box<dyn Error>> {
        let edited_contents = self.regex.replace_all(contents, &self.repl);
        Ok(String::from(edited_contents))
    }
}
