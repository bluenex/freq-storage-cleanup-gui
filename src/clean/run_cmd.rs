use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::rules::{Cmd, home};

/// Runs the tool and returns its combined output.
pub fn run_cmd(cmd: &Cmd) -> Result<String> {
    let home = home();
    let program = cmd
        .programs
        .iter()
        .map(|p| {
            p.strip_prefix("~/")
                .map_or_else(|| PathBuf::from(p), |rest| home.join(rest))
        })
        .find(|p| p.exists())
        .with_context(|| format!("not found: {}", cmd.programs.join(", ")))?;
    // A fixed PATH, so tools that are scripts (like pnpm) find Homebrew's node.
    let out = Command::new(program)
        .args(cmd.args)
        .current_dir(&home)
        .env("PATH", "/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin")
        .output()?;
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    if !out.status.success() {
        bail!("{} ({})", text.trim(), out.status);
    }
    Ok(text)
}
