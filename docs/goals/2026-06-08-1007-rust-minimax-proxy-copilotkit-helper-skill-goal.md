GOAL: Add a CopilotKit helper skill and wire it into the MiniMax goal-engineering workflow for future agentic solutions. The repo now has CopilotKit reference docs under `.pi/skills/copilotkit_helper/`; future features that ask for agentic UI/agent behavior should use those docs together with the goal+rider process.

**Read first.**
- `/Users/luis/projects/rust_minimax_proxy/architecture_decisions.md` — current skill and base-solution decisions.
- `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-08-1007-rust-minimax-proxy-copilotkit-helper-skill-rider.md` — exact phases and validation.
- `/Users/luis/projects/rust_minimax_proxy/.pi/skills/minimax-goal-engineering/SKILL.md` — skill to integrate.
- `/Users/luis/projects/rust_minimax_proxy/.pi/skills/copilotkit_helper/*.md` — CopilotKit reference corpus.

**Posture.** Skill/documentation work only. No proxy/runtime behavior changes. No `git push`. Generated validation scripts go in `.test/`.

**Feature.** Create `.pi/skills/copilotkit_helper/SKILL.md` as a project skill for building agentic solutions with CopilotKit UI patterns. Update minimax-goal-engineering so agentic/CopilotKit feature requests explicitly load and apply that helper skill while still creating goals, riders, tests, Playwright screenshot analysis, and architecture updates.

**Verification.**
- CopilotKit helper skill has valid frontmatter.
- Helper skill points to the reference docs and explains pattern selection.
- MiniMax goal-engineering skill references the helper skill for CopilotKit/agentic solutions.
- `architecture_decisions.md` records the skill composition.
- `cargo check` and `cargo test` remain green.

**Stop when** both skills compose cleanly and validation passes.
