# Codex Guardrails

## Binding Path Rule

- Do not reference `crate::bindings::*` directly in normal code.
- Use `crate::canonical::*` for default ABI callsites.
- Use `crate::v0_6_0::*` only when explicitly version-pinning behavior.

## Allowed Exceptions

- `src/abi/mod.rs` (facade wiring)
- generated bindgen glue

## When Adding a New ABI Version

1. Add the versioned facade in `src/abi/mod.rs`.
2. Decide whether `canonical` should stay on current version or move.
3. Update `docs/vision/v0_6.md` (or the new version vision doc).
4. Keep `ci/steps/external_consumer_check.sh` passing for default features.
