use std::thread;

use super::disk_space::disk_space;
use super::expand_rule::expand_rule;
use super::measure_dir::measure_dir;
use super::types::{Item, ScanResult};
use crate::rules::{RULES, home};

pub fn scan_all() -> ScanResult {
    let home = home();
    let mut items: Vec<Item> = thread::scope(|s| {
        let handles: Vec<_> = RULES.iter().map(|rule| s.spawn(|| expand_rule(rule, &home))).collect();
        handles.into_iter().flat_map(|h| h.join().unwrap_or_default()).collect()
    });

    thread::scope(|s| {
        for item in &mut items {
            s.spawn(move || item.size = item.paths.iter().map(|p| measure_dir(p)).sum());
        }
    });

    items.sort_by(|a, b| b.size.cmp(&a.size));
    ScanResult {
        items,
        disk: disk_space(),
    }
}
