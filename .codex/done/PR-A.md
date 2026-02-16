PR-A: greentic-interfaces – 0.6.0 Context + Descriptor Setup Contract + I18n Id

Base branch: release/0.6.0
Do not change master for breaking work. Urgent fixes go to master then cherry-pick.

Goal

Introduce 0.6.0-ready additions to support:

session-wide i18n_id propagation

optional component “dynamic setup” via greentic-qa specs embedded/referenced in component descriptors

keep this orthogonal to invocation ABI / CBOR I/O

Design constraints

WIT remains simple; keep large blobs as list<u8> / string sources.

i18n_id is a WIT string; validation happens in Rust wrappers.

Setup contract is optional: components may omit it.

No imperative QA tools language in 0.6.0.

Work items
1) Add i18n_id to canonical context records

Identify the canonical “ctx/session” record(s) used by Greentic worlds and add:

i18n_id: string

Notes:

Hosts default it when absent (legacy), e.g. tenant default or en-US.

Do not add full resolved profile to ctx in 0.6.0.

Acceptance

Generated bindings compile.

Updated fixtures/tests.

2) Add optional setup contract to component descriptor/manifest

Add a new optional field on the component descriptor record(s):

setup: option<setup_contract>

Suggested WIT shapes (adapt to repo naming):

record setup_contract {
  qa_spec: qa_spec_source,
  answers_schema: option<schema_source>,
  examples: list<example_answers>,
  outputs: list<setup_output>,
}

variant qa_spec_source {
  inline_cbor(list<u8>),
  inline_json(string),
  ref_uri(string),
  ref_pack_path(string),
}

variant schema_source {
  cbor_schema_id(string),
  inline_cbor(list<u8>),
  inline_json(string),
}

record example_answers {
  title: string,
  answers_cbor: list<u8>,
  notes: option<string>,
}

variant setup_output {
  config_only,
  template_scaffold(record {
    template_ref: string,
    output_layout: option<string>,
  }),
}


Notes:

Prefer CBOR for 0.6.0; allow JSON inline for transition.

ref_pack_path convention: document whether it’s relative to component root or pack root.

Acceptance

Descriptor encode/decode round-trips.

Old descriptors without setup remain valid.

Host tooling can read new setup.

3) Docs + small fixture

Add:

doc page: meaning of i18n_id, flow through ctx, how setup is used by greentic-component wizard / greentic-pack wizard.

fixture: descriptor example with inline CBOR QA spec + config-only output + one example answers.

Tests

Unit tests: setup contract round-trip.

Compile-time check: i18n_id exists in ctx type(s).

Update golden WIT/bindings if repo uses them.

Compatibility plan

Keep 0.5 worlds intact.

0.6 additions: new package/version or feature-gated per repo conventions.

MIGRATION note:

“hosts default i18n_id when absent”

“setup is optional; ignore unknown fields”