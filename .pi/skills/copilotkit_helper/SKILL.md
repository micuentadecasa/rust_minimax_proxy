---
name: copilotkit-agentic-solutions
description: Use when designing or implementing CopilotKit/CopilotKit UI agentic solutions, including chat/sidebar experiences, shared state, agent config, generative UI, A2UI, HITL, sub-agents, LangGraph-backed agents, and Playwright screenshot verification. Use with minimax-goal-engineering for new agentic features in this repo.
license: Apache-2.0
---

# CopilotKit Agentic Solutions Helper

This skill turns the local CopilotKit reference docs in this folder into a working implementation workflow for agentic UI solutions.

Use it together with `.pi/skills/minimax-goal-engineering/SKILL.md` when the user asks for:

- a new agentic solution
- CopilotKit UI work
- a CopilotKit chat/sidebar app
- an agent that uses shared UI state or user-editable config
- generative UI, A2UI, HITL, sub-agents, or LangGraph
- a browser feature that needs visual/screenshot verification

## Required composition with goal engineering

When this skill is used inside this repository, also follow the MiniMax goal-engineering skill:

1. Read `architecture_decisions.md` first.
2. Read this `SKILL.md` and the relevant CopilotKit reference docs listed below.
3. Create a goal+rider pair under `docs/goals/`.
4. Clean generated artifacts with `rm -rf .test && mkdir -p .test`.
5. Write tests before implementation.
6. For UI work, create Playwright screenshot tests under `.test/playwright/`.
7. Update `architecture_decisions.md` before declaring done.

The CopilotKit helper decides the agentic architecture and UI pattern. The MiniMax goal-engineering skill decides the round discipline, tests, and closure.

## Reference docs

Read only the files relevant to the requested feature. Resolve these paths relative to this skill folder:

- `architecture.md` — overall CopilotKit architecture and runtime shape.
- `frontend.md` — frontend/provider setup and app wiring.
- `copilotchat.md` — embedded chat UI pattern.
- `copilotSideBar.md` — sidebar assistant UI pattern.
- `agents.md` — agent setup concepts.
- `agent_config.md` — typed user config forwarded into agent reasoning.
- `shared_state.md` — shared state between UI and agent.
- `attachments.md` — files/media passed into the agent.
- `programatic.md` — programmatic control of CopilotKit interactions.
- `reasoning.md` — reasoning-token rendering.
- `state_rendering.md` — render agent streamed state in the UI.
- `hitl.md` — generative UI overview and primitive selection.
- `ui_fixed.md` — fixed-schema A2UI: pre-authored UI schema, agent supplies data.
- `ui_dynamic.md` — dynamic-schema A2UI: agent composes schema from a catalog.
- `sub_agents.md` — sub-agent/generative UI concepts.
- `langgraph.md` — LangGraph integration patterns.

If a file appears mislabeled or duplicated, still inspect it before deciding it is irrelevant; use its actual content, not only its filename.

## Pattern selection

Choose the smallest CopilotKit pattern that proves the feature:

| User need | Preferred pattern |
|---|---|
| Simple assistant conversation | `CopilotChat` or `CopilotSidebar` |
| Existing custom app plus assistant dock | `CopilotSidebar` |
| Agent behavior depends on tone/persona/domain knobs | Agent config (`agent_config.md`) |
| UI and agent co-edit the same domain data | Shared state (`shared_state.md`) |
| Agent calls an existing backend tool and needs branded cards | Tool call rendering / render-tool pattern |
| Agent should render a known React component with typed props | Components as tools |
| UI should update as agent state streams | State rendering (`state_rendering.md`) |
| Reasoning/progress should be visible | Reasoning rendering (`reasoning.md`) |
| Known visual surface, dynamic data only | Fixed-schema A2UI (`ui_fixed.md`) |
| Agent can compose layout from approved catalog | Dynamic-schema A2UI (`ui_dynamic.md`) |
| Human approval/edit is required before action | HITL / generative UI (`hitl.md`) |
| Multi-agent workflow or specialist delegation | Sub-agents / LangGraph (`sub_agents.md`, `langgraph.md`) |

Default bias:

1. Start with controlled UI (`CopilotChat`, `CopilotSidebar`, typed components) before dynamic UI.
2. Prefer fixed-schema A2UI when the visual layout is known.
3. Use dynamic A2UI only when the agent genuinely needs to compose the interface.
4. Keep agent state and UI state typed.
5. Keep runtime endpoints explicit and documented.

## Implementation workflow

### 1. Pre-design

Before coding, identify:

- The user-visible job the agent performs.
- The runtime boundary: browser-only, local runtime endpoint, LangGraph backend, or Rust proxy-backed calls.
- The CopilotKit primitive(s) needed.
- The state shape shared between UI and agent.
- Tools/actions the agent can call.
- Human approval points, if any.
- What screenshot evidence proves the feature works.

### 2. Frontend contract

For every CopilotKit UI feature, specify in the rider:

- Provider wiring (`CopilotKit` runtime URL/properties).
- Chat surface (`CopilotChat`, `CopilotSidebar`, custom shell, or direct proxy chat).
- Registered tools/components/renderers.
- Shared state/config shape.
- Loading, error, and completion states.
- Data-testid selectors for Playwright.

### 3. Runtime/agent contract

For every agentic feature, specify:

- Agent runtime endpoint and protocol.
- Agent prompt/config inputs.
- Tool names and schemas.
- State keys and ownership: UI-owned, agent-owned, shared.
- Refusal/error behavior.
- Whether MiniMax proxy calls are direct OpenAI-compatible calls or through a CopilotKit runtime.

### 4. Tests and screenshots

Use the repository `.test/` policy:

- Generated Playwright files go under `.test/playwright/`.
- Screenshots go under `.test/playwright/screenshots/`.
- Capture multiple screenshots for slow MiniMax/LLM/API calls.
- Deduplicate identical waiting-state images by hash or pixel comparison.
- Write a screenshot analysis report under `.test/playwright/`.
- Assert visible user outcomes, not only DOM selectors.

Minimum generated Playwright shape:

```ts
// .test/playwright/tests/<feature>.spec.ts
// 1. Open app.
// 2. Trigger agentic action.
// 3. Poll screenshots until done or timeout.
// 4. Skip duplicate screenshots.
// 5. Assert final visible feature state.
// 6. Write screenshot-analysis.json.
```

## Rider requirements for CopilotKit features

When creating a goal/rider for an agentic solution, the rider must include:

- `## CopilotKit pattern` — chosen primitive(s) and why.
- `## Agent/runtime contract` — endpoint, state, tools, prompts/config.
- `## UI contract` — components, selectors, loading/error/done states.
- `## Visual verification` — screenshots, polling interval, timeout, dedupe method, final assertion.
- `## Out of scope` — especially dynamic UI/runtime features not included in the current round.

## Lessons from local agent-graph UI rounds

When building a CopilotKit/LangGraph or multi-agent chat UI in this repo:

- If the user asks for an agentic workflow, make the agentic path the default or make the launch control impossible to confuse with normal chat. Do not leave the UI defaulting to a generic assistant path unless explicitly requested.
- Agent metadata must appear in the conversation surface the user is watching, not in a secondary area below/outside the chat. For multi-agent flows, render ordered cards/messages for each agent with:
  - agent name
  - role/purpose
  - status: waiting/running/complete/error
  - tools used
  - output/artifact
- Browser tests must verify the actual default user path, not a manually preconfigured happy path. For example: open app -> do not change the mode selector -> type request -> click the visible submit button -> assert agent cards appear.
- For slow or API-backed flows, mocked Playwright tests are acceptable for deterministic UI contract checks, but they must still assert visible text and DOM placement such as agent cards inside `data-testid="message-list"`.
- Screenshots alone are not enough; pair screenshot capture with assertions that agent names/tools are visible in the expected conversation container.

## Done criteria

A CopilotKit feature is done only when:

- The selected reference docs were read.
- The goal/rider names the chosen CopilotKit pattern.
- The UI has deterministic selectors for tests.
- Tests pass, including Playwright screenshot analysis when UI-visible.
- `architecture_decisions.md` records the implemented CopilotKit architecture and future gaps.
