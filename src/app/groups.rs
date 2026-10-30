use std::path::Path;

use crate::scan::Item;

pub struct Group {
    pub name: String,
    pub indices: Vec<usize>,
    pub size: u64,
}

/// Items grouped by worktree, or by their rule's group when not in a worktree.
/// Groups keep the scan order (rule order), so they're in the same place on every scan.
/// Items in a group are largest first.
pub fn group_items(items: &[Item], home: &Path, hide_empty: bool) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    for (i, item) in items.iter().enumerate() {
        if hide_empty && item.size == 0 {
            continue;
        }
        let name = match &item.worktree {
            Some(path) => path.strip_prefix(home).unwrap_or(path).display().to_string(),
            None => item.rule.group.to_owned(),
        };
        match groups.iter_mut().find(|g| g.name == name) {
            Some(group) => {
                group.indices.push(i);
                group.size += item.size;
            }
            None => groups.push(Group {
                name,
                indices: vec![i],
                size: item.size,
            }),
        }
    }
    for group in &mut groups {
        group.indices.sort_by_key(|&i| std::cmp::Reverse(items[i].size));
    }
    groups
}
