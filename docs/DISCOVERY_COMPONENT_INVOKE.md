# Component Invoke & Messaging Ops Contract

## Invocation contract (component@0.5.0)

Components implement `node.invoke(ctx, op, input)` as defined in `greentic:component@0.5.0`. `ctx` is an `exec-ctx` with a `tenant` identity, `flow-id`, and optional `node-id` providing the host with tenant scoping, correlation, and tracing metadata. `op` is a host-defined string that selects which capability the component should execute, and `input` is a JSON payload encoded as UTF-8. Every call returns an `invoke-result` (either `ok(json)` or `err(node-error)`) so the host can surface component responses or failures in a uniform way.

### Execution and tenant context

- `tenant-ctx` carries tenant identifier, optional team/user names, trace and correlation IDs, deadline (Unix milliseconds), attempt count, and optional idempotency key; hosts populate whatever subset is relevant and components may log or forward them for observability and deduplication.
- `exec-ctx.flow-id` identifies the flow that triggered the call, and `exec-ctx.node-id` is provided when the host invokes a specific flow node.
- The `json` type in this interface is just a string alias; components should deserialise it with their preferred JSON library and emit responses as JSON strings as well.

## Operation dispatch

Op string dispatch is entirely up to the host-component contract. Components SHOULD treat `op` values as versioned, semantic keywords so the host can add new operations without changing the WIT surface. The host must also document the semantics of every supported op and the expected JSON schema for requests and responses.

## Node error & retry semantics

`node-error` is the shared error record returned by `invoke` failures. Its fields are:

- `code`: a structured identifier for the class of failure.
- `message`: a human-readable description.
- `retryable`: when `true`, the host/orchestrator is invited to re-run the operation (often with exponential or bounded backoff). When `false`, the failure is terminal and the host should not retry.
- `backoff-ms` (optional): the minimum number of milliseconds the host should wait before issuing another attempt; the host may treat the absence of `backoff-ms` as a suggestion that retry is safe immediately but should still cap repeated retries.
- `details` (optional): opaque JSON that components can use to give hosts richer telemetry or guidance for debugging.

Hosts must interpret `retryable` together with `backoff-ms` so they do not overwhelm downstream services—retryable errors should honor the grace period in `backoff-ms`, while non-retryable errors should surface to operators immediately.

## Recommended op names

To keep the op vocabulary consistent across components and simplify wiring, prefer the following canonical names whenever the semantics align:

1. `ingest_http` – pull raw data from an HTTP source, validate it, and normalize headers/payload for downstream processing.
2. `render_plan` – take flow state and render a plan or UI model (cards, steps, etc.) that downstream hosts can present to users.
3. `encode` – transform in-memory or domain models into wire-encoded payloads (JSON, CBOR, etc.) without sending them yet.
4. `send_payload` – deliver an already-encoded payload to an external system (HTTP, message queue, etc.) and report its status.

These names are guidelines, not enforcement; hosts may extend the list as needed, but new names should be documented alongside the corresponding JSON schema so components and hosts stay in sync.

## Migration note: CBOR payloads (component@0.6.0)

Future releases (starting with `component@0.6.0`) plan to migrate the predominant payload encoding from stringified JSON to CBOR bytes for both `invoke` requests and responses. When that migration happens, expect:

- `node.invoke` will continue to take an `op` string but will pass a byte buffer instead of a UTF-8 JSON string. The host and guest contracts will clearly version the replacement (`json` will likely become `bytes`).
- Components should isolate serialization logic so switching from JSON to CBOR becomes a matter of swapping the codec, not rewriting business logic.
- Rich metadata such as `tenant-ctx`/`exec-ctx` will still flow through the WIT record; only the `input`/`output` payload encoding is changing.

Document your component's current JSON schema today and track the analogous CBOR schema so migration to `component@0.6.0` is smooth for both hosts and guests.
