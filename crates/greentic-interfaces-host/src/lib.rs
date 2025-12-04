#![deny(unsafe_code)]
#![warn(missing_docs, clippy::unwrap_used, clippy::expect_used)]
//! Host-facing bindings and mappers re-exported from `greentic-interfaces`.

#[cfg(target_arch = "wasm32")]
compile_error!("greentic-interfaces-host is intended for native host targets.");

pub use greentic_interfaces::{bindings, mappers, validate};

/// Component control and exports.
pub mod component {
    /// Compatibility exports for `greentic:component/component@0.4.0`.
    pub mod v0_4 {
        pub use greentic_interfaces::component_v0_4::*;
    }
    /// Describe-only schema export world `greentic:component/component@1.0.0`.
    pub mod describe_v1 {
        pub use greentic_interfaces::component_describe_v1::*;
    }
    /// Lifecycle hooks world `greentic:lifecycle/component-lifecycle@1.0.0`.
    pub mod lifecycle_v1 {
        pub use greentic_interfaces::component_lifecycle_v1::*;
    }
}

/// Legacy host import bundles.
pub mod host_import {
    /// Host imports `0.2.0` bundle.
    pub mod v0_2 {
        pub use greentic_interfaces::host_import_v0_2::*;
    }
    /// Host imports `0.4.0` bundle.
    pub mod v0_4 {
        pub use greentic_interfaces::host_import_v0_4::*;
    }
    /// Host imports `0.6.0` bundle.
    pub mod v0_6 {
        pub use greentic_interfaces::host_import_v0_6::*;
    }
    /// Runner host bundle `greentic:host@1.0.0`.
    pub mod runner_host_v1 {
        pub use greentic_interfaces::runner_host_v1::*;
    }
}

/// Pack exporters.
pub mod pack_exports {
    /// Pack exports `0.2.0` world.
    pub mod v0_2 {
        pub use greentic_interfaces::pack_export_v0_2::*;
    }
    /// Pack exports `0.4.0` world.
    pub mod v0_4 {
        pub use greentic_interfaces::pack_export_v0_4::*;
    }
}

/// Core types.
pub mod types {
    /// Shared event envelope types.
    pub mod events_v1 {
        pub use greentic_interfaces::events_v1::*;
    }
    /// Core type defs for the 0.2 line.
    pub mod types_core_v0_2 {
        pub use greentic_interfaces::types_core_v0_2::*;
    }
    /// Core type defs for the 0.4 line.
    pub mod types_core_v0_4 {
        pub use greentic_interfaces::types_core_v0_4::*;
    }
}

/// v1 host capability contracts.
pub mod secrets {
    /// `greentic:secrets/store@1.0.0` host imports.
    pub mod store_v1 {
        pub use greentic_interfaces::secrets_store_v1::*;
    }
    /// Legacy `greentic:secrets/host@0.1.0`.
    pub mod secrets_v0_1 {
        pub use greentic_interfaces::secrets_v0_1::*;
    }
}

/// v1 host capability contracts.
pub mod state {
    pub use greentic_interfaces::state_store_v1::*;
}

/// v1 host capability contracts.
pub mod messaging_session {
    pub use greentic_interfaces::messaging_session_v1::*;
}

/// v1 host capability contracts.
pub mod events_broker {
    pub use greentic_interfaces::events_broker_v1::*;
}

/// v1 host capability contracts.
pub mod events_source {
    pub use greentic_interfaces::events_source_v1::*;
}

/// v1 host capability contracts.
pub mod events_sink {
    pub use greentic_interfaces::events_sink_v1::*;
}

/// v1 host capability contracts.
pub mod events_bridge {
    pub use greentic_interfaces::events_bridge_event_to_message_v1::EventToMessageBridge;
    pub use greentic_interfaces::events_bridge_message_to_event_v1::MessageToEventBridge;

    pub use greentic_interfaces::bindings::greentic_events_bridge_1_0_0_event_to_message_bridge::exports::greentic::events_bridge::bridge_api as event_to_message_bridge;
    pub use greentic_interfaces::bindings::greentic_events_bridge_1_0_0_message_to_event_bridge::exports::greentic::events_bridge::bridge_api as message_to_event_bridge;
}

/// v1 host capability contracts.
pub mod http_client {
    pub use greentic_interfaces::http_client_v1::*;
}

/// v1 host capability contracts.
pub mod telemetry {
    pub use greentic_interfaces::telemetry_logger_v1::*;
}

/// v1 host capability contracts.
pub mod oauth_broker {
    pub use greentic_interfaces::oauth_broker_v1::*;
}

/// v1 OAuth broker client imports.
pub mod oauth_broker_client {
    pub use greentic_interfaces::oauth_broker_client_v1::*;
}

/// Generic worker ABI world.
pub mod worker {
    pub use greentic_interfaces::worker_v1::*;
}

/// Supply-chain provider contracts.
pub mod supply_chain {
    /// Source provider world `greentic:source/source-sync@1.0.0`.
    pub mod source {
        pub use greentic_interfaces::bindings::greentic_source_1_0_0_source_sync::exports::greentic::source::source_api::*;
    }
    /// Build provider world `greentic:build/builder@1.0.0`.
    pub mod build {
        pub use greentic_interfaces::bindings::greentic_build_1_0_0_builder::exports::greentic::build::builder_api::*;
    }
    /// Scanner world `greentic:scan/scanner@1.0.0`.
    pub mod scan {
        pub use greentic_interfaces::bindings::greentic_scan_1_0_0_scanner::exports::greentic::scan::scanner_api::*;
    }
    /// Signing world `greentic:signing/signer@1.0.0`.
    pub mod signing {
        pub use greentic_interfaces::bindings::greentic_signing_1_0_0_signer::exports::greentic::signing::signer_api::*;
    }
    /// Attestation world `greentic:attestation/attester@1.0.0`.
    pub mod attestation {
        pub use greentic_interfaces::bindings::greentic_attestation_1_0_0_attester::exports::greentic::attestation::attester_api::*;
    }
    /// Policy evaluation world `greentic:policy/policy-evaluator@1.0.0`.
    pub mod policy {
        pub use greentic_interfaces::bindings::greentic_policy_1_0_0_policy_evaluator::exports::greentic::policy::policy_api::*;
    }
    /// Metadata store world `greentic:metadata/metadata-store@1.0.0`.
    pub mod metadata {
        pub use greentic_interfaces::bindings::greentic_metadata_1_0_0_metadata_store::exports::greentic::metadata::metadata_api::*;
    }
    /// OCI distribution world `greentic:oci/oci-distribution@1.0.0`.
    pub mod oci {
        pub use greentic_interfaces::bindings::greentic_oci_1_0_0_oci_distribution::exports::greentic::oci::oci_api::*;
    }
}

/// Desired state distribution contracts.
pub mod distribution {
    /// `greentic:distribution/distribution@1.0.0`.
    pub mod v1 {
        pub use greentic_interfaces::bindings::greentic_distribution_1_0_0_distribution::exports::greentic::distribution::distribution_api::*;
    }
}

/// Distributor API contracts.
pub mod distributor_api {
    /// `greentic:distributor-api/distributor-api@1.0.0`.
    pub mod v1 {
        pub use greentic_interfaces::bindings::greentic_distributor_api_1_0_0_distributor_api::exports::greentic::distributor_api::distributor::*;
    }
}

/// Stable alias for messaging session imports.
pub mod messaging {
    pub use super::messaging_session::*;
}

/// Stable alias for HTTP client imports.
pub mod http {
    pub use super::http_client::*;
}

/// Stable alias for OAuth broker imports.
pub mod oauth {
    pub use super::oauth_broker::*;
}

/// MCP router surfaces (multiple protocol snapshots).
pub mod mcp {
    /// `wasix:mcp@24.11.5` snapshot (2024-11-05 spec).
    #[cfg(feature = "wasix-mcp-24-11-05-host")]
    pub mod v24_11_05 {
        pub use greentic_interfaces::wasix_mcp_24_11_05::*;
    }

    /// `wasix:mcp@25.3.26` snapshot with annotations/audio/completions/progress.
    #[cfg(feature = "wasix-mcp-25-03-26-host")]
    pub mod v25_03_26 {
        pub use greentic_interfaces::wasix_mcp_25_03_26::*;
    }

    /// `wasix:mcp@25.6.18` snapshot with structured output/resources/elicitation.
    #[cfg(feature = "wasix-mcp-25-06-18-host")]
    pub mod v25_06_18 {
        pub use greentic_interfaces::wasix_mcp_25_06_18::*;
    }
}

/// UI action handler contracts.
pub mod ui_actions {
    /// UI action handler world `greentic:repo-ui-actions/repo-ui-worker@1.0.0`.
    pub mod repo_ui_worker {
        pub use greentic_interfaces::bindings::greentic_repo_ui_actions_1_0_0_repo_ui_worker::exports::greentic::repo_ui_actions::ui_action_api::*;
    }
}
