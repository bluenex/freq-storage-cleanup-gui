use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};

use crate::rules::Cmd;

/// Runs the tool and returns its combined output.
pub fn run_cmd(cmd: &Cmd) -> Result<String> {
    let program = cmd
        .programs
        .iter()
        .find(|p| Path::new(p).exists())
        .with_context(|| format!("not found: {}", cmd.programs.join(", ")))?;
    let out = Command::new(program).args(cmd.args).output()?;
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
