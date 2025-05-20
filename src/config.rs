use std::path::PathBuf;
use std::{env, fs};

use anyhow::{Result, bail};
use rand::{Rng, distr::Alphanumeric};

use crate::Repo;

#[derive(Debug)]
pub struct Config {
    pub work: PathBuf,
    pub bootstrap: PathBuf,
    pub repo: Repo,
}

impl Config {
    pub fn new() -> Result<Self> {
        let home = Self::env("HOME").unwrap();
        let name = "yadm-rs";

        // TODO: remove extra join when I really wanna use it
        let work = Self::env("YADM_RS_WORK").unwrap_or_else(|| home.clone().join(name));
        let config = Self::env("YADM_RS_CONFIG")
            .or_else(|| Self::env("XDG_CONFIG_HOME").map(|path| path.join(name)))
            .unwrap_or_else(|| home.clone().join(".config").join(name));
        let data = Self::env("YADM_RS_DATA")
            .or_else(|| Self::env("XDG_DATA_HOME").map(|path| path.join(name)))
            .unwrap_or_else(|| home.clone().join(".local/share").join(name));

        if !work.exists() {
            bail!("work tree does not exist: {}", work.display());
        }
        if !config.exists() {
            fs::create_dir_all(&config)?;
        }
        if !data.exists() {
            fs::create_dir_all(&data)?;
        }

        Ok(Self {
            work,
            bootstrap: config.join("bootstrap"),
            repo: Repo::new(data, "repo.git"),
        })
    }

    fn env(key: &str) -> Option<PathBuf> {
        env::var(key).map(PathBuf::from).ok()
    }

    pub fn temp(&self) -> PathBuf {
        let data = self.repo.root.parent().unwrap();
        let suffix: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();
        data.join(format!("tmp.{suffix}"))
    }
}
