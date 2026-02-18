#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET="${ROOT}/contracts/0.6.0"

if ! command -v rg >/dev/null 2>&1; then
  echo "[err] ripgrep (rg) is required for naming lint"
  exit 1
fi

if [[ ! -d "${TARGET}" ]]; then
  echo "[warn] contracts/0.6.0 not found; skipping naming lint"
  exit 0
fi

FAILURES=0
check_pattern() {
  local pattern="$1"
  local desc="$2"
  local matches
  matches=$(rg -n --no-heading --color=never "${pattern}" "${TARGET}" || true)
  if [[ -n "${matches}" ]]; then
    echo "[error] ${desc} (pattern: ${pattern})"
    echo "${matches}"
    FAILURES=$((FAILURES + 1))
  fi
}

check_pattern "node_id" "Use step_id instead of node_id"
check_pattern "node-id" "Use step_id instead of node-id"
check_pattern "node\.[[:alnum:]]+" "Old node.<step> helpers should be renamed"
check_pattern "ctx:" "Call specs should not include ctx fields; host owns TenantCtx"
check_pattern "envelope:" "Call specs should not include envelope fields"

if [[ ${FAILURES} -gt 0 ]]; then
  echo "[fail] naming lint detected ${FAILURES} forbidden pattern(s)"
  exit 1
fi

echo "[ok] naming lint passed (contracts/0.6.0 uses step/call spec vocabulary)"
