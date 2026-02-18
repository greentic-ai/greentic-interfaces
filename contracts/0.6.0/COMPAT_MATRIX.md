# Compatibility Matrix (0.6.0 canonical line)

| Domain | Package / Version | Status | Notes |
| --- | --- | --- | --- |
| Components | `component@0.4.0` | Legacy (deprecated) | Replaced by 0.6.0; keep around only for emergency rollback documentation. |
| Components | `component@0.5.0` | Legacy (deprecated) | Hosts may still load 0.5.0 behind `LEGACY_APPROVED` guards, but new tooling must not emit 0.5 descriptors. |
| Components | `component@1.0.0` | Legacy (deferred) | Maintained for historical artifacts; descriptor metadata now flows through 0.6.0. |
| Types | `types-core@0.2.0` / `types-core@0.4.0` | Legacy (deprecated) | Fields already mirrored in 0.6.0; tooling should point consumers toward `types-core@0.6.0`. |
| Types | `types-core@0.6.0` | Canonical | Source for all env/tenant/team/user/flow/node/component IDs plus `i18n_id`. |
| State | `greentic:state/store@1.0.0` | Canonical | The only persistent state API components should use. |
| Secrets | `greentic:secrets-store/store@1.0.0` | Canonical | Host-only import; components may read secrets when granted capability. |
| Secrets providers | `secrets-provider@0.1.0` (+ generators) | Removed | Replaced by provider-core schema manifests; no new edits allowed. |
| HTTP | `http-client@1.0.0` | Legacy (maintained) | Still published for consumers that cannot immediately switch, but new work should target 1.1.0 and no longer extend the old world. |
| HTTP | `http-client@1.1.0` | Canonical | Preferred world for making outbound requests. |
| Telemetry | `telemetry-logger@1.0.0` | Canonical | Observability hook that components use with `TenantCtx` metadata. |
| Flows | `pack-export@0.2.0` / `pack-export-v1@0.1.0` | Legacy | Flow builders should move to the new CBOR call spec; no new envelopes may be serialized by cards2pack. |
| Descriptors | Pre-PR descriptor metadata (JSON-only) | Deprecated | 0.6.0 descriptors now require `ops`, CBOR examples, and schema refs. |
| Ad-hoc contexts | `node+step` combos (non-canonical names) | Deprecated | Rename to the unified names in `RENAMES.md`. |

## Legacy versions scheduled for removal
- `component@0.4.0`, `component@0.5.0`, `component@1.0.0`
- `types-core@0.2.0`, `types-core@0.4.0`
- `secrets-provider@0.1.0` and every auxiliary generator/policy tool tied to it
- `http-client@1.0.0`, `pack-export@0.2.0`, `pack-export-v1@0.1.0`
- Any custom `exec-ctx`/`tenant-ctx` duplicates that are not sourced from `types-core@0.6.0`

## Guardrail
- Do not mutate record shapes in place; spinning up a new canonical field set always warrants a version bump (`component@0.6.1`, `types-core@0.6.1`, etc.).
- Tools that convert legacy flows must emit `migration_report.json` entries for each deprecated version touched so we can audit the transition history.
