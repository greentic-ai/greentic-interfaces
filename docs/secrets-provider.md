# Secrets provider WIT packages

The legacy typed secrets provider worlds (`greentic:secrets-provider@0.1.0` plus the generators/audit-exporter/policy-validator add-ons) have been removed. Provider implementations must migrate to `greentic:provider-schema-core@1.0.0` and publish JSON schemas instead of relying on WIT-defined provider operations.

- Hosts continue to expose `greentic:secrets-store/store@1.0.0` for consumers that need to read secrets; this surface is unchanged.
- Secrets requirements are still modeled via `greentic:secrets-types@1.0.0`, but provider components should no longer import typed provider worlds.
- Use digest-pinned provider-core schemas to describe capabilities and configuration for secrets providers going forward.
