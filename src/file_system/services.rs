use crate::file_system::directories::{DirEntryIterator, PublicDirIterator};
use crate::visitors::base::FileVisitor;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use clap::error::Error;

pub struct ServiceEditor<P: AsRef<Path>> {
    service_root: P,
}

impl<P: AsRef<Path>> ServiceEditor<P> {
    pub fn new(service_root: P) -> Self {
        ServiceEditor {
            service_root,
        }
    }

    fn create_iterator(&self) -> io::Result<PublicDirIterator> {
        Ok(PublicDirIterator::for_path(&self.service_root)?)
    }

    pub fn accept_file_visitor(&self, file_name: &str, visitor: &impl FileVisitor) {
        let dir_iterator = self.create_iterator().unwrap();

        for dir_result in dir_iterator {
            let dir_entry = dir_result.unwrap();
            let file_path = dir_entry.path().join(file_name);

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

    fn create_cmd(&self, program: &str, args: &[String]) -> Result<Command, Error> {
        let mut command = Command::new(program);
        command.args(args);
        Ok(command)
    }


    pub fn run_command(&self, mut cmd: Command) -> io::Result<()>{
        let dir_iterator = self.create_iterator().unwrap();
        for dir_result in dir_iterator {
            let dir_entry = dir_result?;
            let output = cmd.current_dir(&dir_entry.path()).output()?;
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        };
        Ok(())
    }

    pub fn run_program(&self, program: &str, args: &[String]) -> Result<(), Error> {
        let cmd = self.create_cmd(program, args)?;
        self.run_command(cmd).unwrap();
        Ok(())
    }
}
