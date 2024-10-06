//! Cleaner-Upper-RS
//! 
//! A fast, cross-platform file system cleaner for removing recreatable folders.
//! This program scans a specified directory for patterns of folders that can be safely
//! removed to save space, such as virtual environments, build artifacts, and dependency caches.

mod config;
mod scanner;
mod cleanup;

use anyhow::{Context, Result};
use config::Config;
use scanner::scan_directory;
use cleanup::cleanup_directories;

/// The main entry point of the program.
/// 
/// This function loads the configuration, scans for directories to clean,
/// performs the cleanup, and reports the results.
fn main() -> Result<()> {
    // Load configuration
    let config = Config::load().context("Failed to load configuration")?;
    
    println!("Scanning directory: {}", config.root_dir.display());
    
    // Scan for directories to clean
    let directories_to_clean = scan_directory(&config.root_dir, &config.patterns)
        .context("Failed to scan directories")?;
    
    println!("Found {} directories to clean", directories_to_clean.len());
    
    // Perform cleanup
    let cleaned_count = cleanup_directories(directories_to_clean)
        .context("Failed to clean directories")?;
    
    println!("Successfully cleaned {} directories", cleaned_count);
    
    Ok(())
}

// Usage instructions
#[doc = r#"
# Usage

1. Ensure you have a `config.json` file in the same directory as the executable.
2. The `config.json` file should contain:
   - `root_dir`: The directory to start scanning from.
   - `patterns`: An array of cleanup patterns, each with:
     - `target_dir`: The name of the directory to remove.
     - `indicator_file`: A file that must be present in the parent directory.

Example `config.json`:
```json
{
    "root_dir": ".",
    "patterns": [
        {
            "target_dir": ".venv",
            "indicator_file": "pyproject.toml"
        },
        {
            "target_dir": "node_modules",
            "indicator_file": "package.json"
        }
    ]
}
```

3. Run the executable. It will scan the specified root directory and its subdirectories,
   removing any folders that match the patterns and have the corresponding indicator file.

Note: Use this tool with caution. Always ensure you have backups of important data before running cleanup operations.
"#]
struct Usage;
