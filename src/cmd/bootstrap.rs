use anyhow::Result;
use clap::Parser;

use crate::Task;

#[derive(Debug, Parser)]
/// Execute the bootstrap script
pub struct Bootstrap {}

impl Task for Bootstrap {
    fn run(&self) -> Result<()> {
        // cargo run -- bootstrap
        // cargo run -- bootstrap --help
        println!("BOOTSTRAP : {:?}", self);
        Ok(())
    }
}
