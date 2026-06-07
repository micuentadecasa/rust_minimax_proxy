# MiniMax OAuth Proxy App Base

This repository is a reusable base for building MiniMax-powered applications without putting API keys in each app. It combines:

- A Rust OAuth proxy that exposes an OpenAI-compatible chat endpoint.
- A React/CopilotKit chat app that calls the proxy from the browser.
- Shell scripts for local smoke testing and full app launch.
- A project skill that forces future changes through goal+rider planning, tests-first implementation, `.test/` artifacts, and architecture-decision updates.

Use this repo as the starting point for other solutions that need a local authenticated LLM proxy plus a browser UI.

## What this solution does

### 1. Rust MiniMax OAuth proxy

The proxy starts with:

```bash
cargo run
```

On startup it:

1. Loads existing MiniMax OAuth credentials from `~/.mmx/config.json`.
2. Falls back to legacy `~/.mmx/credentials.json` if needed.
3. Starts MiniMax OAuth device-code login when credentials are missing or expiring.
4. Opens the browser for authorization.
5. Starts an Axum server after authentication.

Main endpoint:

```text
POST http://localhost:8080/v1/chat/completions
```

The endpoint accepts OpenAI-style chat requests and forwards them to MiniMax's Anthropic-compatible messages API using OAuth credentials.

Other endpoints:

```text
GET  /health
GET  /auth/status
POST /auth/refresh
POST /auth/token
POST /auth/login  # disabled; startup auth is automatic
```

### 2. React/CopilotKit chat app

The browser app lives in `app/` and uses:

- Vite
- React
- CopilotKit packages:
  - `@copilotkit/react-core`
  - `@copilotkit/react-ui`

For now the UI is intentionally simple: a chat screen posts directly to the proxy's `/v1/chat/completions` endpoint and renders the assistant response. The app is wrapped with CopilotKit so future solutions can add CopilotKit runtime features, actions, richer widgets, and agent state.

Run the full local app:

```bash
./run_app.sh
```

This script:

1. Stops stale listeners on the proxy/app ports.
2. Starts the Rust proxy on `http://localhost:8080`.
3. Waits for `/health`.
4. Installs app dependencies if missing.
5. Starts the React app on `http://localhost:5173`.
6. Cleans up both processes on exit or Ctrl+C.

Open:

```text
http://localhost:5173
```

### 3. Smoke script

To verify only the proxy and Python client:

```bash
./first.sh
```

This starts the proxy, waits for health, runs `test_client.py`, and stops the proxy.

## Project skill: goal-engineered feature work

The project-local skill is here:

```text
.pi/skills/minimax-goal-engineering/SKILL.md
```

Use this skill for every new feature, bug fix, endpoint, UI change, or behavior change. It makes this repository self-documenting and reusable.

The skill requires each round to:

1. Read `architecture_decisions.md` first.
2. Create a goal+rider pair in `docs/goals/`.
3. Clean generated test artifacts:
   ```bash
   rm -rf .test
   mkdir -p .test
   ```
4. Write tests before implementation.
5. Put generated scripts, screenshots, traces, and reports in `.test/`.
6. Use Playwright screenshot polling/analysis for UI features.
7. Update `architecture_decisions.md` before marking work done.
8. Run verification commands.

Goal/rider files are stored in:

```text
docs/goals/
```

Architecture memory is stored in:

```text
architecture_decisions.md
```

This is the main mechanism that makes the repo useful as a base for future solutions: every change leaves behind the goal, the rider, the tests, and the architecture update needed by the next agent.

## Testing policy

Generated or temporary test artifacts go under ignored `.test/`.

Examples:

```text
.test/test_run_app_interrupt.sh
.test/playwright/screenshots/
.test/playwright/screenshot-analysis.json
.test/playwright/report/
```

Durable code tests can still live in normal project test locations when they become part of the maintained suite.

For UI features, Playwright tests should:

- Capture screenshots into `.test/playwright/screenshots/`.
- Poll over time for slow MiniMax/LLM responses.
- Discard duplicate waiting-state screenshots.
- Assert the final visible UI state, not only DOM selectors.

## Common commands

```bash
# Rust verification
cargo check
cargo test

# Proxy smoke test
./first.sh

# Full proxy + React app
./run_app.sh

# Frontend build
cd app && npm install && npm run build
```

## Reusing this as a base for another solution

When creating a new solution from this base:

1. Keep the Rust proxy if you want MiniMax OAuth without app-level API keys.
2. Replace or extend `app/` with your product UI.
3. Keep `run_app.sh` as the local orchestration pattern.
4. Keep `.pi/skills/minimax-goal-engineering/SKILL.md` and adapt names/paths if needed.
5. Keep `architecture_decisions.md` as the living map for the new solution.
6. Keep `docs/goals/` as the historical corpus of feature decisions.
7. Keep `.test/` ignored for generated test scripts and visual artifacts.
8. For each feature, use the skill: goal -> rider -> test first -> implement -> verify -> architecture update.

## Current known gaps

- OpenAI-compatible streaming translation is not implemented yet.
- The React app is a first-pass chat UI; it does not yet use a CopilotKit runtime server/actions.
- There is no dedicated Rust integration test harness yet.
- `src/main.rs` is still monolithic and can be split once durable tests cover current behavior.
