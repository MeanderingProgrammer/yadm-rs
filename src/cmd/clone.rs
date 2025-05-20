use anyhow::Result;
use clap::Parser;

use crate::Task;

#[derive(Debug, Parser)]
/// Clone an existing repository
pub struct Clone {
    /// Repository to clone
    url: String,

    /// Execute the bootstrap script
    #[arg(short, long)]
    bootstrap: bool,
}

impl Task for Clone {
    fn run(&self) -> Result<()> {
        // cargo run -- clone
        // cargo run -- clone --help
        // cargo run -- clone git@github.com:MeanderingProgrammer/dotfiles.git
        // cargo run -- clone --bootstrap git@github.com:MeanderingProgrammer/dotfiles.git
        println!("CLONE : {:?}", self);
        Ok(())
    }
}
