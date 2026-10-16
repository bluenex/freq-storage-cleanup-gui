use std::fmt;
use std::path::PathBuf;

use crate::rules::{Cmd, Rule};

pub enum Step {
    Remove { path: PathBuf, rule: &'static Rule },
    Run(&'static Cmd),
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Step::Remove { path, .. } => write!(f, "rm -rf \"{}\"", path.display()),
            Step::Run(cmd) => write!(f, "{} {}", cmd.programs[0], cmd.args.join(" ")),
        }
    }
}
