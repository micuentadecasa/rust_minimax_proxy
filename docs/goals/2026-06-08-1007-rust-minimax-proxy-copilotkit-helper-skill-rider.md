# Rust MiniMax Proxy — CopilotKit Helper Skill Rider

This rider implements the goal at `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-08-1007-rust-minimax-proxy-copilotkit-helper-skill-goal.md`. It creates a CopilotKit helper skill from the local reference docs and composes it with the MiniMax goal-engineering skill.

## Posture

- Documentation/skill work only.
- No app/proxy runtime changes.
- Keep `.pi/skills/copilotkit_helper/*.md` as reference docs; add `SKILL.md` in that same folder.
- No `git push`.

## Touch points

- `.pi/skills/copilotkit_helper/SKILL.md` — new skill.
- `.pi/skills/minimax-goal-engineering/SKILL.md` — integration instructions.
- `architecture_decisions.md` — architecture memory update.
- `.test/` — structural validation script.

## Phases

### P1 — Validation script first
- Create `.test/validate_copilotkit_skill.sh` before implementation.
- It should fail until `SKILL.md` and minimax-skill integration exist.
Depth tests:
- `copilotkit_helper_skill_structural_validation`

### P2 — CopilotKit helper skill
- Create `.pi/skills/copilotkit_helper/SKILL.md`.
- Include valid frontmatter.
- Tell future agents how to select CopilotKit primitives: chat/sidebar, agent config, shared state, components-as-tools, tool rendering, state rendering, reasoning, fixed/dynamic A2UI, HITL, sub-agents, LangGraph, attachments.
- Tell future agents to read relevant reference docs before designing/implementing.
Depth tests:
- `copilotkit_helper_skill_structural_validation`

### P3 — Compose with goal-engineering skill
- Update `.pi/skills/minimax-goal-engineering/SKILL.md` so new agentic/CopilotKit feature requests load the helper skill and cite it in goal/rider files.
Depth tests:
- `minimax_skill_references_copilotkit_helper`

### P4 — Architecture closure
- Update `architecture_decisions.md` with the skill composition and intended use.
Depth tests:
- `architecture_mentions_copilotkit_helper_skill`

### P5 — Verification
- Run `.test/validate_copilotkit_skill.sh`, `cargo check`, and `cargo test`.
Depth tests:
- `verification_green`

## Verification

```bash
bash .test/validate_copilotkit_skill.sh
cargo check
cargo test
```

## Out of scope

- Implementing a new CopilotKit runtime feature.
- Changing the current React app.
- Installing new dependencies.

## Done criteria

- Helper skill exists and validates.
- MiniMax goal-engineering skill references helper skill for agentic work.
- Architecture decisions updated.
- Verification green.
