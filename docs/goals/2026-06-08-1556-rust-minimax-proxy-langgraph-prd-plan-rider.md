# Rust MiniMax Proxy — LangGraph PRD/Plan Rider

This rider implements the goal at `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-08-1556-rust-minimax-proxy-langgraph-prd-plan-goal.md`. It adds a two-agent PRD-to-plan workflow to the React app while preserving the Rust proxy as the OpenAI-compatible MiniMax boundary.

## Posture

- Keep the proxy OpenAI-compatible at the edge.
- Keep the new agent orchestration small and browser-local for this round.
- No `git push`.
- No unrelated Rust refactors.
- If a true backend CopilotKit runtime is needed later, record it as future work.

## Touch points

- `app/src/agentGraph.js` — new LangGraph-style two-node workflow and MiniMax proxy call helpers.
- `app/src/main.jsx` — chat UI mode that runs the PRD agent followed by the planning agent.
- `app/src/styles.css` — visible two-agent artifact cards.
- `app/package.json` / `app/package-lock.json` — test script and any LangGraph dependency.
- `app/src/agentGraph.test.mjs` — durable unit test with mocked proxy calls.
- `.test/playwright/` — generated screenshot verification when run.
- `architecture_decisions.md` — closure notes.

## Test contract

Each non-doc phase names a test and writes it before implementation. Generated one-off scripts and screenshots belong under `.test/`.

## CopilotKit pattern

Chosen primitive: existing CopilotKit-wrapped chat surface plus LangGraph/sub-agent orchestration. This is the smallest slice because the app already wraps `ChatApp` in `CopilotKit` and sends direct OpenAI-compatible requests to the Rust proxy. The new feature adds a two-agent graph behind the existing chat rather than introducing a CopilotKit runtime server in this round.

## Agent/runtime contract

- Runtime endpoint: browser `fetch` to `POST ${VITE_PROXY_BASE_URL}/v1/chat/completions`.
- Model: `VITE_MINIMAX_MODEL` or `MiniMax-M2.7`.
- Agent 1 (`prdAgent`): input is the latest user request; output is markdown PRD with problem, users, goals, requirements, constraints, and success metrics.
- Agent 2 (`planningAgent`): input is the user request plus PRD; output is markdown implementation plan with phases, files, tests, risks, and done criteria.
- State keys: `userInput` (UI-owned), `prd` (agent-owned), `plan` (agent-owned), `messages`/`steps` (UI-owned render state).
- Refusal/error behavior: if either proxy call fails or returns no text, the UI shows an error assistant message and does not silently fake the artifact.

## UI contract

- Mode toggle/select with `data-testid="mode-select"`: normal chat vs PRD → plan graph.
- Submit area remains the existing textarea/button.
- Loading state shows `data-testid="agent-step-status"` with the current agent step.
- Final graph output renders deterministic cards:
  - `data-testid="prd-output"`
  - `data-testid="plan-output"`
- Error state remains `role="alert"` and status text says `Proxy error`.

## Visual verification

Generated Playwright test path: `.test/playwright/tests/langgraph-prd-plan-ui.spec.mjs`.

The script should:
1. Open the app.
2. Select PRD → plan mode.
3. Submit a feature request.
4. Poll screenshots every 2 seconds up to 90 seconds.
5. Hash screenshots and skip duplicate waiting states.
6. Assert that PRD and implementation plan cards are visible in the final screenshot/DOM.
7. Write `.test/playwright/screenshot-analysis.json`.

## Phases

### P1 — Baseline and failing test
- Add `app/src/agentGraph.test.mjs` that mocks two proxy responses and asserts two ordered calls plus returned `prd`/`plan`.
- Run `cd app && npm test` and observe failure before implementation when feasible.
Depth tests:
- `agentGraph runs the PRD agent before the planning agent`

### P2 — Implement graph helper
- Add `app/src/agentGraph.js` with a small two-node graph and robust OpenAI-compatible response parsing.
- Run `cd app && npm test`.
Depth tests:
- `agentGraph runs the PRD agent before the planning agent`

### P3 — Wire UI
- Update `app/src/main.jsx` so graph mode calls the helper and renders PRD/plan cards.
- Preserve normal chat behavior.
Depth tests:
- `cd app && npm run build`

### P4 — Edge cases and errors
- Extend the unit test for failed/malformed proxy responses.
Depth tests:
- `agentGraph surfaces proxy failures`

### P5 — UI/Playwright
- Add generated Playwright screenshot script under `.test/playwright/`.
- Run it only if local proxy/app and credentials are practical; otherwise document skipped reason.
Depth tests:
- `.test/playwright/tests/langgraph-prd-plan-ui.spec.mjs`

### P6 — Documentation and runner updates
- Update README if user-facing app behavior changed.
Depth tests:
- README mentions PRD → plan graph mode.

### P7 — Architecture closure
- Update `architecture_decisions.md` with changed files, verification commands, and known gaps.
- No new feature code.

## Verification

Run before declaring done:

```bash
cargo check
cargo test
cd app && npm test
cd app && npm run build
```

Network/API smoke:

```bash
./first.sh
```

Run only when credentials/network cost permit; otherwise report not run.

## Out of scope

- A dedicated CopilotKit runtime backend endpoint.
- Streaming graph state from the Rust proxy.
- Tool calls, HITL approval, persistence, or multi-user sessions.
- Replacing the Rust proxy with a JavaScript backend.

## Done criteria

- Goal file exists and is <= 4000 chars.
- Rider exists and names the CopilotKit pattern and tests.
- Durable JS tests cover graph orchestration success and failure.
- Build/check/test commands pass or blocked commands are explicitly named.
- `architecture_decisions.md` is updated.
