use std::path::PathBuf;

/// All paths except the most recently modified one.
pub fn without_newest(mut paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let newest = paths
        .iter()
        .enumerate()
        .max_by_key(|(_, p)| p.symlink_metadata().and_then(|m| m.modified()).ok())
        .map(|(i, _)| i);
    if let Some(i) = newest {
        paths.remove(i);
    }
    paths
}
