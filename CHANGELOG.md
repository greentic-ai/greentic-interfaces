# Changelog

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
