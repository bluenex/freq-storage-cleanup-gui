use std::collections::HashSet;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use jwalk::{Parallelism, WalkDir};

/// Allocated size like `du`: counts blocks, not file length, so sparse files
/// and APFS clones don't inflate it. Hard links are counted once.
/// Serial, because many parallel walks on rayon's shared pool abort with "thread-pool busy"
/// and silently undercount; the caller already measures items on separate threads.
pub fn measure_dir(path: &Path) -> u64 {
    let mut seen = HashSet::new();
    let mut total = 0;
    for entry in WalkDir::new(path)
        .skip_hidden(false)
        .follow_links(false)
        .parallelism(Parallelism::Serial)
        .into_iter()
        .flatten()
    {
        let Ok(meta) = entry.metadata() else { continue };
        if meta.nlink() > 1 && !meta.is_dir() && !seen.insert((meta.dev(), meta.ino())) {
            continue;
        }
        total += meta.blocks() * 512;
    }
    total
}
