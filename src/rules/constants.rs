use glob::MatchOptions;

use super::types::{Clean, Cmd, Expand, Rule};

/// `*` must not cross `/`, so a match can't jump to a deeper or unrelated dir.
pub const GLOB_OPTIONS: MatchOptions = MatchOptions {
    case_sensitive: true,
    require_literal_separator: true,
    require_literal_leading_dot: false,
};

pub const STALE_DAYS: u64 = 30;

const BREW: &[&str] = &["/opt/homebrew/bin/brew", "/usr/local/bin/brew"];

pub const BREW_CLEANUP: Cmd = Cmd {
    programs: BREW,
    args: &["cleanup", "--prune=all"],
};

pub const BREW_UNINSTALL_MINICONDA: Cmd = Cmd {
    programs: BREW,
    args: &["uninstall", "--cask", "miniconda"],
};

pub const SIMCTL_DELETE_UNAVAILABLE: Cmd = Cmd {
    programs: &["/usr/bin/xcrun"],
    args: &["simctl", "delete", "unavailable"],
};

pub const SIMCTL_ERASE_ALL: Cmd = Cmd {
    programs: &["/usr/bin/xcrun"],
    args: &["simctl", "erase", "all"],
};

/// Module files are read-only, so a plain delete fails.
pub const GO_CLEAN_MODCACHE: Cmd = Cmd {
    programs: &["/opt/homebrew/bin/go", "/usr/local/go/bin/go"],
    args: &["clean", "-modcache"],
};

/// Drops packages no project uses, but keeps the store that current installs read.
pub const PNPM_STORE_PRUNE: Cmd = Cmd {
    programs: &["~/Library/pnpm/pnpm", "/opt/homebrew/bin/pnpm"],
    args: &["store", "prune"],
};

const CACHES: &str = "Package caches";
const XCODE: &str = "Xcode and simulators";
const ANDROID: &str = "Android";
const APPS: &str = "App caches";
const LEFTOVERS: &str = "Leftovers";
const OLD_PROJECTS: &str = "Old projects in ~/Dev";

pub const RULES: &[Rule] = &[
    Rule {
        label: "Turbo cache",
        group: CACHES,
        patterns: &[
            "~/orca/workspaces/*/*/node_modules/.cache/turbo",
            "~/Dev/_papaya/*/node_modules/.cache/turbo",
        ],
        expand: Expand::Worktree { depth: 3 },
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "Next.js build",
        group: CACHES,
        patterns: &["~/orca/workspaces/*/*/apps/*/.next", "~/Dev/_papaya/*/apps/*/.next"],
        expand: Expand::Worktree { depth: 3 },
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "Yarn cache",
        group: CACHES,
        patterns: &["~/Library/Caches/Yarn"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "npm cache",
        group: CACHES,
        patterns: &["~/.npm/_cacache"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "npx cache",
        group: CACHES,
        patterns: &["~/.npm/_npx"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "pnpm metadata cache",
        group: CACHES,
        patterns: &["~/Library/Caches/pnpm"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "pnpm store",
        group: CACHES,
        patterns: &["~/Library/pnpm/store/v*"],
        expand: Expand::Newest,
        clean: Clean::Run(&PNPM_STORE_PRUNE),
        running_app: None,
    },
    // Each pnpm major version uses its own store dir and never reads older ones.
    Rule {
        label: "Old pnpm stores",
        group: CACHES,
        patterns: &["~/Library/pnpm/store/v*"],
        expand: Expand::AllButNewest,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "Bun cache",
        group: CACHES,
        patterns: &["~/.bun/install/cache"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "uv cache",
        group: CACHES,
        patterns: &["~/.cache/uv"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "Go module cache",
        group: CACHES,
        patterns: &["~/go/pkg/mod"],
        expand: Expand::Each,
        clean: Clean::Run(&GO_CLEAN_MODCACHE),
        running_app: None,
    },
    Rule {
        label: "CocoaPods cache",
        group: CACHES,
        patterns: &["~/Library/Caches/CocoaPods"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "node-gyp, React Native, dotslash",
        group: CACHES,
        patterns: &[
            "~/Library/Caches/node-gyp",
            "~/Library/Caches/ReactNative",
            "~/Library/Caches/dotslash",
        ],
        expand: Expand::All,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "Homebrew cache",
        group: CACHES,
        patterns: &["~/Library/Caches/Homebrew"],
        expand: Expand::Each,
        clean: Clean::Run(&BREW_CLEANUP),
        running_app: None,
    },
    Rule {
        label: "Xcode DerivedData",
        group: XCODE,
        patterns: &["~/Library/Developer/Xcode/DerivedData"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "iOS DeviceSupport",
        group: XCODE,
        patterns: &["~/Library/Developer/Xcode/iOS DeviceSupport/*"],
        expand: Expand::DeviceSupport,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "Unavailable simulators",
        group: XCODE,
        patterns: &["~/Library/Developer/CoreSimulator/Devices"],
        expand: Expand::UnavailableSimulators,
        clean: Clean::Run(&SIMCTL_DELETE_UNAVAILABLE),
        running_app: None,
    },
    Rule {
        label: "Simulator data (erase all)",
        group: XCODE,
        patterns: &["~/Library/Developer/CoreSimulator/Devices"],
        expand: Expand::Each,
        clean: Clean::Run(&SIMCTL_ERASE_ALL),
        running_app: None,
    },
    Rule {
        label: "Gradle caches",
        group: ANDROID,
        patterns: &["~/.gradle/caches"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "Emulator snapshots",
        group: ANDROID,
        patterns: &["~/.android/avd/*.avd/snapshots"],
        expand: Expand::All,
        clean: Clean::RemoveDirs,
        running_app: Some("qemu-system-aarch64"),
    },
    Rule {
        label: "Android NDK 27.0",
        group: ANDROID,
        patterns: &["~/Library/Android/sdk/ndk/27.0.12077973"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "Chrome code-sign clones",
        group: APPS,
        patterns: &["$USER_VAR/X/com.google.Chrome.code_sign_clone/code_sign_clone.*"],
        expand: Expand::AllButNewest,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "Chrome web cache",
        group: APPS,
        patterns: &["~/Library/Caches/Google"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "Chrome on-device AI model",
        group: APPS,
        patterns: &["~/Library/Application Support/Google/Chrome/OptGuideOnDeviceModel"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
        running_app: Some("Google Chrome"),
    },
    Rule {
        label: "Claude desktop VM bundles",
        group: APPS,
        patterns: &["~/Library/Application Support/Claude/vm_bundles"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
        running_app: Some("Claude"),
    },
    Rule {
        label: "Notion offline cache",
        group: APPS,
        patterns: &["~/Library/Application Support/Notion/Partitions"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
        running_app: Some("Notion"),
    },
    Rule {
        label: "Updater downloads",
        group: APPS,
        patterns: &[
            "~/Library/Caches/orca-updater",
            "~/Library/Caches/bitwarden-updater",
            "~/Library/Caches/com.postmanlabs.mac.ShipIt",
            "$USER_VAR/T/*.ShipIt.*",
        ],
        expand: Expand::All,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "Docker Desktop VM and cask",
        group: LEFTOVERS,
        patterns: &[
            "~/Library/Containers/com.docker.docker/Data/vms",
            "/opt/homebrew/Caskroom/docker-desktop",
        ],
        expand: Expand::All,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "Podman machine",
        group: LEFTOVERS,
        patterns: &["~/.local/share/containers/podman/machine"],
        expand: Expand::Each,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    Rule {
        label: "Miniconda",
        group: LEFTOVERS,
        patterns: &["/opt/homebrew/Caskroom/miniconda"],
        expand: Expand::Each,
        clean: Clean::Run(&BREW_UNINSTALL_MINICONDA),
        running_app: None,
    },
    Rule {
        label: "Old Node versions (v18, v20, v22.4)",
        group: LEFTOVERS,
        patterns: &[
            "~/.nvm/versions/node/v18.14.0",
            "~/.nvm/versions/node/v20.11.0",
            "~/.nvm/versions/node/v22.4.0",
        ],
        expand: Expand::All,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
    // `_[!p]*` is every `_` folder except `_papaya`, whose worktrees are covered above.
    Rule {
        label: "Build output",
        group: OLD_PROJECTS,
        patterns: &[
            "~/Dev/_[!p]*/*/node_modules",
            "~/Dev/_[!p]*/*/*/node_modules",
            "~/Dev/_[!p]*/*/.next",
            "~/Dev/_[!p]*/*/*/.next",
            "~/Dev/_[!p]*/*/target",
            "~/Dev/_[!p]*/*/*/target",
        ],
        expand: Expand::Stale,
        clean: Clean::RemoveDirs,
        running_app: None,
    },
];
