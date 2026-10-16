#!/bin/sh
# Build (if needed) and run the release binary.
set -e
cd "$(dirname "$0")"
PATH="/opt/homebrew/opt/rustup/bin:$HOME/.cargo/bin:$PATH"
cargo build --release --quiet
exec ./target/release/freq-storage-cleanup "$@"
