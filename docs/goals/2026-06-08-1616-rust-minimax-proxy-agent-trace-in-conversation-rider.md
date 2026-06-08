# Rust MiniMax Proxy — Agent Trace In Conversation Rider

## CopilotKit pattern

Controlled CopilotKit-wrapped chat plus browser-local LangGraph/sub-agent trace UI. This keeps the app small and makes agent activity visible without adding a backend CopilotKit runtime.

## Agent/runtime contract

- `app/src/agentGraph.js` emits ordered progress events for PRD Agent then Planning Agent.
- Each event includes name, role, tools, status, and output.
- Browser test mocks `/v1/chat/completions` so it can verify UI deterministically.

## UI contract

- Agent trace appears inside `data-testid="message-list"`, not below/outside the conversation.
- `data-testid="agent-card-prd"` shows `PRD Agent`, `MiniMax proxy chat completion`, and PRD output.
- `data-testid="agent-card-plan"` shows `Planning Agent`, `MiniMax proxy chat completion`, and plan output.

## Visual verification

Create `.test/playwright/tests/agent-trace-conversation.spec.mjs` and run it against Vite with mocked network. Capture screenshots and analysis JSON under `.test/playwright/`.

## Phases

### P1 — Failing browser-visible test
- Write Playwright test that expects agent cards inside message list.

### P2 — UI fix
- Move trace rendering into conversation message list and keep ordered agent card content.

### P3 — Verify
- Run JS unit tests, build, browser test, cargo check/test.

### P4 — Closure
- Update architecture decisions.

## Out of scope

- Live MiniMax smoke.
- Backend CopilotKit runtime.
- Token streaming.
