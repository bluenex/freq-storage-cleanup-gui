use std::thread;

use super::disk_space::disk_space;
use super::expand_rule::expand_rule;
use super::measure_dir::measure_dir;
use super::types::{Item, ScanEvent, ScanResult};
use crate::rules::RULES;

pub fn scan_all(on_event: &(dyn Fn(ScanEvent) + Sync)) -> ScanResult {
    let mut items: Vec<Item> = thread::scope(|s| {
        let handles: Vec<_> = RULES.iter().map(|rule| s.spawn(|| expand_rule(rule))).collect();
        handles.into_iter().flat_map(|h| h.join().unwrap_or_default()).collect()
    });

    on_event(ScanEvent::Found {
        labels: items.iter().map(Item::full_label).collect(),
    });

    thread::scope(|s| {
        for (i, item) in items.iter_mut().enumerate() {
            s.spawn(move || {
                item.size = item.paths.iter().map(|p| measure_dir(p)).sum();
                on_event(ScanEvent::Measured(i));
            });
        }
    });

    ScanResult {
        items,
        disk: disk_space(),
    }
}
