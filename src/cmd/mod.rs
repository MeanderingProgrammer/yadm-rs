pub mod bootstrap;
pub mod clone;

use anyhow::Result;
use clap::Subcommand;

use crate::{Config, Exec, Task};
use bootstrap::Bootstrap;
use clone::Clone;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Clone(Clone),
    Bootstrap(Bootstrap),
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
                Exec::run(&mut repo.cmd(args))
            }
        }
    }
}
