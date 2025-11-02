#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIT_ROOT="${ROOT}/crates/greentic-interfaces/wit"
PACKAGE_SCRIPT="${ROOT}/scripts/package-wit.sh"
DEFAULT_OUT_DIR="${ROOT}/target/wit-packages"
DEFAULT_REGISTRY="ghcr.io"
DEFAULT_PREFIX="wit"

OUT_DIR="${DEFAULT_OUT_DIR}"
REGISTRY="${DEFAULT_REGISTRY}"
REPO_PREFIX="${DEFAULT_PREFIX}"
DRY_RUN=0
SKIP_PACKAGE=0

source "${ROOT}/scripts/wit-common.sh"

usage() {
  cat <<USAGE
Usage: $(basename "$0") [options]

Options:
  --out-dir DIR        Directory containing packaged artifacts (default: ${DEFAULT_OUT_DIR})
  --registry HOST      OCI registry host (default: ${DEFAULT_REGISTRY})
  --repo-prefix NAME   Base repository prefix (default: ${DEFAULT_PREFIX})
  --skip-package       Skip rebuilding artifacts before publishing
  --dry-run            Show actions without pushing
  -h, --help           Show this message

Environment:
  GHCR_USER / GHCR_TOKEN   Credentials for ghcr.io (required when publishing to ghcr.io)
  OCI_NAMESPACE            Repository namespace for non-ghcr registries (e.g. acme/wit)
  OCI_USER / OCI_TOKEN     Optional credentials for other registries
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir) OUT_DIR="$2"; shift 2;;
    --registry) REGISTRY="$2"; shift 2;;
    --repo-prefix) REPO_PREFIX="$2"; shift 2;;
    --skip-package) SKIP_PACKAGE=1; shift;;
    --dry-run) DRY_RUN=1; shift;;
    -h|--help) usage; exit 0;;
    *) echo "Unknown option: $1" >&2; usage >&2; exit 1;;
  esac
done

mapfile -t WIT_SOURCES < <(list_wit_sources | sort -u)
[[ ${#WIT_SOURCES[@]} -gt 0 ]] || { echo "No WIT sources found under ${WIT_ROOT}"; exit 0; }

if [[ "${SKIP_PACKAGE}" -ne 1 ]]; then
  if [[ "${DRY_RUN}" -eq 1 ]]; then
    echo "(dry-run) ${PACKAGE_SCRIPT} ${OUT_DIR}"
  else
    "${PACKAGE_SCRIPT}" "${OUT_DIR}"
  fi
fi

mkdir -p "${OUT_DIR}"

repo_root=""
if [[ "${REGISTRY}" == "ghcr.io" ]]; then
  ensure_cmd docker
  : "${GHCR_USER:?Set GHCR_USER for ghcr.io publishes}"
  : "${GHCR_TOKEN:?Set GHCR_TOKEN for ghcr.io publishes}"
  repo_root="${GHCR_USER}/${REPO_PREFIX}"
  if [[ "${DRY_RUN}" -eq 1 ]]; then
    echo "(dry-run) echo \"***\" | docker login ${REGISTRY} -u \"${GHCR_USER}\" --password-stdin"
  else
    echo "Logging into ${REGISTRY} as ${GHCR_USER}"
    echo "${GHCR_TOKEN}" | docker login "${REGISTRY}" -u "${GHCR_USER}" --password-stdin >/dev/null
  fi
else
  repo_root="${OCI_NAMESPACE:-}"
  if [[ -z "${repo_root}" ]]; then
    echo "Set OCI_NAMESPACE (e.g. acme/wit) when publishing to ${REGISTRY}" >&2
    exit 1
  fi
  if command -v docker >/dev/null 2>&1 && [[ -n "${OCI_USER:-}" && -n "${OCI_TOKEN:-}" ]]; then
    if [[ "${DRY_RUN}" -eq 1 ]]; then
      echo "(dry-run) echo \"***\" | docker login ${REGISTRY} -u \"${OCI_USER}\" --password-stdin"
    else
      echo "Logging into ${REGISTRY} as ${OCI_USER}"
      echo "${OCI_TOKEN}" | docker login "${REGISTRY}" -u "${OCI_USER}" --password-stdin >/dev/null
    fi
  fi
fi

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
  artifact="${OUT_DIR}/${sanitized}.wasm"
  if [[ ! -f "${artifact}" ]]; then
    echo "Missing artifact ${artifact}; re-run without --skip-package." >&2
    status=1
    continue
  fi

  name="${ref}"
  version_tag="latest"
  if [[ "${ref}" == *@* ]]; then
    name="${ref%@*}"
    version_tag="${ref##*@}"
  fi

  namespace="${name%%:*}"
  package="${name#*:}"
  sanitized_version="$(sanitize_ref "${version_tag}")"
  [[ -z "${sanitized_version}" ]] && sanitized_version="latest"

  image_base="${REGISTRY%/}"
  if [[ "${REGISTRY}" == "ghcr.io" ]]; then
    image="${image_base}/${repo_root}/${namespace}/${package}:${sanitized_version}"
  else
    image="${image_base}/${repo_root}/${namespace}/${package}:${sanitized_version}"
  fi

  echo "Publishing ${ref} -> ${image}"
  if [[ "${DRY_RUN}" -eq 1 ]]; then
    echo "  (dry-run) wkg oci push ${image} ${artifact}"
    continue
  fi

  if ! wkg oci push "${image}" "${artifact}"; then
    echo "  Failed to publish ${ref}" >&2
    status=1
  fi
done

exit "${status}"
