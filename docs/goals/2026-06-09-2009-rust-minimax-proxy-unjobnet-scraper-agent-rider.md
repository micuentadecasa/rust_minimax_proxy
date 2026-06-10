# Rust MiniMax Proxy — UNJobNet Scraper Jobs Agent Rider

This rider implements the goal at `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-09-2009-rust-minimax-proxy-unjobnet-scraper-agent-goal.md`.

## Posture

- Keep the MiniMax Rust proxy OpenAI-compatible and unchanged unless a test proves a proxy change is required.
- Add the UNJobNet scraper as a Python local companion service, not browser scraping.
- Use the Python `scrapy` library for parsing/scraping logic.
- Do not make durable tests depend on live UNJobNet network availability; use fixture HTML and mocked fetches.
- UI must render job cards inside `data-testid="message-list"`.
- No unrelated CopilotKit runtime migration in this round.

## CopilotKit pattern

Controlled CopilotKit-wrapped custom chat with browser-local LangGraph coordinator and a new specialist Jobs Agent. This uses the existing sub-agent/LangGraph pattern and fixed-schema job cards: the agent returns structured job objects, and React renders approved card UI.

## Agent/runtime contract

- Coordinator adds a third route: `jobs`.
- Jobs Agent handles requests mentioning jobs/work/positions/vacancies/careers; informatics/data/artificial-intelligence terms are used by the scraper filter when present, and the service still defaults to those domains for generic job prompts.
- Jobs Agent calls a local Python service, default `VITE_JOBS_API_BASE_URL=http://localhost:8090`.
- Python service endpoint: `GET /jobs/search?q=<query>&limit=<n>`.
- Service scrapes/parses `https://www.unjobnet.org/countries/Spain` using Scrapy selectors.
- Service returns JSON:
  ```json
  { "source": "https://www.unjobnet.org/countries/Spain", "query": "data", "jobs": [{ "title": "...", "organization": "...", "location": "...", "deadline": "...", "url": "...", "summary": "...", "matchedTerms": ["data"] }] }
  ```
- If no jobs match, Jobs Agent completes with an empty list and a visible no-results message.

## UI contract

- `AGENTS.jobs` metadata exists with name `Jobs Agent`.
- Deterministic selectors:
  - `agent-card-jobs`
  - `jobs-output`
  - `job-card`
  - `job-title`
  - `job-organization`
  - `job-location`
  - `job-deadline`
  - `job-link`
  - `jobs-empty`
- Conversation remains vertical: user message, Coordinator Agent card, Jobs Agent card with nested job cards.
- Welcome/placeholder copy mentions jobs.
- Submit flow remains the default coordinator path.

## Visual verification

Add mocked Playwright test under `.test/playwright/tests/` that:

1. Opens the app without changing mode.
2. Mocks the jobs service response with two matching Spain jobs.
3. Enters a job prompt about data/AI/informatics.
4. Captures screenshots under `.test/playwright/screenshots/`.
5. Asserts Coordinator Agent and Jobs Agent cards appear inside `message-list`.
6. Asserts two visible job cards with title/org/location/deadline/link.
7. Writes `.test/playwright/screenshot-analysis.json`.

## Touch points

- `requirements.txt` — Python dependencies for scraper service/tests.
- `jobs_scraper/` — Python package with Scrapy parsing and FastAPI service.
- `app/src/agentGraph.js` — coordinator route, Jobs Agent, jobs API fetcher.
- `app/src/agentGraph.test.mjs` — routing/jobs agent tests with mocked fetch.
- `app/src/main.jsx` — render job cards.
- `app/src/styles.css` — job card styling.
- `run_app.sh` — launch Python jobs service before React app.
- `.test/playwright/` — generated browser visual test and screenshots.
- `architecture_decisions.md` — closure note.

## Test contract

Write tests before implementation where feasible:

- Python parser fixture test for UNJobNet-like HTML.
- Python service test for query filtering.
- JS route test for job prompts.
- JS graph test that Jobs Agent calls the jobs service and not MiniMax.
- Playwright mocked UI test for visible cards.

## Phases

### P1 — Python scraper tests
- Add fixture/parser tests for informatics/data/AI job filtering.
Depth tests:
- `python -m pytest jobs_scraper/tests`

### P2 — Python scraper service
- Implement Scrapy-selector parser and FastAPI endpoint.
- Keep live fetch isolated and injectable/testable.
Depth tests:
- `python -m pytest jobs_scraper/tests`

### P3 — Coordinator graph tests
- Add route and jobs-agent tests to `app/src/agentGraph.test.mjs`.
Depth tests:
- `cd app && npm test`

### P4 — Coordinator graph implementation
- Add `jobs` route and Jobs Agent API call.
Depth tests:
- `cd app && npm test`

### P5 — UI job cards
- Render job cards in the Jobs Agent card with deterministic selectors.
Depth tests:
- `cd app && npm test`
- `cd app && npm run build`

### P6 — Playwright visual verification
- Add mocked browser test with screenshots and analysis JSON under `.test/playwright/`.
Depth tests:
- `cd app && npx playwright test --config=../.test/playwright/playwright.config.mjs`

### P7 — Launcher and documentation closure
- Update `run_app.sh` to launch jobs service.
- Update `architecture_decisions.md`.
Depth tests:
- `cargo check`
- `cargo test`
- `cd app && npm test`
- `cd app && npm run build`

## Out of scope

- Crawling every UNJobNet pagination page.
- Persisting/caching jobs.
- LLM ranking of jobs.
- Backend CopilotKit runtime migration.
- Rust proxy changes.

## Done criteria

- Goal/rider exist.
- Scrapy-based parser/service exists and is tested.
- Coordinator routes job requests to Jobs Agent.
- Jobs Agent returns structured job artifacts.
- UI renders one card per matching job.
- Playwright visual test passes with screenshots and analysis.
- Launcher starts proxy, jobs service, then React app.
- `architecture_decisions.md` is updated.
