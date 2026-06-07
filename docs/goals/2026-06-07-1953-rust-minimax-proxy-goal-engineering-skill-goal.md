GOAL: Add a project skill that makes future MiniMax proxy features follow goal+rider planning, test-first implementation, and architecture closure. The proxy now works, but future changes need a repeatable method so an agent knows where to look, what to test, when to update docs, and how to decide that a feature is done.

**Read first.**
- `/Users/luis/projects/rust_minimax_proxy/architecture_decisions.md` — current map and prior decisions.
- `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-07-1953-rust-minimax-proxy-goal-engineering-skill-rider.md` — implementation phases and done criteria.
- `/Users/luis/projects/rust_minimax_proxy/src/main.rs` — current proxy implementation.
- `/Users/luis/projects/rust_minimax_proxy/test_client.py` and `/Users/luis/projects/rust_minimax_proxy/first.sh` — smoke path.

**Posture.** Keep this a small Rust proxy. No `git push`. Edits stay inside the project. The skill should be project-specific and load automatically for future feature/bug requests.

**Phases.** The rider defines the documentation and validation work for this small round. Future feature rounds should use named tests first, then implementation, then architecture closure.

**Verification.**
- Skill exists at `.pi/skills/minimax-goal-engineering/SKILL.md` with valid frontmatter.
- Goal is under 4000 chars.
- `architecture_decisions.md` records the skill, current architecture, test path, and known gaps.
- `cargo check` and `cargo test` stay green.

**Stop when** the skill, this goal/rider pair, and `architecture_decisions.md` are in place and verification passes.
