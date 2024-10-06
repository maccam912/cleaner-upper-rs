use crate::config::CleanupPattern;
use anyhow::Result;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Scans the directory for patterns that match the cleanup criteria
pub fn scan_directory(root: &Path, patterns: &[CleanupPattern]) -> Result<Vec<PathBuf>> {
    let directories_to_clean: Vec<PathBuf> = WalkDir::new(root)
        .into_iter()
        .par_bridge()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_dir())
        .filter_map(|dir| {
            patterns.iter().find_map(|pattern| {
                if dir.file_name().to_str() == Some(&pattern.target_dir) {
                    let parent = dir.path().parent()?;
                    let indicator_file = parent.join(&pattern.indicator_file);
                    if indicator_file.exists() {
                        Some(dir.path().to_path_buf())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .collect();

    Ok(directories_to_clean)
}