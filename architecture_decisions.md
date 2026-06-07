# Architecture Decisions

This document is the living map for the MiniMax OAuth proxy. Future feature work should update this file during the documentation/closure phase so the next agent knows where to touch code and tests.

## 1. Project shape

- Rust binary crate: `minimax_proxy`.
- Main implementation: `src/main.rs`.
- Python smoke/client script: `test_client.py`.
- Convenience runner: `first.sh` starts the proxy, waits for `/health`, runs `test_client.py`, then stops the proxy.
- Full app runner: `run_app.sh` starts the proxy, waits for `/health`, starts the React app, and cleans up proxy/app listeners on Ctrl+C or termination.
- Project skill for feature work: `.pi/skills/minimax-goal-engineering/SKILL.md`.
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

## 8. Base-solution documentation

`README.md` is now written as a base-solution guide for future projects. It documents:

- The Rust MiniMax OAuth proxy and OpenAI-compatible endpoint.
- The React/CopilotKit chat app.
- `first.sh` and `run_app.sh` usage.
- The `.pi/skills/minimax-goal-engineering` workflow.
- `.test/` generated artifact policy.
- Playwright screenshot polling/deduplication policy for UI verification.
- A reuse checklist for deriving new solutions from this repository.

## 9. Known gaps / future candidates

- No OpenAI-compatible streaming response translation yet.
- No dedicated Rust integration test harness yet.
- The React app is a first-pass direct chat surface; it does not yet use a CopilotKit runtime server, actions, agent state, or built-in Copilot chat widgets.
- Error mapping is functional but could be made more OpenAI-compatible.
- `src/main.rs` is still monolithic; future work may split auth, chat, config, and server modules once tests cover current behavior.
