GOAL: Rewrite README as a reusable base-solution guide and make the app launcher shut down cleanly on Ctrl+C. This repository is now a template for future proxy-backed AI apps: Rust MiniMax OAuth proxy, React/CopilotKit chat UI, goal+rider skill workflow, disposable `.test/` artifacts, and screenshot-based UI verification.

**Read first.**
- `/Users/luis/projects/rust_minimax_proxy/architecture_decisions.md` — current proxy, skill, React app, launcher, and testing decisions.
- `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-07-2010-rust-minimax-proxy-readme-and-launcher-cleanup-rider.md` — phases and verification.
- `/Users/luis/projects/rust_minimax_proxy/README.md` — document to rewrite.
- `/Users/luis/projects/rust_minimax_proxy/run_app.sh` — launcher Ctrl+C cleanup path.

**Posture.** Documentation plus small launcher hardening only. No proxy behavior changes. No `git push`. Generated test scripts/artifacts go under `.test/`.

**Feature.** README should explain what the solution does, how to run it, how the skill works, and how to reuse it as a base for other solutions. `run_app.sh` should handle Ctrl+C with explicit cleanup/exit so the terminal returns cleanly after stopping proxy and app.

**Verification.**
- README mentions proxy, React/CopilotKit app, project skill, goal/rider flow, `.test/`, Playwright screenshot policy, and reuse/template guidance.
- Generated `.test/` launcher interrupt check passes.
- `cargo check` and `cargo test` pass.

**Stop when** README is a clear base-solution guide, Ctrl+C cleanup is hardened, architecture decisions are updated if needed, and verification passes.
