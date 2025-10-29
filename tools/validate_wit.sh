#!/usr/bin/env bash
set -euo pipefail

if ! command -v wit-bindgen >/dev/null 2>&1; then
  echo "wit-bindgen not found in PATH. Install via 'cargo install wit-bindgen-cli'." >&2
  exit 1
fi

if ! command -v wasm-tools >/dev/null 2>&1; then
  echo "wasm-tools not found in PATH. Install via 'cargo install wasm-tools'." >&2
  exit 1
fi

for f in wit/*/world.wit wit/*.wit; do
  if [ -f "$f" ]; then
    echo "Validating $f"
    tmpdir="$(mktemp -d)"
    wit-bindgen markdown "$f" --out-dir "$tmpdir" >/dev/null 2>&1 || {
      rm -rf "$tmpdir"; exit 1;
    }
    rm -rf "$tmpdir"
    tmpwasm="$(mktemp)"
    if ! wasm-tools component wit --wasm "$f" -o "$tmpwasm" >/dev/null 2>&1; then
      rm -f "$tmpwasm"
      exit 1
    fi
    rm -f "$tmpwasm"
  fi
done
