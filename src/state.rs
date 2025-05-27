use std::path::PathBuf;
use std::{env, fs};

use anyhow::{Result, bail};
use directories::BaseDirs;
use rand::{Rng, distr::Alphanumeric};

use crate::{Config, Repo};

#[derive(Debug)]
pub struct State {
    pub work: PathBuf,
    pub bootstrap: PathBuf,
    pub repo: Repo,
}

impl State {
    pub fn new() -> Result<Self> {
        let base = BaseDirs::new().unwrap();
        let name = "yadm-rs";

        // TODO: remove extra join when I really wanna use it
        let work = Self::env("YADM_RS_WORK").unwrap_or_else(|| base.home_dir().join(name));
        if !work.exists() {
            bail!("work tree does not exist: {}", work.display());
        }
        let config = Self::env("YADM_RS_CONFIG")
            .or_else(|| Self::env("XDG_CONFIG_HOME").map(|path| path.join(name)))
            .unwrap_or_else(|| base.config_dir().join(name));
        if !config.exists() {
            fs::create_dir_all(&config)?;
        }
        let data = Self::env("YADM_RS_DATA")
            .or_else(|| Self::env("XDG_DATA_HOME").map(|path| path.join(name)))
            .unwrap_or_else(|| base.data_dir().join(name));
        if !data.exists() {
            fs::create_dir_all(&data)?;
        }

        let cfg = Config::new(&config.join("config.toml"))?;

        Ok(Self {
            work,
            bootstrap: config.join("bootstrap"),
            repo: Repo::new(&cfg.git.program, data, "repo.git"),
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
