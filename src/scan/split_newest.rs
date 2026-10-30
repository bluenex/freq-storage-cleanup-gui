use std::path::PathBuf;

/// The most recently modified path, and all the others.
pub fn split_newest(mut paths: Vec<PathBuf>) -> (Option<PathBuf>, Vec<PathBuf>) {
    let newest = paths
        .iter()
        .enumerate()
        .max_by_key(|(_, p)| p.symlink_metadata().and_then(|m| m.modified()).ok())
        .map(|(i, _)| i);
    (newest.map(|i| paths.remove(i)), paths)
}
