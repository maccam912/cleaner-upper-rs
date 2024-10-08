//! Cleaner-Upper-RS
//!
//! A fast, cross-platform file system cleaner for removing recreatable folders and cache directories.
//! This program cleans common cache locations across different operating systems.
//!
//! # Usage
//!
//! Run the executable without any arguments to clean common cache directories:
//!
//! ```
//! cleaner-upper-rs
//! ```
//!
//! The program will automatically detect the operating system and clean relevant cache directories.
//!
//! Note: Use this tool with caution. Always ensure you have backups of important data before running cleanup operations.
//!
//! Future feature: Specify additional directories to clean using the `--additional-dirs` (or `-a`) option:
//!
//! ```
//! cleaner-upper-rs --additional-dirs /path/to/clean /another/path/to/clean
//! ```
//!
//! This feature is not yet implemented.

mod cleanup;
mod config;
mod scanner;

use anyhow::{Context, Result};
use clap::Parser;
use cleanup::cleanup_directories;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional: Specify additional directories to clean
    #[arg(short, long, value_name = "DIR")]
    additional_dirs: Vec<PathBuf>,
}

/// The main entry point of the program.
///
/// This function parses command-line arguments, performs the cleanup of common cache directories,
/// and reports the results.
fn main() -> Result<()> {
    let args = Args::parse();

    println!("Starting cleanup process...");

    // Perform cleanup of common cache directories and pattern-based directories
    let cleaned_count = cleanup_directories().context("Failed to clean directories")?;

    println!("Successfully cleaned {} directories", cleaned_count);

    // TODO: Handle additional directories specified by the user
    if !args.additional_dirs.is_empty() {
        println!("Cleaning additional specified directories is not yet implemented.");
        println!("Specified directories: {:?}", args.additional_dirs);
    }

    Ok(())
}
