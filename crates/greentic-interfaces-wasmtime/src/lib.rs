#![forbid(unsafe_code)]

include!(concat!(env!("OUT_DIR"), "/gen_all_worlds.rs"));

pub mod host_helpers;

pub use host_helpers::{SecretsError, SecretsStoreHost, add_secrets_store_to_linker};

/// Typed helpers for `greentic:events-bridge@1.0.0` worlds.
pub mod events_bridge {
    pub use crate::events_bridge_event_to_message_bridge_v1_0::EventToMessageBridge;
    pub use crate::events_bridge_message_to_event_bridge_v1_0::MessageToEventBridge;
}
