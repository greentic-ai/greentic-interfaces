# Events WIT packages

The `greentic:events@1.0.0` package now groups the shared event envelope and three worlds:

- `broker`: publish/subscribe with pull-based `subscribe` + `next-event` + `ack-event`.
- `source`: start a provider source and pull events via `next-event`.
- `sink`: deliver an event to an external transport and return a structured `delivery-result`.

The canonical `event-envelope` mirrors the shared types crate: id/topic/type/source, tenant context, subject, RFC3339 time, correlation id, JSON payload, and metadata key/value pairs. Subscription options are durable/deliver-existing with manual or auto ack.

`greentic:events-bridge@1.0.0` adds two worlds for message↔event conversion:

- `message-to-event-bridge.handle-message(msg: channel-message-envelope) -> list<event-envelope>`
- `event-to-message-bridge.handle-event(ev: event-envelope) -> list<channel-message-envelope>`

Channel message envelopes live in `greentic:messaging/session@1.0.0` and remain provider-agnostic (id, tenant, channel, session, optional text/user, generic attachments, metadata).

## Host/guest examples

Host-side (Wasmtime) instantiation and call into a message→event bridge:

```rust
use greentic_interfaces_host::events_bridge::MessageToEventBridge;
use wasmtime::component::{Component, Linker};
use wasmtime::Store;

let engine = wasmtime::Engine::default();
let component = Component::from_file(&engine, "bridge.component.wasm")?;
let mut store = Store::new(&engine, ());
let mut linker = Linker::new(&engine);

let bindings = MessageToEventBridge::instantiate(&mut store, &component, &linker)?;
let api = bindings.greentic_events_bridge_bridge_api();
let emitted = api.call_handle_message(&mut store, incoming_message)?;
```

Guest-side export for the same world:

```rust
use greentic_interfaces_guest::events_bridge::message_to_event_bridge::{export, bridge_api, Guest};

struct MyBridge;

impl Guest for MyBridge {
    fn handle_message(msg: bridge_api::ChannelMessageEnvelope) -> Vec<bridge_api::EventEnvelope> {
        vec![bridge_api::EventEnvelope {
            id: "id-1".into(),
            topic: "topic".into(),
            type_: "example".into(),
            source: "component://demo".into(),
            tenant: msg.tenant.clone(),
            subject: None,
            time: "2024-01-01T00:00:00Z".into(),
            correlation_id: None,
            payload_json: msg.payload_json.clone(),
            metadata: vec![],
        }]
    }

    fn handle_event(_ev: bridge_api::EventEnvelope) -> Vec<bridge_api::ChannelMessageEnvelope> {
        Vec::new()
    }
}

export!(MyBridge);
```

## Ack semantics (broker/source)

- If `ack-mode` is `manual`, the host **must** call `ack-event(sub, event-id)` after successfully handling an event. Failure to ack means the host may redeliver according to its retry/DLQ policy; the ABI does not enforce timing.
- If `ack-mode` is `auto`, providers may auto-ack on delivery; hosts should still be prepared for occasional redeliveries.
- `next-event` returns `option<event-envelope>`; a `none` typically means timeout/idle.

## Delivery result semantics (sink)

`delivery-result` reports `status` (`ok` | `retryable-failure` | `permanent-failure`) plus optional `error-code` / `error-message`. Hosts can use this to decide retry vs DLQ handling; providers should map transport errors into these buckets.

## For consumers

- greentic-events: use the broker/source/sink worlds as-is for providers and routing; do not introduce custom WIT.
- Provider/bridge components: implement the relevant world (`broker`, `source`, `sink`, `message-to-event-bridge`, `event-to-message-bridge`) and speak the shared envelopes.
- greentic-messaging: reuse `channel-message-envelope` from `messaging/session@1.0.0` and the bridge worlds; keep channel-specific details in adapters/metadata.

## Backwards compatibility

- The legacy emitter world has been removed; prefer the broker/source/sink worlds for all new work.
