GOAL: Make the PRD → plan workflow visibly agentic by showing each agent name, role, tools, status, and output in order. The current graph returns code/artifacts without enough runtime trace; this round adds deterministic UI trace cards for the PRD agent first, then the planning agent after the PRD finishes.

**Read first.**
- `/Users/luis/projects/rust_minimax_proxy/architecture_decisions.md` — current map and prior decisions.
- `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-08-1607-rust-minimax-proxy-agent-trace-ui-rider.md` — implementation phases and tests.
- `/Users/luis/projects/rust_minimax_proxy/app/src/agentGraph.js` — current two-agent graph.
- `/Users/luis/projects/rust_minimax_proxy/app/src/main.jsx` — current UI.
- `/Users/luis/projects/rust_minimax_proxy/.pi/skills/copilotkit_helper/SKILL.md` — CopilotKit agentic UI discipline.

**Posture.** Keep the existing browser-local LangGraph orchestration and Rust proxy boundary. No backend runtime expansion. No `git push`. Tests before implementation.

**Verification.**
- `cd app && npm test`
- `cd app && npm run build`
- `cargo check`
- `cargo test`

**Stop when** the UI clearly shows PRD Agent with tools/status/output before Planning Agent with tools/status/output, tests pass, and `architecture_decisions.md` records the trace UI.
