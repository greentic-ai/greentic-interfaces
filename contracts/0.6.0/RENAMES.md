# Renaming dictionary (0.6.0)

Greentic 0.6.0 tightens the vocabulary so docs, tooling, and linters stop reusing different nouns for the same protocol. The canonical terms are:

- **Component**: the executable WASM unit that implements `describe`/`invoke`.
- **Flow**: the graph definition that wires `steps` together and owns the resulting call spec payloads.
- **Step**: every flow node instance (previously called a node or step interchangeably). Each step is identified by `step_id` and always resolves to a single component + op pairing.
- **CallSpec**: the serialized `{op, payload_cbor, metadata_cbor}` that flows persist and hosts later wrap with contextual metadata.
- **InvocationEnvelope**: the host-owned wrapper containing `TenantCtx`, `flow_id`, `step_id`, `component_id`, `attempt`, and the call spec bytes; components never see the envelope directly.

## ID naming
| Concept | Canonical field | Notes |
| --- | --- | --- |
| Flow identifier | `flow_id` | Flows always expose `flow_id`. Avoid aliases like `run_id` or `execution_id`. |
| Step identifier | `step_id` | Replaces the legacy per-node identifier; store this whenever a flow references a step. |
| Component identifier | `component_id` | Declared in descriptors and used by runners to select the WASM artifact. |
| Environment identifier | `env_id` | Carried inside `TenantCtx`. Never rename it to `environment` or `platform`. |
| Tenant/team/user | `tenant_id`, `team_id`, `user_id` | Keep the `_id` suffix for every identity fields. |

## Payload naming
| Concept | Canonical field | Notes |
| --- | --- | --- |
| Operation key | `op` | Always a literal string (do not hyphenate or camelCase it for internal docs). |
| Payload bytes | `payload_cbor` | CBOR bytes only; legacy `payload_json` values must be converted at build time. |
| Metadata bytes | `metadata_cbor` | Optional; hosts may leave it `null` when not used. |
| Call spec | `CallSpec` | Flow manifests and migration tools store `{op, payload_cbor, metadata_cbor}` under this name. |
| Template helpers | `entry`, `prev`, `steps.<step_id>` | Continue to use these helpers when rendering node configuration; do not invent new helpers like `last` or `outputs`. |

## Tenant context naming
| Concept | Canonical field | Notes |
| --- | --- | --- |
| Tenant context | `TenantCtx` record | Maps to the `tenant-ctx` WIT record; keep the PascalCase struct in Rust while files stay with `tenant-ctx`. |
| Locale hint | `i18n_id` | Mandatory in 0.6.0 contexts; never call it `locale` or `language`. |

## Protocol boundary terms
- **CallSpec** is the only thing flows store. Hosts build the invocation envelope (with `TenantCtx`, `flow_id`, `step_id`, and `component_id`) when they actually invoke the component.
- **InvocationEnvelope** is host-owned; do not serialize it inside packs/flows. Any descriptive docs should call this out explicitly so we never confuse the envelope with the call spec.
- Legacy helpers that returned the legacy per-step identifier should now return `step_id`; the naming lint rejects the old field names in the canonical contract tree.
