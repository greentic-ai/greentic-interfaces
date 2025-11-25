#![deny(unsafe_code)]
#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]
//! Guest-facing bindings and mappers without host-world leakage.

pub mod bindings;

#[cfg(all(not(target_arch = "wasm32"), feature = "host-bridge"))]
pub mod host_bridge;

/// Component exports for `greentic:component/component@0.4.0`.
pub mod component {
    pub use crate::bindings::greentic_component_0_4_0_component::exports::greentic::component::*;
}

/// Lifecycle hooks for `greentic:lifecycle/component-lifecycle@1.0.0`.
pub mod lifecycle {
    pub use crate::bindings::greentic_lifecycle_1_0_0_component_lifecycle::exports::greentic::lifecycle::*;
}

/// Secret store imports for `greentic:secrets/store@1.0.0`.
pub mod secrets_store {
    pub use crate::bindings::greentic_secrets_1_0_0_store::greentic::secrets::secret_store::*;
}

/// State store imports for `greentic:state/store@1.0.0`.
pub mod state_store {
    pub use crate::bindings::greentic_state_1_0_0_store::greentic::state::state_store::*;
}

/// Messaging session imports for `greentic:messaging/session@1.0.0`.
pub mod messaging_session {
    pub use crate::bindings::greentic_messaging_1_0_0_session::greentic::messaging::session_api::*;
}

/// Event broker exports for `greentic:events/broker@1.0.0`.
pub mod events_broker {
    pub use crate::bindings::greentic_events_1_0_0_broker::exports::greentic::events::broker_api::*;
}

/// Event source exports for `greentic:events/source@1.0.0`.
pub mod events_source {
    pub use crate::bindings::greentic_events_1_0_0_source::exports::greentic::events::source_api::*;
}

/// Event sink exports for `greentic:events/sink@1.0.0`.
pub mod events_sink {
    pub use crate::bindings::greentic_events_1_0_0_sink::exports::greentic::events::sink_api::*;
}

/// Event/message bridge exports for `greentic:events-bridge@1.0.0`.
pub mod events_bridge {
    pub use crate::bindings::greentic_events_bridge_1_0_0_event_to_message_bridge::exports::greentic::events_bridge::bridge_api as event_to_message_bridge;
    pub use crate::bindings::greentic_events_bridge_1_0_0_message_to_event_bridge::exports::greentic::events_bridge::bridge_api as message_to_event_bridge;
}

/// HTTP client imports for `greentic:http/client@1.0.0`.
pub mod http_client {
    pub use crate::bindings::greentic_http_1_0_0_client::greentic::http::http_client::*;
}

/// Telemetry logger imports for `greentic:telemetry/logger@1.0.0`.
pub mod telemetry_logger {
    pub use crate::bindings::greentic_telemetry_1_0_0_logger::greentic::telemetry::logger_api::*;
}

/// OAuth broker imports for `greentic:oauth-broker/broker@1.0.0`.
pub mod oauth_broker {
    pub use crate::bindings::greentic_oauth_broker_1_0_0_broker::exports::greentic::oauth_broker::broker_v1::*;
}

/// Supply-chain provider contracts implemented by components.
pub mod supply_chain {
    /// Source provider world `greentic:source/source-sync@1.0.0`.
    pub mod source {
        pub use crate::bindings::greentic_source_1_0_0_source_sync::exports::greentic::source::source_api::*;
    }
    /// Build provider world `greentic:build/builder@1.0.0`.
    pub mod build {
        pub use crate::bindings::greentic_build_1_0_0_builder::exports::greentic::build::builder_api::*;
    }
    /// Scanner world `greentic:scan/scanner@1.0.0`.
    pub mod scan {
        pub use crate::bindings::greentic_scan_1_0_0_scanner::exports::greentic::scan::scanner_api::*;
    }
    /// Signing world `greentic:signing/signer@1.0.0`.
    pub mod signing {
        pub use crate::bindings::greentic_signing_1_0_0_signer::exports::greentic::signing::signer_api::*;
    }
    /// Attestation world `greentic:attestation/attester@1.0.0`.
    pub mod attestation {
        pub use crate::bindings::greentic_attestation_1_0_0_attester::exports::greentic::attestation::attester_api::*;
    }
    /// Policy evaluation world `greentic:policy/policy-evaluator@1.0.0`.
    pub mod policy {
        pub use crate::bindings::greentic_policy_1_0_0_policy_evaluator::exports::greentic::policy::policy_api::*;
    }
    /// Metadata store world `greentic:metadata/metadata-store@1.0.0`.
    pub mod metadata {
        pub use crate::bindings::greentic_metadata_1_0_0_metadata_store::exports::greentic::metadata::metadata_api::*;
    }
    /// OCI distribution world `greentic:oci/oci-distribution@1.0.0`.
    pub mod oci {
        pub use crate::bindings::greentic_oci_1_0_0_oci_distribution::exports::greentic::oci::oci_api::*;
    }
}

/// Stable alias for messaging/session imports.
pub mod messaging {
    pub use super::messaging_session::*;
}

/// Stable alias for OAuth broker imports.
pub mod oauth {
    pub use super::oauth_broker::*;
}
