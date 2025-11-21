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
pub mod events_emitter {
    pub use greentic_interfaces::events_emitter_v1::*;
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

/// Legacy/compat helpers.
pub mod misc {
    pub use greentic_interfaces::oauth_v0_1::*;
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
