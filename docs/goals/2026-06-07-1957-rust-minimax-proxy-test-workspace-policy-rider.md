# Rust MiniMax Proxy — Test Workspace Policy Rider

This rider implements the goal at `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-07-1957-rust-minimax-proxy-test-workspace-policy-goal.md`. It updates the local skill so generated feature tests/scripts/artifacts use a disposable ignored `.test/` workspace and UI work uses screenshot-based Playwright verification with slow-LLM polling and duplicate-image discard.

## Posture

- Methodology/documentation only; no proxy behavior changes.
- `.test/` is ignored and disposable.
- Keep durable project tests in normal test locations only when they become part of the maintained suite; exploratory/generated round scripts start in `.test/`.
- No `git push`.

## Touch points

- `.gitignore` — add `.test/`.
- `.pi/skills/minimax-goal-engineering/SKILL.md` — encode cleanup, workspace, and screenshot policy.
- `architecture_decisions.md` — document the test workspace and UI verification policy.
- `docs/goals/` — this pair.

## Phases

### P1 — Ignore disposable test workspace
- Add `.test/` to `.gitignore`.
Depth tests:
- `gitignore_contains_test_workspace`

### P2 — Skill startup cleanup policy
- Skill must instruct agents to run `rm -rf .test && mkdir -p .test` at round start after pre-work and before writing generated scripts/artifacts.
Depth tests:
- `skill_removes_previous_test_scripts_at_start`

### P3 — Skill generated test placement policy
- Skill must require generated feature scripts, smoke clients, temporary fixtures, screenshots, traces, and reports to live under `.test/`.
- Durable source tests may still live in `src`/`tests` when they are intended to remain maintained.
Depth tests:
- `skill_routes_generated_artifacts_to_test_workspace`

### P4 — Playwright screenshot analysis policy
- Skill must require Playwright UI tests to record screenshots under `.test/playwright/screenshots/`.
- Tests for slow LLM calls must take multiple screenshots over time until a done condition or timeout.
- Duplicate/effectively-identical waiting screenshots must be discarded or skipped in the final analysis.
- Final UI assertion must analyze the kept screenshots for the expected visible feature state.
Depth tests:
- `skill_requires_playwright_screenshot_poll_dedupe_analysis`

### P5 — Architecture closure and verification
- Update `architecture_decisions.md` with the `.test/` and Playwright screenshot policy.
- Run structural checks plus `cargo check` and `cargo test`.
Depth tests:
- `architecture_documents_test_workspace_policy`

## Verification

```bash
grep -q '^.test/$' .gitignore
grep -q 'rm -rf .test' .pi/skills/minimax-goal-engineering/SKILL.md
grep -q '.test/playwright/screenshots' .pi/skills/minimax-goal-engineering/SKILL.md
grep -q 'duplicate' .pi/skills/minimax-goal-engineering/SKILL.md
grep -q '.test/' architecture_decisions.md
cargo check
cargo test
```

## Out of scope

- Installing Playwright now; there is no UI yet.
- Replacing Cargo tests with `.test/` scripts.
- Committing generated `.test/` artifacts.

## Done criteria

- Ignore policy updated.
- Skill startup cleanup and generated artifact policy updated.
- Playwright screenshot polling/deduplication/analysis policy updated.
- Architecture decisions updated.
- Verification passes.
