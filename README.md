# freq-storage-cleanup

A small native macOS app (Rust + egui) that finds disk space I free up often, shows how big each item is, and deletes the ones I pick.

## Run

```sh
./app
```

The script builds the release binary when needed and opens the window. Requires a Rust toolchain.

## What it cleans

The rules are hard-coded. The app has no free-form path input.

| Target | Path | How |
|---|---|---|
| Turbo cache, per worktree | `~/orca/workspaces/*/*/node_modules/.cache/turbo`, `~/Dev/_papaya/*/node_modules/.cache/turbo` | delete |
| Next.js build, per worktree | `~/orca/workspaces/*/*/apps/*/.next`, `~/Dev/_papaya/*/apps/*/.next` | delete |
| Yarn cache | `~/Library/Caches/Yarn` | delete |
| npm / npx cache | `~/.npm/_cacache`, `~/.npm/_npx` | delete |
| Homebrew cache | `~/Library/Caches/Homebrew` | `brew cleanup --prune=all` |
| Xcode DerivedData | `~/Library/Developer/Xcode/DerivedData` | delete |
| iOS DeviceSupport | `~/Library/Developer/Xcode/iOS DeviceSupport/*` | delete, keeps the newest version per device |
| Unavailable simulators | `~/Library/Developer/CoreSimulator/Devices/*` | `xcrun simctl delete unavailable` |
| Time Machine local snapshots | `tmutil listlocalsnapshots /` | `tmutil deletelocalsnapshots`, keeps the last-backup reference (and any `(dataless)` one) |

To change the targets, edit `src/rules/constants.rs`.

## How it works

1. **Scan:** measures all targets in parallel by allocated blocks, like `du`, and shows them as a treemap and a list grouped by worktree.
2. **Select:** click tiles or tick rows. The disk bar shows free space before and after.
3. **Confirm:** lists every path and command before anything runs.
4. **Clean:** deletes files directly, not to the Trash. Each path is checked again first: it must match its rule and contain no symlinks.
5. **Snapshots:** deletes local snapshots last, because snapshots taken before the cleanup still hold the deleted files. It asks for your password only if `tmutil` needs root.
6. **Re-scan:** shows the space freed.
