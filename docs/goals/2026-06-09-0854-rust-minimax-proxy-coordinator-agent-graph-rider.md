# Rust MiniMax Proxy — Coordinator Agent Graph Rider

## CopilotKit pattern

Controlled CopilotKit-wrapped chat with browser-local LangGraph multi-agent routing. The UI must default to the agentic path and render agent identity/tool/status/output in the conversation.

## Agent/runtime contract

- Coordinator Agent inspects user input and routes to one specialist:
  - PRD Agent for requests asking for PRD, requirements, product spec, user stories, success metrics, or product definition.
  - Planning Agent for requests asking for plan, implementation, phases, tasks, tests, roadmap, or engineering steps.
- Coordinator emits a visible trace message with the selected route and reason.
- Selected specialist calls `${VITE_PROXY_BASE_URL}/v1/chat/completions` and returns the answer.
- For plan requests without a provided PRD, the Planning Agent should still create a plan from the user's request and note assumptions.

## UI contract

- Default mode is coordinator graph.
- Conversation is vertical: user message, Coordinator Agent card, selected specialist card.
- No side-by-side grid for agent cards.
- Deterministic selectors:
  - `agent-card-coordinator`
  - `agent-card-prd`
  - `agent-card-plan`
  - cards live inside `message-list`.

## Visual verification

Use mocked Playwright tests under `.test/playwright/` for both PRD and planning routes, screenshots under `.test/playwright/screenshots/`, and analysis JSON.

## Phases

### P1 — Failing graph tests
- Extend JS tests to prove coordinator routes PRD requests to PRD Agent and plan requests to Planning Agent.

### P2 — Implement coordinator graph
- Add Coordinator Agent metadata, route detection, conditional graph edges, and progress events.

### P3 — Vertical UI
- Render coordinator/specialist cards as vertical conversation messages.
- Update labels from fixed PRD→plan to coordinator graph.

### P4 — Browser tests
- Write/run Playwright mocked tests for both routing paths and vertical DOM placement.

### P5 — Closure
- Update architecture decisions.

## Out of scope

- LLM-based coordinator classification; use deterministic routing for this round.
- Calling both PRD and Planning Agent in one request unless future UI asks for it.
- Backend CopilotKit runtime.
