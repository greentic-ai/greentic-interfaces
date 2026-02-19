# Events WIT packages

> Legacy note: this page documents removed/legacy provider surfaces.
> Canonical provider direction is `greentic:provider-schema-core@1.0.0`; see `docs/vision/v0_6.md` and `docs/vision/legacy.md`.

The legacy `greentic:events@1.0.0` broker/source/sink worlds and the `greentic:events-bridge@1.0.0` message↔event bridge have been removed. Provider components must now expose `greentic:provider-schema-core@1.0.0` schemas instead of typed events provider worlds. Hosts should drop any remaining dependencies on the legacy events packages and rely on provider-core plus JSON schemas for event-centric providers.
