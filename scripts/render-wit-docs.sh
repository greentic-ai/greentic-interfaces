#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${1:-${ROOT}/target/wit-pages}"
PKGS=("component@1.0.0" "host@1.0.0" "lifecycle@1.0.0" "events@1.0.0")
FEATURES=("describe-v1" "runner-host-v1" "component-lifecycle-v1" "events-v1")

rm -rf "${OUT_DIR}"
mkdir -p "${OUT_DIR}"

cat > "${OUT_DIR}/index.html" <<'HTML'
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Greentic WIT 1.0.0</title>
    <style>
      body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; margin: 2rem; }
      h1 { font-size: 1.8rem; }
      ul { line-height: 1.6; }
      code { background: #f4f4f4; padding: 0.1rem 0.3rem; border-radius: 4px; }
    </style>
  </head>
  <body>
    <h1>Greentic WIT packages @ 1.0.0</h1>
    <p>The following packages are exported from the <code>greentic-interfaces</code> workspace.</p>
    <ul>
HTML

for idx in "${!PKGS[@]}"; do
  pkg="${PKGS[${idx}]}"
  feature="${FEATURES[${idx}]}"
  src="${ROOT}/crates/greentic-interfaces/wit/greentic/${pkg}/package.wit"
  dest="${OUT_DIR}/${pkg}"
  mkdir -p "${dest}"
  cp "${src}" "${dest}/package.wit"
  rel_path="./${pkg}/package.wit"
  cat >> "${OUT_DIR}/index.html" <<HTML
      <li><strong>${pkg}</strong> (feature <code>${feature}</code>) — <a href="${rel_path}">package.wit</a></li>
HTML
done

cat >> "${OUT_DIR}/index.html" <<'HTML'
    </ul>
  </body>
</html>
HTML

echo "Wrote WIT docs to ${OUT_DIR}"
