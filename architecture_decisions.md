# Architecture Decisions

This document is the living map for the MiniMax OAuth proxy. Future feature work should update this file during the documentation/closure phase so the next agent knows where to touch code and tests.

## 1. Project shape

- Rust binary crate: `minimax_proxy`.
- Main implementation: `src/main.rs`.
- Python smoke/client script: `test_client.py`.
- Convenience runner: `first.sh` starts the proxy, waits for `/health`, runs `test_client.py`, then stops the proxy.
- Full app runner: `run_app.sh` starts the proxy, waits for `/health`, starts the React app, and cleans up proxy/app listeners on Ctrl+C or termination.
- Project skill for feature work: `.pi/skills/minimax-goal-engineering/SKILL.md`.
- CopilotKit helper skill for agentic UI/agent architecture: `.pi/skills/copilotkit_helper/SKILL.md`.
- CopilotKit reference docs live beside that helper skill in `.pi/skills/copilotkit_helper/*.md`.
- Goal/rider specs for future rounds live in `docs/goals/`.

## 2. Runtime API

The proxy exposes OpenAI-compatible chat over Axum:

- `GET /health` — reports basic status and whether credentials are loaded.
- `GET /auth/status` — reports OAuth authentication status and expiry.
- `POST /auth/login` — intentionally disabled because login is automatic on startup.
- `POST /auth/token` — manual device-code polling helper.
- `POST /auth/refresh` — refreshes OAuth credentials.
- `POST /v1/chat/completions` — accepts OpenAI-style chat requests and forwards to MiniMax Anthropic messages API.

## 3. MiniMax OAuth decisions

- OAuth uses MiniMax account hosts:
  - Global: `https://account.minimax.io`
  - China: `https://account.minimaxi.com`
- OAuth endpoints:
  - Device code: `/oauth2/device/code`
  - Token/refresh: `/oauth2/token`
- Client ID: `659cf4c1-615c-45f6-a5f6-4bf15eb476e5`.
- Scope: `openid profile coding_plan`.
- PKCE verifier is random bytes encoded with URL-safe base64, and challenge is SHA-256 of the verifier.
- OAuth `expired_in` is normalized defensively because MiniMax may return absolute epoch milliseconds; code also tolerates epoch seconds or relative seconds.

## 4. Credential storage decisions

- Primary credential file is MiniMax CLI-compatible `~/.mmx/config.json` under the `oauth` key.
- `MMX_CONFIG_DIR` is honored by resolving credentials from `$MMX_CONFIG_DIR/config.json`.
- Legacy `~/.mmx/credentials.json` is still loaded as a fallback for older local state.
- Saving credentials preserves existing config fields where possible and writes OAuth values back to `config.json`.

## 5. Chat proxy decisions

- Incoming `ChatRequest` supports:
  - `messages`
  - `model`
  - `max_tokens`
  - `temperature`
  - `top_p`
  - `stream`
  - `system`
- Default model is `MiniMax-M2.7`.
- Incoming `system` role messages are lifted into the Anthropic-style `system` field; non-system messages are forwarded.
- OpenAI streaming translation is not implemented yet. The proxy currently sends `stream: false` upstream and returns a normal JSON completion.
- Upstream MiniMax Anthropic endpoint is `{api_base}/anthropic/v1/messages`.
- Upstream auth header is `x-api-key: <OAuth access token>`.
- Response is converted back to OpenAI-style `chat.completion` with one choice.

## 6. Testing and verification decisions

Current fast verification commands:

```bash
cargo check
cargo test
./first.sh
```

`./first.sh` is the end-to-end smoke path. It kills any stale process on the selected port, starts the current Rust proxy, waits for health, runs `test_client.py`, then cleans up.

Future feature work must add at least one automated test before implementation. Preferred order:

1. Rust unit/integration tests for request/response/auth logic.
2. Python smoke test updates when the behavior is visible through HTTP.
3. Playwright tests if a browser/UI surface is added later.

Generated test scripts and artifacts use a disposable workspace:

- `.test/` is ignored by git.
- Each skill-driven feature round starts by removing previous generated artifacts: `rm -rf .test && mkdir -p .test`.
- Generated round scripts, fixtures, screenshots, traces, logs, captured responses, and analysis reports go under `.test/`.
- Durable tests may still be added to maintained test locations when they should live with the codebase.

UI/Playwright policy for future UI work:

- Do not install/configure Playwright until a UI exists or a UI feature requires it.
- Playwright screenshots go under `.test/playwright/screenshots/`.
- For UI states that depend on slow MiniMax/LLM calls, tests should capture multiple screenshots over time until completion or timeout.
- Duplicate/effectively-identical waiting screenshots should be discarded or skipped using hashes, pixel-diff thresholds, or Playwright screenshot comparison.
- The final assertion should analyze the remaining screenshot progression and verify that the feature is visibly working, not only that selectors exist.

## 7. React/CopilotKit chat app

A browser UI now lives under `app/`:

- Build tool: Vite + React.
- CopilotKit packages: `@copilotkit/react-core` and `@copilotkit/react-ui`.
- Entry point: `app/src/main.jsx`.
- Styling: `app/src/styles.css`.
- The app is wrapped in `CopilotKit` for future CopilotKit runtime expansion, but this first chat surface sends direct OpenAI-compatible `POST /v1/chat/completions` requests to the Rust proxy.
- Browser proxy URL is configurable with `VITE_PROXY_BASE_URL`; default is `http://localhost:8080`.
- Default model is configurable with `VITE_MINIMAX_MODEL`; default is `MiniMax-M2.7`.

Browser support required a CORS layer on the Axum app:

- `Cargo.toml` includes `tower-http` with the `cors` feature.
- `src/main.rs` applies `CorsLayer` allowing browser `GET`, `POST`, and `OPTIONS` requests.

The combined launcher is `run_app.sh`:

- Stops stale listeners on the proxy and app ports.
- Starts the Rust proxy on `PORT` (default `8080`).
- Waits for `/health`.
- Installs app dependencies if `app/node_modules` is missing.
- Starts Vite on `APP_PORT` (default `5173`).
- Cleans up proxy and app listeners on exit, Ctrl+C, or termination.
- Disables repeated trap execution during cleanup so interrupt handling returns the terminal cleanly.

Generated UI verification for this round lives in ignored `.test/playwright/`:

- Test: `.test/playwright/tests/chat-ui.spec.mjs`.
- Screenshots: `.test/playwright/screenshots/`.
- Analysis report: `.test/playwright/screenshot-analysis.json`.
- The test sends a chat message, polls screenshots while the LLM call is pending, skips duplicate screenshots by hash, and asserts the final assistant answer is visibly rendered.

## 8. Skill composition for agentic solutions

Two project skills are intended to compose:

- `.pi/skills/minimax-goal-engineering/SKILL.md` controls round discipline: read architecture, create goal+rider, clean `.test/`, write tests first, verify, and update architecture decisions.
- `.pi/skills/copilotkit_helper/SKILL.md` controls CopilotKit agentic architecture: choose chat/sidebar, agent config, shared state, components/tools, tool rendering, state rendering, reasoning, fixed/dynamic A2UI, HITL, sub-agents, or LangGraph patterns from the local reference docs.

When a user asks for a new feature that is also an agentic solution, both skills should be used. The goal/rider should cite the CopilotKit helper skill, name the selected CopilotKit pattern, define the agent/runtime contract, define the UI contract, and include Playwright screenshot verification for visible UI behavior.

## 9. Base-solution documentation

`README.md` is now written as a base-solution guide for future projects. It documents:

- The Rust MiniMax OAuth proxy and OpenAI-compatible endpoint.
- The React/CopilotKit chat app.
- `first.sh` and `run_app.sh` usage.
- The `.pi/skills/minimax-goal-engineering` workflow.
- `.test/` generated artifact policy.
- Playwright screenshot polling/deduplication policy for UI verification.
- A reuse checklist for deriving new solutions from this repository.

## 10. React LangGraph coordinator workflow

The React app defaults to a coordinator-routed agent graph.

- Graph module: `app/src/agentGraph.js`.
- Durable graph tests: `app/src/agentGraph.test.mjs` using Node's built-in test runner with mocked proxy responses.
- Frontend package changes:
  - `@langchain/langgraph` is installed for a coordinator graph.
  - `npm test` runs `node --test src/*.test.mjs`.
- Runtime boundary remains browser → Rust proxy:
  - Specialist agents call `POST ${VITE_PROXY_BASE_URL}/v1/chat/completions`.
  - No new Rust routes were added.
  - No dedicated CopilotKit runtime server exists yet.
- Agent graph contract:
  - Coordinator Agent uses a deterministic router and does not call the LLM.
  - PRD Agent handles PRD/product requirements/spec/story/success-metric requests.
  - Planning Agent handles implementation plan/phase/task/test/roadmap requests.
  - Ambiguous product requests default to PRD Agent.
  - Only one specialist is called per user request in this round.
- UI contract in `app/src/main.jsx`:
  - Default mode is `agent-router`; normal chat remains manually selectable.
  - Mode selector: `data-testid="mode-select"`.
  - Submit/loading copy says `Ask coordinator` / `Coordinator Agent is choosing a specialist…` while in graph mode.
  - Loading step text: `data-testid="agent-step-status"`.
  - Ordered trace wrapper: `data-testid="agent-trace"`, rendered inside `data-testid="message-list"` so agent identity/tool cards appear in the conversation.
  - Conversation is vertical: user message, Coordinator Agent card, then the selected specialist card.
  - Coordinator card: `data-testid="agent-card-coordinator"`, showing route decision and reason.
  - PRD Agent card: `data-testid="agent-card-prd"`, showing name, role, status, tool list, and PRD output when selected.
  - Planning Agent card: `data-testid="agent-card-plan"`, showing name, role, status, tool list, and plan output when selected.
  - Final outputs remain queryable as `data-testid="prd-output"` and `data-testid="plan-output"`.
- Agent metadata is exported from `app/src/agentGraph.js` as `AGENTS`; graph progress events include `agentId`, `agentName`, `role`, `tools`, `status`, and `output`.
- Styling for vertical graph agent trace cards lives in `app/src/styles.css`.
- Generated Playwright verification for this round lives at `.test/playwright/tests/coordinator-agent-routing.spec.mjs`.

Verification run for this change:

```bash
cargo check
cargo test
cd app && npm test
cd app && npm run build
cd app && npx playwright test --config=../.test/playwright/playwright.config.mjs
```

The Playwright test uses mocked proxy responses, asserts that the default selector value is `agent-router`, submits PRD and plan prompts, verifies that the Coordinator Agent chooses exactly one specialist, and captures `.test/playwright/screenshots/coordinator-prd-route.png`, `.test/playwright/screenshots/coordinator-plan-route.png`, plus `.test/playwright/screenshot-analysis.json`.

`npm run build` currently succeeds with Vite warnings from dependency/browser externalization and large chunks after adding LangGraph/CopilotKit dependencies. Future production hardening should consider code-splitting or moving LangGraph orchestration behind a backend runtime.

## 11. UNJobNet Spain Jobs Agent

The coordinator graph now includes a third specialist route for job-search requests.

- Python scraper package: `jobs_scraper/`.
- Python dependencies: `requirements.txt` with FastAPI, Uvicorn, HTTPX, Scrapy, and pytest.
- Scraper parser: `jobs_scraper/unjobnet.py`.
  - Source URLs:
    - `https://www.unjobnet.org/countries/Spain`
    - `https://www.unjobnet.org/jobs?keywords=artificial`
    - `https://unjobs.org/duty_stations/spain`
  - Uses Scrapy `Selector` parsing over fetched HTML.
  - Normalizes relative UNJobNet job URLs against `https://www.unjobnet.org` and UNJobs relative URLs against `https://unjobs.org`.
  - Supports UNJobs `/vacancies/...` links in addition to UNJobNet `/jobs/detail/...` links.
  - Merges sources and deduplicates jobs by URL.
  - Loads `jobs_scraper/cv.json` for Luis Molina Martinez and deterministically filters jobs by CV fit.
  - CV-fit terms prioritize Responsible AI, AI governance, GDPR, EU AI Act, model governance, RAG, LangGraph, AI architecture, product/technical leadership, cloud, Python, data/AI engineering, security, and UN experience.
  - Filters first for informatics/data/artificial-intelligence related search terms, then excludes jobs without enough CV overlap. Returned jobs include `cvMatchedTerms`, `cvMatchScore`, and `cvMatchReason`.
  - Explicit queries such as `artificial intelligence` narrow matching to explicit terms, while generic job prompts use the full default domain term list.
- Local scraper API: `jobs_scraper/service.py`.
  - `GET /health` — service health.
  - `GET /jobs/search?q=<query>&limit=<n>` — returns `{ source, sources, query, jobs }`; `source` remains the Spain URL for backwards compatibility and `sources` lists all scraped URLs.
  - Default port in launcher: `JOBS_PORT=8090`.
- React/LangGraph integration:
  - `app/src/agentGraph.js` adds `AGENTS.jobs` and a `jobs` coordinator route.
  - Job prompts route to Jobs Agent when they mention jobs/positions/vacancies/careers/work. Informatics/data/AI/technology terms improve the route reason and scraper filtering, while generic job prompts still search the default informatics/data/AI domain.
  - Jobs Agent calls `VITE_JOBS_API_BASE_URL` directly; default is `http://localhost:8090`.
  - Jobs Agent does not call the MiniMax proxy in this round; it returns structured scraper artifacts.
- UI integration in `app/src/main.jsx`:
  - Jobs Agent cards render inside `data-testid="message-list"`.
  - Job cards use deterministic selectors: `job-card`, `job-title`, `job-organization`, `job-location`, `job-deadline`, `job-link`, `job-cv-match`, and `jobs-empty`.
  - Conversation remains vertical: user message, Coordinator Agent card, Jobs Agent card with nested job cards.
- Styling for job cards lives in `app/src/styles.css`.
- `run_app.sh` now starts three local processes in order:
  1. MiniMax Rust proxy.
  2. UNJobNet jobs scraper service.
  3. React app with `VITE_PROXY_BASE_URL` and `VITE_JOBS_API_BASE_URL`.

Verification run for this change:

```bash
.venv/bin/python -m pytest jobs_scraper/tests
bash -n run_app.sh
cargo check
cargo test
cd app && npm test
cd app && npm run build
cd app && npx playwright test --config=../.test/playwright/playwright.config.mjs
```

A live local scraper smoke also returned current UNJobNet Spain matches for `data ai`, including `Data Center Assistant Internship` and `INFORMATION SYSTEMS OFFICER (Temporary Job Opening)` at the time of the run. A later live smoke for `artificial intelligence` confirmed the service fetched both `https://www.unjobnet.org/countries/Spain` and `https://www.unjobnet.org/jobs?keywords=artificial`, applied CV-fit filtering, and returned jobs with CV match metadata. The scraper was then extended to include `https://unjobs.org/duty_stations/spain`; fixture tests prove UNJobs relative `/vacancies/...` URLs normalize to `https://unjobs.org/...` and still pass through CV-fit filtering. Durable tests use fixture/mocked HTML and mocked browser service responses so they do not depend on live UNJobNet/UNJobs availability.

## 12. Known gaps / future candidates

- No OpenAI-compatible streaming response translation yet.
- No dedicated Rust integration test harness yet.
- The React app is still browser-orchestrated; it does not yet use a CopilotKit runtime server, actions, backend agent state, or built-in Copilot chat widgets.
- Error mapping is functional but could be made more OpenAI-compatible.
- `src/main.rs` is still monolithic; future work may split auth, chat, config, and server modules once tests cover current behavior.
