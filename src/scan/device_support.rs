use std::collections::HashMap;

use super::types::Item;

/// Splits a DeviceSupport dir name such as `iPhone15,2 17.0.3 (21A350) arm64e`
/// into the device (`iPhone15,2`, empty for old names) and the version numbers.
fn parse_name(name: &str) -> Option<(String, Vec<u32>)> {
    let tokens: Vec<&str> = name.split_whitespace().collect();
    let at = tokens
        .iter()
        .position(|t| t.starts_with(|c: char| c.is_ascii_digit()) && t.contains('.'))?;
    let version = tokens[at].split('.').map(|n| n.parse().unwrap_or(0)).collect();
    Some((tokens[..at].join(" "), version))
}

/// Marks the newest version per device as kept.
pub fn mark_newest(items: &mut [Item]) {
    let mut newest: HashMap<String, (Vec<u32>, usize)> = HashMap::new();
    for (i, item) in items.iter().enumerate() {
        let Some((device, version)) = parse_name(&item.label) else {
            continue;
        };
        let entry = newest.entry(device).or_insert((version.clone(), i));
        if version > entry.0 {
            *entry = (version, i);
        }
    }
    for (_, i) in newest.into_values() {
        items[i].keep_reason = Some("newest version for this device");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::RULES;

    fn item(name: &str) -> Item {
        Item::new(&RULES[0], name, Vec::new())
    }

    #[test]
    fn keeps_newest_per_device() {
        let mut items = vec![
            item("iPhone15,2 17.0.3 (21A350)"),
            item("iPhone15,2 17.10 (21H16) arm64e"),
            item("iPhone14,5 16.1.1 (20B101)"),
            item("16.4 (20E247)"),
        ];
        mark_newest(&mut items);
        let kept: Vec<bool> = items.iter().map(|i| i.keep_reason.is_some()).collect();
        assert_eq!(kept, [false, true, true, true]);
    }
}
