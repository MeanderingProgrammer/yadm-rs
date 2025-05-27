use anyhow::Result;
use clap::Parser;

use crate::{State, Task};

#[derive(Debug, Parser)]
/// Create links for alternates
pub struct Alt {}

impl Task for Alt {
    fn run(&self, state: &State) -> Result<()> {
        let repo = &state.repo;
        repo.require()?;

        Ok(())
    }
}
