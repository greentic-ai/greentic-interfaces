#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-${ROOT}/target/wit-packages}"
DRY_RUN=${DRY_RUN:-0}

if ! command -v wasm-tools >/dev/null 2>&1; then
  echo "Error: wasm-tools not found in PATH. Install via 'cargo install wasm-tools'." >&2
  exit 1
fi

if ! command -v wkg >/dev/null 2>&1; then
  echo "Error: wkg not found in PATH. Install via 'cargo install wkg'." >&2
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

status=0

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

  if [[ "$(basename "${wit_file}")" == "world.wit" ]]; then
    pkg_dir="$(dirname "${wit_file}")"
    pkg_name="$(basename "${pkg_dir}")"
    if [[ "${pkg_name}" == "wasix-mcp@0.0.5" ]]; then
      echo "  Skipping packaging for upstream dependency ${package_ref}"
      continue
    fi
    if [[ "${DRY_RUN}" -eq 1 ]]; then
      echo "  (dry-run) wkg wit build --wit-dir ${pkg_dir} -o ${out_path}"
      continue
    fi
    if ! wkg wit build --wit-dir "${pkg_dir}" -o "${out_path}" >/dev/null 2>&1; then
      echo "  Failed to package ${package_ref}" >&2
      status=1
    fi
  else
    if [[ "${DRY_RUN}" -eq 1 ]]; then
      echo "  (dry-run) wasm-tools component wit --wasm ${wit_file} -o ${out_path}"
      continue
    fi
    if ! wasm-tools component wit --wasm "${wit_file}" -o "${out_path}" >/dev/null 2>&1; then
      echo "  Failed to package ${package_ref}" >&2
      status=1
    fi
  fi
done

echo "Artifacts written to ${OUT_DIR}" 

exit "${status}"
