use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{Result, bail};

#[derive(Debug)]
pub struct Repo {
    pub program: String,
    pub root: PathBuf,
    pub folder: String,
}

impl Repo {
    pub fn new(program: &str, parent: PathBuf, folder: &str) -> Self {
        Self {
            program: program.into(),
            root: parent.join(folder),
            folder: folder.into(),
        }
    }

    pub fn absent(&self) -> Result<()> {
        if self.root.exists() {
            bail!("repo already exists: {}", self.root.display());
        } else {
            Ok(())
        }
    }

    pub fn require(&self) -> Result<()> {
        if !self.root.exists() {
            bail!("repo does not exist: {}", self.root.display());
        } else {
            Ok(())
        }
    }

    pub fn cmd<S: AsRef<OsStr>>(&self, args: &[S]) -> Command {
        let mut cmd = Command::new(&self.program);
        cmd.args(args);
        // yadm uses GIT_DIR=$(mixed_path "$YADM_REPO")
        cmd.env("GIT_DIR", &self.root);
        cmd
    }
}
