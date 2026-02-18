# PR-01: Rename upgrade -> update in 0.6 WIT interfaces

## Goals
- Rename WIT enum value **`upgrade`** to **`update`** in the 0.6 wizard / component QA interfaces.
- Regenerate bindings/lockfiles as required by the repo.
- Provide a migration bridge if the repo supports parallel legacy WIT (0.5) and 0.6.

## Implementation Steps
1) Identify the 0.6 WIT sources:
   - Locate `component-wizard-v0_6.wit`, `package.wit`, `world.wit`, or template files.
   - Find the enum defining lifecycle mode: `qa-mode` / `wizard-mode` / similar.

2) Rename enum label:
   - Replace `upgrade` -> `update` in WIT.
   - Ensure comments/docs in WIT reflect “update changes configuration”.

3) Update any WIT consumers inside this repo:
   - generated Rust bindings (wasmtime bindings or component bindings)
   - any helper enums/mappings or tests that reference `upgrade`

4) Compatibility strategy:
   - If you keep legacy WIT around, keep 0.5 untouched.
   - If you have “compat shim” interfaces, consider exporting a host-side alias: accept incoming `"upgrade"` only where it is stringly-typed.
   - If not feasible at WIT level, document that only 0.6 uses `update` and older components use old ABI.

5) Repo validation:
   - Run lint/scripts: `./wit_lint.sh` (or repo equivalent)
   - Run tests/build: `cargo test` and/or `cargo test -p greentic-interfaces-*`

6) Update any snapshot/ownership maps if present.

## Acceptance Criteria
- `rg -n "\bupgrade\b" wit src tests docs` finds no 0.6 references (except legacy 0.5 WIT if intentionally kept).
- WIT lint passes and Rust bindings compile.
- Any downstream-facing docs/READMEs show `update`.


