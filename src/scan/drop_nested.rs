use std::path::PathBuf;

/// Removes matches that sit inside another match, so nothing is counted or deleted twice.
pub fn drop_nested(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    paths
        .iter()
        .filter(|p| !paths.iter().any(|q| q != *p && p.starts_with(q)))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_outermost() {
        let paths = vec![
            PathBuf::from("/a/node_modules"),
            PathBuf::from("/a/node_modules/x/node_modules"),
            PathBuf::from("/b/target"),
        ];
        assert_eq!(
            drop_nested(paths),
            [PathBuf::from("/a/node_modules"), PathBuf::from("/b/target")]
        );
    }
}
