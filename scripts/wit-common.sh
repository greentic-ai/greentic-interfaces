#!/usr/bin/env bash

# Common helpers for working with the Greentic WIT workspace.
# Assumes the caller defines ROOT (repository root) and WIT_ROOT pointing at
# the directory that contains the WIT sources (crates/greentic-interfaces/wit).

ensure_cmd() {
  local cmd="$1"
  command -v "${cmd}" >/dev/null 2>&1 || {
    echo "Error: required command '${cmd}' is not available in PATH." >&2
    exit 1
  }
}

# Replace any character not allowed in file names / OCI tags with "-".
sanitize_ref() {
  local ref="$1"
  echo "${ref}" | tr -c 'A-Za-z0-9._-' '-' | sed -e 's/--*/-/g' -e 's/^-//' -e 's/-$//'
}

list_wit_sources() {
  find "${WIT_ROOT}" -maxdepth 1 -type f -name "*.wit"
  find "${WIT_ROOT}" -mindepth 1 -type f -name "package.wit" ! -path "*/deps/*"
}

package_ref_from_file() {
  local file="$1"
  local line
  line="$(grep -m1 '^package ' "${file}" || true)"
  [[ -z "${line}" ]] && return 1
  local ref="${line#package }"
  ref="${ref%;}"
  ref="${ref%% }"
  ref="${ref## }"
  echo "${ref}"
}

dest_dir_for_ref() {
  local ref="$1"
  local sanitized="${ref//[:@]/-}"
  sanitized="${sanitized//\//-}"
  echo "${sanitized}"
}

parse_deps() {
  local file="$1"
  local regex='(use|import)[[:space:]]+([[:alnum:]:-]+)/[[:alnum:]_.-]+@([0-9A-Za-z._-]*[0-9A-Za-z_-])'
  local deps=()
  while IFS= read -r line; do
    if [[ $line =~ $regex ]]; then
      deps+=("${BASH_REMATCH[2]}@${BASH_REMATCH[3]}")
    fi
  done < "${file}"
  if [[ ${#deps[@]} -gt 0 ]]; then
    printf '%s\n' "${deps[@]}" | sort -u
  fi
}

resolve_wit_source() {
  local ref="$1"
  local pkg="${ref%@*}"
  local ver="${ref##*@}"
  local namespace="${pkg%%:*}"
  local remainder="${pkg#*:}"
  local dir="${WIT_ROOT}"
  if [[ "${pkg}" == "${remainder}" ]]; then
    dir="${dir}/${pkg}@${ver}"
  else
    dir="${dir}/${namespace}/${remainder}@${ver}"
  fi
  dir="${dir//\/\//\/}"
  local package_wit="${dir}/package.wit"
  if [[ -f "${package_wit}" ]]; then
    echo "${package_wit}"
    return 0
  fi

  local found
  found="$(grep -R -F -l "package ${ref};" "${WIT_ROOT}" | head -n1 || true)"
  if [[ -n "${found}" ]]; then
    echo "${found}"
    return 0
  fi

  return 1
}

copy_with_deps() {
  local ref="$1"
  local dest_root="$2"

  [[ -z "${ref}" ]] && return 0

  local rel_dest
  rel_dest="$(dest_dir_for_ref "${ref}")"
  local dest_dir="${dest_root}/${rel_dest}"
  if [[ -d "${dest_dir}" ]]; then
    return 0
  fi

  local src
  if ! src="$(resolve_wit_source "${ref}")"; then
    echo "Missing dependency ${ref}" >&2
    return 1
  fi

  mkdir -p "${dest_dir}"
  cp "${src}" "${dest_dir}/package.wit"
  local src_dir
  src_dir="$(dirname "${src}")"
  if [[ "$(basename "${src}")" == "package.wit" && -d "${src_dir}/deps" ]]; then
    cp -R "${src_dir}/deps" "${dest_dir}/"
  fi

  local subdeps
  subdeps="$(parse_deps "${src}")"
  if [[ -n "${subdeps}" ]]; then
    mkdir -p "${dest_dir}/deps"
    while IFS= read -r dep_ref; do
      [[ -z "${dep_ref}" ]] && continue
      copy_with_deps "${dep_ref}" "${dest_dir}/deps" || return 1
    done <<< "${subdeps}"
  fi
}

prepare_package_layout() {
  local source_file="$1"
  local tmpdir
  tmpdir="$(mktemp -d)"

  cp "${source_file}" "${tmpdir}/package.wit"

  local src_dir
  src_dir="$(dirname "${source_file}")"
  if [[ "$(basename "${source_file}")" == "package.wit" && -d "${src_dir}/deps" ]]; then
    cp -R "${src_dir}/deps" "${tmpdir}/"
  fi

  local deps
  deps="$(parse_deps "${source_file}")"
  if [[ -n "${deps}" ]]; then
    mkdir -p "${tmpdir}/deps"
    while IFS= read -r dep_ref; do
      [[ -z "${dep_ref}" ]] && continue
      if ! copy_with_deps "${dep_ref}" "${tmpdir}/deps"; then
        rm -rf "${tmpdir}"
        return 1
      fi
    done <<< "${deps}"
  fi

  printf '%s\n' "${tmpdir}"
}
