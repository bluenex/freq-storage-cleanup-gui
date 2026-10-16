use std::path::PathBuf;

/// `$HOME`, canonicalized so matched paths can be compared with their canonical form.
pub fn home() -> PathBuf {
    let raw = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/"));
    raw.canonicalize().unwrap_or(raw)
}
