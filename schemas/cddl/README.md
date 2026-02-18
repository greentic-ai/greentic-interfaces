# CBOR Schema References (CDDL)

This folder holds the CBOR Description Language (CDDL) drafts that describe the canonical 0.6 invocation language.

- `invocation-envelope.cddl` describes the host-owned envelope (TenantCtx + flow/step metadata + payload/metadata bytes).
- `component-descriptor.cddl` defines the layout of the descriptor that advertise ops, schemas, examples, and optional setup contracts for self-describing components.

Future tooling (schema validation, descriptor linting, pack exports) can import these CDDL fragments via the `schemas/cddl/*` files so the CBOR payloads remain self-describing and hashable.
