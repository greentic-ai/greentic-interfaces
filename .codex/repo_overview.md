# Repository Overview

## 1. High-Level Purpose
- Provides the authoritative Greentic WebAssembly Interface Types (WIT) contracts plus generated Rust bindings for both hosts and guests. Covers core types, host import bundles, pack/component exports, telemetry, HTTP, OAuth, events, supply-chain providers, MCP router snapshots, and related utilities.
- Supplies Wasmtime integration helpers and minimal example components/hosts so downstream runtimes and packs can wire the contracts without regenerating WIT locally.

## 2. Main Components and Functionality
- **Path:** `crates/greentic-interfaces`  
  **Role:** ABI crate containing WIT sources and generated bindings.  
  **Key functionality:** Build script stages all WIT packages under `wit/greentic/*@*`; `bindings` exposes `wit-bindgen` output under `bindings::greentic::*`; `mappers` convert WIT types to/from `greentic-types` (TenantCtx, Outcome, AllowList, SpanContext, PackRef, etc.); `validate` checks provider metadata invariants; `wit_all` re-exports feature-gated bindings for convenience. Integration tests validate WIT staging, MCP packages, provider schema snapshot, and mapper round-trips. Adds the GUI fragment world `greentic:gui/gui-fragment@1.0.0` (server-rendered HTML fragments) and the distributor API world `greentic:distributor-api@1.0.0` (resolve/get/warm) while keeping `greentic:distribution@1.0.0` marked experimental.  
  **Key dependencies / integration points:** Depends on `greentic-types` for rich models; consumers set `GREENTIC_INTERFACES_BINDINGS` env from build.rs output.
- **Path:** `crates/greentic-interfaces-host`  
  **Role:** Host-facing re-export of curated bindings/mappers.  
  **Key functionality:** Namespaced modules for component exports (0.4/v1, lifecycle), legacy and runner host import bundles, pack export worlds, core type packages, and v1 capability contracts (secrets/state/messaging/events/bridge/http/telemetry/oauth/worker/supply-chain/distribution/distributor-api). Adds host feature gates `worker-v1` and `oauth-broker-v1` that expose stable modules `worker::*` and `oauth_broker::*` for the corresponding WIT worlds, plus compile-only tests under `tests/` to ensure the feature surfaces stay available. Host-friendly worker helpers (`HostWorkerRequest/Response` with serde_json payloads) map to/from WIT worker types using `greentic-types::TenantCtx` via mappers. Optional GUI fragment host bindings (`gui_fragment` feature) plus MCP snapshots and repo UI actions re-exported. Denies wasm32 targets.  
  **Key dependencies / integration points:** Wraps `greentic-interfaces` with the `wit-all` feature enabled.
- **Path:** `crates/greentic-interfaces-guest`  
  **Role:** Guest-facing bindings for `wasm32-wasip2` components.  
  **Key functionality:** Build script stages WIT packages (from local `wit/` or fallback to the ABI crate) and generates bindings for enabled worlds; feature flags map to each world and default `guest` enables the full surface; macro `export_component_node!` exports the 0.4 node world; modules expose imports/exports for lifecycle, secrets/state/messaging/events/bridge/http/telemetry/oauth/worker, supply-chain provider worlds, deploy/distribution/distributor-api/pack-export, runner host bundles, MCP snapshots, repo UI actions, and the GUI fragment world (`gui-fragment` feature). Optional `host-bridge` feature re-exports mappers when building native. Distributor API also exposes import-side bindings plus a `DistributorApiImports` wrapper under the `distributor-api-imports` feature for calling resolve/get/warm from guests.  
  **Key dependencies / integration points:** Uses `wit-bindgen` 0.48; optionally depends on `greentic-interfaces` when bridging on host.
- **Path:** `crates/greentic-interfaces-wasmtime`  
  **Role:** Wasmtime glue for all Greentic WIT worlds.  
  **Key functionality:** Build script discovers every `wit/greentic/*@*/package.wit` world and emits `component::bindgen` modules with a `Component::instantiate` helper (plus optional control helpers); `events_bridge` module aliases typed bridge helpers. Tests verify modules compile and exercise repo-ui-actions and distributor-api component round-trips under Wasmtime+p2 WASI.  
  **Key dependencies / integration points:** Relies on Wasmtime component model; consumes staged WIT sources in this crate.
- **Path:** `examples/component-describe`  
  **Role:** Minimal `wasm32-wasip2` component exporting `describe-json` for the describe-only world.  
  **Key functionality:** Uses `wit_bindgen` against `component@1.0.0`, returns static JSON schema; gated to wasm builds.
- **Path:** `examples/guest-node-minimal`  
  **Role:** Skeleton guest component for `greentic:component/node@0.4.0`.  
  **Key functionality:** Implements node lifecycle/invoke/invoke-stream with trivial responses and exports via `export_component_node!`.
- **Path:** `examples/crates-io-consumer`  
  **Role:** Host binary smoke test for the published ABI crate.  
  **Key functionality:** Instantiates an AllowList from the bindings to confirm dependency wiring.
- **Path:** `examples/runner-host-smoke`  
  **Role:** Host-side Wasmtime smoke test.  
  **Key functionality:** Builds/loads the `component-describe` Wasm artifact (auto-builds if missing), wires runner-host imports plus WASI preview2, and calls `describe-json`, asserting valid JSON.
- **Path:** `guest-tests/repo-ui-actions-dummy`  
  **Role:** Fixture component implementing `repo-ui-worker@1.0.0`.  
  **Key functionality:** Echo-style handler returning payloads; used by the Wasmtime integration test asset in `crates/greentic-interfaces-wasmtime/tests/assets`.
- **Path:** `guest-tests/distributor-api-dummy`  
  **Role:** Fixture component for `greentic:distributor-api@1.0.0`.  
  **Key functionality:** Returns static ready/dummy data for resolve-component, static JSON for pack status, and no-op warm-pack; built via `scripts/build-distributor-api-dummy.sh` to feed the Wasmtime smoke test asset.
- **Path:** `docs/`  
  **Role:** Reference docs for WIT domains.  
  **Key functionality:** Outlines event contracts (`events-wit.md`), supply-chain provider packages (`supply-chain-wit.md`), and worker envelope semantics (`worker.md`).
- **Path:** `ci/local_check.sh`  
  **Role:** Local CI entrypoint mirroring formatting/clippy/tests (optional example builds when toggled).  
  **Key functionality:** Supports verbosity/strict/online toggles; expects `wasm32-wasip2` target installed when example checks are enabled.

## 3. Work In Progress, TODOs, and Stubs
- None found; no `TODO`/`FIXME` markers or stubbed `unimplemented!`/`todo!` code present across the workspace.

## 4. Broken, Failing, or Conflicting Areas
- Current status: all tests pass with `ci/local_check.sh` (fmt, clippy, WIT validation, build, test) as of this change. No failing tests or known conflicts observed. Distributor-api Wasmtime smoke test skips unless `scripts/build-distributor-api-dummy.sh` has been run to generate the component asset.

## 5. Notes for Future Work
- When adding or changing WIT packages, update conversion helpers (`src/mappers.rs`), provider schema snapshots, and re-run the staging/validation tests to keep host/guest crates in sync.
- If the repo-ui-actions or distributor-api test assets are regenerated, ensure the files under `crates/greentic-interfaces-wasmtime/tests/assets/` stay in sync with their guest fixtures (`scripts/build-repo-ui-actions-dummy.sh`, `scripts/build-distributor-api-dummy.sh`).
- Use `ci/local_check.sh` or `cargo test --workspace` to validate changes; include the `wasm32-wasip2` target when exercising the example builds.
