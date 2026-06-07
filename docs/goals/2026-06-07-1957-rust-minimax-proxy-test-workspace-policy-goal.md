GOAL: Update the project goal-engineering skill so every future feature round uses `.test/` as a disposable test workspace and uses screenshot-based Playwright verification for UI work. Future agents need a clean place for generated scripts/artifacts, and UI features need visual evidence that accounts for slow LLM/API calls without storing repeated waiting screenshots.

**Read first.**
- `/Users/luis/projects/rust_minimax_proxy/architecture_decisions.md` — current testing and skill decisions.
- `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-07-1957-rust-minimax-proxy-test-workspace-policy-rider.md` — exact policy changes.
- `/Users/luis/projects/rust_minimax_proxy/.pi/skills/minimax-goal-engineering/SKILL.md` — skill to update.
- `/Users/luis/projects/rust_minimax_proxy/.gitignore` — ignore policy.

**Posture.** This is methodology/documentation work, not proxy runtime code. No `git push`. Edits stay inside the project. `.test/` is intentionally disposable and ignored.

**Phases.** Update ignore policy, skill instructions, and architecture decisions. Validate structure and keep Rust checks green.

**Verification.**
- `.gitignore` contains `.test/`.
- Skill says to remove and recreate `.test/` at round start.
- Skill says generated tests/scripts/artifacts live under `.test/`.
- Skill describes Playwright screenshot capture, repeated polling for slow LLM calls, duplicate-image discard, and screenshot analysis.
- `cargo check` and `cargo test` stay green.

**Stop when** the policy is encoded in the skill, architecture decisions are updated, this goal/rider exists, and verification passes.
