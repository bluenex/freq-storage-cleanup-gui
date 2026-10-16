use glob::MatchOptions;

use super::types::{Clean, Cmd, Expand, Rule};

/// `*` must not cross `/`, so a match can't jump to a deeper or unrelated dir.
pub const GLOB_OPTIONS: MatchOptions = MatchOptions {
    case_sensitive: true,
    require_literal_separator: true,
    require_literal_leading_dot: false,
};

pub const BREW_CLEANUP: Cmd = Cmd {
    programs: &["/opt/homebrew/bin/brew", "/usr/local/bin/brew"],
    args: &["cleanup", "--prune=all"],
};

pub const SIMCTL_DELETE_UNAVAILABLE: Cmd = Cmd {
    programs: &["/usr/bin/xcrun"],
    args: &["simctl", "delete", "unavailable"],
};

pub const RULES: &[Rule] = &[
    Rule {
        label: "Turbo cache",
        patterns: &[
            "orca/workspaces/*/*/node_modules/.cache/turbo",
            "Dev/_papaya/*/node_modules/.cache/turbo",
        ],
        expand: Expand::Worktree { depth: 3 },
        clean: Clean::RemoveDirs,
    },
    Rule {
        label: "Next.js build",
        patterns: &["orca/workspaces/*/*/apps/*/.next", "Dev/_papaya/*/apps/*/.next"],
        expand: Expand::Worktree { depth: 3 },
        clean: Clean::RemoveDirs,
    },
    Rule {
        label: "Yarn cache",
        patterns: &["Library/Caches/Yarn"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
    },
    Rule {
        label: "npm cache",
        patterns: &[".npm/_cacache"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
    },
    Rule {
        label: "npx cache",
        patterns: &[".npm/_npx"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
    },
    Rule {
        label: "Homebrew cache",
        patterns: &["Library/Caches/Homebrew"],
        expand: Expand::Each,
        clean: Clean::Run(&BREW_CLEANUP),
    },
    Rule {
        label: "Xcode DerivedData",
        patterns: &["Library/Developer/Xcode/DerivedData"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
    },
    Rule {
        label: "iOS DeviceSupport",
        patterns: &["Library/Developer/Xcode/iOS DeviceSupport/*"],
        expand: Expand::DeviceSupport,
        clean: Clean::RemoveDirs,
    },
    Rule {
        label: "Unavailable simulators",
        patterns: &["Library/Developer/CoreSimulator/Devices"],
        expand: Expand::UnavailableSimulators,
        clean: Clean::Run(&SIMCTL_DELETE_UNAVAILABLE),
    },
];
