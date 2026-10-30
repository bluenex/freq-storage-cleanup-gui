use std::path::Path;

use crate::scan::Item;

pub struct Group {
    pub name: String,
    pub indices: Vec<usize>,
    pub size: u64,
}

/// Items grouped by worktree, or by their rule's group when not in a worktree, largest group first.
/// Items keep their order, which is already by size.
pub fn group_items(items: &[Item], home: &Path) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::new();
    for (i, item) in items.iter().enumerate() {
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
    groups.sort_by(|a, b| b.size.cmp(&a.size));
    groups
}
