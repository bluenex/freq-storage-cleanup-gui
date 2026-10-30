use std::process::Command;

use super::utc_to_snapshot_id::utc_to_snapshot_id;

const PREFS: &str = "/Library/Preferences/com.apple.TimeMachine.plist";

/// Snapshot ids Time Machine uses as the reference for each backup destination.
pub fn reference_snapshot_ids() -> Vec<String> {
    let Ok(out) = Command::new("/usr/bin/defaults").args(["read", PREFS]).output() else {
        return Vec::new();
    };
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|line| line.contains("ReferenceLocalSnapshotDate"))
        .filter_map(|line| utc_to_snapshot_id(line.split('"').nth(1)?))
        .collect()
}
