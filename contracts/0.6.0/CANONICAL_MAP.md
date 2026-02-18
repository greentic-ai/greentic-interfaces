# Canonical Map for Greentic 0.6.0

This map picks a single authoritative source for every contract concept before the 0.6 reset begins. Each domain lists the previous candidate versions, the 0.6.0 canonical shape, what we intentionally drop, and the migration/compatibility notes that drive the choice.

## 1. Contexts: tenant-ctx + exec-ctx
- **Versions considered:** `component@0.5.0`/`component@1.0.0` built on `greentic:interfaces-types/types@0.1.x`; frameworks that defined ad-hoc `exec-ctx`; and the direction of the new `greentic-types-core` line that already carries telemetry metadata.
- **0.6 canonical shape:** every host-run invocation passes a `TenantCtx` record with `tenant_id`, `team_id`, `user_id`, `env_id`, `trace_id`, `correlation_id`, `deadline_ms`, `attempt`, `idempotency_key`, and (new for 0.6) a mandatory `i18n_id`. The execution context adds `flow_id`, optional `step_id`, and `component_id` references so the host can build the `InvocationEnvelope` without relying on step-local state.
- **Dropped fields:** no JSON payload strings (now CBOR), and we stop maintaining parallel `exec-ctx` records in downstream packs because the host owns the envelope. Legacy hosts that filled both `tenant-ctx` and scattered `ctx` records can now converge on the single `TenantCtx` record.
- **Compatibility notes:** 0.5/1.0 components still expect most of the same fields, so hosts MUST continue populating the `trace_id`, `correlation_id`, and `deadline` pieces while also providing the new `i18n_id`. The canonical 0.6.0 `TenantCtx` is emitted by `greentic:types-core@0.6.0`, and bindings in `crates/greentic-interfaces` already ensure that any generated `TenantCtx` includes the new fields.

## 2. Identities: env, tenant, team, user, flow, step, component
- **Versions considered:** prior `types-core@0.2.0/0.4.0`, `common-types@0.1.0`, and the `component` worlds that each redefined `env-id`/`tenant-id` interleaved with `exec-ctx`/`tenant-ctx` copies.
- **0.6 canonical shape:** `greentic:types-core@0.6.0` publishes primitive aliases (`env_id`, `tenant_id`, `team_id`, `user_id`, `flow_id`, `step_id`, `component_id`) plus `trace_id`, `correlation_id`, and `i18n_id`. All WIT packages import these aliases instead of redefining strings. Flow graphs keep the canonical IDs in their YML/JSON manifests (prefer `steps.<id>` for per-step keys), and runtime bindings map them into the host-local `TenantCtx` without duplication.
- **Dropped fields:** ad-hoc `exec-ctx.user_id` variants that duplicated tenant/team semantics are retired in favor of the single `user_id` alias. Hosts should stop inventing composite `step.component` ids and instead surface the canonical `component_id` and the `flow_id`/`step_id` pair passed through the call spec.
- **Compatibility notes:** When migrating older manifests, replace legacy scaffolding with the canonical step names listed in `contracts/0.6.0/RENAMES.md` (the new aliases treat each execution unit as a `step`). Legacy worlds that still reference `types-core@0.2.0` or `component@0.4.x` will receive the same string values, but tooling should emit warnings that only the 0.6 aliases are the authoritative set going forward.

## 3. Invocation envelope vs. call spec
- **Versions considered:** the earlier practice of persistently storing the host-created `component-env` envelope within packs/flows vs. the updated 0.6 spec that only stores a call spec.
- **0.6 canonical shape:** hosts build an `InvocationEnvelope` containing `TenantCtx`, `flow_id`, `step_id`, `attempt`, `payload_cbor: list<u8>`, and optional `metadata_cbor`. Packs/flows only record the immutable call spec with `op`, `payload_cbor`, and optional `metadata_cbor`; runtime wiring adds the contextual metadata (TenantCtx plus flow/step/attempt) right before invoking the component. Runtime examples live in `docs/payload-and-state.md`.
- **Dropped fields:** flows no longer serialize any portion of the envelope (tenant entries, attempts, heartbeat). Pack definitions that previously included `envelope` objects are deprecated in favor of the 0.6 call spec.
- **Compatibility notes:** Lints (PR-00/PR-08) will reject call specs that resemble envelopes. Hosts which still accept envelope-style inputs for legacy components must translate them into the new call spec before invoking 0.6 components.

## 4. Error model
- **Versions considered:** `component@0.4.x/0.5.x/1.0.0` with the shared `node-error` record and `greentic:common-types` diagnostics.
- **0.6 canonical shape:** `node-error` retains `code`, `message`, `retryable`, optional `backoff_ms`, and optional `details`. `retryable` continues to gate exponential/backoff strategies, while `details` remains opaque (usually JSON) for richer telemetry. Hosts MUST respect `backoff_ms` when present and propagate `details` through diagnostics.
- **Dropped fields:** there are no new fields to drop, but `components` should stop inventing incompatible error payloads—they must return `node-error` or `result` wrappers so hosts keep consistent handling.
- **Compatibility notes:** existing components on 0.5.x already follow this schema, so the primary compatibility work is ensuring the host does not mutate the error contract while streaming CBOR payloads upstream.

## 5. State-store / secrets-store / http-client / telemetry
- **Versions considered:** older `kv-v1` helpers, `secrets-provider@0.1.0`, `http-client@1.0.0`, `telemetry-logger@1.0.0`, and earlier driver extras copied into downstream repos.
- **0.6 canonical shape:** prefer `greentic:state/store@1.0.0` for all persistent state operations, `greentic:secrets-store/store@1.0.0` for host-only secrets reads, `greentic:http-client@1.1.0` for HTTP interactions, and `greentic:telemetry-logger@1.0.0` for observability. These packages continue to live in `wit/greentic/*` and are the only bindings honored by `docs/interfaces_inventory.md` (platform-base list).
- **Dropped fields:** remove any packaged `secrets-provider@0.1.0` worlds and any locally defined state helpers that diverge from the host-facing store; `kv-v1` is no longer considered canonical. Component authors should consult `docs/secrets-provider.md` for future provider-core guidance.
- **Compatibility notes:** Hosts keep the state/secret/http imports stable, but migration tooling (PR-09) must rewrite any recipes that relied on deprecated `secrets-provider` or `kv-v1` packages so they call the canonical stores instead.

## 6. i18n identity/profile linkage
- **Versions considered:** earlier `tenant-ctx` definitions that had `i18n_id` optional, along with `greentic:types-core@0.4.0`-style locale hints.
- **0.6 canonical shape:** `TenantCtx.i18n_id` is mandatory and the host populates it with the runtime’s locale stack (defaulting to `en-US` when the flow has no explicit preference). All bindings (`crates/greentic-interfaces`) include build-time checks that fail when a `TenantCtx` lacks `i18n_id` (see `build.rs`).
- **Dropped fields:** nothing is dropped, but old component descriptors that only emitted `i18n_id` when `profile.locale` was present must now always provide the field—even if it is an empty string, hosts still need the identifier to resolve telemetry/localization rules.
- **Compatibility notes:** tooling migrating from 0.5.x should fill `i18n_id` in new descriptors and manifests. 0.6.0 components can still run when the host sends `None`, but the guardrails document that future releases may mark the field required.

## 7. Component descriptor + capabilities + op schemas
- **Versions considered:** descriptor-less 0.5.x flows, limited `describe()` metadata, and ad-hoc capability lists maintained by runtimes.
- **0.6 canonical shape:** descriptor JSON (or CBOR) MUST include `ops` with `input`/`output` schema sources, `content_type: application/cbor`, and `examples` of actual CBOR bytes; `describe()` also advertises capabilities (`state-store`, `secrets-store`, optional HTTP clients) plus the optional `setup` contract. Inline/packed descriptor examples are available under `tests/fixtures/component-descriptor-example.json` and `docs/component-descriptor.md`.
- **Dropped fields:** the 0.5 descriptor relied on JSON payload strings; 0.6 refuses to surface `inline_json`. Host runtimes should stop recording descriptor metadata outside the official descriptor (no config in separate metadata registries).
- **Compatibility notes:** Hosts use the descriptor to resolve whether the component speaks 0.5 or 0.6 (see `docs/DISCOVERY_COMPONENT_INVOKE.md`). Capabilities enumerated by earlier releases are translated to the canonical capability names; this ensures `describe()` returns deterministic lists for capability enforcement (PR-06).

## 8. Pack/flow manifests + invocation specs
- **Versions considered:** pack manifest drafts that stored full envelopes or developer-specific payload fields, plus pack exporters runtime-specific logic.
- **0.6 canonical shape:** flow steps reference components via `{ op: "<op>", payload_cbor: "<bytes>", metadata_cbor: "<bytes?>" }`. Pack manifests list component descriptors, required world versions (0.6.x) and schema IDs with stable blake3 hashes. The templates/flows treat payloads as typed CBOR, while templating helpers continue to use `entry`, `prev`, and the new `steps.<id>` references for each prior helper key, as described in `docs/payload-and-state.md`.
- **Dropped fields:** `entry_payload`, `envelope`, or any JSON-only `input` objects inside saved flows are retired—payloads must be stored as raw CBOR bytes. The `builder` tool (PR-09) rewrites old steps to match the new schema.
- **Compatibility notes:** `cards2pack` migration and downstream packs must adopt the call spec frame, but legacy flows may remain in 0.5 compatibility mode behind feature flags until all dependencies converge.

## Versioning guardrail
- Never edit record shapes or field names in place; every breaking change must bump the package version (0.6.0 → 0.6.1, component → component@0.6.1, etc.).
- When migrating legacy flows, document which version prevailed and keep the older version in `COMPAT_MATRIX.md` so future tooling can still understand the correspondence.
- The canonical map above should be the source of truth for any re-exported names or new tooling that relies on Greentic 0.6.x semantics.
