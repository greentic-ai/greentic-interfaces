You are working in the greentic-interfaces repository, specifically the crate `greentic-interfaces-wasmtime`.

Goal
Fix published build breakage (v0.4.95) caused by WIT dependency resolution failures like:
- `package 'greentic:interfaces-types@0.1.0' not found` when bindgen runs on per-package staging dirs.
Implement Option 1: run `wasmtime::component::bindgen!` against ONE canonical WIT root that contains ALL packages, so `use greentic:interfaces-types/...` resolves without staging deps.
Do this WITHOUT replicating/copying WIT files across crates: there must be a single canonical WIT tree.

Constraints
- Do not duplicate canonical WIT under greentic-interfaces-guest / greentic-interfaces-wasmtime / host, etc.
- `greentic-interfaces` crate remains the single canonical WIT owner (one committed WIT tree).
- `greentic-interfaces-wasmtime` should reference that canonical tree at build time via a stable helper exported by `greentic-interfaces` (publish-safe), not via `../` paths.

High-level approach (Option 1)
1) Ensure canonical WIT root is accessible as a Path from the `greentic-interfaces` crate:
   - Add/confirm `pub fn wit_root() -> PathBuf` in `greentic-interfaces` returning `PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("wit")`.
   - This must work for crates.io builds (points to registry source dir).
2) In `greentic-interfaces-wasmtime` build pipeline:
   - STOP generating / using per-package directories like `target/wit-staging-wasmtime/greentic-http-1.0.0` as bindgen input paths.
   - Instead, call `wasmtime::component::bindgen!` with `path: greentic_interfaces::wit_root()` for every world.
   - Keep specifying the `world: "greentic:http/client@1.0.0"` (etc.) per bindgen invocation.
   - If the code currently autogenerates `out/gen_all_worlds.rs`, update it so each invocation uses the SAME root path.
   - Do not create `deps/` subtrees; do not copy WIT files.
3) Make sure the canonical root contains the required dependency packages referenced by your WIT (including `greentic:interfaces-types@0.1.0` and any others). Add missing packages to the canonical root if needed (once), not per crate.
4) Packaging correctness:
   - Ensure `greentic-interfaces` published crate includes the `wit/**` directory in the crate package (avoid restrictive `include` that drops it).
   - Ensure `greentic-interfaces-wasmtime` depends on `greentic-interfaces` (normal dependency) so it can call `greentic_interfaces::wit_root()` in build.rs (build-dependency if needed).
5) Add regression tests that prove this works from crates.io packaging perspective:
   - Add a CI step or test that runs `cargo publish --dry-run -p greentic-interfaces -p greentic-interfaces-wasmtime` (or at least `cargo package` for both).
   - Add a minimal “external consumer” test:
     - Create a tiny temp crate in `tests/` (or a script) that depends on `greentic-interfaces-wasmtime = { path = ... }` for local, then ensure it also works in a publish-dry-run scenario.
   - The key is to catch missing `wit/**` at publish time.

Implementation details to check and update
- Locate `greentic-interfaces-wasmtime/build.rs` (or `wit_build.rs`) and the generator that writes `out/gen_all_worlds.rs`.
- Remove any logic that builds per-package wit staging directories for wasmtime bindgen input.
- Replace it with a single `let wit_root = greentic_interfaces::wit_root();` and embed `wit_root.display().to_string()` as the `path` for bindgen invocations.

Acceptance criteria
- `cargo build` in an unrelated repo depending on published greentic-interfaces-wasmtime no longer fails with missing `greentic:interfaces-types@0.1.0`.
- No duplicated canonical WIT trees are introduced anywhere; canonical WIT exists only once in `greentic-interfaces/wit`.
- `cargo publish --dry-run` passes for the relevant crates.
- Existing guardrail (no duplicate canonical WIT) remains green.

Now implement:
- Reproduce the current failure locally (simulate consumer build).
- Make the changes for Option 1.
- Add the regression test/CI guard for publish-dry-run.
- Ensure everything builds and tests pass.

## Review decisions (applied)

1) `wit_root()` behavior
- Strict and deterministic: `PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("wit")`
- No fallback lookup logic.
- Build scripts must fail loudly if this path is missing.

2) Regression guard placement
- Add guardrails in `.github/workflows/ci.yml` for fast PR failure:
  - `cargo package -p greentic-interfaces -p greentic-interfaces-wasmtime`
- Keep publish workflow behavior as-is.

3) External consumer smoke test
- Add a dedicated script harness instead of relying on examples:
  - `ci/smoke/wasmtime_consumer.sh`
- Script responsibilities:
  - Create temp crate
  - Depend on `greentic-interfaces-wasmtime` by workspace path
  - `cargo build`
  - Package `greentic-interfaces` and assert packaged `.crate` contains `wit/**`
