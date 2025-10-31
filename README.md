# Greentic Interfaces Workspace

This repository contains the shared ABI contracts and Wasmtime runtime helpers used across the Greentic platform.

- [`crates/greentic-interfaces`](crates/greentic-interfaces) exposes the WebAssembly Interface Types (WIT) packages, generated Rust bindings, and thin mappers that bridge the generated types to the richer structures in [`greentic-types`](https://github.com/greentic-ai/greentic-types). It is intentionally ABI-only and has no Wasmtime dependency.
- [`crates/greentic-interfaces-wasmtime`](crates/greentic-interfaces-wasmtime) hosts the Wasmtime integration layer. It wires the Greentic host imports into a Wasmtime linker, instantiates components, and forwards calls through the ABI bindings.

Both crates are published from this workspace. Downstream components that only need the ABI can depend solely on `greentic-interfaces`. Runtimes that execute packs should add `greentic-interfaces-wasmtime` and choose whether to stay on the stable Wasmtime feature path or opt into the nightly configuration.

## What is `greentic-interfaces-wasmtime`?

The runtime crate provides Wasmtime glue for the Greentic WIT packages: an engine builder, feature-gated `add_*_to_linker` helpers, and mapper utilities that bridge between the ABI structs re-exported from `greentic-interfaces` and the richer models in `greentic-types`. It does **not** regenerate WIT on its own – everything flows through the ABI crate – so downstream consumers (runner, MCP, packs) can instantiate components and call exports without duplicating linker boilerplate.
