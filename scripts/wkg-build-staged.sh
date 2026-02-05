#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIT_ROOT="${WIT_ROOT:-${ROOT}/crates/greentic-interfaces/wit}"
CANONICAL_WIT_ROOT="${CANONICAL_WIT_ROOT:-${ROOT}/crates/greentic-interfaces/wit}"
source "${ROOT}/scripts/wit-common.sh"

TARGET_WIT="${1:-${ROOT}/crates/greentic-interfaces/wit/greentic/attestation@1.0.0/package.wit}"
if [[ ! -f "${TARGET_WIT}" ]]; then
  echo "error: missing WIT package: ${TARGET_WIT}" >&2
  exit 1
fi

KEEP_TMP="${KEEP_TMP:-1}"
TMPDIR_BASE="${TMPDIR:-/tmp}"
export TMPDIR="${TMPDIR_BASE}"
TMPDIR=""
cleanup() {
  if [[ -n "${TMPDIR}" && "${KEEP_TMP}" != "1" ]]; then
    rm -rf "${TMPDIR}"
  fi
}
trap cleanup EXIT

TMPDIR="$(prepare_package_layout_full "${TARGET_WIT}")"

echo "[stage] staged WIT package: ${TARGET_WIT}"
echo "[stage] tmpdir: ${TMPDIR}"
echo "[stage] deps staged:"
if [[ -d "${TMPDIR}/deps" ]]; then
  find "${TMPDIR}/deps" -type f -print | sed 's/^/  - /'
else
  echo "  - (none)"
fi

# Sanity checks: ensure interfaces-types is fully staged and exposes host-error in interface types.
interfaces_types_dst="${TMPDIR}/deps/greentic-interfaces-types-0.1.0"
if [[ ! -d "${interfaces_types_dst}" ]]; then
  echo "[error] greentic-interfaces-types-0.1.0 not staged under deps/" >&2
  exit 1
fi
if ! rg -n "interface[[:space:]]+types" "${interfaces_types_dst}" >/dev/null; then
  echo "[error] staged greentic-interfaces-types-0.1.0 missing interface types" >&2
  exit 1
fi
if ! grep -q 'include "types.wit";' "${interfaces_types_dst}/package.wit"; then
  echo "[error] staged greentic-interfaces-types-0.1.0 package.wit missing include \"types.wit\"" >&2
  exit 1
fi
if ! rg -n "host-error" "${interfaces_types_dst}" >/dev/null; then
  echo "[error] staged greentic-interfaces-types-0.1.0 missing host-error" >&2
  exit 1
fi
if ! rg -n "iface-error" "${interfaces_types_dst}" >/dev/null; then
  echo "[error] staged greentic-interfaces-types-0.1.0 missing iface-error" >&2
  exit 1
fi

wkg wit build --wit-dir "${TMPDIR}" --output "${TMPDIR}/wkg-package.wasm"

echo "[ok] wkg-package.wasm generated at ${TMPDIR}/wkg-package.wasm"
if [[ "${KEEP_TMP}" == "1" ]]; then
  echo "[info] KEEP_TMP=1 set; tmpdir preserved at ${TMPDIR}"
fi
