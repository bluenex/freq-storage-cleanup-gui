use std::path::PathBuf;

use super::home::home;
use super::user_var::user_var;

/// The absolute glob for a rule pattern. Patterns start with `~/` (`$HOME`),
/// `$USER_VAR/` (see `user_var`) or `/`.
pub fn resolve_pattern(pattern: &str) -> Option<PathBuf> {
    if let Some(rest) = pattern.strip_prefix("~/") {
        return Some(home().join(rest));
    }
    if let Some(rest) = pattern.strip_prefix("$USER_VAR/") {
        return user_var().map(|dir| dir.join(rest));
    }
    pattern.starts_with('/').then(|| PathBuf::from(pattern))
}
