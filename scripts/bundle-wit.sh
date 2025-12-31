#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-${ROOT}/target/wit-packages}"
# Ensure OUT_DIR is absolute so later subshells (cd into tmp dirs) can still
# write artifacts using the same path.
if [[ "${OUT_DIR}" != /* ]]; then
  OUT_DIR="${ROOT}/${OUT_DIR}"
fi
BUNDLE_VERSION="1.0.0"
BUNDLE_NAME="greentic-interfaces-wit-${BUNDLE_VERSION}"
TMP_DIR="${OUT_DIR}/${BUNDLE_NAME}"

PKGS=("component@1.0.0" "host@1.0.0" "lifecycle@1.0.0")

rm -rf "${TMP_DIR}"
mkdir -p "${TMP_DIR}"

for pkg in "${PKGS[@]}"; do
  src="${ROOT}/crates/greentic-interfaces/wit/greentic/${pkg}"
  dest="${TMP_DIR}/${pkg}"
  if [[ ! -d "${src}" ]]; then
    echo "Missing WIT package ${pkg} at ${src}" >&2
    exit 1
  fi
  cp -R "${src}" "${dest}"
done

tarball="${OUT_DIR}/${BUNDLE_NAME}.tar.gz"
rm -f "${tarball}"
(cd "${TMP_DIR}" && tar -czf "${tarball}" .)
sha256sum "${tarball}" > "${tarball}.sha256"
echo "Bundle ready at ${tarball}"
