GOAL: Replace the fixed PRD-then-plan graph UX with a coordinator-routed multi-agent chat. Depending on the user's request, a Coordinator Agent decides whether to call the PRD Agent or Planning Agent, then the selected agent answers in a vertical chat-style conversation.

**Read first.**
- `/Users/luis/projects/rust_minimax_proxy/architecture_decisions.md` — current map and prior decisions.
- `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-09-0854-rust-minimax-proxy-coordinator-agent-graph-rider.md` — implementation phases and tests.
- `/Users/luis/projects/rust_minimax_proxy/app/src/agentGraph.js` — current two-agent graph and metadata.
- `/Users/luis/projects/rust_minimax_proxy/app/src/main.jsx` — current UI and message rendering.
- `/Users/luis/projects/rust_minimax_proxy/.pi/skills/copilotkit_helper/SKILL.md` — CopilotKit lessons for agentic UI defaults and browser verification.

**Posture.** Keep this browser-local LangGraph orchestration backed by the Rust OpenAI-compatible proxy. Keep normal chat available only if already present, but make agent routing the clear default. No backend runtime expansion. Tests first.

**Verification.**
- `cd app && npm test`
- `cd app && npm run build`
- `cd app && npx playwright test --config=../.test/playwright/playwright.config.mjs`
- `cargo check`
- `cargo test`

**Stop when** default submit shows a vertical conversation with Coordinator Agent followed by either PRD Agent or Planning Agent, and tests prove both routing paths.
