GOAL: Add an agentic LangGraph-style PRD-to-plan workflow in the CopilotKit React chat app. The current app is a direct one-shot chat surface; this round adds a two-agent flow where the first agent turns the user's request into a PRD and the second agent turns that PRD into an implementation plan, with both artifacts visible in the chat UI.

**Read first.**
- `/Users/luis/projects/rust_minimax_proxy/architecture_decisions.md` — current map and prior decisions.
- `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-08-1556-rust-minimax-proxy-langgraph-prd-plan-rider.md` — implementation phases and tests.
- `/Users/luis/projects/rust_minimax_proxy/src/main.rs` — current proxy implementation.
- `/Users/luis/projects/rust_minimax_proxy/app/src/main.jsx` — current CopilotKit-wrapped chat app.
- `/Users/luis/projects/rust_minimax_proxy/.pi/skills/copilotkit_helper/SKILL.md` plus `architecture.md`, `frontend.md`, `copilotchat.md`, `agents.md`, and `langgraph.md` — CopilotKit/LangGraph pattern references.

**Posture.** Keep the Rust proxy unchanged unless a failing test proves it must change. No `git push`. Edits stay inside the project. Prefer tests before implementation. Keep the browser runtime explicit: React calls the Rust OpenAI-compatible proxy; the LangGraph orchestration runs in frontend code for this slice.

**Phases.** The rider defines P1..P7. Each implementation phase writes a named failing test first, implements the slice, then runs verification.

**Verification.**
- `cargo check`
- `cargo test`
- `cd app && npm test`
- `cd app && npm run build`
- If live credentials/network are available: `./first.sh` or document why skipped.
- If UI/browser verification is practical: generated Playwright script under `.test/playwright/` captures screenshots and asserts visible PRD/plan output.

**Stop when** the two-agent PRD/plan workflow is visible in the app, named tests pass, and `architecture_decisions.md` records the new behavior and touch points.
