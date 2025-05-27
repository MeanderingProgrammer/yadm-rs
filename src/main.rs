mod cli;
mod cmd;
mod config;
mod exec;
mod repo;
mod state;

use std::process::Command;

use anyhow::{Result, bail};
use clap::Parser;

use cli::{Cli, Task};
use cmd::Commands;
use config::Config;
use exec::Exec;
use repo::Repo;
use state::State;

fn main() -> Result<()> {
    // cargo run -- --help

    // cargo run -- clone
    // cargo run -- clone --help
    // cargo run -- clone git@github.com:MeanderingProgrammer/dotfiles.git
    // cargo run -- clone --bootstrap git@github.com:MeanderingProgrammer/dotfiles.git

    // cargo run -- bootstrap
    // cargo run -- bootstrap --help

    let state = State::new()?;
    validate(&state.repo.program)?;
    let cli = Cli::parse();
    cli.run(&state)?;
    Ok(())
}

fn validate(program: &str) -> Result<()> {
    let mut cmd = Command::new(program);
    cmd.arg("--version");
    if cmd.output().is_ok() {
        Ok(())
    } else {
        bail!("git is required, command not found: {:?}", cmd)
    }
}
