pub mod bootstrap;
pub mod clone;

use anyhow::Result;
use clap::Subcommand;

use crate::{Exec, State, Task};
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
    fn run(&self, state: &State) -> Result<()> {
        match self {
            Self::Clone(task) => task.run(state),
            Self::Bootstrap(task) => task.run(state),
            Self::Git(args) => {
                let repo = &state.repo;
                repo.require()?;
                Exec::run(&mut repo.cmd(args))
            }
        }
    }
}
