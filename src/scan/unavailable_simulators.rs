use std::process::Command;

fn is_uuid(s: &str) -> bool {
    s.len() == 36
        && s.char_indices().all(|(i, c)| match i {
            8 | 13 | 18 | 23 => c == '-',
            _ => c.is_ascii_hexdigit(),
        })
}

/// UDIDs of simulators that `simctl` lists as unavailable. Empty if Xcode isn't installed.
pub fn unavailable_simulator_ids() -> Vec<String> {
    let Ok(out) = Command::new("/usr/bin/xcrun")
        .args(["simctl", "list", "devices", "unavailable"])
        .output()
    else {
        return Vec::new();
    };
    if !out.status.success() {
        return Vec::new();
    }
    String::from_utf8_lossy(&out.stdout)
        .split(['(', ')'])
        .filter(|s| is_uuid(s))
        .map(str::to_owned)
        .collect()
}
