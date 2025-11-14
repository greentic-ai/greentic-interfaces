#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIT_ROOT="${ROOT}/crates/greentic-interfaces/wit"
OUT_DIR="${1:-${ROOT}/target/wit-packages}"
DRY_RUN=${DRY_RUN:-0}

source "${ROOT}/scripts/wit-common.sh"

ensure_cmd wasm-tools
ensure_cmd wkg

WIT_SOURCES=()
while IFS= read -r src; do
  if [[ -n "${src}" ]]; then
    WIT_SOURCES+=("${src}")
  fi
done < <(list_wit_sources | sort -u)

if [[ ${#WIT_SOURCES[@]} -eq 0 ]]; then
  echo "No WIT sources found under ${WIT_ROOT}" >&2
  exit 0
fi

mkdir -p "${OUT_DIR}"

status=0
for source in "${WIT_SOURCES[@]}"; do
  ref="$(package_ref_from_file "${source}" || true)"
  if [[ -z "${ref}" ]]; then
    echo "Skipping ${source}: missing package declaration" >&2
    status=1
    continue
  fi

  if [[ "${ref}" == "wasix:mcp@0.0.5" ]]; then
    echo "Skipping upstream package ${ref}"
    continue
  fi

  sanitized="$(sanitize_ref "${ref}")"
  out_path="${OUT_DIR}/${sanitized}.wasm"
  echo "Packaging ${ref} -> ${out_path}"

  if [[ "${DRY_RUN}" -eq 1 ]]; then
    echo "  (dry-run) staging and building"
    continue
  fi

  tmpdir="$(prepare_package_layout "${source}" 2>/dev/null || true)"
  if [[ -z "${tmpdir}" ]]; then
    echo "  Failed to stage ${ref}" >&2
    status=1
    continue
  fi

  if ! wasm-tools component wit "${tmpdir}" --wasm -o "${out_path}" >/dev/null 2>&1; then
    echo "  wasm-tools component wit failed for ${ref}" >&2
    status=1
  fi

  rm -rf "${tmpdir}"
done

echo "Artifacts written to ${OUT_DIR}"
exit "${status}"
