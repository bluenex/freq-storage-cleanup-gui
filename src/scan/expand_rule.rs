use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::device_support::mark_newest;
use super::drop_nested::drop_nested;
use super::glob_dirs::glob_dirs;
use super::is_running::is_running;
use super::modified_within::modified_within;
use super::split_newest::split_newest;
use super::types::Item;
use super::unavailable_simulators::unavailable_simulator_ids;
use crate::rules::{Expand, Rule, STALE_DAYS, home};

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// One row for all paths, with the count in the label when there are several.
fn combined(rule: &'static Rule, paths: Vec<PathBuf>) -> Vec<Item> {
    let label = match paths.len() {
        0 => return Vec::new(),
        1 => rule.label.to_owned(),
        n => format!("{} ({n})", rule.label),
    };
    vec![Item::new(rule, label, paths)]
}

/// Turns one rule into unmeasured rows.
pub fn expand_rule(rule: &'static Rule) -> Vec<Item> {
    let mut items = rows(rule);
    if rule.running_app.is_some_and(is_running) {
        for item in &mut items {
            item.keep_reason = Some("the app is running; quit it and scan again");
        }
    }
    items
}

fn rows(rule: &'static Rule) -> Vec<Item> {
    let matches = drop_nested(rule.patterns.iter().flat_map(|p| glob_dirs(p)).collect());

    match rule.expand {
        Expand::Each => matches
            .into_iter()
            .map(|p| Item::new(rule, rule.label, vec![p]))
            .collect(),

        Expand::All => combined(rule, matches),

        Expand::AllButNewest => combined(rule, split_newest(matches).1),

        Expand::Newest => combined(rule, split_newest(matches).0.into_iter().collect()),

        Expand::Stale => {
            let home = home();
            matches
                .into_iter()
                .map(|p| {
                    let label = p.strip_prefix(&home).unwrap_or(&p).display().to_string();
                    let recent = modified_within(&p, STALE_DAYS);
                    Item {
                        keep_reason: recent.then_some("changed recently, so probably in use"),
                        ..Item::new(rule, label, vec![p])
                    }
                })
                .collect()
        }

        Expand::Worktree { depth } => {
            let mut by_worktree: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();
            for path in matches {
                let worktree = path.ancestors().nth(depth).unwrap_or(&path).to_path_buf();
                by_worktree.entry(worktree).or_default().push(path);
            }
            by_worktree
                .into_iter()
                .map(|(worktree, paths)| {
                    let label = match paths.len() {
                        1 => rule.label.to_owned(),
                        n => format!("{} ({n} dirs)", rule.label),
                    };
                    Item {
                        worktree: Some(worktree),
                        ..Item::new(rule, label, paths)
                    }
                })
                .collect()
        }

        Expand::DeviceSupport => {
            let mut items: Vec<Item> = matches
                .into_iter()
                .map(|p| Item::new(rule, file_name(&p), vec![p]))
                .collect();
            mark_newest(&mut items);
            items
        }

        Expand::UnavailableSimulators => {
            let Some(devices_dir) = matches.first() else {
                return Vec::new();
            };
            let paths: Vec<PathBuf> = unavailable_simulator_ids()
                .into_iter()
                .map(|id| devices_dir.join(id))
                .filter(|p| p.is_dir())
                .collect();
            combined(rule, paths)
        }
    }
}
