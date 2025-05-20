pub mod bootstrap;
pub mod clone;

use std::process::{Command, Stdio};

use anyhow::{Result, bail};
use clap::Subcommand;

use crate::Task;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Clone(clone::Clone),
    Bootstrap(bootstrap::Bootstrap),
    #[command(external_subcommand)]
    Git(Vec<String>),
}

impl Task for Commands {
    fn run(&self) -> Result<()> {
        match self {
            Self::Clone(cmd) => cmd.run(),
            Self::Bootstrap(cmd) => cmd.run(),
            Self::Git(args) => git(args),
        }
    }
}

fn git(args: &[String]) -> Result<()> {
    // cargo run -- stat
    // cargo run -- status
    println!("GIT ARGS : {:?}", args);
    let status = Command::new("git")
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;
    if status.success() {
        Ok(())
    } else {
        bail!("Git command failed '{status}'")
    }
}
