use std::path::PathBuf;

use crate::rules::{GLOB_OPTIONS, resolve_pattern};

/// Real directories (not symlinks) that match the rule pattern.
pub fn glob_dirs(pattern: &str) -> Vec<PathBuf> {
    let Some(full) = resolve_pattern(pattern) else {
        return Vec::new();
    };
    let Ok(paths) = glob::glob_with(&full.to_string_lossy(), GLOB_OPTIONS) else {
        return Vec::new();
    };
    paths
        .flatten()
        .filter(|p| p.symlink_metadata().is_ok_and(|m| m.is_dir()))
        .collect()
}
