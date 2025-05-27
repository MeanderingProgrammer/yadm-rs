use anyhow::Result;
use clap::Parser;

use crate::{Exec, State, Task};

#[derive(Debug, Parser)]
/// List tracked files
pub struct List {
    /// Include all managed files
    #[arg(short, long)]
    all: bool,
}

impl Task for List {
    fn run(&self, state: &State) -> Result<()> {
        let work = &state.work;
        let repo = &state.repo;
        repo.require()?;

        let mut list = repo.cmd(&["ls-files"]);
        if self.all {
            list.current_dir(work);
        }
        Exec::run(&mut list)
    }
}
