//! # Dead fast assistance for monotonous mono-repo tasks.
mod cli;
mod execution;
mod file_system;
mod visitors;

use clap::Parser;

use crate::cli::Cli;
use crate::execution::Executable;

fn main() {
    Cli::parse().execute();
}
