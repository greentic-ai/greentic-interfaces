# Changelog

## Unreleased
- Add tenant-scoped supply-chain WIT worlds @1.0.0: source-sync, builder, scanner, signer, attester, policy-evaluator, metadata-store, and oci-distribution, all using shared `TenantCtx` and structured records with provider-agnostic `extra` fields.

## v0.4.20
- Add `greentic:component@1.0.0` describe-only world so runners can fetch schema/default metadata over JSON without binding to a specific component ABI.
- Add `greentic:host@1.0.0` with `http-v1`, `secrets-v1`, and `kv-v1` host imports to model runner capabilities that mocks/dev profiles can override.
- Add `greentic:lifecycle@1.0.0` optional exports (`init`, `health`, `shutdown`) that runners can probe and invoke when implemented.
- Add `greentic:events@1.0.0` envelope record for shared telemetry/event payloads.
- Keep the new contracts opt-in via the `describe-v1`, `runner-host-v1`, `component-lifecycle-v1`, and `events-v1` features (use the new `wit-all` helper to flip everything on) and release `greentic-interfaces-wasmtime` 0.2.3 so the Wasmtime helpers pick up the 1.0 packages.

### Migration
- Existing consumers remain unchanged. To call `describe-v1` from Wasmtime, enable the `describe-v1` feature (or `wit-all`) on `greentic-interfaces` and depend on `greentic-interfaces-wasmtime` ≥ 0.2.3 along with the bundled smoke-example pattern.

## v0.4.18
- Add optional `session-id`, `flow-id`, `node-id`, and `provider-id` fields to `TenantCtx` in the 0.4.x WIT contracts and regenerate bindings.
- Extend the Rust mappers/tests to round-trip the new identifiers while preserving backward compatibility.

## v0.4.9
- Stage generated WIT bundles inside `OUT_DIR` and expose `WIT_STAGING_DIR` so builds succeed from Cargo's read-only registry.
- Restore the deprecated `host_import_v0_2` module as a compatibility shim for `greentic-mcp` while it migrates to the Wasmtime crate.
- Reintroduce the `host_import_v0_4` Wasmtime module backed by the staged `greentic:host-import@0.4.0` package.
- Add pure Rust bindings for `component_v0_4`, `pack_export_v0_4`, and `types_core_v0_4` using the archived 0.4 WIT contracts (no Wasmtime dependency).
- Document the new staging behavior and add a crates.io consumer example.

## v0.4.0
- Refresh `greentic:types-core@0.4.0`, `greentic:host-import@0.4.0`, and `greentic:pack-export@0.4.0` to the shared WIT v0.4 contracts (flow/tenant types, secrets + telemetry + HTTP host imports, pack discovery/execution/A2A search).
- Stage WIT dependencies for Wasmtime bindgen (`build.rs` -> `target/wit-bindgen/`) and update `scripts/*wit.sh` to validate/package multi-file packages.
- Wire the new Rust bindings (`types_core_v0_4`, `host_import_v0_4`, `pack_export_v0_4`) and keep `wasix:mcp@0.0.5` byte-for-byte.

## v0.2.2
- Add `wasix:mcp@0.0.5` WIT package and reference bindings.
- Document MCP router module and bump crate to `0.2.2`.

## v0.2.1
- Document new MCP executor and update scripts/CI to validate/package it.
- Bump crate to `0.2.1`.

## v0.2.0
- Add `greentic:types-core@0.2.0` for shared tenant/flow/run structures.
- Introduce `greentic:host-import@0.2.0` for secrets, telemetry, tool, and HTTP host callbacks.
- Add `greentic:pack-export@0.2.0` describing pack execution and discovery APIs.
- Drop the unused `greentic:component@0.3.0` schema and `component_v0_3` bindings.
- Expand the Rust crate with `types_core_v0_2`, `host_import_v0_2`, and `pack_export_v0_2` modules.
- Update validation/publish scripts to cover all new WIT packages.
