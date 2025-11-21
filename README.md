# Greentic Interfaces Workspace

This repository contains the shared ABI contracts and Wasmtime runtime helpers used across the Greentic platform.

[![CI](https://github.com/greentic-ai/greentic-interfaces/actions/workflows/ci.yml/badge.svg)](https://github.com/greentic-ai/greentic-interfaces/actions/workflows/ci.yml)
[![WIT Docs](https://img.shields.io/badge/docs-WIT%20packages-4c9)](https://greentic-ai.github.io/greentic-interfaces/)
[![MSRV](https://img.shields.io/badge/MSRV-1.88%2B-blue)](#minimum-supported-rust-version)

- [`crates/greentic-interfaces`](crates/greentic-interfaces) exposes the WebAssembly Interface Types (WIT) packages, generated Rust bindings, and thin mappers that bridge the generated types to the richer structures in [`greentic-types`](https://github.com/greentic-ai/greentic-types). It is intentionally ABI-only and has no Wasmtime dependency.
- [`crates/greentic-interfaces-host`](crates/greentic-interfaces-host) curates the host-facing bindings: Wasmtime-ready WIT worlds plus the shared mappers.
- [`crates/greentic-interfaces-guest`](crates/greentic-interfaces-guest) curates the guest-facing bindings for components built against `wasm32-wasip2`.
- [`crates/greentic-interfaces-wasmtime`](crates/greentic-interfaces-wasmtime) hosts the Wasmtime integration layer. It wires the Greentic host imports into a Wasmtime linker, instantiates components, and forwards calls through the ABI bindings.

> Node configuration schemas always live alongside their components. This repository only ships shared WIT contracts plus the corresponding bindings/mappers.

These crates are published from this workspace. Downstream components that only need the ABI can depend solely on `greentic-interfaces`. Runtimes that execute packs should add `greentic-interfaces-wasmtime` and choose whether to stay on the stable Wasmtime feature path or opt into the nightly configuration. Hosts that just want re-exported bindings can depend on `greentic-interfaces-host`, while guest components can pull `greentic-interfaces-guest` for `wasm32-wasip2` builds.

```rust
// Host side: wire imports into a Wasmtime linker.
use greentic_interfaces_host::host_import::v0_6::add_to_linker;

// Guest side: call host capabilities from inside a component.
use greentic_interfaces_guest::component::node::Guest;
use greentic_interfaces_guest::secrets_store::secret_store;
```

## Which crate should I use?

- Hosts (runner, deployer, gateways): `greentic-interfaces-host`
- Wasm components (`wasm32-wasip2`): `greentic-interfaces-guest`
- Wasmtime glue / linker helpers: `greentic-interfaces-wasmtime`
- ABI/WIT tooling and validation: `greentic-interfaces`

### Host examples

```rust
use greentic_interfaces_host::http::http_client;
use greentic_interfaces_host::secrets::store_v1::secret_store;
use greentic_interfaces_host::telemetry::log;
```

### Guest examples

```rust
use greentic_interfaces_guest::component::node::Guest;
use greentic_interfaces_guest::secrets_store::secret_store;
use greentic_interfaces_guest::http_client::http_client;
use greentic_interfaces_guest::telemetry_logger::logger_api;
```

## Migration guide: move to host/guest crates

1. Replace direct `greentic-interfaces` imports in hosts with `greentic-interfaces-host` and switch to the curated modules (`secrets`, `state`, `messaging`, `http`, `telemetry`, `oauth`).
2. Replace direct bindgen usage in wasm components with `greentic-interfaces-guest`; import from the module for the capability you need (`secrets_store`, `state_store`, `messaging`, `oauth`).
3. Update your target/toolchain: guests should build with `--target wasm32-wasip2`; hosts stay native.
4. For Wasmtime wiring, depend on `greentic-interfaces-wasmtime` alongside the host crate if you need linker helpers.
5. Drop local WIT regeneration: the host/guest crates ship the generated bindings; WIT remains the source of truth here.

For local development you can override the crates.io dependency on `greentic-types` by copying `.cargo/local-patch.example.toml` to `.cargo/config.toml` and pointing it at a sibling checkout of `greentic-types`.

## Feature flags

| Feature | World(s) enabled | Published package | Notes |
| --- | --- | --- | --- |
| `secrets-store-v1` | `greentic:secrets/store@1.0.0` (`store`) | [`package.wit`](https://greentic-ai.github.io/greentic-interfaces/secrets-store@1.0.0/package.wit) | Generic secret read/write/delete imports aligned with `HostCapabilities.secrets`. |
| `state-store-v1` | `greentic:state/store@1.0.0` (`store`) | [`package.wit`](https://greentic-ai.github.io/greentic-interfaces/state-store@1.0.0/package.wit) | Tenant-scoped blob store aligned with `HostCapabilities.state`. |
| `messaging-session-v1` | `greentic:messaging/session@1.0.0` (`session`) | [`package.wit`](https://greentic-ai.github.io/greentic-interfaces/messaging-session@1.0.0/package.wit) | Generic outbound messaging surface for session flows. |
| `events-emitter-v1` | `greentic:events/emitter@1.0.0` (`emitter`) | [`package.wit`](https://greentic-ai.github.io/greentic-interfaces/events-emitter@1.0.0/package.wit) | Fire-and-forget event emission aligned with `HostCapabilities.events`. |
| `http-client-v1` | `greentic:http/client@1.0.0` (`client`) | [`package.wit`](https://greentic-ai.github.io/greentic-interfaces/http-client@1.0.0/package.wit) | Preview 2 HTTP client matching `HostCapabilities.http`. |
| `telemetry-logger-v1` | `greentic:telemetry/logger@1.0.0` (`logger`) | [`package.wit`](https://greentic-ai.github.io/greentic-interfaces/telemetry-logger@1.0.0/package.wit) | Tenant-aware telemetry logger aligned with `HostCapabilities.telemetry`. |
| `oauth-broker-v1` | `greentic:oauth-broker@1.0.0` (`broker`) | [`package.wit`](https://greentic-ai.github.io/greentic-interfaces/oauth-broker@1.0.0/package.wit) | Generic OAuth broker: build consent URLs, exchange codes, fetch tokens; provider details remain in host config/greentic-oauth. |
| `describe-v1` | `greentic:component@1.0.0` (`describe-v1`) | [`package.wit`](https://greentic-ai.github.io/greentic-interfaces/component@1.0.0/package.wit) | Describe-only schema export for packs without the full component ABI. |
| `runner-host-v1` | `greentic:host@1.0.0` (`http-v1`, `secrets-v1`, `kv-v1`) | [`package.wit`](https://greentic-ai.github.io/greentic-interfaces/host@1.0.0/package.wit) | Legacy host import bundle (still available for older packs). |
| `component-lifecycle-v1` | `greentic:lifecycle@1.0.0` (`lifecycle-v1`) | [`package.wit`](https://greentic-ai.github.io/greentic-interfaces/lifecycle@1.0.0/package.wit) | Optional lifecycle hooks (`init`, `health`, `shutdown`). |
| `events-v1` | `greentic:events@1.0.0` (`events-v1`) | [`package.wit`](https://greentic-ai.github.io/greentic-interfaces/events@1.0.0/package.wit) | Shared telemetry/event envelope record. |
| `wit-all` | Aggregates every feature above plus the legacy defaults (`component-v0-4`, `types-core-*`, etc.) | – | Handy opt-in when you just want “everything on”. |

## Deployment plan world

Deployment packs can import `greentic:deploy-plan@1.0.0` to read the current `DeploymentPlan` and emit status updates. The world exposes two funcs:

- `get-deployment-plan()` – returns the JSON-encoded `DeploymentPlan` built by the host/deployer for this execution.
- `emit-status(message)` – reports a free-form status line that hosts may log or display in a UI.

Hosts wire this world alongside the existing runner-host imports, so deployment flows still run as ordinary events flows with an additional channel for structured deployment context.

## Minimum Supported Rust Version

The workspace targets **Rust 1.88** or newer (required by the 2024 edition). CI pins the same stable toolchain for formatting/clippy, so make sure your local toolchain matches 1.88+ when hacking.

## Examples

Two smoke-level examples live under `examples/`:

- `component-describe`: a `no_std` component that implements `describe-v1::describe-json`.
- `runner-host-smoke`: a host-side binary that links the new runner-host imports, instantiates the `component-describe` Wasm artifact, and executes `describe-json`.
  The runner repository also ships a secrets-oriented guest fixture (`component-secrets`) that exercises the `secrets-v1` imports end-to-end.

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

## Fetching WIT packages from OCI

The published WIT bundles live in GitHub Container Registry under `ghcr.io/greentic-ai/wit`. The registry metadata served from `https://greentic.ai/.well-known/wasm-pkg/registry.json` advertises this prefix, so any `wkg` client can resolve the `greentic:*` namespace automatically.

```bash
# 1. Install the wasm packaging CLI
cargo install wkg

# 2. Point your config at the Greentic registry (writes ~/.config/wasm-pkg/config.toml)
wkg config --default-registry greentic.ai

# 3. Fetch the desired package (auto-discovers ghcr.io/greentic-ai/wit/<namespace>/<pkg>)
wkg get greentic:component@1.0.0 --output ./component.wasm
# or grab the raw WIT:
wkg get greentic:component@1.0.0 --output ./component.wit --format wit
```

If you prefer to edit the config file manually, add this stanza:

```toml
[namespace_registries]
greentic = "greentic.ai"
```

With that mapping in place the CLI will transparently pull from GHCR using the namespace prefix advertised by the registry metadata (`greentic-ai/wit/`).

## Using `secrets-store-v1` from guests

The `secrets-store-v1` feature gates the `greentic:secrets/store@1.0.0` package. Components that need to work with secrets should:

1. Enable `secrets-store-v1` (or `wit-all`) on the dependency.
2. Import the interface in their WIT (`use greentic:secrets/store@1.0.0`) or via `wit-bindgen`.
3. Call the synchronous host functions surfaced by the runner:

```wit
interface secret-store {
  record host-error { code: string, message: string }
  enum op-ack { ok }

  read: func(name: string) -> result<list<u8>, host-error>;
  write: func(name: string, bytes: list<u8>) -> result<op-ack, host-error>;
  delete: func(name: string) -> result<op-ack, host-error>;
}
```

- `read` returns the raw bytes stored at `name` or a structured `host-error` describing the failure.
- `write`/`delete` return `op-ack::ok` on success and otherwise raise `host-error` (for example, the default `EnvSecretsManager` denies writes/deletes and reports a `permission` error string).

The ABI maps directly onto the [`greentic-secrets-api`](https://github.com/greentic-ai/greentic-secrets) trait. Hosts created with `greentic-runner` currently use the `EnvSecretsManager`, so setting `TEST_KEY=value` in the environment and calling `store.read("TEST_KEY")` from a guest yields `value`. For a working reference component, see the `component-secrets` fixture in the runner repository, which reads `TEST_KEY` via `secrets-store` and echoes it back to the host.
