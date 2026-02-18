PR-B: greentic-interfaces – 0.6.0 CBOR Invoke ABI + Self-Describing Op I/O in Descriptor

Base branch: release/0.6.0
This PR introduces the actual 0.6.0 IO boundary and “self-describing components” for runtime invoke.

Goal

Define the 0.6.0 component invocation boundary as:

CBOR in / CBOR out

with self-describing op schemas (input/output) published in the component descriptor

This PR is separate from setup/wizard concerns (PR-A).

Design constraints

CBOR is list<u8> in WIT (type cbor = list<u8>).

Invocation is deterministic and schema-validatable.

Do not require JSON schema bytes or other flaky-bytes formats on the wire.

Keep schema mechanism flexible: allow “schema id” + optional inline CBOR schema.

Avoid forcing every component to ship huge schema blobs; allow references.

Work items
1) Introduce 0.6.0 invoke interface (new world/package)

Add a new world/package for 0.6.0 invocation (naming per your conventions), e.g.:

greentic:component@0.6.0 (or your chosen path/version)

Core types:

type cbor = list<u8>;

record invoke_ctx {
  session_id: string,
  i18n_id: string, // from PR-A; include it here too
  // keep room for future capabilities
}

variant invoke_result {
  ok(cbor),
  err(node_error),
}

interface component {
  invoke: func(ctx: invoke_ctx, op: string, input: cbor) -> invoke_result;
}


Notes:

If you have streaming invoke variants, define CBOR streaming similarly.

Keep errors structured (node_error) as you already do; just move payload to CBOR.

Acceptance

Bindings generate.

A minimal sample component can compile against the new world.

2) Descriptor: self-describing op definitions

Extend the component descriptor to declare operations and their I/O schemas.

Add something like:

record op_descriptor {
  name: string,
  summary: option<string>,
  input: io_schema,
  output: io_schema,
  examples: list<op_example>,
}

record io_schema {
  schema: schema_source,        // see below
  content_type: string,         // e.g. "application/cbor"
  // optionally: schema_version or semantic version
}

record op_example {
  title: string,
  input_cbor: list<u8>,
  output_cbor: option<list<u8>>,
  notes: option<string>,
}

variant schema_source {
  cbor_schema_id(string),       // stable id understood by tooling/runner
  inline_cbor(list<u8>),        // optional for small schemas
  ref_pack_path(string),        // path to schema inside pack/component
  ref_uri(string),
}


Where to attach:

either descriptor.ops: list<op_descriptor>

or descriptor.capabilities.ops: ... if you have a nested structure

Important

This is invoke-time I/O description (op schemas), distinct from PR-A setup QA.

Use CBOR schema format consistent with your 0.6.0 plan (even if tooling initially treats it as opaque bytes).

Acceptance

Descriptor round-trip includes ops.

Tooling can read ops list, schema sources, and examples.

3) Migration strategy in interfaces

Document:

0.5 invoke is legacy JSON-string IO (or current)

0.6 invoke is CBOR IO

hosts may support both during transition

Add a MIGRATION note:

how runners/hosts choose which invoke interface to call

how to validate CBOR against schemas when available

4) Tests / fixtures

Add fixtures for:

a descriptor with two ops and inline schema references + CBOR examples

ensure encode/decode stability (no accidental re-ordering or truncation)

Add compile tests:

consumer crate can depend on both 0.5 and 0.6 WIT worlds (if you support that)

at minimum, bindings compile for 0.6 world

Non-goals (0.6.0)

Do not implement the schema engine in interfaces (that’s runner/tooling).

Do not require localization of schema docs in this PR.

Do not add QA setup contract here (PR-A owns setup).

Acceptance criteria summary

New 0.6 invoke world exists: CBOR input/output.

Descriptor supports self-describing per-op IO schema + examples.

Repo builds and bindings generate cleanly.