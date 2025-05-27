use anyhow::Result;
use clap::Parser;

use crate::{Commands, State};

pub trait Task {
    fn run(&self, state: &State) -> Result<()>;
}

#[derive(Debug, Parser)]
#[command(version)]
/// Manage dotfiles maintained in a Git repository
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Task for Cli {
    fn run(&self, state: &State) -> Result<()> {
        self.command.run(state)
    }
}
