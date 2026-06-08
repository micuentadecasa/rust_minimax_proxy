# Rust MiniMax Proxy — Agent Trace UI Rider

Implements `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-08-1607-rust-minimax-proxy-agent-trace-ui-goal.md`.

## CopilotKit pattern

Existing CopilotKit-wrapped chat + LangGraph/sub-agent orchestration. Add controlled trace UI rather than dynamic generative UI.

## Agent/runtime contract

- Agent metadata is explicit in frontend code.
- PRD Agent uses tool `MiniMax proxy chat completion` with PRD prompt.
- Planning Agent uses tool `MiniMax proxy chat completion` with PRD-to-plan prompt.
- UI receives progress callbacks: running PRD, completed PRD, running plan, completed plan.

## UI contract

- `data-testid="agent-trace"` wraps ordered agent cards.
- `data-testid="agent-card-prd"` shows PRD Agent name, tools, status, and PRD output.
- `data-testid="agent-card-plan"` shows Planning Agent name, tools, status, and plan output.
- Planning Agent should not show as running until PRD output exists.

## Visual verification

Generated Playwright can assert ordered cards and screenshots under `.test/playwright/`; live run is optional if OAuth/API is available.

## Phases

### P1 — Failing test
- Extend `app/src/agentGraph.test.mjs` to assert progress events and metadata order.
Depth tests:
- `agentGraph emits ordered agent progress events`

### P2 — Graph metadata/events
- Export agent metadata and emit progress after each agent completes.
Depth tests:
- `cd app && npm test`

### P3 — UI trace cards
- Replace generic artifact grid with ordered agent cards showing name, role, tools, status, and output.
Depth tests:
- `cd app && npm run build`

### P4 — Closure
- Update architecture decisions.

## Out of scope

- Backend CopilotKit runtime.
- Real tool-call protocol rendering.
- Streaming tokens.
