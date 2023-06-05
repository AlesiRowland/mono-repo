//! # Execution logic for the Cli.
//!
//! Applies the Executor trait to the structs/enums of the CLI.
use std::env;
use std::path::Path;

use crate::cli::{Cli, DepCommands, PoetryCommands, Tools};
use crate::file_system::files::TMP;
use crate::file_system::services::ServiceEditor;
use crate::visitors::crud::FileRemover;
use crate::visitors::poetry::{StringReplacer, PYPROJECT_TOML};

fn get_service_root(service_root: &Option<String>) -> Box<Path> {
    match service_root {
        Some(root) => Path::new(&root).to_path_buf().into_boxed_path(),
        None => env::current_dir().unwrap().into_boxed_path(),
    }
}

fn execute_clean(service_root: &Option<String>) {
    let service_root = get_service_root(service_root);
    let visitor = FileRemover;
    let service_editor = ServiceEditor::new(service_root, TMP.to_string());
    service_editor.accept_command(&visitor);
}

fn execute_update(package_name: &str, version: &str, service_root: &Option<String>) {
    let visitor = StringReplacer::package_version_editor(package_name.into(), version.into())
        .expect("Could not compile regex for editing version.");

    let service_root = get_service_root(service_root);
    let service_editor = ServiceEditor::new(service_root, PYPROJECT_TOML.to_string());
    service_editor.accept_command(&visitor);
}

fn execute_rm(package_name: &str, service_root: &Option<String>) {
    let visitor = StringReplacer::package_remover(package_name.into())
        .expect("Could not compile regex for removing package.");
    let service_root = get_service_root(service_root);
    let service_editor = ServiceEditor::new(service_root, PYPROJECT_TOML.to_string());
    service_editor.accept_command(&visitor);
}

pub trait Executable {
    fn execute(&self);
}

impl Executable for Cli {
    fn execute(&self) {
        self.tool.execute()
    }
}

impl Executable for Tools {
    fn execute(&self) {
        match self {
            Tools::Poetry { command } => command.execute(),
            Tools::Clean { service_root } => execute_clean(service_root),
        }
    }
}

impl Executable for PoetryCommands {
    fn execute(&self) {
        match self {
            PoetryCommands::Dep { command } => command.execute(),
        }
    }
}

impl Executable for DepCommands {
    fn execute(&self) {
        match self {
            DepCommands::Update {
                package_name,
                version,
                service_root,
            } => execute_update(package_name, version, service_root),
            DepCommands::Rm {
                package_name,
                service_root,
            } => execute_rm(package_name, service_root),
        };
    }
}
