use anyhow::Result;
use clap::Parser;

use crate::{Commands, Config};

pub trait Task {
    fn run(&self, config: &Config) -> Result<()>;
}

#[derive(Debug, Parser)]
#[command(version)]
/// Manage dotfiles maintained in a Git repository
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Task for Cli {
    fn run(&self, config: &Config) -> Result<()> {
        self.command.run(config)
    }
}
