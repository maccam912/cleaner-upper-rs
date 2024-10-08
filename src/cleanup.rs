use crate::config::CleanupPattern;
use crate::scanner::scan_directory;
use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Get a list of cache directories based on the current OS
fn get_cache_directories() -> Vec<PathBuf> {
    let dirs = Vec::new();
    let _home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_default();

    // TODO: Implement OS-specific cache directories
    // This is a placeholder for future implementation
    // We'll keep the '_home' variable for now as it will likely be used in the future

    dirs
}

/// Get cleanup patterns for non-cache directories
fn get_cleanup_patterns() -> Vec<CleanupPattern> {
    vec![
        CleanupPattern {
            target_dir: "node_modules".to_string(),
            indicator_file: "package.json".to_string(),
        },
        CleanupPattern {
            target_dir: "target".to_string(),
            indicator_file: "Cargo.toml".to_string(),
        },
        // Add more patterns as needed
    ]
}

/// Clean up the specified directories
pub fn cleanup_directories() -> Result<usize> {
    let cache_directories = get_cache_directories();
    let cleanup_patterns = get_cleanup_patterns();
    let cleaned_count = AtomicUsize::new(0);
    let errors = std::sync::Mutex::new(Vec::new());

    // Clean cache directories
    for dir in cache_directories {
        if dir.exists() {
            match remove_directory_contents(&dir) {
                Ok(_) => {
                    println!("Cleaned cache directory: {}", dir.display());
                    cleaned_count.fetch_add(1, Ordering::Relaxed);
                }
                Err(e) => {
                    let error_msg =
                        format!("Failed to clean cache directory {}: {}", dir.display(), e);
                    errors.lock().unwrap().push(error_msg);
                }
            }
        }
    }

    // Scan and clean directories based on patterns
    let root = PathBuf::from("/"); // Start from root directory
    match scan_directory(&root, &cleanup_patterns) {
        Ok(dirs_to_clean) => {
            for dir in dirs_to_clean {
                match remove_directory(&dir) {
                    Ok(_) => {
                        println!("Removed directory: {}", dir.display());
                        cleaned_count.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(e) => {
                        let error_msg =
                            format!("Failed to remove directory {}: {}", dir.display(), e);
                        errors.lock().unwrap().push(error_msg);
                    }
                }
            }
        }
        Err(e) => {
            errors
                .lock()
                .unwrap()
                .push(format!("Failed to scan directories: {}", e));
        }
    }

    // Print any errors that occurred
    let error_list = errors.into_inner().unwrap();
    for error in error_list {
        eprintln!("{}", error);
    }

    Ok(cleaned_count.load(Ordering::Relaxed))
}

/// Remove a single directory
pub fn remove_directory(path: &PathBuf) -> Result<()> {
    fs::remove_dir_all(path)
        .with_context(|| format!("Failed to remove directory: {}", path.display()))?;
    Ok(())
}

/// Remove contents of a directory without removing the directory itself
pub fn remove_directory_contents(dir: &Path) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}
