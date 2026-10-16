use std::path::{Path, PathBuf};

/// The literal directory a pattern lives under: the part before the first glob
/// component, or the parent for a fully literal pattern.
pub fn pattern_root(pattern: &Path) -> PathBuf {
    let mut root = PathBuf::new();
    for part in pattern.components() {
        if part.as_os_str().to_string_lossy().contains(['*', '?', '[']) {
            return root;
        }
        root.push(part);
    }
    root.parent().map(Path::to_path_buf).unwrap_or(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stops_at_first_glob() {
        assert_eq!(
            pattern_root(Path::new("/h/orca/workspaces/*/*/.next")),
            Path::new("/h/orca/workspaces")
        );
    }

    #[test]
    fn literal_pattern_uses_parent() {
        assert_eq!(
            pattern_root(Path::new("/h/Library/Caches/Yarn")),
            Path::new("/h/Library/Caches")
        );
    }
}
