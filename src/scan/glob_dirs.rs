use std::path::{Path, PathBuf};

use crate::rules::GLOB_OPTIONS;

/// Real directories (not symlinks) that match `pattern` under `home`.
pub fn glob_dirs(home: &Path, pattern: &str) -> Vec<PathBuf> {
    let full = home.join(pattern);
    let Ok(paths) = glob::glob_with(&full.to_string_lossy(), GLOB_OPTIONS) else {
        return Vec::new();
    };
    paths
        .flatten()
        .filter(|p| p.symlink_metadata().is_ok_and(|m| m.is_dir()))
        .collect()
}
