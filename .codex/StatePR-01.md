# StatePR-01 — greentic-interfaces: Canonical component state ABI + payload templating contract

## Repo
`greentic-interfaces`

## Goal
1) Make `greentic:state/store@1.0.0` the **only** canonical component-state ABI for Wasm components.
2) Publish a **small, stable payload templating contract** for flow node configuration using Handlebars-like expressions:
   - Context vars: `entry`, `prev`, `node`, `state`
   - Typed insertion rule: `{{expr}}` used as a full scalar inserts a JSON value (not a string)

This PR is primarily **documentation + examples** and must not introduce breaking WIT changes unless absolutely required.

## Non-goals
- Do **not** introduce a “payload host interface/capability”. Payload wiring is runner responsibility.
- Do **not** require components to import `greentic-state` (that is host-side only).
- Do **not** document runner-internal snapshot key schemes as a component API.

---

## Work Items

### 1) Canonicalize state ABI
- Audit WIT packages and docs to ensure the canonical state-store interface is:
  - `greentic:state/store@1.0.0` with `read/write/delete`
- Remove or clearly de-emphasize any alternative state APIs as “canonical” (e.g., lightweight KV surfaces).
- Search and fix wording like “v0.6 state store” or other version ambiguity; docs must reference the actual WIT package version.

### 2) Publish “Payload vs State” rules (canonical doc)
Add or update a doc (e.g. `docs/payload-and-state.md`) stating:

**Payload**
- Payload is ephemeral data passed node-to-node within a flow execution.
- Components receive payload as the invocation `input` JSON and return an output JSON.
- Components should not “reach back” to earlier nodes by reading state as a default rule.
- Runners provide prior outputs to node config templating via `prev` and `node.<id>`.

**State**
- State is persistent, tenant-scoped storage exposed to components via `greentic:state/store@1.0.0`.
- Host decides backend and enforces tenant/team/user scoping using `TenantCtx`.
- State access must be capability-gated by the host/runner.

### 3) Define the payload templating contract (context vars)
In the docs, define the runner-provided templating context:

- `entry`: initial flow input payload (immutable; `{}` if none)
- `prev`: output JSON of the previously executed node (or `{}` for the first node in a path)
- `node`: map of executed node outputs by node id, accessible like `{{node.start.user.id}}`
- `state`: runner-provided **state view** (runner-defined; see below)

#### Important: what `state` means in templates
Interfaces docs must NOT imply that `{{state.foo}}` magically reads arbitrary persistent store keys.
Instead, specify:

- `state` in templates is a runner-defined view (e.g., current node state, or flow-local memory), OR
- runner may provide explicit helper functions to read/write persistent state, but those are runner-specific.

### 4) Typed insertion rule (avoid “json helpers”)
To avoid “programmy helpers” and prevent implicit stringification:
- If a YAML/JSON scalar value is **exactly** `{{expr}}` (no surrounding text), the runner MUST:
  - evaluate `expr` against the template context, and
  - insert the resulting value as a **typed JSON value** (number/bool/object/array/string/null),
  - not as a rendered string.
- If a scalar contains additional text (e.g., `"https://x/{{entry.user_id}}"`), it is rendered as a string.

Document this rule clearly with examples.

### 5) Example (simple, no helpers)
Add a concise example in docs that matches expected node syntax:

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
- `process.transform.user_id` uses typed insertion (number stays number).
- This doc does NOT prescribe whether the operation key is `do:` or `transform:`; it just demonstrates “operation key with input object”.

---

## Acceptance Criteria
- Docs clearly state `greentic:state/store@1.0.0` is the canonical component state ABI.
- Payload vs state responsibilities are unambiguous.
- Templating context vars (`entry`, `prev`, `node`, `state`) are documented.
- Typed insertion rule is documented and explained with examples.
- No docs recommend runner-internal snapshot key derivations for components.

## Notes for Codex
- Keep changes doc-focused unless you discover real duplicated/conflicting WIT packages that must be resolved.
- If you touch WIT, do it additively and avoid breaking downstream imports.
