use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::{GumroadError, Result};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub access_token: Option<String>,
}

impl Config {
    pub fn path() -> Result<PathBuf> {
        let dir = dirs::config_dir()
            .ok_or_else(|| GumroadError::Config("cannot determine config directory".into()))?
            .join("gumroad");
        Ok(dir.join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string_pretty(self)?;
        fs::write(&path, contents)?;
        Ok(())
    }
}

/// Resolve access token: CLI flag > env var > config file.
pub fn resolve_token(flag: Option<&str>) -> Result<String> {
    if let Some(token) = flag {
        return Ok(token.to_string());
    }
    if let Ok(token) = std::env::var("GUMROAD_ACCESS_TOKEN")
        && !token.is_empty()
    {
        return Ok(token);
    }
    let config = Config::load()?;
    config.access_token.ok_or(GumroadError::NoToken)
}
