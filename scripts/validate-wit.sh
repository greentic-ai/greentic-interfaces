#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIT_ROOT="${1:-${ROOT}/crates/greentic-interfaces/wit}"

source "${ROOT}/scripts/wit-common.sh"

ensure_cmd wasm-tools
HAVE_WKG=0
if command -v wkg >/dev/null 2>&1; then
  HAVE_WKG=1
else
  echo "[info] wkg not found; skipping wkg wit build checks" >&2
fi
WKG_STRICT=${WKG_STRICT:-0}

WIT_SOURCES=()
while IFS= read -r src; do
  if [[ -n "${src}" ]]; then
    WIT_SOURCES+=("${src}")
  fi
done < <(list_wit_sources | sort -u)

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

  if [[ ${HAVE_WKG} -eq 1 ]]; then
    out_path="${tmpdir}/wkg-package.wasm"
    if ! wkg wit build --wit-dir "${tmpdir}" --output "${out_path}" >/dev/null 2>&1; then
      echo "  wkg wit build failed for ${rel}" >&2
      if [[ ${WKG_STRICT} -eq 1 ]]; then
        status=1
      else
        echo "    (warning only; set WKG_STRICT=1 to make this fatal)" >&2
      fi
    fi
  fi

  rm -rf "${tmpdir}"
done

exit "${status}"
