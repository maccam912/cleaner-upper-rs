use crate::config::CleanupPattern;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// Checks if the directory name is "cache" (case-insensitive)
fn is_cache_directory(dir_name: &str) -> bool {
    dir_name.to_lowercase() == "cache"
}

/// Deletes the contents of a directory
fn delete_directory_contents(dir: &Path) -> Result<()> {
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

/// Scans the directory for patterns that match the cleanup criteria and cache directories
pub fn scan_directory(root: &Path, patterns: &[CleanupPattern]) -> Result<Vec<PathBuf>> {
    let mut directories_to_clean: Vec<PathBuf> = Vec::new();

    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            let dir_name = entry.file_name().to_str().unwrap_or("");

            if is_cache_directory(dir_name) {
                if let Err(e) = delete_directory_contents(entry.path()) {
                    eprintln!(
                        "Error deleting contents of cache directory {}: {}",
                        entry.path().display(),
                        e
                    );
                }
            } else if let Some(pattern) = patterns.iter().find(|p| dir_name == p.target_dir) {
                let parent = entry.path().parent().unwrap_or(Path::new(""));
                let indicator_file = parent.join(&pattern.indicator_file);
                if indicator_file.exists() {
                    directories_to_clean.push(entry.path().to_path_buf());
                }
            }
        }
    }

    Ok(directories_to_clean)
}
