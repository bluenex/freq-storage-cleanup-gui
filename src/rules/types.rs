/// How a rule's glob matches turn into rows.
pub enum Expand {
    /// One row per match.
    Each,
    /// One row per worktree. The worktree is the match minus `depth` trailing components.
    Worktree { depth: usize },
    /// One row per version; the newest version per device can't be selected.
    DeviceSupport,
    /// One row for all devices that `simctl` reports as unavailable.
    UnavailableSimulators,
}

/// An external tool. The first existing path in `programs` is used.
pub struct Cmd {
    pub programs: &'static [&'static str],
    pub args: &'static [&'static str],
}

pub enum Clean {
    RemoveDirs,
    Run(&'static Cmd),
}

pub struct Rule {
    pub label: &'static str,
    /// Globs relative to `$HOME`.
    pub patterns: &'static [&'static str],
    pub expand: Expand,
    pub clean: Clean,
}
