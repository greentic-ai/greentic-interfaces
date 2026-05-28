//! Validation routines for WIT-derived metadata.

use greentic_types::error::{ErrorCode, GResult, GreenticError};
use semver::Version;

fn invalid_input(message: impl Into<String>) -> GreenticError {
    GreenticError::new(ErrorCode::InvalidInput, message)
}

/// Validates provider metadata generated from WIT bindings.
pub fn validate_provider_meta(meta: crate::abi::v0_6_0::provider::ProviderMeta) -> GResult<()> {
    if meta.name.trim().is_empty() {
        return Err(invalid_input("provider name must not be empty"));
    }

    Version::parse(&meta.version)
        .map_err(|err| invalid_input(format!("invalid semantic version: {err}")))?;

    for domain in &meta.allow_list.domains {
        if domain.trim().is_empty() {
            return Err(invalid_input(
                "allow-list domains must not contain empty entries",
            ));
        }
    }

    for port in &meta.allow_list.ports {
        if *port == 0 {
            return Err(invalid_input("allow-list ports must be greater than zero"));
        }
    }

    for protocol in &meta.allow_list.protocols {
        if let crate::canonical::types::Protocol::Custom(value) = protocol
            && value.trim().is_empty()
        {
            return Err(invalid_input(
                "custom protocol identifiers must not be empty",
            ));
        }
    }

    if meta.network_policy.deny_on_miss {
        // strict policies must include at least one allowed entry to avoid total lockout
        let allow = &meta.network_policy.egress;
        if allow.domains.is_empty() && allow.ports.is_empty() && allow.protocols.is_empty() {
            return Err(invalid_input(
                "network policy denying misses requires explicit allow rules",
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::abi::v0_6_0::provider::ProviderMeta;
    use crate::canonical::types::{AllowList, NetworkPolicy, Protocol};

    fn valid_meta() -> ProviderMeta {
        ProviderMeta {
            name: "provider".into(),
            version: "1.2.3".into(),
            capabilities: vec!["read".into()],
            allow_list: AllowList {
                domains: vec!["api.example.com".into()],
                ports: vec![443],
                protocols: vec![Protocol::Https],
            },
            network_policy: NetworkPolicy {
                egress: AllowList {
                    domains: vec!["api.example.com".into()],
                    ports: Vec::new(),
                    protocols: Vec::new(),
                },
                deny_on_miss: true,
            },
        }
    }

    fn assert_invalid(meta: ProviderMeta, expected: &str) {
        let err = match validate_provider_meta(meta) {
            Ok(()) => panic!("metadata should be rejected"),
            Err(err) => err,
        };
        assert!(
            err.to_string().contains(expected),
            "expected '{err}' to contain '{expected}'"
        );
    }

    #[test]
    fn accepts_valid_provider_metadata_with_strict_network_policy() {
        if let Err(err) = validate_provider_meta(valid_meta()) {
            panic!("valid metadata rejected: {err}");
        }
    }

    #[test]
    fn rejects_empty_provider_name() {
        let mut meta = valid_meta();
        meta.name = " \t ".into();
        assert_invalid(meta, "provider name");
    }

    #[test]
    fn rejects_invalid_semver() {
        let mut meta = valid_meta();
        meta.version = "2026-05-28".into();
        assert_invalid(meta, "semantic version");
    }

    #[test]
    fn rejects_empty_allow_list_domain() {
        let mut meta = valid_meta();
        meta.allow_list.domains.push(" ".into());
        assert_invalid(meta, "domains");
    }

    #[test]
    fn rejects_zero_allow_list_port() {
        let mut meta = valid_meta();
        meta.allow_list.ports.push(0);
        assert_invalid(meta, "ports");
    }

    #[test]
    fn rejects_empty_custom_protocol_identifier() {
        let mut meta = valid_meta();
        meta.allow_list.protocols.push(Protocol::Custom(" ".into()));
        assert_invalid(meta, "custom protocol");
    }

    #[test]
    fn rejects_deny_on_miss_without_allow_rules() {
        let mut meta = valid_meta();
        meta.network_policy.egress = AllowList {
            domains: Vec::new(),
            ports: Vec::new(),
            protocols: Vec::new(),
        };
        assert_invalid(meta, "explicit allow rules");
    }
}
