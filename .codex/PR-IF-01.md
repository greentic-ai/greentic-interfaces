PR-IF-01: Add canonical facade for v0.6.0 bindings + forbid direct bindings::* usage + external-consumer package check
Why

External consumers of greentic-interfaces fail with:

could not find greentic in bindings when code references:

bindings::greentic::interfaces_types::types::*

This happens because wit-bindgen generates world/version scoped modules, e.g.

bindings::greentic_component_0_4_0_component::greentic::interfaces_types::types

So there is no stable bindings::greentic::* root in a multi-world crate.

We want to keep supporting multiple versions (today v0.6.0, tomorrow 1.2.3, 3.4.5), while giving the crate and its consumers a stable, canonical API surface.

Goals

Introduce a stable crate::canonical::* facade that points at the default ABI (v0.6.0).

Ensure all internal Rust code uses crate::canonical (never bindings::… paths).

Provide versioned modules (v0_6_0, later v1_2_3, etc.) behind features.

Add a CI check that compiles the packaged crate as a dependency (reproduces real consumer builds).

Keep the crate multi-version capable (do not revert to “only one version exists”).

Non-goals

No redesign of WIT contracts beyond naming/layout changes required for facade.

No changes to downstream repos in this PR.

No functional runtime changes beyond build-time module path stability.

Design
A) Version modules

Expose versioned modules:

crate::v0_6_0::* (default enabled)

future: crate::v1_2_3::*, crate::v3_4_5::* (feature gated)

Each version module re-exports the key generated binding modules under a stable per-version namespace.

B) Canonical module

Expose:

pub mod canonical {
    pub use crate::v0_6_0::*;
}


This gives you:

stable imports for all helpers: crate::canonical::types::ErrorCode etc.

one place to change when you later decide v1.2.3 becomes the default.

C) Internal rule: never import from bindings::*

All crate code must use crate::canonical::* or crate::vX_Y_Z::*.

Implementation plan (concrete)
1) Cargo features

In Cargo.toml:

Add features:

default = ["wit-v0_6_0"]

wit-v0_6_0 = []

future placeholders ok: wit-v1_2_3 = [], wit-v3_4_5 = []

This PR only needs v0.6.0 implemented, but sets the pattern.

2) Create facade modules

Add a new module file (suggestion):

src/abi/mod.rs

Inside:

#[cfg(feature = "wit-v0_6_0")]
pub mod v0_6_0 {
    // Replace the module path below with the actual generated module name for v0.6.0 component world.
    pub use crate::bindings::greentic_component_0_6_0_component::greentic::interfaces_types::types as types;

    // If you need core types:
    // pub use crate::bindings::greentic_component_0_6_0_component::greentic::types_core::core as core;

    // If you need node interface exports:
    // pub use crate::bindings::greentic_component_0_6_0_component::greentic::component::node as node;
}

/// Canonical ABI for greentic-interfaces helpers.
/// Today: v0.6.0. Tomorrow can be switched to v1.2.3 without changing callsites.
#[cfg(feature = "wit-v0_6_0")]
pub mod canonical {
    pub use super::v0_6_0::*;
}


Then in src/lib.rs:

pub mod abi;
pub use abi::canonical;


Important: do not guess the generated module name. Use the actual one produced by your wit-bindgen step (e.g., greentic_component_0_6_0_component or similar).

3) Rewrite all internal references away from bindings::greentic::...

Fix your exact failing lines like:

type WitErrorCode = bindings::greentic::interfaces_types::types::ErrorCode;


to:

type WitErrorCode = crate::canonical::types::ErrorCode;


In pattern matches, import from canonical:

use crate::canonical::types::Protocol;


Acceptance criterion: ripgrep finds no occurrences of:

bindings::greentic::

or any other direct binding use in general-purpose code (except inside src/abi/*).

4) Add a “packaged external consumer” CI test (mandatory)

Add ci/steps/external_consumer_check.sh:

What it must do:

cargo package --no-verify -p greentic-interfaces

locate the produced .crate

unpack to temp dir

generate temp consumer crate:

depends on unpacked greentic-interfaces by path

uses default features only

cargo check

This catches:

missing packaged files

broken build.rs assumptions

missing bindings paths (your current failure)

Wire it into CI after cargo test.

5) Docs: document canonical + multi-version strategy

Add / update:

docs/vision/v0.6.md:

canonical ABI is crate::canonical (today v0.6.0)

multi-version is supported via crate::vX_Y_Z modules and features

internal code MUST use canonical

docs/vision/codex.md:

rule: do not reference bindings::* in normal code

when adding a new version, only touch src/abi/mod.rs + docs

add external-consumer test updates if adding new features

docs/vision/deprecations.md:

how to deprecate old versions without breaking multi-version support

6) Packaging include policy (optional in this PR; recommended)

Add an explicit [package] include = [...] list to ensure WIT and build assets needed by bindgen are packaged.

(If you already have a packaging check and it passes, keep minimal change here.)

Files likely touched

Cargo.toml

src/lib.rs

src/abi/mod.rs (new)

src/mappers.rs

src/validate.rs

any other file referencing bindings::greentic::...

ci/steps/external_consumer_check.sh (new)

.github/workflows/ci.yml (wire in)

docs/vision/* (new or updated)

README link to vision docs

Acceptance criteria

 cargo test passes in greentic-interfaces

 cargo package --no-verify produces a crate that a fresh consumer can compile

 The consumer check runs in CI and passes

 Internal code references only crate::canonical::* (or crate::v0_6_0::*), never bindings::greentic::*

 Docs explain canonical facade + multi-version plan clearly

Future extension (explicitly supported by this PR)

To add v1_2_3 later:

add feature = "wit-v1_2_3"

generate bindings for that WIT version

add pub mod v1_2_3 { … } in src/abi/mod.rs

(optionally) switch canonical re-export to v1_2_3 when ready

No other code changes required.

Codex prompt (greentic-interfaces)

Implement PR-IF-01 exactly.

First: locate the generated module name for component v0.6.0 bindings.

Create src/abi/mod.rs with v0_6_0 + canonical.

Replace all bindings::greentic::... references to use crate::canonical.

Add CI external-consumer package check script and wire it in.

Decisions / Clarifications (authoritative)

1) Scope of “no direct bindings” rule

- Apply this rule to production code + tests + README/examples.
- Anything meant to be copied or compiled must use canonical paths.
- Production `.rs`: use `crate::canonical::*` (or `crate::abi::v0_6_0::*` only in rare internal cases).
- Tests: use `crate::canonical::*` as well.
- README/examples: use canonical imports/paths.
- Exception: only `src/abi/mod.rs` and generated bindgen glue may reference `crate::bindings::*` directly.

2) Canonical facade exports for this PR

- Expose only the v0.6 canonical surface needed now:
- `types` (interfaces-types/types-core)
- `node` (v0.6 component node interface / related record types)
- descriptor-related contracts already used by helpers (e.g. schema-source, io-schema, setup-contract)
- Do not over-export domain/host/provider-specific surfaces in this PR.
- Specifically avoid adding provider packs, operator/runner specifics, and host/provider interfaces unless strictly required by `greentic-interfaces` itself.

3) Feature policy

- Keep existing default feature matrix; do not simplify/reduce defaults in this PR.
- Add `wit-v0_6_0` as a feature.
- Include it in `default` only if this is behavior-preserving.
- Primary requirement: a fresh external consumer using default features compiles successfully.

4) External-consumer CI scope

- Scope this PR to `greentic-interfaces` only.
- Do not add the same check for `greentic-interfaces-guest` in PR-IF-01.
- Follow-up PR can extend the pattern to guest crate.

Implementation nudges

- Canonical module layout:
- `src/abi/mod.rs` defines `v0_6_0` re-exports from generated bindings and `canonical` as `pub use super::v0_6_0::*`.
- `src/lib.rs` exports `pub mod abi; pub use abi::canonical;`.
- Refactor all `bindings::greentic::interfaces_types::types::X` uses to `crate::canonical::types::X`.
- Refactor enum matches similarly, e.g. `Protocol::Custom(...)` through canonical path.
- Update README/example snippets to canonical paths.
- Minimum CI external consumer check:
- package `greentic-interfaces`
- unpack crate
- create temp consumer crate
- depend on unpacked crate by path
- run `cargo check` with default features

Add/update docs/vision.
