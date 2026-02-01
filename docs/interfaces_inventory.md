# Interfaces Inventory

Status of WIT worlds shipped by `greentic-interfaces`.

- **Platform-base (kept)**: component@0.4.0/0.5.0/0.6.0/1.0.0, component-v1@0.1.0, pack-export (all), pack-validate@0.1.0, provision@0.1.0, common-types@0.1.0, types-core@0.2.0/0.4.0, state-store@1.0.0, http-client@1.0.0/1.1.0, telemetry-logger@1.0.0, lifecycle@1.0.0, worker@1.0.0, build@1.0.0, scan@1.0.0, signing@1.0.0, attestation@1.0.0, policy@1.0.0, metadata@1.0.0, source@1.0.0, distribution@1.0.0, oci@1.0.0, repo-ui-actions@1.0.0, deploy-plan@1.0.0, oauth-broker@1.0.0, runner-host@1.0.0, wasix-mcp*, provider-common@0.0.2.
- **Provider-core (keep)**: provider-schema-core@1.0.0 (schema-core world). New providers must use this contract plus JSON schemas.
- **Removed legacy provider protocols**: messaging@1.0.0 (session), events@1.0.0 (broker/source/sink/events bridge), secrets-provider@0.1.0 (+ generators/audit-exporter/policy-validator deps). Provider-core is the only supported provider protocol going forward.
- **Other/unrelated**: gui@1.0.0, distributor@1.0.0/1.1.0 (distributor-api), secrets-store@1.0.0 (host-only import), pack-export-v1@0.1.0, describe-v1, misc host-import helpers.

Migration rule: future providers MUST publish provider-core manifests and schemas and should not extend legacy WIT worlds. Use digest-pinned schemas with provider-core.
