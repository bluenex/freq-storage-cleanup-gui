use std::fs;
use std::path::Path;

use anyhow::{Result, bail};
use glob::Pattern;

use crate::rules::{GLOB_OPTIONS, Rule, home, pattern_root};

/// Deletes `path` only if it still matches one of the rule's patterns, sits
/// strictly under that pattern's root, and has no symlink anywhere in it.
pub fn safe_remove(path: &Path, rule: &Rule) -> Result<()> {
    let meta = fs::symlink_metadata(path)?;
    if !meta.is_dir() {
        bail!("not a directory");
    }
    // A symlinked parent makes the canonical path differ from the matched one.
    let canonical = fs::canonicalize(path)?;
    if canonical != path {
        bail!("resolves to {}", canonical.display());
    }

    let home = home();
    let allowed = rule.patterns.iter().any(|p| {
        let full = home.join(p);
        let root = pattern_root(&full);
        let matches = Pattern::new(&full.to_string_lossy()).is_ok_and(|pat| pat.matches_path_with(path, GLOB_OPTIONS));
        matches && path.starts_with(&root) && path != root
    });
    if !allowed {
        bail!("does not match the rule \"{}\"", rule.label);
    }

    // `remove_dir_all` removes symlinks inside the tree without following them.
    fs::remove_dir_all(path)?;
    Ok(())
}
