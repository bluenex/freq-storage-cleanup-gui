use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::device_support::mark_newest;
use super::glob_dirs::glob_dirs;
use super::types::Item;
use super::unavailable_simulators::unavailable_simulator_ids;
use crate::rules::{Expand, Rule};

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Turns one rule into unmeasured rows.
pub fn expand_rule(rule: &'static Rule, home: &Path) -> Vec<Item> {
    let matches: Vec<PathBuf> = rule.patterns.iter().flat_map(|p| glob_dirs(home, p)).collect();

    match rule.expand {
        Expand::Each => matches
            .into_iter()
            .map(|p| Item::new(rule, rule.label, vec![p]))
            .collect(),

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
            if paths.is_empty() {
                return Vec::new();
            }
            let label = format!("{} ({})", rule.label, paths.len());
            vec![Item::new(rule, label, paths)]
        }
    }
}
