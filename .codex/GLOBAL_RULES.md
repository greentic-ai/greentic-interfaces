# GLOBAL RULES (Unified 0.6.0 Reset)

For every PR:
- PRE-PR: refresh `.codex/repo_overview.md` using `.codex/repo_overview_task.md`
- POST-PR: refresh `.codex/repo_overview.md` again and run `ci/local_check.sh`
- Update `.codex/STATE.json` at start, during checkpoints, and at completion.

0.6 Program invariants:
- One canonical contract tree for WIT+schemas (no copies).
- CBOR-first protocol, JSON only as authoring convenience converted to CBOR.
- Self-describing components with describe()+capabilities+schemas.
- Strict versioning: any shape change => new version.
- Deterministic builds and reproducible artifacts.
