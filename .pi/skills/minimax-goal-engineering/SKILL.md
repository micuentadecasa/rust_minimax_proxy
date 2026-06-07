---
name: minimax-goal-engineering
description: Use for any new feature, bug fix, or behavior change in this Rust MiniMax OAuth proxy. Applies Greg Ceccarelli-style goal+rider methodology: create a small goal, create a detailed rider, write tests first, implement, update architecture_decisions.md, and only mark done when verification passes. Also use when the user says "new feature", "goal", "rider", "use goal engineering", or asks to evolve this solution.
license: Apache-2.0
---

# MiniMax Proxy Goal Engineering

This skill adapts Greg Ceccarelli's goal+rider method to this project. For every feature round, create a checked-in plan pair and then execute against it.

Core idea: two markdown files per round:

- **Goal**: <= 4000 characters. The spine: what to do, what to read, posture, verification, stop conditions.
- **Rider**: unbounded. The mechanics: phases, named tests, implementation touch points, out-of-scope boundaries, architecture closure.

A feature is **not done** until its named tests pass and `architecture_decisions.md` is updated with what changed.

## When to use

Use this skill automatically when the user asks for a new feature, bug fix, endpoint, testability improvement, UI, auth change, MiniMax behavior change, or anything like:

- "add feature X"
- "make the proxy do Y"
- "create a goal/rider"
- "use goal engineering"
- "next round"

## Project-specific map

Read these before drafting a goal/rider:

1. `architecture_decisions.md` — living map of what exists and where to touch.
2. Most recent files in `docs/goals/`, if any.
3. `src/main.rs` — current Rust implementation.
4. `Cargo.toml` — dependencies and test tooling.
5. `test_client.py` and `first.sh` — smoke/e2e verification.
6. `README.md` and `guide.md` — intended user flow.
7. Recent commits: `git log --oneline -30`.

If `architecture_decisions.md` is missing, create it before implementation and record the current architecture.

## Round workflow

### Step 1 — Pre-work, no code yet

- Understand the feature request.
- Read the project map above.
- Identify exact files/functions likely to change.
- Identify the fastest automated test that can prove the feature.
- If a UI is involved and Playwright is not configured, add Playwright setup only when it is truly needed.

### Step 1.5 — Clean disposable test workspace

Before writing any generated test scripts, smoke clients, fixtures, screenshots, traces, reports, or temporary artifacts, remove the previous round's disposable workspace and recreate it:

```bash
rm -rf .test
mkdir -p .test
```

`.test/` is intentionally ignored by git. Generated round artifacts go there. Durable tests that should become part of the maintained suite may still be added to Rust/Python/Playwright test locations, but any helper scripts and captured artifacts for the agent round start in `.test/`.

### Step 2 — Create goal+rider files

Create `docs/goals/` if missing.

Use one timestamp for both files:

```bash
DATE=$(date +%Y-%m-%d)
HHMM=$(date +%H%M)
```

Filename shape:

```text
docs/goals/<YYYY-MM-DD>-<HHMM>-rust-minimax-proxy-<topic>-goal.md
docs/goals/<YYYY-MM-DD>-<HHMM>-rust-minimax-proxy-<topic>-rider.md
```

### Goal template, <= 4000 chars

```markdown
GOAL: <one sentence>. <short paragraph: current pain -> change -> expected result>.

**Read first.**
- `<absolute path>/architecture_decisions.md` — current map and prior decisions.
- `<absolute path>/docs/goals/<...>-rider.md` — implementation phases and tests.
- `<absolute path>/src/main.rs` — current proxy implementation.
- `<absolute path>/test_client.py` and `<absolute path>/first.sh` — smoke path.

**Posture.** Keep this a small Rust proxy. No `git push`. Edits stay inside the project. Prefer tests before implementation. Do not silently add unrelated architecture.

**Phases.** The rider defines P1..P11 or a smaller justified phase set. Each implementation phase writes a named failing test first, implements the slice, then runs verification.

**Verification.**
- `cargo check`
- `cargo test`
- `./first.sh` or a narrower smoke command when network/API cost matters
- If UI exists: Playwright test command passes.

**Stop when** the named tests pass, the feature works through the relevant endpoint/script/UI, and `architecture_decisions.md` records the new behavior and touch points.
```

Run `wc -c <goal>` and keep it <= 4000 characters.

### Rider template

```markdown
# Rust MiniMax Proxy — <Topic> Rider

This rider implements the goal at `<absolute path to goal>`. It adds <summary> and preserves prior architecture decisions in `architecture_decisions.md`.

## Posture

- Keep the proxy OpenAI-compatible at the edge unless the goal explicitly changes that.
- Keep MiniMax upstream behavior isolated and documented.
- No `git push`.
- No unrelated refactors unless required by the tests.
- No silent scope expansion; put future ideas in the rider's Out of scope section or `architecture_decisions.md` Known gaps.

## Touch points

- `<file>` — <why it may change>.
- `<test file>` — <test to add/update>.

## Test contract

Each non-doc phase must name at least one test and write it before implementation. Watch it fail when feasible.

Preferred test order:

1. Rust unit/integration tests for pure behavior and HTTP handlers.
2. Python smoke/client updates for user-visible HTTP behavior.
3. Playwright tests only for actual UI/browser flows.

Generated test scripts, one-off smoke clients, fixtures, captured responses, screenshots, traces, and reports belong under `.test/`. At the start of each skill-driven round, run `rm -rf .test && mkdir -p .test` so stale scripts and screenshots cannot make a feature look done.

## Phases

### P1 — Baseline and failing test
- Add the narrowest named test that proves the requested feature or reproduces the bug.
- Run the relevant command and observe failure.
Depth tests:
- `<test_name>`

### P2 — Implement the smallest slice
- Implement only enough to satisfy P1.
- Run `cargo check` and the named test.
Depth tests:
- `<test_name>`

### P3 — HTTP/client smoke coverage
- Update or add smoke coverage in `test_client.py`, `first.sh`, or a Rust integration test when the behavior crosses HTTP.
Depth tests:
- `<test_name_or_smoke_assertion>`

### P4 — Edge cases and errors
- Add tests for auth, bad request, expired token, invalid payload, or feature-specific edge cases.
Depth tests:
- `<edge_case_test>`

### P5 — UI/Playwright if applicable
- If the feature adds UI, configure or update Playwright and add a browser test.
- Screenshots must be recorded under `.test/playwright/screenshots/`.
- For UI flows that depend on MiniMax/LLM calls or any slow async process, take screenshots multiple times over time until the UI reaches the expected done state or a rider-defined timeout.
- Discard or skip duplicate/effectively-identical screenshots that only show the same waiting/loading state. Use a deterministic comparison when possible (file hash, pixel diff threshold, or Playwright screenshot comparison), then keep only meaningful state changes for final analysis.
- The final UI assertion must analyze the kept screenshots and verify that the requested feature is visibly working as specified, not merely that a selector exists.
- If no UI exists, explicitly mark this phase not applicable.
Depth tests:
- `<playwright_test_name_or_not_applicable>`

### P6 — Documentation and runner updates
- Update README/guide/scripts if user-facing behavior changed.
Depth tests:
- `<doc_or_script_smoke>`

### P7 — Architecture closure
- Update `architecture_decisions.md` with:
  - What changed.
  - Files/functions touched.
  - Test commands that prove it.
  - Known gaps or future candidates.
- No new feature code in this phase.

## Verification

Run before declaring done:

```bash
cargo check
cargo test
./first.sh
```

If UI/Playwright was added:

```bash
npm test
# or the exact Playwright command configured by the rider
```

Playwright artifacts must be captured under `.test/playwright/`, including screenshots in `.test/playwright/screenshots/`. For slow LLM-backed screens, the Playwright test should poll with multiple screenshots, discard duplicate waiting-state images, and assert from the kept visual progression that the feature reached the intended UI state.

## Out of scope

- <explicitly excluded work>.

## Done criteria

- Goal file exists and is <= 4000 chars.
- Rider file exists and names the tests.
- At least one code test/smoke test was created or updated for the feature.
- All relevant verification commands pass.
- `architecture_decisions.md` is updated.
- Final response lists changed files and verification results.
```

Use 7 phases for small changes or expand to P1..P11 for larger work. The important invariant is test-first implementation and architecture closure.

## Implementation rules

- Use `read` to inspect files; use `bash` for `rg`, `find`, `cargo`, `python`, and script execution.
- Use `edit` for precise changes, `write` for new files.
- After pre-work and before generated tests/artifacts, run `rm -rf .test && mkdir -p .test`.
- Put generated test scripts, temporary fixtures, screenshots, traces, logs, and analysis reports in `.test/`.
- Never claim a feature is complete without running tests, unless the user explicitly asks not to run them.
- If a network/API smoke test is expensive or blocked, still run local tests and explain exactly which smoke was not run and why.
- If Playwright is requested or needed, add it as a project test tool and write a browser test that asserts visible behavior, not implementation details. The Playwright test must record screenshots, handle slow LLM/API responses by polling over time, discard duplicate waiting-state images, and analyze the remaining screenshots for the expected visible result.

## Validation snippets

```bash
# Goal size
wc -c docs/goals/*-goal.md

# Rider has phases
grep -n '^### P[0-9]' docs/goals/*-rider.md

# Core verification
cargo check && cargo test
./first.sh

# Disposable generated-test workspace policy
rm -rf .test && mkdir -p .test
```
