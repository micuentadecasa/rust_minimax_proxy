GOAL: Put the PRD and Planning agent identity/tool trace inside the visible chat conversation and verify it in a browser. The current agent metadata can render outside the conversation or be missed by users; this round makes each agent appear as an ordered conversation item with name, tools, status, and output.

**Read first.**
- `/Users/luis/projects/rust_minimax_proxy/architecture_decisions.md` — current map and prior decisions.
- `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-08-1616-rust-minimax-proxy-agent-trace-in-conversation-rider.md` — implementation phases and tests.
- `/Users/luis/projects/rust_minimax_proxy/app/src/main.jsx` — current UI.
- `/Users/luis/projects/rust_minimax_proxy/app/src/agentGraph.js` — graph metadata/progress events.
- `/Users/luis/projects/rust_minimax_proxy/.pi/skills/copilotkit_helper/SKILL.md` — CopilotKit agentic UI discipline.

**Posture.** Keep the browser-local LangGraph flow. No backend runtime expansion. Use a mocked browser test so verification does not depend on live MiniMax credentials.

**Verification.**
- `cd app && npm test`
- `cd app && npm run build`
- Browser verification under `.test/playwright/` with route-mocked proxy responses and screenshots.
- `cargo check`
- `cargo test`

**Stop when** the browser test proves the visible conversation contains PRD Agent and Planning Agent cards with tools and outputs.
