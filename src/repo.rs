use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Result, bail};

#[derive(Debug)]
pub struct Repo {
    pub root: PathBuf,
    pub folder: String,
}

impl Repo {
    pub fn new(parent: PathBuf, folder: &str) -> Self {
        Self {
            root: parent.join(folder),
            folder: folder.into(),
        }
    }

    pub fn absent(&self) -> Result<()> {
        if self.root.exists() {
            bail!("Repo already exists: {}", self.root.display());
        } else {
            Ok(())
        }
    }

    pub fn require(&self) -> Result<()> {
        if !self.root.exists() {
            bail!("Repo does not exist: {}", self.root.display());
        } else {
            Ok(())
        }
    }

    pub fn cmd<S: Debug + AsRef<OsStr>>(&self, cwd: Option<&Path>, args: &[S]) -> Command {
        let mut cmd = Command::new("git");
        cmd.args(args);
        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }
        // equivalent to yadm using GIT_DIR=$(mixed_path "$YADM_REPO")
        cmd.env("GIT_DIR", self.root.to_str().unwrap());
        cmd
    }
}
