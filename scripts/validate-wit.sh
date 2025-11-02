#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIT_ROOT="${ROOT}/crates/greentic-interfaces/wit"

source "${ROOT}/scripts/wit-common.sh"

ensure_cmd wasm-tools

mapfile -t WIT_SOURCES < <(list_wit_sources | sort -u)

if [[ ${#WIT_SOURCES[@]} -eq 0 ]]; then
  echo "No WIT sources found under ${WIT_ROOT}"
  exit 0
fi

status=0
for source in "${WIT_SOURCES[@]}"; do
  rel="${source#"${ROOT}/"}"
  ref="$(package_ref_from_file "${source}" || true)"
  echo "Validating ${rel}"

  tmpdir="$(prepare_package_layout "${source}" 2>/dev/null || true)"
  if [[ -z "${tmpdir}" ]]; then
    echo "  Failed to stage ${rel}" >&2
    status=1
    continue
  fi

  if ! wasm-tools component wit "${tmpdir}" --wasm -o /dev/null >/dev/null 2>&1; then
    echo "  wasm-tools component wit failed for ${rel}" >&2
    status=1
  fi

  rm -rf "${tmpdir}"
done

exit "${status}"
