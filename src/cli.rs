use anyhow::Result;
use clap::Parser;

use crate::cmd::Commands;

pub trait Task {
    fn run(&self) -> Result<()>;
}

#[derive(Debug, Parser)]
#[command(version)]
/// Manage dotfiles maintained in a Git repository
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Task for Cli {
    fn run(&self) -> Result<()> {
        // cargo run -- --help
        self.command.run()
    }
}
