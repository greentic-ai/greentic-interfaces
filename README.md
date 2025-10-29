# greentic-interfaces

Shared WebAssembly Interface Types (WIT) and Rust bindings for the Greentic stack. All packages are MIT licensed and designed to evolve additively: new fields and functions land in minor releases, breaking changes require a new major package or crate version.

## Package overview

- `greentic:types-core@0.2.0` – shared records and enums (flow metadata, tenant context, run results, error codes).
- `greentic:host-import@0.2.0` – host services that components import (secrets, telemetry, tool invocation, outbound HTTP).
- `greentic:pack-export@0.2.0` – pack services that components export (discover flows, execute flows, A2A search).
- `greentic:component@0.4.0` – component runtime contract with lifecycle hooks, invoke/invoke-stream, and structured errors.
- `greentic:secrets@0.1.0` – legacy secrets host interface (still published for compatibility).

Each WIT package lives under `wit/<package>@<version>.wit`. CI validates every file with `wit-bindgen markdown` and `wasm-tools component wit` so the schemas stay well-formed.

## Rust bindings

Add the crate to your Wasmtime host:

```toml
[dependencies]
greentic-interfaces = "0.2"
wasmtime = { version = "38", features = ["component-model"] }
```

Then wire in the bindings you need. Example for `greentic:component@0.4.0` control imports:

```rust,ignore
use greentic_interfaces::component_v0_4::{self, Component};
use greentic_interfaces::component_v0_4::greentic::component::node::{ExecCtx, TenantCtx};
use wasmtime::{component::Linker, Config, Store};
use std::{future::Future, pin::Pin};

struct MyControl;

type HostFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

impl component_v0_4::greentic::component::control::Host for MyControl {
    fn should_cancel<'a, 'async_trait>(&'a mut self) -> HostFuture<'async_trait, bool>
    where
        'a: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move { false })
    }

    fn yield_now<'a, 'async_trait>(&'a mut self) -> HostFuture<'async_trait, ()>
    where
        'a: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {})
    }
}

let mut config = Config::new();
config.wasm_component_model(true);
let engine = wasmtime::Engine::new(&config)?;
let mut linker = Linker::new(&engine);
component_v0_4::add_control_to_linker(&mut linker, |state: &mut MyControl| state)?;

let component = wasmtime::component::Component::from_file(&engine, "component.wasm")?;
let mut store = Store::new(&engine, MyControl);
let (bindings, _instance) = Component::instantiate_async(&mut store, &component, &linker).await?;
let node = bindings.exports().greentic_component_node()?;
```

Host integrations can use the helper modules:

- `types_core_v0_2` exposes the canonical structs/enums (duplication of `greentic:types-core@0.2.0`).
- `host_import_v0_2` provides the `HostImports` trait plus `add_to_linker` for wiring the host services.
- `pack_export_v0_2` re-exports the generated exports when hosting pack components.

## Versioning

- Minor bumps add optional fields or functions; existing behaviour remains.
- Breaking changes require a new package (e.g., `greentic:host-import@0.3.0`) and crate minor bump.
- The crate is published to crates.io (`0.2.x` for the current set) and tagged `v0.2.x` in git.

## Validation and tooling

- `bash scripts/validate-wit.sh` – run `wit-bindgen markdown` and `wasm-tools component wit --wasm` over every `.wit` file.
- `bash scripts/package-wit.sh <out-dir>` – emit wasm-encoded packages for publishing.
- `bash scripts/publish_wit.sh` – package + push all WIT packages to GHCR (`ghcr.io/<user>/wit/...`).
- CI (`.github/workflows/ci.yml`) installs `wit-bindgen`, validates WIT, and runs `cargo build`.

## CHANGELOG

See [CHANGELOG.md](CHANGELOG.md) for release notes.

## License

MIT
