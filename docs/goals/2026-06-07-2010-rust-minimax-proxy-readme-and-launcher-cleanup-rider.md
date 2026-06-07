# Rust MiniMax Proxy — README and Launcher Cleanup Rider

This rider implements the goal at `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-07-2010-rust-minimax-proxy-readme-and-launcher-cleanup-goal.md`. It turns README into a reusable base-solution guide and hardens `run_app.sh` interrupt cleanup.

## Posture

- No proxy runtime behavior changes.
- Keep launcher changes small and shell-only.
- Generated test scripts live under `.test/`.
- No `git push`.

## Touch points

- `README.md` — rewrite as base-solution guide.
- `run_app.sh` — Ctrl+C trap/cleanup behavior.
- `.test/` — generated interrupt smoke script and logs.
- `architecture_decisions.md` — closure update if launcher behavior changes.

## Phases

### P1 — Generated launcher interrupt test
- Create `.test/test_run_app_interrupt.sh`.
- The test starts `run_app.sh`, waits for app readiness, sends interrupt, and asserts ports 8080/5173 are not listening.
Depth tests:
- `launcher_interrupt_cleans_proxy_and_app_ports`

### P2 — README rewrite
- Explain proxy purpose, OAuth credential behavior, OpenAI-compatible endpoint, React/CopilotKit UI, scripts, and skill methodology.
- Include reuse checklist for future solutions.
Depth tests:
- `readme_describes_base_solution_and_skill`

### P3 — Launcher cleanup hardening
- Adjust traps so Ctrl+C runs cleanup once and exits intentionally.
- Avoid leaving app/proxy listeners.
Depth tests:
- `launcher_interrupt_cleans_proxy_and_app_ports`

### P4 — Architecture closure
- Update `architecture_decisions.md` with README/base-solution purpose and launcher interrupt cleanup if needed.
Depth tests:
- `architecture_mentions_base_solution_and_cleanup`

### P5 — Verification
- Run generated interrupt test, `cargo check`, and `cargo test`.
Depth tests:
- `verification_green`

## Verification

```bash
bash .test/test_run_app_interrupt.sh
grep -qi 'goal+rider' README.md
grep -qi 'CopilotKit' README.md
grep -qi '.test' README.md
cargo check
cargo test
```

## Out of scope

- Runtime proxy refactors.
- New React UI features.
- Installing permanent Playwright tests.

## Done criteria

- README is suitable as a starting guide for future solutions.
- Launcher interrupt test passes.
- Cargo checks pass.
- Architecture decisions updated.
