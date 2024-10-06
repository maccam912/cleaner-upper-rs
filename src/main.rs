//! Cleaner-Upper-RS
//! 
//! A fast, cross-platform file system cleaner for removing recreatable folders.
//! This program scans a specified directory for patterns of folders that can be safely
//! removed to save space, such as virtual environments, build artifacts, and dependency caches.

mod config;
mod scanner;
mod cleanup;

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use config::Config;
use scanner::scan_directory;
use cleanup::cleanup_directories;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The root directory to start scanning from
    #[arg(short, long, value_name = "DIR")]
    root_dir: PathBuf,
}

/// The main entry point of the program.
/// 
/// This function parses command-line arguments, creates the configuration,
/// scans for directories to clean, performs the cleanup, and reports the results.
fn main() -> Result<()> {
    let args = Args::parse();
    
    // Create configuration
    let config = Config::new(args.root_dir);
    
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

1. Run the executable with the `--root-dir` (or `-r`) option to specify the directory to start scanning from.

Example:
```
cleaner-upper-rs --root-dir /path/to/scan
```

The program will scan the specified root directory and its subdirectories,
removing any folders that match the predefined patterns.

Predefined patterns include:
- Removing "node_modules" directories when a "package.json" file is present
- Removing "target" directories when a "Cargo.toml" file is present

Note: Use this tool with caution. Always ensure you have backups of important data before running cleanup operations.
"#]
struct Usage;
