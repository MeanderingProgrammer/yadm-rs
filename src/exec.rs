use std::process::Command;

use anyhow::{Result, bail};

const DEBUG: bool = false;

#[derive(Debug)]
pub struct Exec;

impl Exec {
    pub fn run(cmd: &mut Command) -> Result<()> {
        println!("exec run: {:?}", cmd);
        if DEBUG {
            Ok(())
        } else {
            let status = cmd.status()?;
            if status.success() {
                Ok(())
            } else {
                bail!("command failed: {status}")
            }
        }
    }

    pub fn output(cmd: &mut Command) -> Result<Vec<String>> {
        println!("exec output: {:?}", cmd);
        if DEBUG {
            Ok(Vec::default())
        } else {
            let output = cmd.output()?;
            let text = String::from_utf8(output.stdout)?;
            Ok(text.lines().map(String::from).collect())
        }
    }
}
