# greentic-interfaces

Define the WebAssembly interface types shared by Greentic component modules and their hosts. The WIT files under `wit/` are versioned by package name to enable additive evolution without breaking existing components.

## Package overview

- `greentic:component@0.3.0` exposes the original contract used by early components. Hosts provide a `tenant-ctx` and receive a JSON payload or plain string error. Streaming and lifecycle hooks are not available.
- `greentic:component@0.4.0` adds structured execution context, explicit error envelopes, and streaming progress. Components surface cooperative control via the `control` interface and may opt into lifecycle hooks (`on-start`, `on-stop`).

## Usage

Add the crate to your Wasmtime host:

```toml
[dependencies]
greentic-interfaces = "0.1"
wasmtime = { version = "21", features = ["component-model"] }
```

Then wire the bindings into a component host:

```rust,ignore
use greentic_interfaces::component_v0_4::{self, ControlHost, Component};
use greentic_interfaces::component_v0_4::greentic::component::node::{ExecCtx, TenantCtx};
use wasmtime::{component::Linker, Config, Store};
use std::{future::Future, pin::Pin};

struct MyControl;

type HostFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

impl ControlHost for MyControl {
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

let ctx = ExecCtx {
    tenant: TenantCtx {
        tenant: "tenant-id".into(),
        team: None,
        user: None,
        trace_id: None,
        correlation_id: None,
        deadline_unix_ms: None,
        attempt: 0,
        idempotency_key: None,
    },
    flow_id: "flow".into(),
    node_id: None,
};

let result = node.invoke(&mut store, &ctx, "op", &"{}".into()).await?;
```

The helper `HostFuture` alias keeps the trait signatures readable; the generated bindings expect pinned `Future`s.

At build time, ensure `scripts/validate-wit.sh` runs so the packaged WIT stays well-formed.

### Mapping to `greentic-types`

| Rust type (`greentic-types`) | WIT type                             |
| ---------------------------- | ------------------------------------ |
| `TenantCtx`                  | `node::tenant-ctx`                    |
| `ExecCtx`                    | `node::exec-ctx`                      |
| `InvokeResult`               | `node::invoke-result`                 |
| `StreamEvent`                | `node::stream-event`                  |
| `NodeError`                  | `node::node-error`                    |
| `LifecycleStatus` (newtype)  | `node::lifecycle-status`              |

All structured payloads are JSON strings (`type json = string`) to stay language-agnostic. Components should deserialize into native types as needed.

## Version negotiation

Hosts should enumerate supported packages in descending order and attempt to instantiate the highest version offered by the component module. When a module only implements `component@0.3.0`, the host must fall back to the legacy interface and avoid calling lifecycle or streaming APIs. When both parties expose `component@0.4.0`, hosts are expected to:

1. Bind the `control` interface (if the component imports it) to cooperate with cancellation and scheduling.
2. Invoke lifecycle hooks opportunistically. Components that do not need them must still export the functions but may return `Ok(())` immediately.
3. Prefer `invoke-stream` when progressive updates are desired; otherwise call `invoke`.

Consumers should document the chosen version in their manifest to simplify future migrations.

## Error semantics

- `invoke` returns `invoke-result`. A successful execution wraps the JSON payload in `ok(json)`.
- Recoverable failures surface as `err(node-error)` with `retryable = true` and an optional `backoff-ms` hint.
- Non-retryable errors set `retryable = false`. Hosts may surface `message` directly to operators.
- `invoke-stream` is expected to end with either `stream-event::done` or `stream-event::error`. Specific progress updates use `stream-event::progress` with values from `0` to `100`.
- `on-start`/`on-stop` return `lifecycle-status::ok` when the hook succeeds and `Err` with a descriptive string otherwise.

Legacy `component@0.3.0` continues to use `result<json, string>` for synchronous replies; hosts should translate that into `invoke-result::ok` or a generic `node-error` when bridging between versions.

## Rust bindings

`src/component_v0_3.rs` and `src/component_v0_4.rs` expose `wasmtime::component::bindgen!` output to make hosting components easy:

- `component_v0_3::Component` provides strongly‑typed access to the `@0.3.0` exports.
- `component_v0_4::Component` does the same for `@0.4.0`, and `component_v0_4::add_control_to_linker` wraps the generated helper so hosts can wire the `control` interface into a `wasmtime::component::Linker`.

The generated modules follow Wasmtime defaults: identifiers with hyphens in WIT are mapped to snake_case in Rust (`tenant-ctx` → `TenantCtx`).

## Publishing

### crates.io

1. Run `cargo fmt`, `cargo test`, and `cargo check` (or `cargo publish --dry-run`) to make sure the crate builds cleanly.
2. Update `CHANGELOG.md` (if applicable) and bump the version in `Cargo.toml`.
3. Authenticate with crates.io using `cargo login`.
4. Execute `cargo publish` from the repository root.

The `include` section in `Cargo.toml` restricts the published archive to source code, scripts, README, and license.

### wa.dev (WIT packages)

1. Install [wasm-tools](https://github.com/bytecodealliance/wasm-tools) and the [warg CLI](https://github.com/bytecodealliance/warg). Both are available via `cargo install wasm-tools warg-cli`.
2. Encode each WIT package into its binary form (required by warg registries). The repository includes `scripts/package-wit.sh` to automate this:

   ```bash
   bash scripts/package-wit.sh target/wit-packages
   # Produces target/wit-packages/greentic-component-0.3.0.wasm etc.
   ```

3. Authenticate with wa.dev: `warg login --registry https://registry.wa.dev/v1`.
4. Initialize the package namespace on the registry (one-time):

   ```bash
   warg publish init greentic:component --registry https://registry.wa.dev/v1
   ```

5. Release both versions so hosts can negotiate:

   ```bash
   warg publish release \
     --registry https://registry.wa.dev/v1 \
     --name greentic:component \
     --version 0.3.0 \
     target/wit-packages/greentic-component-0.3.0.wasm

   warg publish release \
     --registry https://registry.wa.dev/v1 \
     --name greentic:component \
     --version 0.4.0 \
     target/wit-packages/greentic-component-0.4.0.wasm
   ```

6. Tag the release (`git tag v0.4.0 && git push --tags`).

As part of CI, run `bash scripts/validate-wit.sh` to catch schema regressions before publishing. Use `wasm-tools component wit --out-dir` during review if you need to diff the resolved package graph.

## Validation

Run `scripts/validate-wit.sh` (once `wit-bindgen` is installed) to lint the interface definitions:

```bash
bash scripts/validate-wit.sh
```

## License

MIT
