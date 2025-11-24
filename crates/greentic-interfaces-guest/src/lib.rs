#![deny(unsafe_code)]
#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]
//! Guest-facing bindings and mappers re-exported from `greentic-interfaces`.

pub use greentic_interfaces::{bindings, mappers};

/// Component exports for `greentic:component/component@0.4.0`.
pub mod component {
    pub use greentic_interfaces::bindings::greentic_component_0_4_0_component::exports::greentic::component::*;
}

/// Lifecycle hooks for `greentic:lifecycle/component-lifecycle@1.0.0`.
pub mod lifecycle {
    pub use greentic_interfaces::bindings::greentic_lifecycle_1_0_0_component_lifecycle::exports::greentic::lifecycle::*;
}

/// Shared host/guest types.
pub mod types {
    pub use greentic_interfaces::bindings::greentic::interfaces_types::types::*;
}

/// Secret store imports for `greentic:secrets/store@1.0.0`.
pub mod secrets_store {
    pub use greentic_interfaces::bindings::greentic_secrets_1_0_0_store::greentic::secrets::secret_store::*;
}

/// State store imports for `greentic:state/store@1.0.0`.
pub mod state_store {
    pub use greentic_interfaces::bindings::greentic_state_1_0_0_store::greentic::state::state_store::*;
}

/// Messaging session imports for `greentic:messaging/session@1.0.0`.
pub mod messaging_session {
    pub use greentic_interfaces::bindings::greentic_messaging_1_0_0_session::greentic::messaging::session_api::*;
}

/// Event broker exports for `greentic:events/broker@1.0.0`.
pub mod events_broker {
    pub use greentic_interfaces::bindings::greentic_events_1_0_0_broker::exports::greentic::events::broker_api::*;
}

/// Event source exports for `greentic:events/source@1.0.0`.
pub mod events_source {
    pub use greentic_interfaces::bindings::greentic_events_1_0_0_source::exports::greentic::events::source_api::*;
}

/// Event sink exports for `greentic:events/sink@1.0.0`.
pub mod events_sink {
    pub use greentic_interfaces::bindings::greentic_events_1_0_0_sink::exports::greentic::events::sink_api::*;
}

/// Event/message bridge exports for `greentic:events-bridge@1.0.0`.
pub mod events_bridge {
    pub use greentic_interfaces::bindings::greentic_events_bridge_1_0_0_event_to_message_bridge::exports::greentic::events_bridge::bridge_api as event_to_message_bridge;
    pub use greentic_interfaces::bindings::greentic_events_bridge_1_0_0_message_to_event_bridge::exports::greentic::events_bridge::bridge_api as message_to_event_bridge;
}

/// HTTP client imports for `greentic:http/client@1.0.0`.
pub mod http_client {
    pub use greentic_interfaces::bindings::greentic_http_1_0_0_client::greentic::http::http_client::*;
}

/// Telemetry logger imports for `greentic:telemetry/logger@1.0.0`.
pub mod telemetry_logger {
    pub use greentic_interfaces::bindings::greentic_telemetry_1_0_0_logger::greentic::telemetry::logger_api::*;
}

/// OAuth broker imports for `greentic:oauth-broker/broker@1.0.0`.
pub mod oauth_broker {
    pub use greentic_interfaces::bindings::greentic_oauth_broker_1_0_0_broker::exports::greentic::oauth_broker::broker_v1::*;
}

/// Stable alias for messaging/session imports.
pub mod messaging {
    pub use super::messaging_session::*;
}

/// Stable alias for OAuth broker imports.
pub mod oauth {
    pub use super::oauth_broker::*;
}
