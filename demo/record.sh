#!/usr/bin/env bash
# Regenerate demo/wordle-tui.gif. Requires: vhs (brew install vhs).
#
# Builds the release binary and runs VHS against demo/demo.tape. The tape pins
# the answer with WORDLE_TUI_SOLUTION so the scripted playthrough always wins.
set -euo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cargo build --release --manifest-path "$REPO/Cargo.toml"

export WORDLE_TUI_BIN="$REPO/target/release/wordle-tui"
( cd "$REPO/demo" && vhs demo.tape )
echo "wrote $REPO/demo/wordle-tui.gif"
