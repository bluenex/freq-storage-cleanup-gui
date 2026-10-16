use std::path::PathBuf;

use crate::scan::Item;

pub struct Group {
    pub worktree: Option<PathBuf>,
    pub indices: Vec<usize>,
    pub size: u64,
}

/// Items grouped by worktree (global items in one group), largest group first.
/// Items keep their order, which is already by size.
pub fn group_items(items: &[Item]) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    for (i, item) in items.iter().enumerate() {
        match groups.iter_mut().find(|g| g.worktree == item.worktree) {
            Some(group) => {
                group.indices.push(i);
                group.size += item.size;
            }
            None => groups.push(Group {
                worktree: item.worktree.clone(),
                indices: vec![i],
                size: item.size,
            }),
        }
    }
    groups.sort_by(|a, b| b.size.cmp(&a.size));
    groups
}
