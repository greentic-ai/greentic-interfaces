#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-.}"

CANONICAL="$ROOT/crates/greentic-interfaces/wit"

# Pattern for ALL canonical greentic packages
PATTERN='^package\s+greentic:'

MATCHES_ALL="$(rg -n --hidden --glob '!.git/*' --glob '*.wit' \
  --glob '!**/target/**' \
  --glob '!**/out/**' \
  --glob '!**/wit-staging/**' \
  --glob '!**/wit-staging-wasmtime/**' \
  "$PATTERN" "$ROOT" || true)"

if [[ -z "$MATCHES_ALL" ]]; then
  echo "ERROR: No greentic:* packages found. Guardrail misconfigured."
  exit 1
fi

DUPES="$(echo "$MATCHES_ALL" | rg -v "^${CANONICAL}/" || true)"

if [[ -n "$DUPES" ]]; then
  echo "ERROR: Canonical greentic:* packages declared outside canonical root:"
  echo
  echo "$DUPES"
  echo
  echo "Canonical root is: $CANONICAL"
  exit 1
fi

echo "OK: No duplicated canonical WIT packages detected."
