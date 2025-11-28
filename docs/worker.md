# greentic:worker@1.0.0

Generic worker ABI for invoking assistants/workers without domain coupling. Runner and messaging use this envelope to call repo/store/brand assistants or other workers over any transport.

- **World:** `greentic:worker/worker@1.0.0` with `exec(req: WorkerRequest) -> result<WorkerResponse, WorkerError>`.
- **WorkerRequest:** `version`, `tenant: TenantCtx`, `worker-id`, optional `correlation-id` / `session-id` / `thread-id`, `payload-json` (opaque), `timestamp-utc` (ISO8601 UTC).
- **WorkerMessage:** `{ kind, payload-json }` for text/cards/events.
- **WorkerResponse:** mirrors request identifiers, includes `messages: list<WorkerMessage>`, `timestamp-utc`.
- **WorkerError:** `{ code, message }` for structured failures.

Notes:
- JSON payloads stay as strings to keep the ABI language-neutral.
- Domain-agnostic: no repo/store/channel concepts; higher layers provide semantics.
- Responses are non-streaming; multiple messages are returned via the `messages` list.
