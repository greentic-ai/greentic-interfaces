# Greentic Interfaces Workspace

This repository contains the shared ABI contracts and Wasmtime runtime helpers used across the Greentic platform.

[![CI](https://github.com/greentic-ai/greentic-interfaces/actions/workflows/ci.yml/badge.svg)](https://github.com/greentic-ai/greentic-interfaces/actions/workflows/ci.yml)
[![WIT Docs](https://img.shields.io/badge/docs-WIT%20packages-4c9)](https://greentic-ai.github.io/greentic-interfaces/)
[![MSRV](https://img.shields.io/badge/MSRV-1.88%2B-blue)](#minimum-supported-rust-version)

- [`crates/greentic-interfaces`](crates/greentic-interfaces) exposes the WebAssembly Interface Types (WIT) packages, generated Rust bindings, and thin mappers that bridge the generated types to the richer structures in [`greentic-types`](https://github.com/greentic-ai/greentic-types). It is intentionally ABI-only and has no Wasmtime dependency.
- [`crates/greentic-interfaces-wasmtime`](crates/greentic-interfaces-wasmtime) hosts the Wasmtime integration layer. It wires the Greentic host imports into a Wasmtime linker, instantiates components, and forwards calls through the ABI bindings.

> Node configuration schemas always live alongside their components. This repository only ships shared WIT contracts plus the corresponding bindings/mappers.

Both crates are published from this workspace. Downstream components that only need the ABI can depend solely on `greentic-interfaces`. Runtimes that execute packs should add `greentic-interfaces-wasmtime` and choose whether to stay on the stable Wasmtime feature path or opt into the nightly configuration.

For local development you can override the crates.io dependency on `greentic-types` by copying `.cargo/local-patch.example.toml` to `.cargo/config.toml` and pointing it at a sibling checkout of `greentic-types`.

## Feature flags

| Feature | World(s) enabled | Published package | Notes |
| --- | --- | --- | --- |
| `describe-v1` | `greentic:component@1.0.0` (`describe-v1`) | [`package.wit`](https://greentic-ai.github.io/greentic-interfaces/component@1.0.0/package.wit) | Describe-only schema export for packs without the full component ABI. |
| `runner-host-v1` | `greentic:host@1.0.0` (`http-v1`, `secrets-v1`, `kv-v1`) | [`package.wit`](https://greentic-ai.github.io/greentic-interfaces/host@1.0.0/package.wit) | Host-side imports runner implementations can satisfy/mocks can replace. |
| `component-lifecycle-v1` | `greentic:lifecycle@1.0.0` (`lifecycle-v1`) | [`package.wit`](https://greentic-ai.github.io/greentic-interfaces/lifecycle@1.0.0/package.wit) | Optional lifecycle hooks (`init`, `health`, `shutdown`). |
| `events-v1` | `greentic:events@1.0.0` (`events-v1`) | [`package.wit`](https://greentic-ai.github.io/greentic-interfaces/events@1.0.0/package.wit) | Shared telemetry/event envelope record. |
| `wit-all` | Aggregates every feature above plus the legacy defaults (`component-v0-4`, `types-core-*`, etc.) | – | Handy opt-in when you just want “everything on”. |

## Minimum Supported Rust Version

The workspace targets **Rust 1.88** or newer (required by the 2024 edition). CI pins the same stable toolchain for formatting/clippy, so make sure your local toolchain matches 1.88+ when hacking.

## Examples

Two smoke-level examples live under `examples/`:

- `component-describe`: a `no_std` component that implements `describe-v1::describe-json`.
- `runner-host-smoke`: a host-side binary that links the new runner-host imports, instantiates the `component-describe` Wasm artifact, and executes `describe-json`.

### Running the examples locally

```bash
# Install the WASI Preview 2 target once (matches CI)
rustup target add wasm32-wasip2 --toolchain 1.88.0

# Compile the component to Wasm (targets wasm32-wasip2)
CARGO_TARGET_DIR=target cargo build --manifest-path examples/component-describe/Cargo.toml --target wasm32-wasip2

# Run the host smoke test (reuses the artifact above)
COMPONENT_DESCRIBE_WASM=target/wasm32-wasip2/debug/component_describe.wasm \
  CARGO_TARGET_DIR=target cargo run --manifest-path examples/runner-host-smoke/Cargo.toml
```

## What is `greentic-interfaces-wasmtime`?

The runtime crate provides Wasmtime glue for the Greentic WIT packages: an engine builder, feature-gated `add_*_to_linker` helpers, and mapper utilities that bridge between the ABI structs re-exported from `greentic-interfaces` and the richer models in `greentic-types`. It does **not** regenerate WIT on its own – everything flows through the ABI crate – so downstream consumers (runner, MCP, packs) can instantiate components and call exports without duplicating linker boilerplate.

## Provenance

Every tagged release publishes a tarball, checksum, raw `package.wit` files, and a signed provenance note that enumerates per-package hashes. Grab the [latest release notes](https://github.com/greentic-ai/greentic-interfaces/releases/latest/download/RELEASE_NOTES.md) to verify what you downloaded.

## Local Checks

Run the CI-equivalent checks locally with:

```bash
ci/local_check.sh
```

Toggles:
- `LOCAL_CHECK_ONLINE=1` – enable networked steps (none today, reserved for future use).
- `LOCAL_CHECK_STRICT=1` – fail immediately if required tools are missing.
- `LOCAL_CHECK_VERBOSE=1` – print every command before executing it.
- `LOCAL_CHECK_EXAMPLES=1` – build/run the example crates (requires the `wasm32-wasip2` target).
- The example steps expect `rustup target add wasm32-wasip2 --toolchain 1.88.0` to have been run first.

A `pre-push` hook is installed automatically (if absent) to run the script before pushing; remove `.git/hooks/pre-push` if you prefer to opt out.
