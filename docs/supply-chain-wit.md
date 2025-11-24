# Supply-chain WIT worlds

All functions take `TenantCtx` (from `greentic:interfaces-types@0.1.0`) as the first parameter and return `result<Success, HostError>`.

- `greentic:source@1.0.0` (`source-sync`): list repositories/branches, fetch commit metadata, register webhooks. Types: repository-ref, branch-ref, commit-ref, commit-metadata, webhook-id (all provider-agnostic with `extra` JSON).
- `greentic:build@1.0.0` (`builder`): execute build plans and fetch logs. Types: build-plan (env, outputs, entrypoint), build-status (state, timestamps, artifacts, logs), artifact-ref, build-log-ref.
- `greentic:scan@1.0.0` (`scanner`): run scans. Types: scan-kind enum, scan-request, scan-result (state, timestamps, findings JSON, optional SBOM ref), sbom-ref.
- `greentic:signing@1.0.0` (`signer`): sign/verify. Types: signing-key-ref, signature-ref, sign-request, verify-request/result.
- `greentic:attestation@1.0.0` (`attester`): generate attestations. Types: attestation-id, predicate-type, statement-ref, subject-ref.
- `greentic:policy@1.0.0` (`policy-evaluator`): evaluate policies. Types: policy-ref, policy-input-ref, policy-decision (allow/deny + reasons).
- `greentic:metadata@1.0.0` (`metadata-store`): upsert/query component metadata. Types: component-ref, version-ref, metadata-record-ref.
- `greentic:oci@1.0.0` (`oci-distribution`): minimal OCI helpers. Types: oci-image-ref, registry-ref; funcs push/get-pull-reference.

All payloads use minimal structured fields plus `extra` for provider-specific JSON, keeping the ABI stable and provider-agnostic.
