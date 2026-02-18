# Codex PR Focus Mechanism (Unified 0.6.0 Reset)

This bundle defines a **program PR queue** under `.codex/PR-XX.md` and a shared tracker `.codex/STATE.json`.

## Operating mode (MANDATORY)
Codex must always:
1. Work on **exactly one** PR at a time.
2. Update `.codex/STATE.json` to:
   - set `active_pr` = "PR-XX"
   - set PR status = "in_progress"
   - record timestamps + checkpoints.
3. Finish the PR fully, then mark it **done** and set `active_pr` to null.

## Completion signal (MANDATORY)
At the end of work, Codex must output this line:

**DONE: PR-XX**

…and must ensure acceptance criteria are met and recorded in STATE.json.

## Checkpoints
Each PR has checkpoints like:
- repo_overview_pre
- implementation
- tests
- ci_local_check
- repo_overview_post
Codex updates checkpoint statuses as it progresses.

