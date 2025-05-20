use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use anyhow::{Result, bail};
use clap::Parser;

use crate::{Config, Task};

#[derive(Debug, Parser)]
/// Execute the bootstrap script
pub struct Bootstrap {}

impl Task for Bootstrap {
    fn run(&self, config: &Config) -> Result<()> {
        let bootstrap = &config.bootstrap;
        if !bootstrap.is_file() || !executable(bootstrap) {
            bail!("bootstrap is not executable: {}", bootstrap.display())
        }
        // TODO: actually execute it
        Ok(())
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
