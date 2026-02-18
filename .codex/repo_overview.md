# Repository Overview

## 1. High-Level Purpose
- Hosts the authoritative Greentic WIT contracts plus generated Rust bindings for both host and guest runtimes, with Wasmtime glue for executing components.
- Centralizes protocol snapshots (MCP), provider-core schema contracts, provisioning/validation worlds, and fixtures/examples for downstream integration.

## 2. Main Components and Functionality
- **Path:** `crates/greentic-interfaces`  
  **Role:** ABI crate containing WIT sources, generated bindings, and mappers.  
  **Key functionality:** `build.rs` stages every WIT package under `wit/` into a deterministic `$OUT_DIR` bundle and exposes `wit-bindgen` output; `src/mappers.rs` converts between WIT shapes and `greentic-types`. Feature flags cover common-types, types-core (0.2/0.4), component (0.4/0.5/1.0 + component-v1), pack-export (0.2/0.4 + pack-export-v1), pack-validate, provision, deploy-plan, runner-host (`greentic:host@1.0.0`), provider schema-core + provider-common, secrets/store + secrets-types, state/store, http-client (1.0/1.1), telemetry-logger, oauth-broker, worker, repo-ui-actions, gui-fragment, distributor-api (1.0/1.1), distribution, oci, source/build/scan/signing/attestation/policy/metadata, plus MCP snapshots (`wasix:mcp@24.11.05`, `@25.03.26`, `@25.06.18`).  
  **Key updates:** adds `i18n-id` to canonical contexts, introduces the `greentic:component@0.6.0` CBOR invoke world, and documents the descriptor’s `ops` metadata + optional setup contract (`docs/component-descriptor.md` + fixture).  
  **Key dependencies / integration points:** Depends on `greentic-types`; uses `wit-bindgen` 0.51; non-wasm builds link Wasmtime for tests.
- **Path:** `crates/greentic-interfaces-host`  
  **Role:** Host-facing re-export of bindings and mappers.  
  **Key functionality:** Curated modules for host imports and exports built on top of `greentic-interfaces` (with `wit-all`); optional features for gui fragments, worker, oauth-broker, MCP snapshots, and provider-core worlds.
- **Path:** `crates/greentic-interfaces-guest`  
  **Role:** Guest-facing bindings for `wasm32-wasip2` components.  
  **Key functionality:** Feature flags map to each world (component, pack-export, provision, pack-validate, deploy-plan, provider-core, distributor API, MCP, etc.); default `guest` enables the full surface and includes import helpers for distributor API v1/v1.1; `host-bridge` optionally reuses the ABI crate.
- **Path:** `crates/greentic-interfaces-wasmtime`  
  **Role:** Wasmtime integration layer for Greentic WIT packages.  
  **Key functionality:** Build script discovers `wit/greentic/*@*/package.wit` and emits `component::bindgen` modules; tests cover repo-ui-actions, distributor API, provider-core smoke, secrets-store linker wiring, and host-helper compile checks.
- **Path:** `examples/component-describe`  
  **Role:** `wasm32-wasip2` component exporting describe-only (`component@1.0.0`).  
  **Key functionality:** Minimal describe JSON implementation used by smoke tests.
- **Path:** `examples/guest-node-minimal`  
  **Role:** Skeleton component for `greentic:component/node@0.4.0`.  
  **Key functionality:** Implements minimal lifecycle/invoke handlers and exports via guest macro.
- **Path:** `examples/crates-io-consumer`  
  **Role:** Host binary smoke test for published ABI crate wiring.  
  **Key functionality:** Imports bindings to ensure crates.io dependencies resolve.
- **Path:** `examples/runner-host-smoke`  
  **Role:** Host-side Wasmtime smoke test runner.  
  **Key functionality:** Builds/loads the describe component and invokes `describe-json` with runner-host imports.
- **Path:** `examples/provider-core-dummy`  
  **Role:** Dummy provider-core component for schema-core world.  
  **Key functionality:** Returns static JSON for describe/validate/healthcheck and echoes invoke input.
- **Path:** `guest-tests/repo-ui-actions-dummy`  
  **Role:** Fixture component implementing `repo-ui-worker@1.0.0`.  
  **Key functionality:** Echo-style handler used by Wasmtime integration tests.
- **Path:** `guest-tests/distributor-api-dummy`  
  **Role:** Fixture component for `greentic:distributor-api@1.0.0`.  
  **Key functionality:** Returns static data for resolve/get/warm; built via `scripts/build-distributor-api-dummy.sh` for Wasmtime test assets.
- **Path:** `guest-tests/oauth-broker-client-harness`  
  **Role:** Harness component that exercises `oauth-broker` imports.  
  **Key functionality:** Calls consent URL/token/exchange helpers to validate import bindings.
- **Path:** `docs/`  
  **Role:** WIT and protocol documentation.  
  **Key functionality:** Covers events, supply-chain WIT, worker envelopes, pack validation, secrets migration, component state, payload/state, and interfaces inventory.
- **Path:** `scripts/`  
  **Role:** WIT packaging, validation, and fixture builders.  
  **Key functionality:** `validate-wit.sh`, `bundle-wit.sh`, `render-wit-docs.sh`, `package-wit.sh`, plus scripts to build guest fixture components.

## 3. Work In Progress, TODOs, and Stubs
- No TODO/FIXME/unimplemented markers found in the workspace.

## 4. Broken, Failing, or Conflicting Areas
- No test runs executed as part of this refresh; current status is unknown. Use `ci/local_check.sh` (defaults to Rust 1.89.0 toolchain and optional `wasm-tools`) for a full local gate, and enable `LOCAL_CHECK_EXAMPLES=1` to build the wasm32-wasip2 fixtures.

## 5. Notes for Future Work
- When adding/changing WIT packages, update `crates/greentic-interfaces/src/mappers.rs` and rerun WIT validation (`scripts/validate-wit.sh` or `ci/local_check.sh`) to keep bindings in sync.
- If guest fixture components are regenerated, refresh the Wasmtime test assets using `scripts/build-repo-ui-actions-dummy.sh` and `scripts/build-distributor-api-dummy.sh`.
- Keep `docs/component-descriptor.md`, `tests/fixtures/component-descriptor-example.json`, and the descriptor round-trip test in sync if the descriptor schema evolves across pack/component releases.

## 6. Current 0.6 Reset Work
- `PR-03` just wrapped: the canonical 0.6 WIT tree now lives under `wit/greentic/**@0.6.0/**`, the shared CBOR schemas land under `schemas/cddl/`, and `scripts/wit_lint.sh` enforces no `@0.6.0` copies escape the canonical path before the rest of the checks run.
- `PR-04` is active: we are authoring `greentic:types-core@0.6.0` (TenantCtx with i18n/tracing), `greentic:codec@0.6.0` helpers, and the CBOR-first `greentic:component/component@0.6.0` world/descriptor so hosts always send separated call specs and rely on the new descriptor metadata for schemas/capabilities.
- The canonical descriptor, CBOR schemas, and component world feed `scripts/wit_lint.sh`/CI once `ci/local_check.sh` reruns successfully with the new 0.6 packages; downstream host/guest build scripts now look at `./wit` for every WIT source.
