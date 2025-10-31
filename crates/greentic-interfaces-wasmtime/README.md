# Greentic Interfaces Wasmtime Runtime Helpers

`greentic-interfaces-wasmtime` provides the Wasmtime integration layer for the Greentic platform. It wires host services into a Wasmtime component linker, offers convenience helpers for building engines, and exposes mapper utilities that bridge the ABI structs published by [`greentic-interfaces`](../greentic-interfaces) with the richer models from [`greentic-types`](https://github.com/greentic-ai/greentic-types).

## Feature flags

- `stable-wasmtime` (default): builds against Wasmtime releases `< 38`, compatible with the stable Rust channel.
- `nightly-wasmtime`: switches to Wasmtime `38.0.3`, which currently requires the nightly toolchain due to edition 2024 support and fiber features.

Enable the right flag depending on your toolchain:

```toml
[dependencies]
greentic-interfaces-wasmtime = { version = "0.1", default-features = false, features = ["nightly-wasmtime"] }
```

## Quick start

```rust
use greentic_interfaces_wasmtime::{build_engine, EngineOptions, LinkerBuilder};

let engine = build_engine(EngineOptions::default())?;
let mut linker = LinkerBuilder::new(&engine).finish();
// Register host services as needed:
// greentic_interfaces_wasmtime::add_secrets_to_linker(&mut linker)?;
```

The crate currently focuses on ergonomic wiring and leaves host trait implementations to downstream crates.
