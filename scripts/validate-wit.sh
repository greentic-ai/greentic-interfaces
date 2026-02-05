#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIT_ROOT="${1:-${ROOT}/crates/greentic-interfaces/wit}"
CANONICAL_WIT_ROOT="${CANONICAL_WIT_ROOT:-${ROOT}/crates/greentic-interfaces/wit}"

source "${ROOT}/scripts/wit-common.sh"

ensure_cmd wasm-tools
HAVE_WKG=0
if command -v wkg >/dev/null 2>&1; then
  HAVE_WKG=1
else
  echo "[info] wkg not found; skipping wkg wit build checks" >&2
fi
WKG_STRICT=${WKG_STRICT:-0}
WKG_FULL_STAGE=${WKG_FULL_STAGE:-0}
WKG_ONLINE=${WKG_ONLINE:-1}
if [[ -n "${CI:-}" ]]; then
  WKG_ONLINE=0
fi
if [[ -n "${LOCAL_CHECK_ONLINE:-}" && "${LOCAL_CHECK_ONLINE}" != "1" ]]; then
  WKG_ONLINE=0
fi
WIT_KEEP_TMP=${WIT_KEEP_TMP:-0}

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

  case "$(basename "${source}")" in
    host_imports.wit|pack_api.wit|provider.wit|world.wit)
      echo "  skipping non-package WIT file"
      continue
      ;;
  esac
  if [[ "$(basename "${source}")" == wasix-mcp@*.wit ]]; then
    echo "  skipping legacy standalone WIT"
    continue
  fi

  if [[ "${WKG_FULL_STAGE}" == "1" ]]; then
    tmpdir="$(prepare_package_layout_full "${source}" 2>/dev/null || true)"
  else
    tmpdir="$(prepare_package_layout "${source}" 2>/dev/null || true)"
  fi
  if [[ -z "${tmpdir}" ]]; then
    echo "  Failed to stage ${rel}" >&2
    status=1
    continue
  fi

  if ! wasm-tools component wit "${tmpdir}" --wasm -o /dev/null >/dev/null 2>&1; then
    echo "  wasm-tools component wit failed for ${rel}" >&2
    status=1
  fi

  if [[ ${HAVE_WKG} -eq 1 && "${WKG_ONLINE}" == "1" && "$(basename "${source}")" == "package.wit" ]]; then
    out_path="${tmpdir}/wkg-package.wasm"
    wkg_log="${tmpdir}/wkg-build.log"
    if [[ -d "${tmpdir}/deps" && -z "$(ls -A "${tmpdir}/deps")" ]]; then
      echo "  wkg skipped for ${rel} (empty deps directory)" >&2
      if [[ "${WIT_KEEP_TMP}" == "1" ]]; then
        echo "  [debug] keeping tmpdir: ${tmpdir}" >&2
      else
        rm -rf "${tmpdir}"
      fi
      continue
    fi
    if [[ -d "${tmpdir}/deps" ]]; then
      python3 - <<'PY' "${tmpdir}"
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
deps = root / "deps"
overrides = {}
if deps.is_dir():
    for pkg_wit in deps.rglob("package.wit"):
        try:
            line = next(l for l in pkg_wit.read_text().splitlines() if l.startswith("package "))
        except StopIteration:
            continue
        ref = line.replace("package ", "").replace(";", "").strip()
        pkg = ref.split("@", 1)[0]
        overrides[pkg] = str(pkg_wit.parent)

if overrides:
    lines = ["[overrides]"]
    for pkg, path in sorted(overrides.items()):
        lines.append(f"\"{pkg}\" = {{ path = \"{path}\" }}")
    (root / "wkg.toml").write_text("\n".join(lines) + "\n")
PY
    fi
    wkg_args=(wit build --wit-dir "${tmpdir}" --output "${out_path}")
    if ! (cd "${tmpdir}" && wkg "${wkg_args[@]}") >"${wkg_log}" 2>&1; then
      echo "  wkg wit build failed for ${rel}" >&2
      echo "  --- wkg tail ---" >&2
      tail -n 80 "${wkg_log}" >&2 || true
      echo "  ---------------" >&2
      if [[ ${WKG_STRICT} -eq 1 ]]; then
        status=1
      else
        echo "    (warning only; set WKG_STRICT=1 to make this fatal)" >&2
      fi
    fi
  fi

  if [[ "${WIT_KEEP_TMP}" == "1" ]]; then
    echo "  [debug] keeping tmpdir: ${tmpdir}" >&2
  else
    rm -rf "${tmpdir}"
  fi
done

exit "${status}"
