mod cli;
mod cmd;

use std::process::Command;

use anyhow::{Result, bail};
use clap::Parser;

use crate::cli::Task;

fn main() -> Result<()> {
    require_git()?;
    let cli = cli::Cli::parse();
    println!("{:?}", cli);
    cli.run()?;
    Ok(())
}

fn require_git() -> Result<()> {
    let cmd = "git";
    if Command::new(cmd).arg("--version").output().is_ok() {
        Ok(())
    } else {
        bail!("Git is required, command '{cmd}' cannot be located.")
    }
}
