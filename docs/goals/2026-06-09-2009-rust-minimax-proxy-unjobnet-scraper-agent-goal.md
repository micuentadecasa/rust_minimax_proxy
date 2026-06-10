GOAL: Add a UNJobNet Spain jobs scraper specialist agent to the coordinator graph. When the user asks about jobs related to informatics, data, or artificial intelligence, the coordinator routes to a new Jobs Agent that uses a Python Scrapy-based scraper service to fetch latest matching jobs from `https://www.unjobnet.org/countries/Spain` and the UI renders one visible card per matching job.

**Read first.**
- `/Users/luis/projects/rust_minimax_proxy/architecture_decisions.md` — current Rust proxy, React app, coordinator graph, testing policy.
- `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-09-2009-rust-minimax-proxy-unjobnet-scraper-agent-rider.md` — implementation phases and tests.
- `/Users/luis/projects/rust_minimax_proxy/app/src/agentGraph.js` and `app/src/main.jsx` — coordinator routing, specialist execution, and visible agent cards.
- `/Users/luis/projects/rust_minimax_proxy/.pi/skills/copilotkit_helper/SKILL.md` — CopilotKit/LangGraph UI discipline and visual verification.

**Posture.** Keep Rust proxy unchanged unless needed. Add the Python scraper as a local companion service with explicit env vars. No `git push`. Prefer tests before implementation. The scraper should be deterministic under tests by allowing fixture HTML and mocked fetches; do not depend on live UNJobNet in unit tests.

**Phases.** The rider defines Python scraper/service tests, JS graph routing tests, UI card rendering tests, Playwright visual verification, launcher updates, and architecture closure.

**Verification.**
- Python scraper tests pass.
- `cd app && npm test`
- `cd app && npm run build`
- Playwright mocked visual test for job route/cards passes.
- `cargo check` and `cargo test` remain passing.

**Stop when** job-related prompts route to the Jobs Agent, the scraper service returns matching jobs, the UI shows one card per job in the conversation, launch scripts start the scraper service with the app, and `architecture_decisions.md` records the new agent/runtime boundary.
