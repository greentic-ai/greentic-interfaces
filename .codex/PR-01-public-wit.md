# PR-01: Public guest export wrapper for `greentic:component@0.6.0`

## Scope
This PR is scoped to exposing a stable public guest export wrapper macro for v0.6 only.

It does **not** include WIT tree restructuring or single-source enforcement across crates.

## Goal
1. Make it possible for downstream crates to export a `greentic:component/component@0.6.0` implementation without touching private generated macros.
2. Provide a stable public macro API:
   - `greentic_interfaces_guest::export_component_v060!(MyImpl);`
3. Keep API behavior predictable via existing feature gating.

## Implementation Decisions
1. **Do not rely on changing wit-bindgen generated `pub(crate) use ... as export;` as primary strategy.**
2. Prefer a crate-owned stable wrapper macro that does not require generated `export` to be public.
3. `export_component_v060!` must be available only with `feature = "component-v0-6"`.
4. Add a real CI-checked compile regression test.
5. Add usage docs in `crates/greentic-interfaces-guest/src/lib.rs` rustdoc.

## Preferred Implementation Order
1. **Option A (preferred):** wrapper macro calls the generated internal `__export_*` path from inside the crate (`$crate::...`).
2. **Option B (fallback only):** post-process generated output in `build.rs` (or generate a tiny public shim) if Option A is not feasible.

Choose Option A if possible. Use Option B only when A cannot be made to work.

## Testing Requirement
Preferred test style: `trybuild` compile test.

- Add `trybuild` as a dev-dependency.
- Add a compile-pass test source under `crates/greentic-interfaces-guest/tests/trybuild/` using `export_component_v060!(Impl)` with a minimal v0.6 implementation.
- Add/extend a Rust test that runs:
  - `trybuild::TestCases::new().pass("tests/trybuild/*.rs");`

Alternative (`cargo check` shell-out fixture) is acceptable only if `trybuild` is unsuitable.

## Documentation Requirement
Document usage in `crates/greentic-interfaces-guest/src/lib.rs` rustdoc (near v0.6 exports):

```rust
// Requires feature = "component-v0-6"
greentic_interfaces_guest::export_component_v060!(MyImpl);
```

No new per-crate README is required in this PR.

## Explicit Non-Goal
This PR does **not** remove or restructure duplicated `wit/greentic/component@0.6.0` trees across guest/host/wasmtime crates.

PR description should include:

> "This PR only exposes a public guest export wrapper for v0.6 and adds a compile-time test. It does not restructure WIT sources or enforce single-source policy."

## Acceptance Criteria
1. With `features = ["component-v0-6"]`, downstream code can compile:
   - `greentic_interfaces_guest::export_component_v060!(MyImpl);`
2. CI has a compile test (`trybuild` or equivalent) that fails on regression.
3. No downstream change is required besides enabling existing `component-v0-6` feature.
