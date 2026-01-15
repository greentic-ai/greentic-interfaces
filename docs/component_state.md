# Component state

This repo exposes component state as a host-provided capability. Components do not manage their own persistence; they call the state-store interface and the host maps it to whatever backing store it chooses. The only canonical component-state ABI is `greentic:state/store@1.0.0`.

## Interfaces

- `greentic:state/store@1.0.0` is the canonical state store interface.
  - WIT: `crates/greentic-interfaces/wit/greentic/state-store@1.0.0/package.wit`
  - Guest WIT mirror: `crates/greentic-interfaces-guest/wit/greentic/state-store@1.0.0/package.wit`
  - Types: `state-key` and `tenant-ctx` come from `greentic:interfaces-types/types@0.1.0`.
- The runner-host world also exposes a simple string KV for host use:
  - WIT: `crates/greentic-interfaces/wit/greentic/host@1.0.0/package.wit` (`kv-v1`)
  - This is not a canonical component-state API and should not be treated as a substitute for `greentic:state/store@1.0.0` in component docs or examples.

## Guest usage

Components import the state-store capability through the guest crate:

- `crates/greentic-interfaces-guest/src/lib.rs` exposes `state_store` (feature `state-store`).
- Calls are `read`, `write`, and `delete`, and operate on opaque `state-key` values and optional tenant context.

The state store is byte-oriented (`list<u8>`), so components decide how to serialize values.

## Host implementation

Hosts implement the state store and decide how persistence works:

- `crates/greentic-interfaces-host/src/lib.rs` re-exports the state-store bindings for hosts.
- `crates/greentic-interfaces-wasmtime/src/host_helpers/v1/state_store.rs` provides linker helpers for Wasmtime hosts.

The host is responsible for:

- Choosing the storage backend (database, object store, in-memory, etc.).
- Enforcing tenant scoping via `tenant-ctx` when provided.
- Defining any namespace or key conventions used by components.

## Example wiring

A minimal host wiring example is in:

- `examples/runner-host-smoke/src/main.rs`

That example shows how a host state struct is attached to the Wasmtime store; it is separate from the persisted component state handled by the state-store interface.
