use crate::clean::Step;
use crate::scan::{DiskSpace, Item};
use crate::snapshots::Snapshot;
use crate::worker::Worker;

#[derive(PartialEq)]
pub enum Screen {
    Scanning,
    Select,
    Confirm,
    Cleaning,
    Snapshots,
    Rescanning,
    Done,
}

pub struct App {
    pub worker: Worker,
    pub screen: Screen,
    pub items: Vec<Item>,
    pub disk: Option<DiskSpace>,
    pub free_before_clean: Option<u64>,
    pub plan: Vec<Step>,
    pub log: Vec<String>,
    /// `None` while listing.
    pub snapshots: Option<Result<Vec<Snapshot>, String>>,
    pub delete_reference: bool,
    /// Full labels of the rows being measured, with whether each is done. Empty outside a scan.
    pub scan_progress: Vec<(String, bool)>,
    /// Hides 0 B rows on the select screen.
    pub hide_empty: bool,
    pub deleting_snapshots: bool,
}
