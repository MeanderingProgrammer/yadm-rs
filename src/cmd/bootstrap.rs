use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{Result, bail};
use clap::Parser;

use crate::{Config, Exec, Task};

#[derive(Debug, Default, Parser)]
/// Execute the bootstrap script
pub struct Bootstrap {
    /// Arguments passed to bootstrap script
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

impl Task for Bootstrap {
    fn run(&self, config: &Config) -> Result<()> {
        let bootstrap = &config.bootstrap;
        if !bootstrap.is_file() || !executable(bootstrap) {
            bail!("bootstrap is not executable: {}", bootstrap.display())
        }
        let mut command = Command::new(bootstrap);
        Exec::run(command.args(&self.args))
    }
}

fn executable(path: &PathBuf) -> bool {
    // TODO: probably needs to be improved
    if let Ok(metadata) = fs::metadata(path) {
        let mode = metadata.permissions().mode();
        mode & 0o111 != 0
    } else {
        false
    }
}
