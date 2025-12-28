# Secrets provider WIT packages

The new provider-focused WIT contracts ship under `greentic:secrets-provider@0.1.0` and the supporting packages `greentic:secrets-generators@0.1.0`, `greentic:secrets-audit-exporter@0.1.0`, and `greentic:secrets-policy-validator@0.1.0`.

## Fetching the WIT packages

Use `wkg get` to pull the provider surface straight into a workspace (fetches deps automatically):

```bash
wkg get greentic:secrets-provider@0.1.0 --format wit --output wit/deps/
```

The generators/audit-exporter/policy-validator packages follow the same layout (e.g. `wkg get greentic:secrets-generators@0.1.0 --format wit --output wit/deps/`).

## Provider component expectations

Provider components should import `greentic:secrets-store/store@1.0.0` to read credentials from the host runtime instead of embedding credentials in their own config. The provider config flowing into the new ABI remains an opaque JSON string so hosts can evolve config fields without regenerating bindings.

## Compatibility

The provider ABI is additive: it does not change `greentic:secrets-store@1.0.0` or `greentic:secrets-types@1.0.0`. Existing consumers of the read-only secrets store continue to work unchanged.
