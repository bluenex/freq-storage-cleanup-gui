use super::run_cmd::run_cmd;
use super::safe_remove::safe_remove;
use super::types::Step;

/// Runs every step and reports each result through `log`. A failed step doesn't stop the rest.
pub fn execute(steps: &[Step], mut log: impl FnMut(String)) {
    for step in steps {
        log(format!("$ {step}"));
        let result = match step {
            Step::Remove { path, rule } => safe_remove(path, rule).map(|()| String::new()),
            Step::Run(cmd) => run_cmd(cmd),
        };
        match result {
            Ok(out) if out.trim().is_empty() => log("  ok".into()),
            Ok(out) => log(format!("  {}", out.trim())),
            Err(e) => log(format!("  FAILED: {e:#}")),
        }
    }
}
