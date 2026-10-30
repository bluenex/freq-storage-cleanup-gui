use std::process::Command;

/// Whether a process with exactly this name is running.
pub fn is_running(process: &str) -> bool {
    Command::new("/usr/bin/pgrep")
        .args(["-x", process])
        .output()
        .is_ok_and(|o| o.status.success())
}
