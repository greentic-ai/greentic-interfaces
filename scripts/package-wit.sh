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
wits=("${ROOT}"/wit/*.wit)
shopt -u nullglob

if [ ${#wits[@]} -eq 0 ]; then
  echo "No WIT files found under ${ROOT}/wit." >&2
  exit 0
fi

for wit_file in "${wits[@]}"; do
  base_name="$(basename "${wit_file%.wit}")"
  out_name="${base_name//@/-}.wasm"
  out_path="${OUT_DIR}/${out_name}"
  echo "Packaging ${base_name}.wit -> ${out_path}"
  wasm-tools component wit --wasm "${wit_file}" -o "${out_path}"
done

echo "Artifacts written to ${OUT_DIR}" 
