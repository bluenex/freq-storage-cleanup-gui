use std::path::PathBuf;

use crate::rules::Rule;

pub struct Item {
    pub rule: &'static Rule,
    /// The worktree this item belongs to, if any.
    pub worktree: Option<PathBuf>,
    pub label: String,
    pub paths: Vec<PathBuf>,
    pub size: u64,
    /// Set when the item must not be cleaned, e.g. the newest DeviceSupport version.
    pub keep_reason: Option<&'static str>,
    pub selected: bool,
}

impl Item {
    pub fn new(rule: &'static Rule, label: impl Into<String>, paths: Vec<PathBuf>) -> Self {
        Self {
            rule,
            worktree: None,
            label: label.into(),
            paths,
            size: 0,
            keep_reason: None,
            selected: false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct DiskSpace {
    pub total: u64,
    pub free: u64,
}

pub struct ScanResult {
    pub items: Vec<Item>,
    pub disk: Option<DiskSpace>,
}
