use std::process::Command;

use super::run_as_admin::run_as_admin;

const TMUTIL: &str = "/usr/bin/tmutil";

fn is_permission_error(text: &str) -> bool {
    let text = text.to_lowercase();
    ["permission", "not permitted", "privilege", "root"]
        .iter()
        .any(|k| text.contains(k))
}

/// Deletes each snapshot without root first. Snapshots that fail with a
/// permission error are retried together behind one password prompt.
pub fn delete_snapshots(dates: &[String], mut log: impl FnMut(String)) {
    let mut need_admin = Vec::new();
    for date in dates {
        log(format!("$ tmutil deletelocalsnapshots {date}"));
        match Command::new(TMUTIL).args(["deletelocalsnapshots", date]).output() {
            Ok(out) if out.status.success() => log("  ok".into()),
            Ok(out) => {
                let text = format!(
                    "{}{}",
                    String::from_utf8_lossy(&out.stdout),
                    String::from_utf8_lossy(&out.stderr)
                );
                if is_permission_error(&text) {
                    log("  needs admin rights".into());
                    need_admin.push(date.as_str());
                } else {
                    log(format!("  FAILED: {}", text.trim()));
                }
            }
            Err(e) => log(format!("  FAILED: {e}")),
        }
    }

    if need_admin.is_empty() {
        return;
    }
    let script = need_admin
        .iter()
        .map(|d| format!("{TMUTIL} deletelocalsnapshots {d}"))
        .collect::<Vec<_>>()
        .join("; ");
    log(format!(
        "Asking for admin rights to delete {} snapshot(s)…",
        need_admin.len()
    ));
    match run_as_admin(&script) {
        Ok(out) => log(format!("  {}", if out.trim().is_empty() { "ok" } else { out.trim() })),
        Err(e) => log(format!("  FAILED: {e:#}")),
    }
}
