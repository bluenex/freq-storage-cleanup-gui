use std::process::Command;

use anyhow::{Result, bail};

use super::types::Snapshot;

const PREFIX: &str = "com.apple.TimeMachine.";

/// Accepts only `YYYY-MM-DD-HHMMSS`, so the id is safe to put in a shell command.
fn is_snapshot_date(s: &str) -> bool {
    s.len() == 17
        && s.char_indices().all(|(i, c)| match i {
            4 | 7 | 10 => c == '-',
            _ => c.is_ascii_digit(),
        })
}

pub fn list_snapshots() -> Result<Vec<Snapshot>> {
    let out = Command::new("/usr/bin/tmutil")
        .args(["listlocalsnapshots", "/"])
        .output()?;
    if !out.status.success() {
        bail!("{}", String::from_utf8_lossy(&out.stderr).trim());
    }
    let snapshots = String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|line| {
            let date = line.split(PREFIX).nth(1)?.get(..17)?;
            is_snapshot_date(date).then(|| Snapshot {
                date: date.to_owned(),
                dataless: line.contains("(dataless)"),
            })
        })
        .collect();
    Ok(snapshots)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_dates() {
        assert!(is_snapshot_date("2026-10-14-101010"));
        assert!(!is_snapshot_date("2026-10-14-10101;"));
        assert!(!is_snapshot_date("2026-10-14"));
    }
}
