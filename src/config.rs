use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_tick_rate")]
    pub tick_rate_ms: u64,

    #[serde(default)]
    pub git: GitConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GitConfig {
    #[serde(default = "default_git_interval")]
    pub interval_secs: u64,

    #[serde(default)]
    pub repos: Vec<String>,
}

fn default_tick_rate() -> u64 {
    250
}

fn default_git_interval() -> u64 {
    5
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tick_rate_ms: default_tick_rate(),
            git: GitConfig::default(),
        }
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            interval_secs: default_git_interval(),
            repos: vec![],
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> crate::errors::Result<Self> {
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }
}
