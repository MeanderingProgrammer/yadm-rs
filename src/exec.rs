use std::process::Command;

use anyhow::{Result, bail};

#[derive(Debug)]
pub struct Exec {
    debug: bool,
}

impl Exec {
    pub fn new(debug: bool) -> Self {
        Self { debug }
    }

    pub fn run(&self, mut cmd: Command) -> Result<()> {
        println!("exec run: {:?}", cmd);
        if self.debug {
            Ok(())
        } else {
            let status = cmd.status()?;
            if status.success() {
                Ok(())
            } else {
                bail!("Command failed: {status}")
            }
        }
    }

    pub fn output(&self, mut cmd: Command) -> Result<Vec<String>> {
        println!("exec output: {:?}", cmd);
        if self.debug {
            Ok(Vec::default())
        } else {
            let output = cmd.output()?;
            let text = String::from_utf8(output.stdout)?;
            Ok(text.lines().map(String::from).collect())
        }
    }
}
