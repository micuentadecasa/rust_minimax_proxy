# Rust MiniMax Proxy — Default Agent Graph Rider

## CopilotKit pattern

Controlled CopilotKit-wrapped chat with browser-local LangGraph/sub-agent workflow. No backend runtime expansion.

## Agent/runtime contract

- Default UI mode is `prd-plan`.
- Submit button and loading message explicitly say agent graph, not assistant chat.
- Normal chat remains selectable.

## UI contract

- On first page load, `data-testid="mode-select"` value is `prd-plan`.
- Without changing the selector, submitting a request creates `agent-card-prd` and `agent-card-plan` inside `message-list`.

## Phases

### P1 — Failing browser test
- Update Playwright to not select graph mode manually; expect default mode to be graph.

### P2 — UI fix
- Default mode to graph and update copy/button/loading labels.

### P3 — Verify and close
- Run JS/Rust/build/browser verification and update architecture decisions.

## Out of scope

- Removing normal chat mode.
- Live MiniMax screenshot smoke.
