#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-${ROOT}/target/wit-packages}"

if ! command -v wasm-tools >/dev/null 2>&1; then
  echo "Error: wasm-tools not found in PATH. Install via 'cargo install wasm-tools'." >&2
  exit 1
fi

mkdir -p "${OUT_DIR}"

shopt -s nullglob
wits=("${ROOT}"/wit/*.wit "${ROOT}"/wit/*/*.wit)
shopt -u nullglob

if [ ${#wits[@]} -eq 0 ]; then
  echo "No WIT files found under ${ROOT}/wit." >&2
  exit 0
fi

for wit_file in "${wits[@]}"; do
  package_line="$(grep -m1 '^package ' "${wit_file}" || true)"
  if [[ -z "${package_line}" ]]; then
    echo "Skipping ${wit_file}: package declaration not found" >&2
    continue
  fi
  package_ref="${package_line#package }"
  package_ref="${package_ref%;}"
  sanitized="${package_ref//[:@]/-}"
  base_name="${sanitized}"
  out_name="${base_name}.wasm"
  out_path="${OUT_DIR}/${out_name}"
  echo "Packaging ${package_ref} -> ${out_path}"
  wasm-tools component wit --wasm "${wit_file}" -o "${out_path}"
done

echo "Artifacts written to ${OUT_DIR}" 
