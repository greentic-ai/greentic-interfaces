#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIT_DIR="${ROOT}/wit"

if ! command -v wit-bindgen >/dev/null 2>&1; then
  echo "Error: wit-bindgen not found in PATH. Install via 'cargo install wit-bindgen-cli'." >&2
  exit 1
fi

if ! command -v wasm-tools >/dev/null 2>&1; then
  echo "Error: wasm-tools not found in PATH. Install via 'cargo install wasm-tools'." >&2
  exit 1
fi

shopt -s nullglob
wits=("${WIT_DIR}"/*.wit)
shopt -u nullglob

if [ ${#wits[@]} -eq 0 ]; then
  echo "No WIT files found under ${WIT_DIR}."
  exit 0
fi

status=0
for wit_file in "${wits[@]}"; do
  rel_path="${wit_file#"${ROOT}/"}"
  echo "Checking ${rel_path}"
  tmpdir="$(mktemp -d)"
  if ! wit-bindgen markdown "${wit_file}" --out-dir "${tmpdir}" >/dev/null 2>&1; then
    status=1
  fi
  rm -rf "${tmpdir}"
  tmpwasm="$(mktemp)"
  if ! wasm-tools component wit --wasm "${wit_file}" -o "${tmpwasm}" >/dev/null 2>&1; then
    status=1
  fi
  rm -f "${tmpwasm}"
done

exit "${status}"
