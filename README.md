# greentic-interfaces

Define the WebAssembly interface types shared by Greentic component modules and their hosts. The WIT files under `wit/` are versioned by package name to enable additive evolution without breaking existing components.

## Package overview

- `greentic:component@0.3.0` exposes the original contract used by early components. Hosts provide a `tenant_ctx` and receive a JSON payload or plain string error. Streaming and lifecycle hooks are not available.
- `greentic:component@0.4.0` adds structured execution context, explicit error envelopes, and streaming progress. Components surface cooperative control via the `control` interface and may opt into lifecycle hooks (`on-start`, `on-stop`).

### Mapping to `greentic-types`

| Rust type (`greentic-types`) | WIT type                          |
| ---------------------------- | --------------------------------- |
| `TenantCtx`                  | `tenant_ctx`                      |
| `ExecCtx`                    | `exec_ctx`                        |
| `InvokeResult`               | `invoke_result`                   |
| `StreamEvent`                | `stream_event`                    |
| `NodeError`                  | `node_error`                      |

All structured payloads are JSON strings (`type json = string`) to stay language-agnostic. Components should deserialize into native types as needed.

## Version negotiation

Hosts should enumerate supported packages in descending order and attempt to instantiate the highest version offered by the component module. When a module only implements `component@0.3.0`, the host must fall back to the legacy interface and avoid calling lifecycle or streaming APIs. When both parties expose `component@0.4.0`, hosts are expected to:

1. Bind the `control` interface (if the component imports it) to cooperate with cancellation and scheduling.
2. Invoke lifecycle hooks opportunistically. Components that do not need them must still export the functions but may return `Ok(())` immediately.
3. Prefer `invoke-stream` when progressive updates are desired; otherwise call `invoke`.

Consumers should document the chosen version in their manifest to simplify future migrations.

## Error semantics

- `invoke` returns `invoke_result`. A successful execution wraps the JSON payload in `ok(json)`.
- Recoverable failures surface as `err(node_error)` with `retryable = true` and an optional `backoff_ms` hint.
- Non-retryable errors set `retryable = false`. Hosts may surface `message` directly to operators.
- `invoke-stream` is expected to end with either `stream_event::done` or `stream_event::error`. Specific progress updates use `stream_event::progress` with values from `0` to `100`.

Legacy `component@0.3.0` continues to use `result<json, string>` for synchronous replies; hosts should translate that into `invoke_result::ok` or a generic `node_error` when bridging between versions.

## Validation

Run `scripts/validate-wit.sh` (once `wit-bindgen` is installed) to lint the interface definitions:

```bash
bash scripts/validate-wit.sh
```

## License

MIT
