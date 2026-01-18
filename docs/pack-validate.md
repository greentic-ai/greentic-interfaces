# Pack validate WIT

The `greentic:pack-validate@0.1.0` package defines a minimal, pure validator ABI so pack validation logic can ship as a WASM component and be executed by tools like `greentic-pack doctor` without hardcoded domain rules.

## Purpose

- Provide a stable WIT contract for pack validators.
- Allow multiple validators to opt in via `applies()` based on pack inputs.
- Emit structured diagnostics that map 1:1 with `greentic-types` diagnostics.

## Security expectations

Validators are expected to be pure functions of their inputs. The host should run them with no network access and no filesystem access by default. Any host-provided capability beyond the supplied inputs should be treated as a security exception.

## Compatibility guarantees (`@0.1.0`)

- Fields and record names are stable and will not be renamed within the `0.1.x` line.
- Additive changes (new optional fields, new interfaces/worlds) may land in future minor versions.
- Breaking changes will ship under a new package version (e.g., `@0.2.0` or `@1.0.0`).
