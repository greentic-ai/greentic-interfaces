# Deprecation Strategy

## Policy

- Do not remove legacy ABI surfaces abruptly.
- Add new versioned modules first (for example `v1_2_3`), then migrate callsites.
- Keep `canonical` stable until the new default ABI is intentionally promoted.

## Promotion Flow

1. Add the new feature gate and versioned facade module.
2. Keep existing `canonical` target unchanged while adoption happens.
3. Announce deprecation of old default in docs and release notes.
4. In a later release, switch `canonical` to the new versioned facade.

## Compatibility Checks

- `cargo test --workspace --all-features`
- `ci/steps/external_consumer_check.sh` for packaged dependency usage
- WIT validation scripts in CI
