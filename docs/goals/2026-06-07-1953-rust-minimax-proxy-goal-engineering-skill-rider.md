# Rust MiniMax Proxy — Goal Engineering Skill Rider

This rider implements the goal at `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-07-1953-rust-minimax-proxy-goal-engineering-skill-goal.md`. It adds a project-local Pi skill for future feature work and creates/updates the architecture decision map that future riders must read.

## Posture

- Keep the proxy's implementation unchanged unless validation reveals breakage.
- Add methodology as project documentation/skill, not runtime code.
- No `git push`.
- No unrelated refactors.
- The skill must require tests before feature implementation and require `architecture_decisions.md` closure before done.

## Touch points

- `.pi/skills/minimax-goal-engineering/SKILL.md` — new project skill.
- `architecture_decisions.md` — living architecture map and closure record.
- `docs/goals/` — first local goal/rider pair documenting this methodology feature.

## Test contract

This round is documentation/skill work, so the depth test is structural validation plus preserving Rust build/test health. Future feature rounds must add code tests first.

## Phases

### P1 — Skill package
- Create `.pi/skills/minimax-goal-engineering/SKILL.md`.
- Include valid skill frontmatter: lowercase hyphenated name and specific description.
- Instruct future agents to create goal+rider files before implementing features.
Depth tests:
- `skill_file_exists_with_valid_frontmatter`

### P2 — Project architecture memory
- Create/update `architecture_decisions.md`.
- Record current runtime endpoints, OAuth decisions, credential storage, chat proxy conversion, smoke script, and known gaps.
Depth tests:
- `architecture_decisions_contains_current_proxy_map`

### P3 — Future feature done criteria
- Ensure the skill requires at least one automated test or smoke assertion for each feature.
- Ensure the skill calls out Rust tests first, Python HTTP smoke second, and Playwright only when a UI exists.
Depth tests:
- `skill_requires_test_first_and_playwright_when_ui_exists`

### P4 — Goal/rider corpus seed
- Add this goal/rider pair under `docs/goals/` so future rounds have a local exemplar.
Depth tests:
- `goal_rider_pair_exists_and_cross_references`

### P5 — Verification
- Run goal size check.
- Run `cargo check` and `cargo test` to prove no runtime regression.
Depth tests:
- `cargo_check_and_test_green`

### P6 — Closure
- Final response lists changed files and verification results.
- No code behavior is declared changed.

## Verification

```bash
wc -c docs/goals/2026-06-07-1953-rust-minimax-proxy-goal-engineering-skill-goal.md
cargo check
cargo test
```

Optional structural checks:

```bash
test -f .pi/skills/minimax-goal-engineering/SKILL.md
grep -q '^name: minimax-goal-engineering' .pi/skills/minimax-goal-engineering/SKILL.md
grep -q 'architecture_decisions.md' .pi/skills/minimax-goal-engineering/SKILL.md
grep -q 'Playwright' .pi/skills/minimax-goal-engineering/SKILL.md
```

## Out of scope

- Implementing a new proxy runtime feature.
- Adding Playwright now; there is no UI yet.
- Creating a large test harness for existing code in this round.

## Done criteria

- Skill exists and is project-specific.
- `architecture_decisions.md` exists and records current decisions.
- Goal/rider corpus is seeded.
- Validation commands pass.
