#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_PATH="${1:-${ROOT}/target/wit-packages/RELEASE_NOTES.md}"
BUNDLE_PATH="${BUNDLE_PATH:-${ROOT}/target/wit-packages/greentic-interfaces-wit-1.0.0.tar.gz}"
RUN_URL="${GITHUB_SERVER_URL:-https://github.com}/${GITHUB_REPOSITORY:-greentic-ai/greentic-interfaces}/actions/runs/${GITHUB_RUN_ID:-local}"
PKGS=(
  "component@1.0.0"
  "host@1.0.0"
  "lifecycle@1.0.0"
  "events@1.0.0"
)

declare -A FEATURE_FLAGS=(
  ["component@1.0.0"]="describe-v1"
  ["host@1.0.0"]="runner-host-v1"
  ["lifecycle@1.0.0"]="component-lifecycle-v1"
  ["events@1.0.0"]="events-v1"
)

bundle_hash="$(sha256sum "${BUNDLE_PATH}" | awk '{print $1}')"

cat >"${OUT_PATH}" <<EOF_NOTES
# Release provenance

- bundle: \
  \
  	\`${bundle_hash}\` — $(basename "${BUNDLE_PATH}")
- workflow run: ${RUN_URL}

## Package hashes

EOF_NOTES

for pkg in "${PKGS[@]}"; do
  file="${ROOT}/crates/greentic-interfaces/wit/greentic/${pkg}/package.wit"
  if [[ ! -f "${file}" ]]; then
    echo "Missing ${file}" >&2
    exit 1
  fi
  hash="$(sha256sum "${file}" | awk '{print $1}')"
  flag="${FEATURE_FLAGS["${pkg}"]}"
  printf -- "- %s (feature \`%s\`): \`%s\`\n" "${pkg}" "${flag}" "${hash}" >>"${OUT_PATH}"

done

echo >>"${OUT_PATH}"
echo "Release notes written to ${OUT_PATH}" >&2
