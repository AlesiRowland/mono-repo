//! # Cli Declaration.
//!
//! This module contains the declaration of the Cli, using clap.
use std::collections::VecDeque;
use clap::{Parser, Subcommand};

const AUTHOR: &str = "Alesi Rowland";
const VERSION: &str = "0.0.0";
const ABOUT: &str = "Mono repo helpers";

/// Top Level ClI struct.
#[derive(Parser)]
#[command(author = AUTHOR, version = VERSION, about = ABOUT)]
pub struct Cli {
    #[command(subcommand)]
    pub tool: Tools, // The CLI is split by the tools supported for edits.
}

/// Tools
#[derive(Subcommand)]
pub enum Tools {
    Poetry {
        #[command(subcommand)]
        command: PoetryCommands,
    },
    Run {
        program: String,
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
        #[arg(short, long)]
        service_root: Option<String>,
    },
    Rm {
        file_name: String,
        #[arg(short, long)]
        service_root: Option<String>,
    },
    Clean {
        #[arg(short, long)]
        service_root: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum PoetryCommands {
    Dep {
        #[command(subcommand)]
        command: DepCommands,
    },
}

#[derive(Subcommand, Clone)]
pub enum DepCommands {
    Update {
        package_name: String,
        version: String,
        #[arg(short, long)]
        service_root: Option<String>,
    },
    Rm {
        package_name: String,
        #[arg(short, long)]
        service_root: Option<String>,
    },
}
