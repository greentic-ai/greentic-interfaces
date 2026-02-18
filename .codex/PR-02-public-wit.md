# PR-02: Canonical WIT Refactor (Full Enforcement Now)

## Objective

- One canonical WIT source:

  `crates/greentic-interfaces/wit/`

- All other crates:
  - MUST NOT contain canonical `greentic:*` package definitions.
  - MUST generate bindings using the canonical WIT path.
  - MUST fail CI if duplication is introduced.

- Enforce via CI guardrail.
- No semantic WIT changes.
- Workspace builds must pass.

## 1) Canonical Source of Truth

Canonical WIT root:

`crates/greentic-interfaces/wit/`

This directory is now the only legal location for:

- `package greentic:component@*`
- `package greentic:lifecycle@*`
- Any shared `greentic:*` package intended for reuse

Everything else must import from here.

## 2) Remove All Duplicate Canonical WIT Trees

In these crates (and any others discovered):

- `greentic-interfaces-guest`
- `greentic-interfaces-wasmtime`
- `greentic-interfaces-host`
- any other crate under `greentic-interfaces` workspace

Delete duplicated canonical trees such as:

- `wit/greentic/component@0.4.0/`
- `wit/greentic/component@0.5.0/`
- `wit/greentic/component@0.6.0/`
- `wit/greentic/component@1.0.0/`
- `wit/greentic/lifecycle@1.0.0/`

If a crate has:

`wit/greentic/component@0.6.0/package.wit`

and that same package exists under canonical root -> delete the crate-local version.

Keep only:

- crate-specific packages
- non-canonical packages

## 3) Update All Binding Generation to Use Canonical Path

Each binding-generating crate must use workspace-relative canonical path.

Add a shared helper file:

`crates/greentic-interfaces/build_support/wit_paths.rs`

`wit_paths.rs`:

```rust
use std::path::PathBuf;

pub fn canonical_wit_root() -> PathBuf {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    manifest_dir
        .join("../greentic-interfaces/wit")
        .canonicalize()
        .expect("Failed to locate canonical WIT root")
}
```

In each `build.rs` that generates bindings:

Replace:

```rust
let wit_dir = PathBuf::from("wit");
```

With:

```rust
include!("../greentic-interfaces/build_support/wit_paths.rs");

let wit_dir = canonical_wit_root();
```

Also ensure:

```rust
println!("cargo:rerun-if-changed={}", wit_dir.display());
```

## 4) Strict Guardrail Script (Fail Hard)

Create:

`ci/check_no_duplicate_canonical_wit.sh`

Final version (strict):

```bash
#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-.}"

CANONICAL="$ROOT/crates/greentic-interfaces/wit"

# Pattern for ALL canonical greentic packages
PATTERN='^package\s+greentic:'

MATCHES_ALL="$(rg -n --hidden --glob '!.git/*' --glob '*.wit' \
  --glob '!**/target/**' \
  --glob '!**/out/**' \
  --glob '!**/wit-staging/**' \
  --glob '!**/wit-staging-wasmtime/**' \
  "$PATTERN" "$ROOT" || true)"

if [[ -z "$MATCHES_ALL" ]]; then
  echo "ERROR: No greentic:* packages found. Guardrail misconfigured."
  exit 1
fi

DUPES="$(echo "$MATCHES_ALL" | rg -v "^${CANONICAL}/" || true)"

if [[ -n "$DUPES" ]]; then
  echo "ERROR: Canonical greentic:* packages declared outside canonical root:"
  echo
  echo "$DUPES"
  echo
  echo "Canonical root is: $CANONICAL"
  exit 1
fi

echo "OK: No duplicated canonical WIT packages detected."
```

## 5) Add CI Enforcement

In top-level GitHub workflow (or main CI job), add step:

```yaml
- name: Check for duplicate canonical WIT
  run: bash ci/check_no_duplicate_canonical_wit.sh .
```

Place this before cargo build/test.

## 6) Update Docs (Minimal, Clear)

Add to repo root `README.md`:

```md
## Canonical WIT Policy

All shared `greentic:*` WIT packages live exclusively under:

    crates/greentic-interfaces/wit/

No other crate may define or copy these packages.
Binding generation must reference the canonical path via build.rs.

CI enforces this.
```

## 7) Validation Checklist

After implementation:

- `cargo clean`
- `cargo check --workspace`
- `cargo test --workspace`
- Run duplication script manually
- Ensure no crate contains canonical WIT copies

## 8) What This Immediately Fixes

- Eliminates version confusion (0.5 vs 0.6 staging bleed)
- Stops drift between guest/wasmtime/host
- Prevents future duplication
- Makes world migration to 0.6 deterministic
- Makes future 0.7 upgrade clean

## 9) Risks (Accepted)

Standalone crates outside workspace may break if they depended on crate-local WIT paths.

Acceptable for now.

If crates.io standalone support is required later -> implement Option C (WIT package crate).

## Final State

After this PR:

- There is exactly one place where canonical WIT lives.
- Every crate consumes it.
- CI enforces it.
- No more silent duplication.
- No more accidental 0.5 bleed-through.
