use std::process::Command;

use anyhow::{Result, bail};

/// Runs a shell script through the native macOS password prompt.
/// The script must not contain `"` or `\`.
pub fn run_as_admin(script: &str) -> Result<String> {
    let apple_script = format!("do shell script \"{script}\" with administrator privileges");
    let out = Command::new("/usr/bin/osascript")
        .args(["-e", &apple_script])
        .output()?;
    if !out.status.success() {
        bail!("{}", String::from_utf8_lossy(&out.stderr).trim());
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}
