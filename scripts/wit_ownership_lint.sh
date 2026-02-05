#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OWNER_PKG="greentic:interfaces-types@0.1.0"
export ROOT

if ! command -v python3 >/dev/null 2>&1; then
  echo "[lint] python3 is required for wit_ownership_lint" >&2
  exit 1
fi

python3 - <<'PY'
import pathlib, re, sys
import os

root = pathlib.Path(os.environ.get("ROOT", ".")).resolve()
owner_pkg = "greentic:interfaces-types@0.1.0"
shared_types = {
    "tenant-ctx",
    "env-id",
    "tenant-id",
    "team-id",
    "user-id",
    "host-error",
    "iface-error",
}

violations = []

for path in root.rglob("*.wit"):
    parts = path.parts
    if "deps" in parts or "target" in parts:
        continue
    text = path.read_text()
    pkg_match = re.search(r"^\s*package\s+([^;]+);", text, re.MULTILINE)
    pkg = pkg_match.group(1).strip() if pkg_match else ""

    imports = re.findall(r"use\s+[^\n]*\{[^}]*\}", text, re.MULTILINE)

    for match in re.finditer(r"^\s*(record|enum|variant|type)\s+([A-Za-z0-9_-]+)\b", text, re.MULTILINE):
        name = match.group(2)
        if name not in shared_types:
            continue
        line = text.count("\n", 0, match.start()) + 1
        if pkg != owner_pkg:
            violations.append(
                f"[lint] {path} defines '{name}' but owner is {owner_pkg} (line {line})"
            )
        if any(re.search(rf"\b{name}\b", imp) for imp in imports):
            violations.append(
                f"[lint] {path} both defines and imports '{name}'. Remove the local definition."
            )

if violations:
    sys.stderr.write("\n".join(violations) + "\n")
    sys.stderr.write("[lint] Shared type ownership violations detected.\n")
    sys.exit(1)

print("[lint] Shared type ownership OK.")
PY
