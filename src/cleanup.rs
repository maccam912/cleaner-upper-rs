use anyhow::{Context, Result};
use rayon::prelude::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Determine the current operating system
fn get_os() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    }
}

/// Get a list of cache directories based on the current OS
fn get_cache_directories() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_default();

    match get_os() {
        "windows" => {
            dirs.push(PathBuf::from(env::var("LOCALAPPDATA").unwrap_or_default()).join("Temp"));
            dirs.push(PathBuf::from(env::var("TEMP").unwrap_or_default()));
            dirs.push(PathBuf::from(env::var("WINDIR").unwrap_or_default()).join("Temp"));
            dirs.push(
                PathBuf::from(&home)
                    .join("AppData\\Local\\Microsoft\\Windows\\Temporary Internet Files"),
            );
            dirs.push(PathBuf::from(&home).join("AppData\\Local\\Microsoft\\Windows\\INetCache"));
            dirs.push(
                PathBuf::from(&home)
                    .join("AppData\\Local\\Google\\Chrome\\User Data\\Default\\Cache"),
            );
            dirs.push(PathBuf::from(&home).join("AppData\\Local\\Mozilla\\Firefox\\Profiles"));
        }
        "macos" => {
            dirs.push(PathBuf::from(&home).join("Library/Caches"));
            dirs.push(PathBuf::from("/Library/Caches"));
            dirs.push(PathBuf::from(&home).join("Library/Safari/Cache.db"));
            dirs.push(PathBuf::from(&home).join("Library/Caches/com.apple.Safari"));
            dirs.push(PathBuf::from(&home).join("Library/Caches/Google/Chrome/Default/Cache"));
            dirs.push(PathBuf::from(&home).join("Library/Caches/Firefox/Profiles"));
        }
        "linux" => {
            dirs.push(PathBuf::from(&home).join(".cache"));
            dirs.push(PathBuf::from("/tmp"));
            dirs.push(PathBuf::from("/var/tmp"));
            dirs.push(PathBuf::from("/var/cache"));
            dirs.push(PathBuf::from(&home).join(".mozilla/firefox"));
            dirs.push(PathBuf::from(&home).join(".config/google-chrome/Default/Cache"));
            dirs.push(PathBuf::from(&home).join(".config/chromium/Default/Cache"));
            dirs.push(PathBuf::from(&home).join(".thumbnails"));
            dirs.push(PathBuf::from(&home).join(".local/share/Trash"));
        }
        _ => {}
    }

    // Cross-platform caches
    dirs.push(PathBuf::from(&home).join(".npm"));
    dirs.push(PathBuf::from(&home).join(".gradle/caches"));
    dirs.push(PathBuf::from(&home).join(".m2/repository"));
    dirs.push(PathBuf::from(&home).join(".cargo/registry"));
    dirs.push(PathBuf::from(&home).join(".pip/cache"));

    dirs
}

/// Clean up the specified directories
pub fn cleanup_directories() -> Result<usize> {
    let directories = get_cache_directories();
    let cleaned_count = AtomicUsize::new(0);
    let errors = std::sync::Mutex::new(Vec::new());

    directories.par_iter().for_each(|dir| {
        if dir.exists() {
            match fs::remove_dir_all(dir) {
                Ok(_) => {
                    println!("Removed directory: {}", dir.display());
                    cleaned_count.fetch_add(1, Ordering::Relaxed);
                }
                Err(e) => {
                    let error_msg = format!("Failed to remove directory {}: {}", dir.display(), e);
                    errors.lock().unwrap().push(error_msg);
                }
            }
        }
    });

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
