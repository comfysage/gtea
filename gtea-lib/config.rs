use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::util::constants;
use crate::util::filepath;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ConfigMain {
    pub branch: String,
}

impl Default for ConfigMain {
    fn default() -> Self {
        Self {
            branch: "main".to_string(),
        }
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ConfigNightly {
    pub branch: String,
    pub enable: bool,
}

impl Default for ConfigNightly {
    fn default() -> Self {
        Self {
            branch: "nightly".to_string(),
            enable: false,
        }
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ConfigFeature {
    pub prefix: String,
}

impl Default for ConfigFeature {
    fn default() -> Self {
        Self {
            prefix: "feature".to_string(),
        }
    }
}

#[derive(Debug, Default)]
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub main: ConfigMain,
    pub nightly: ConfigNightly,
    pub feature: ConfigFeature,
}

impl Config {
    pub fn path() -> Result<String> {
        let dir = &constants::CWD;
        let path = filepath::join(&dir, &*constants::CONFIG_NAME);
        return Ok(path);
    }
    pub fn new() -> Result<Self> {
        let path = Self::path()?;
        if let Some(content) = std::fs::read_to_string(path).ok() {
            let config: Self = toml::from_str(&content).unwrap_or(Self::default());
            Ok(config)
        } else {
            return Ok(Self::default());
        }
    }
    pub fn to_string(&self) -> Result<String> {
        toml::to_string(self).map_err(|_| make_err!(Parse, "couldn't create toml from string"))
    }
}
