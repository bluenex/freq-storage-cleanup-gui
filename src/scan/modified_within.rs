use std::path::Path;
use std::time::{Duration, SystemTime};

/// Whether the path's mtime is less than `days` days old.
pub fn modified_within(path: &Path, days: u64) -> bool {
    let Ok(modified) = path.symlink_metadata().and_then(|m| m.modified()) else {
        return false;
    };
    SystemTime::now()
        .duration_since(modified)
        .is_ok_and(|age| age < Duration::from_secs(days * 24 * 60 * 60))
}
