/// How a rule's glob matches turn into rows.
pub enum Expand {
    /// One row per match.
    Each,
    /// One row for all matches.
    All,
    /// One row for all matches except the most recently modified one.
    AllButNewest,
    /// One row for only the most recently modified match.
    Newest,
    /// One row per match; matches modified in the last `STALE_DAYS` days can't be selected.
    Stale,
    /// One row per worktree. The worktree is the match minus `depth` trailing components.
    Worktree { depth: usize },
    /// One row per version; the newest version per device can't be selected.
    DeviceSupport,
    /// One row for all devices that `simctl` reports as unavailable.
    UnavailableSimulators,
}

/// An external tool. The first existing path in `programs` is used; `~/` means `$HOME`.
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
    /// The list header for rows that aren't in a worktree.
    pub group: &'static str,
    /// Globs; see `resolve_pattern` for the allowed prefixes.
    pub patterns: &'static [&'static str],
    pub expand: Expand,
    pub clean: Clean,
    /// Process name (`pgrep -x`). While it runs, the rows can't be selected.
    pub running_app: Option<&'static str>,
}
