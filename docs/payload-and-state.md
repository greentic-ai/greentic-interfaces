# Payload and state

This document defines the canonical distinction between payload and state for Greentic components and the runner-provided templating contract used in flow node configuration.

## Canonical component state ABI

The only canonical component-state ABI is:

- `greentic:state/store@1.0.0` with `read`, `write`, `delete`.
- WIT: `crates/greentic-interfaces/wit/greentic/state-store@1.0.0/package.wit`
- Guest WIT mirror: `crates/greentic-interfaces-guest/wit/greentic/state-store@1.0.0/package.wit`

Other host interfaces (for example, the runner-host `kv-v1`) are not canonical component-state APIs and should not be treated as such in component docs or examples.

## Payload

- Payload is ephemeral data passed node-to-node within a flow execution.
- Components receive payload as the invocation `input` JSON and return an output JSON.
- Components should not reach back to earlier nodes by reading state as a default rule.
- Runners provide prior outputs to node configuration templating via `prev` and `node.<id>`.

## State

- State is persistent, tenant-scoped storage exposed to components via `greentic:state/store@1.0.0`.
- The host decides the backend and enforces tenant/team/user scoping using `TenantCtx`.
- State access must be capability-gated by the host/runner.

## Templating context

Runners evaluate Handlebars-like expressions against a stable context when rendering node configuration:

- `entry`: initial flow input payload (immutable; `{}` if none).
- `prev`: output JSON of the previously executed node (or `{}` for the first node in a path).
- `node`: map of executed node outputs by node id, accessible like `{{node.start.user.id}}`.
- `state`: runner-provided state view. This is runner-defined and may represent flow-local memory or a current node view.

Important: `state` in templates is not a magical read from the persistent store. If a runner provides helpers to read or write persisted state, those helpers are runner-specific and are not part of the interface contract.

## Typed insertion rule

To avoid implicit stringification and helper-heavy templates:

- If a YAML/JSON scalar value is exactly `{{expr}}` (no surrounding text), the runner MUST evaluate `expr` and insert the resulting value as a typed JSON value (number, bool, object, array, string, or null).
- If a scalar contains any additional text (for example, `"https://x/{{entry.user_id}}"`), it is rendered as a string.

## Example

```yaml
nodes:
  start:
    read:
      url: "https://api.example.com/users/{{entry.user_id}}"
  process:
    transform:
      user_id: {{node.start.user.id}}
      name: {{node.start.user.name}}
      last_status: {{prev.status}}
```

Notes:

- `start.read.url` is string templating.
- `process.transform.user_id` uses typed insertion (a number stays a number).
- This example does not prescribe whether the operation key is `do:` or `transform:`. It only demonstrates an operation key with an input object.
