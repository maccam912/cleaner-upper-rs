use anyhow::{Context, Result};
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Clean up the specified directories
pub fn cleanup_directories(directories: Vec<PathBuf>) -> Result<usize> {
    let cleaned_count = AtomicUsize::new(0);

    directories.par_iter().for_each(|dir| {
        match fs::remove_dir_all(dir) {
            Ok(_) => {
                println!("Removed directory: {}", dir.display());
                cleaned_count.fetch_add(1, Ordering::Relaxed);
            }
            Err(e) => {
                eprintln!("Failed to remove directory {}: {}", dir.display(), e);
            }
        }
    });

    Ok(cleaned_count.load(Ordering::Relaxed))
}

/// Remove a single directory
pub fn remove_directory(path: &PathBuf) -> Result<()> {
    fs::remove_dir_all(path)
        .with_context(|| format!("Failed to remove directory: {}", path.display()))?;
    Ok(())
}