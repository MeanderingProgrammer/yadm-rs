use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
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
