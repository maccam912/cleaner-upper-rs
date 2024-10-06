use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Configuration for the cleanup operation
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub root_dir: PathBuf,
    pub patterns: Vec<CleanupPattern>,
}

/// Pattern to match for cleanup
#[derive(Debug, Serialize, Deserialize)]
pub struct CleanupPattern {
    pub target_dir: String,
    pub indicator_file: String,
}

impl Config {
    /// Load configuration from a JSON file
    pub fn load() -> Result<Self> {
        let config_str = fs::read_to_string("config.json")
            .context("Failed to read config.json")?;
        let config: Config = serde_json::from_str(&config_str)
            .context("Failed to parse config.json")?;
        Ok(config)
    }
}