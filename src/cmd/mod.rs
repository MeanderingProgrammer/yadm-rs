pub mod bootstrap;
pub mod clone;

use anyhow::Result;
use clap::Subcommand;

use crate::{Config, Task};

#[derive(Debug, Subcommand)]
pub enum Commands {
    Clone(clone::Clone),
    Bootstrap(bootstrap::Bootstrap),
    #[command(external_subcommand)]
    Git(Vec<String>),
}

impl Task for Commands {
    fn run(&self, config: &Config) -> Result<()> {
        match self {
            Self::Clone(task) => task.run(config),
            Self::Bootstrap(task) => task.run(config),
            Self::Git(args) => {
                let repo = &config.repo;
                repo.require()?;
                config.exec.run(repo.cmd(None, args))
            }
        }
    }
}
