GOAL: Make the PRD → plan agent graph the obvious default launch path so users do not accidentally chat with the normal assistant. The current UI can remain in normal chat mode, causing user prompts to go to the assistant instead of the two-agent graph; this round defaults to agent graph mode and makes button/loading copy explicit.

**Read first.**
- `/Users/luis/projects/rust_minimax_proxy/architecture_decisions.md` — current map and prior decisions.
- `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-08-1623-rust-minimax-proxy-default-agent-graph-rider.md` — phases and tests.
- `/Users/luis/projects/rust_minimax_proxy/app/src/main.jsx` — mode and submit behavior.
- `/Users/luis/projects/rust_minimax_proxy/app/src/agentGraph.js` — graph behavior.

**Posture.** Keep the normal chat mode available but make PRD → plan LangGraph default. Use mocked browser verification so no live MiniMax call is needed.

**Verification.**
- `cd app && npm test`
- `cd app && npm run build`
- `cd app && npx playwright test --config=../.test/playwright/playwright.config.mjs`
- `cargo check`
- `cargo test`

**Stop when** the default browser path launches the agent graph without manually changing the mode selector and the visible conversation contains both agent cards.
