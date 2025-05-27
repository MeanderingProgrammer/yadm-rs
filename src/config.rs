use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::Result;
use serde::Deserialize;

use crate::Exec;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub alt: Alt,

    #[serde(default)]
    pub git: Git,
}

impl Config {
    pub fn new(path: &Path) -> Result<Self> {
        if path.is_file() {
            let text = fs::read_to_string(path)?;
            let result: Self = toml::from_str(&text)?;
            Ok(result)
        } else {
            Ok(Config::default())
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Alt {
    #[serde(default)]
    class: Vec<String>,

    #[serde(default = "Alt::default_arch")]
    arch: String,

    #[serde(default = "Alt::default_os")]
    os: String,

    #[serde(default = "Alt::default_hostname")]
    hostname: String,

    #[serde(default = "Alt::default_user")]
    user: String,
}

impl Default for Alt {
    fn default() -> Self {
        Self {
            class: Vec::default(),
            arch: Self::default_arch(),
            os: Self::default_os(),
            hostname: Self::default_hostname(),
            user: Self::default_user(),
        }
    }
}

impl Alt {
    fn default_arch() -> String {
        Self::first(Command::new("uname").arg("-m"))
    }

    fn default_os() -> String {
        // TODO: handle WSL
        Self::first(Command::new("uname").arg("-s"))
    }

    fn default_hostname() -> String {
        let result = Self::first(Command::new("uname").arg("-n"));
        match result.split_once('.') {
            Some((prefix, _)) => prefix.to_string(),
            None => result,
        }
    }

    fn default_user() -> String {
        Self::first(Command::new("id").arg("-u").arg("-n"))
    }

    fn first(cmd: &mut Command) -> String {
        let lines = Exec::output(cmd).unwrap();
        lines[0].clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct Git {
    #[serde(default = "Git::default_program")]
    pub program: String,
}

impl Default for Git {
    fn default() -> Self {
        Self {
            program: Self::default_program(),
        }
    }
}

impl Git {
    fn default_program() -> String {
        "git".into()
    }
}
