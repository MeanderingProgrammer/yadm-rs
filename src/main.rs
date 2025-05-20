mod cli;
mod cmd;
mod config;
mod exec;
mod repo;

use std::process::Command;

use anyhow::{Result, bail};
use clap::Parser;

use cli::{Cli, Task};
use cmd::Commands;
use config::Config;
use exec::Exec;
use repo::Repo;

fn main() -> Result<()> {
    // cargo run -- --help

    // cargo run -- clone
    // cargo run -- clone --help
    // cargo run -- clone git@github.com:MeanderingProgrammer/dotfiles.git
    // cargo run -- clone --bootstrap git@github.com:MeanderingProgrammer/dotfiles.git

    // cargo run -- bootstrap
    // cargo run -- bootstrap --help

    validate()?;
    let cli = Cli::parse();
    let config = Config::new()?;
    cli.run(&config)?;
    Ok(())
}

fn validate() -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("--version");
    if cmd.output().is_ok() {
        Ok(())
    } else {
        bail!("git is required, command not found: {:?}", cmd)
    }
}
