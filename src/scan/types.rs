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

impl Item {
    /// The label plus the worktree's folder name, for places that show items outside their group.
    pub fn full_label(&self) -> String {
        match self.worktree.as_ref().and_then(|w| w.file_name()) {
            Some(name) => format!("{} in {}", self.label, name.to_string_lossy()),
            None => self.label.clone(),
        }
    }
}

pub enum ScanEvent {
    /// All rows are known; `labels` are their full labels in scan order.
    Found { labels: Vec<String> },
    /// The row at this index has been measured.
    Measured(usize),
}

pub struct ScanResult {
    pub items: Vec<Item>,
    pub disk: Option<DiskSpace>,
}
