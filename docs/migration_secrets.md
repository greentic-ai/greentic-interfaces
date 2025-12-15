# Secrets Migration (public launch cleanup)

- Legacy secrets WIT packages have been removed in **0.4.58**: `greentic:secrets@0.1.0`, `greentic:host-import@0.2.0/0.4.0/0.6.0`, and the `secrets-v1` import inside `greentic:host@1.0.0`.
- The only secrets surface is now **`greentic:secrets-store/store@1.0.0`** (read-only `get` returning `option<bytes>` with structured errors).
- Secret requirement metadata is modeled in **`greentic:secrets-types@1.0.0`** (key/scope/format/schema/examples); distributor responses reference this shape and never return secret values.
- The runner host bundle (`greentic:host@1.0.0`) remains for HTTP/KV only; it no longer exposes secrets.
- Guest-test/wasmtime mirror copies for the removed packages have been deleted to avoid drift.
- Consumers should switch to `secrets-store@1.0.0` and drop any dependencies on the removed interfaces.
- All secret requirement modeling is handled in `greentic-types`; `greentic-interfaces` only defines the WIT surface.
