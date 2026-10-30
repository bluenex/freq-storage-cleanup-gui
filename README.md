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
| Yarn, npm, npx, pnpm, Bun, uv, CocoaPods caches | `~/Library/Caches/{Yarn,pnpm,CocoaPods}`, `~/.npm/{_cacache,_npx}`, `~/.bun/install/cache`, `~/.cache/uv` | delete |
| node-gyp, React Native, dotslash caches | `~/Library/Caches/{node-gyp,ReactNative,dotslash}` | delete |
| pnpm store | `~/Library/pnpm/store/v*`, the newest | `pnpm store prune` |
| Old pnpm stores | `~/Library/pnpm/store/v*`, all but the newest | delete |
| Go module cache | `~/go/pkg/mod` | `go clean -modcache` |
| Homebrew cache | `~/Library/Caches/Homebrew` | `brew cleanup --prune=all` |
| Xcode DerivedData | `~/Library/Developer/Xcode/DerivedData` | delete |
| iOS DeviceSupport | `~/Library/Developer/Xcode/iOS DeviceSupport/*` | delete, keeps the newest version per device |
| Unavailable simulators | `~/Library/Developer/CoreSimulator/Devices/*` | `xcrun simctl delete unavailable` |
| Simulator data | `~/Library/Developer/CoreSimulator/Devices` | `xcrun simctl erase all` |
| Gradle caches | `~/.gradle/caches` | delete |
| Android emulator snapshots | `~/.android/avd/*.avd/snapshots` | delete, only while the emulator is off |
| Android NDK 27.0 | `~/Library/Android/sdk/ndk/27.0.12077973` | delete |
| Chrome code-sign clones | `/var/folders/…/X/com.google.Chrome.code_sign_clone/*` | delete, keeps the newest |
| Chrome web cache | `~/Library/Caches/Google` | delete |
| Chrome on-device AI model | `~/Library/Application Support/Google/Chrome/OptGuideOnDeviceModel` | delete, only while Chrome is closed |
| Claude desktop VM bundles | `~/Library/Application Support/Claude/vm_bundles` | delete, only while Claude is closed |
| Notion offline cache | `~/Library/Application Support/Notion/Partitions` | delete, only while Notion is closed |
| Updater downloads | `~/Library/Caches/{orca-updater,bitwarden-updater,com.postmanlabs.mac.ShipIt}`, `/var/folders/…/T/*.ShipIt.*` | delete |
| Docker Desktop leftovers | `~/Library/Containers/com.docker.docker/Data/vms`, `/opt/homebrew/Caskroom/docker-desktop` | delete |
| Podman machine | `~/.local/share/containers/podman/machine` | delete |
| Miniconda | `/opt/homebrew/Caskroom/miniconda` | `brew uninstall --cask miniconda` |
| Old Node versions | `~/.nvm/versions/node/{v18.14.0,v20.11.0,v22.4.0}` | delete |
| Build output in old projects | `~/Dev/_*/*/{node_modules,.next,target}` and one level deeper, not `_papaya` | delete, skips dirs changed in the last 30 days |
| Time Machine local snapshots | `tmutil listlocalsnapshots /` | `tmutil deletelocalsnapshots`, keeps the last-backup reference (and any `(dataless)` one) |

To change the targets, edit `src/rules/constants.rs`.

## How it works

1. **Scan:** measures all targets in parallel by allocated blocks, like `du`, and shows them as a list grouped by worktree or by kind.
2. **Select:** tick rows. The disk bar shows free space before and after.
3. **Confirm:** lists every path and command before anything runs.
4. **Clean:** deletes files directly, not to the Trash. Each path is checked again first: it must match its rule and contain no symlinks.
5. **Snapshots:** deletes local snapshots last, because snapshots taken before the cleanup still hold the deleted files. It asks for your password only if `tmutil` needs root.
6. **Re-scan:** shows the space freed.
