# Rust MiniMax Proxy — CopilotKit React Chat Rider

This rider implements the goal at `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-07-2001-rust-minimax-proxy-copilotkit-react-chat-goal.md`. It adds a React/CopilotKit chat UI that calls the existing proxy and a launcher script that runs both proxy and app.

## Posture

- Do not change MiniMax OAuth or chat proxy semantics unless CORS is required for the browser.
- Frontend lives inside this repo, expected path `app/`.
- Generated test scripts, Playwright config, screenshots, traces, reports, and helpers live under `.test/`.
- No `git push`.

## Touch points

- `src/main.rs` — may need permissive CORS for browser requests.
- `Cargo.toml` — may need `tower-http` CORS feature.
- `app/` — React + CopilotKit app.
- `run_app.sh` — start proxy, wait for health, start Vite app, cleanup.
- `.test/` — generated Playwright test and screenshot analysis artifacts.
- `architecture_decisions.md` — closure update.

## Test contract

Write generated tests under `.test/` after cleaning `.test/`. Playwright must capture screenshots under `.test/playwright/screenshots/`, poll for slow LLM/API completion, discard duplicate waiting screenshots, and assert the visible assistant reply.

## Phases

### P1 — Generated UI test skeleton fails
- Clean `.test/`.
- Create a generated Playwright test under `.test/playwright/` that expects the chat UI at `http://localhost:5173`.
- The test should send a short message, capture screenshots over time, dedupe duplicate images, and assert a non-empty assistant response is visible.
Depth tests:
- `playwright_chat_proxy_visual_response`

### P2 — React/CopilotKit app scaffold
- Add `app/package.json`, Vite config, React source, and CSS.
- Include CopilotKit framework dependencies and wrap the app with CopilotKit provider metadata while direct chat calls go to the proxy endpoint for this first pass.
Depth tests:
- `frontend_builds`

### P3 — Browser-to-proxy compatibility
- If needed, add CORS to the Rust proxy so Vite can call `http://localhost:8080/v1/chat/completions` from `http://localhost:5173`.
Depth tests:
- `browser_can_post_to_proxy`

### P4 — Launcher script
- Add `run_app.sh` that starts the proxy on `PORT=8080`, waits for `/health`, installs frontend dependencies if missing, starts Vite, and kills both processes on exit.
Depth tests:
- `launcher_starts_proxy_and_app`

### P5 — Playwright screenshot verification
- Run the generated Playwright test.
- Store screenshots in `.test/playwright/screenshots/`.
- Store an analysis/report file under `.test/playwright/` noting kept screenshots and duplicate skips.
Depth tests:
- `playwright_screenshots_show_chat_answer`

### P6 — Existing verification
- Run `cargo check`, `cargo test`, and frontend build.
Depth tests:
- `cargo_and_frontend_green`

### P7 — Architecture closure
- Update `architecture_decisions.md` with frontend, CORS, launcher, and Playwright policy as implemented.
- No new feature code in this phase.

## Verification

```bash
cargo check
cargo test
(cd app && npm install && npm run build)
bash run_app.sh
# In another process, generated Playwright command from .test/playwright/
```

## Out of scope

- Streaming UI responses.
- CopilotKit runtime server/actions/agent state.
- Deployment packaging.
- Persisting chat history.

## Done criteria

- React app exists and builds.
- CopilotKit packages are present and the app is wrapped for future CopilotKit expansion.
- Chat POSTs to the proxy and renders assistant response.
- Launcher script starts proxy then app and cleans up.
- Playwright visual test passes with screenshot polling/deduplication artifacts in `.test/`.
- Architecture decisions updated.
