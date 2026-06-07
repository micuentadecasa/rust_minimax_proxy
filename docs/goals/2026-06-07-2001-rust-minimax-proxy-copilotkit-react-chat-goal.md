GOAL: Add a React/CopilotKit chat app that talks to the existing MiniMax proxy, plus one launcher script that starts the proxy and then the app. The proxy already exposes `/v1/chat/completions`; this round adds a browser UI for that endpoint without changing MiniMax OAuth behavior.

**Read first.**
- `/Users/luis/projects/rust_minimax_proxy/architecture_decisions.md` — current proxy, testing, and `.test/` policy.
- `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-07-2001-rust-minimax-proxy-copilotkit-react-chat-rider.md` — phases, tests, Playwright screenshot policy.
- `/Users/luis/projects/rust_minimax_proxy/src/main.rs` — proxy endpoint behavior.
- `/Users/luis/projects/rust_minimax_proxy/first.sh` and `/Users/luis/projects/rust_minimax_proxy/test_client.py` — existing smoke path.

**Posture.** Keep the Rust proxy API-compatible. Add a frontend app under the project, not a separate repo. No `git push`. Generated tests/artifacts go under `.test/`; start by cleaning `.test/`.

**Feature.** Create a React app using the CopilotKit framework packages. For this first pass, the visible UI is a simple chat: user types a message, the app POSTs to `http://localhost:8080/v1/chat/completions`, then renders the assistant reply. Add a `.sh` script that starts the proxy, waits for health, starts the app, and cleans up on exit.

**Verification.**
- `cargo check`
- `cargo test`
- frontend install/build succeeds
- generated Playwright test in `.test/` captures screenshots in `.test/playwright/screenshots/`, polls for slow LLM response, deduplicates waiting screenshots, and confirms the assistant response is visible
- launcher script starts proxy and app

**Stop when** the React chat works through the proxy, Playwright visual evidence passes, and `architecture_decisions.md` records the frontend and script touch points.
