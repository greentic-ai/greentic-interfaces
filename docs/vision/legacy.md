# Legacy Compatibility Surfaces

This page tracks older ABI surfaces that remain available for compatibility.

## Policy

- Existing consumers may continue to use these surfaces.
- New projects should not adopt them.
- Migrations should target the canonical v0.6/v1 stack in `docs/vision/v0_6.md`.

## Legacy Surface Matrix

| Legacy surface | Status | Replacement target |
| --- | --- | --- |
| `greentic:component@0.4.0` | compatibility only | `greentic:component@0.6.0` |
| `greentic:component@0.5.0` | compatibility only | `greentic:component@0.6.0` |
| `greentic:pack-export@0.2.0` | compatibility only | `greentic:pack-export-v1@0.1.0` |
| `greentic:pack-export@0.4.0` | compatibility only | `greentic:pack-export-v1@0.1.0` |
| `greentic:types-core@0.2.0` | compatibility only | `greentic:types-core@0.6.0` |
| `greentic:types-core@0.4.0` | compatibility only | `greentic:types-core@0.6.0` |
| `greentic:host@1.0.0` (`runner-host`) | compatibility only | dedicated host imports (`http-client@1.1.0`, `state-store@1.0.0`, `secrets-store@1.0.0`, `telemetry-logger@1.0.0`) |
| `greentic:distributor-api@1.0.0` | compatibility only | `greentic:distributor-api@1.1.0` |
| `wasix:mcp@24.11.5` | compatibility only | `wasix:mcp@25.6.18` |
| `wasix:mcp@25.3.26` | compatibility only | `wasix:mcp@25.6.18` |

## Removed Legacy Families

- `greentic:events@1.0.0` and `greentic:events-bridge@1.0.0`
- `greentic:secrets-provider@0.1.0` (+ add-ons)
- legacy typed provider protocol families replaced by `provider-schema-core@1.0.0`
