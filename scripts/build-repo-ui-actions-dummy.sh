#!/usr/bin/env bash
set -euo pipefail
ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
CRATE_DIR="$ROOT/guest-tests/repo-ui-actions-dummy"
OUT_DIR="$ROOT/crates/greentic-interfaces-wasmtime/tests/assets"
CORE_WASM="$ROOT/target/wasm32-wasip2/release/repo_ui_actions_dummy.wasm"
COMPONENT_WASM="$OUT_DIR/repo_ui_actions_dummy.component.wasm"
WIT_SRC="$ROOT/crates/greentic-interfaces/wit"
WIT_DST="$CRATE_DIR/wit"

if [[ ! -d "$CRATE_DIR" ]]; then
  echo "[repo-ui-actions-dummy] crate not found at $CRATE_DIR" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"

# Sync WIT sources so the guest crate builds against the latest interfaces.
if [[ -d "$WIT_SRC" ]]; then
  mkdir -p "$WIT_DST"
  rsync -a --delete "$WIT_SRC"/ "$WIT_DST"/
else
  echo "[repo-ui-actions-dummy] missing WIT sources at $WIT_SRC" >&2
  exit 1
fi

if ! rustup target list --installed | grep -q "wasm32-wasip2"; then
  echo "Installing wasm32-wasip2 target..." >&2
  rustup target add wasm32-wasip2
fi

echo "Building dummy repo-ui-actions guest..." >&2
cargo build --release --target wasm32-wasip2 --manifest-path "$CRATE_DIR/Cargo.toml"

echo "Copying built component..." >&2
cp "$CORE_WASM" "$COMPONENT_WASM"
echo "Component available at $COMPONENT_WASM" >&2
