use std::path::PathBuf;

/// Configuration for the cleanup operation
#[derive(Debug)]
pub struct Config {
    pub root_dir: PathBuf,
    pub patterns: Vec<CleanupPattern>,
}

/// Pattern to match for cleanup
#[derive(Debug)]
pub struct CleanupPattern {
    pub target_dir: String,
    pub indicator_file: String,
}

impl Config {
    /// Create a new Config instance with hardcoded patterns
    pub fn new(root_dir: PathBuf) -> Self {
        Config {
            root_dir,
            patterns: vec![
                CleanupPattern {
                    target_dir: "node_modules".to_string(),
                    indicator_file: "package.json".to_string(),
                },
                CleanupPattern {
                    target_dir: "target".to_string(),
                    indicator_file: "Cargo.toml".to_string(),
                },
                CleanupPattern {
                    target_dir: ".venv".to_string(),
                    indicator_file: "pyproject.toml".to_string(),
                },
                CleanupPattern {
                    target_dir: ".pixi".to_string(),
                    indicator_file: "pyproject.toml".to_string(),
                },
                CleanupPattern {
                    target_dir: ".pixi".to_string(),
                    indicator_file: "pixi.toml".to_string(),
                },
                // Add more patterns here as needed
            ],
        }
    }
}