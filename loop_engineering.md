# Loop Engineering Guide

This document describes the reusable engineering harness used in this repository: a loop of **skills → goal/rider artifacts → tests → implementation → visual verification → architecture memory**. Use it as a setup guide for a new empty project that should work the same way.

## 1. What this harness is

The harness is a disciplined agent workflow for building software in small, verifiable rounds.

Every meaningful change follows this loop:

1. Read the living architecture map.
2. Load the relevant skill instructions.
3. Create a small **goal** and detailed **rider** in `docs/goals/`.
4. Reset disposable generated artifacts in `.test/`.
5. Write a failing automated test first.
6. Implement the smallest passing slice.
7. Add browser/visual verification when the change has UI.
8. Run verification.
9. Update `architecture_decisions.md` with what changed and how to test it.

The goal is not just to ship a feature. The goal is to leave enough evidence and context that the next agent can continue without rediscovering the system.

## 2. Core repository artifacts

A project using this harness should contain these durable files and directories:

```text
.pi/skills/
  <project-goal-engineering>/SKILL.md
  <product-backlog-orchestrator>/SKILL.md     # recommended when using a PRD/backlog
  <frontend-or-agentic-helper>/SKILL.md       # CopilotKit today; replaceable tomorrow
architecture_decisions.md
docs/prd.md                                  # optional; product source of truth when a PRD exists
docs/implementation_plan.md                  # optional; PRD-to-milestone bridge
docs/tasks.md                                # optional but recommended; execution control board
docs/goals/
Cargo.toml / src/main.rs                    # MiniMax Rust proxy when using this auth/LLM base
first.sh                                    # proxy-only smoke runner
run_app.sh                                  # preferred local launcher: proxy first, then app
.test/                                       # ignored by git; disposable generated artifacts
```

Recommended `.gitignore` entry:

```gitignore
.test/
```

### `architecture_decisions.md`

The living map of the project. It should answer:

- What exists?
- Where are the important files?
- What runtime boundaries exist?
- What tests prove the system works?
- What decisions were made and why?
- What are known gaps/future candidates?

Every completed round updates this file. A feature is not done until this architecture memory is current.

### `docs/goals/`

Persistent feature-round specifications. Each round creates two files with the same timestamp:

```text
docs/goals/<YYYY-MM-DD>-<HHMM>-<project>-<topic>-goal.md
docs/goals/<YYYY-MM-DD>-<HHMM>-<project>-<topic>-rider.md
```

### `.test/`

Disposable workspace for generated evidence:

```text
.test/
  playwright/
    tests/
    screenshots/
    screenshot-analysis.json
  temporary-smoke-scripts
  captured-responses
  traces
  logs
```

At the beginning of each skill-driven round:

```bash
rm -rf .test
mkdir -p .test
```

Durable tests that should remain part of the codebase can live in normal test locations. Temporary scripts, screenshots, traces, and reports go in `.test/`.

## 3. Skills used in this repository

This repo currently uses two project-local skills as the model for the harness.

### 3.1 Goal-engineering skill

Path:

```text
.pi/skills/minimax-goal-engineering/SKILL.md
```

Purpose:

- Controls round discipline.
- Applies the goal+rider method.
- Requires tests before implementation.
- Requires `.test/` cleanup.
- Requires architecture closure before declaring done.

Use it for:

- New features.
- Bug fixes.
- Endpoint changes.
- UI changes.
- Behavior changes.
- Testability improvements.
- Any request that should leave an auditable implementation trail.

Main responsibilities:

1. Read `architecture_decisions.md` first.
2. Read recent `docs/goals/` files to understand prior rounds.
3. Identify files likely to change.
4. Identify the fastest meaningful test.
5. Create a goal+rider pair.
6. Clean `.test/`.
7. Write failing tests first.
8. Implement in small phases.
9. Run verification.
10. Update `architecture_decisions.md`.

### 3.2 CopilotKit/agentic-solutions helper skill

Path:

```text
.pi/skills/copilotkit_helper/SKILL.md
```

Purpose:

- Controls agentic UI architecture choices.
- Selects the right CopilotKit, LangGraph, shared-state, or generative-UI pattern.
- Requires Playwright screenshot verification for visible agentic UI work.

Use it when building:

- CopilotKit chat/sidebar experiences.
- Agent config UX.
- Shared state between UI and agent.
- Generative UI.
- A2UI.
- Human-in-the-loop flows.
- LangGraph-backed workflows.
- Sub-agent or coordinator/specialist systems.
- Browser-visible agentic features requiring visual verification.

The helper skill is composed with the goal-engineering skill. The goal-engineering skill defines the loop; the CopilotKit helper defines the agent/UI pattern.

## 4. How skills compose

For a normal backend or non-agentic feature, use only the goal-engineering skill.

For an agentic or CopilotKit feature, use both:

```text
goal-engineering skill = process, tests, artifacts, closure
CopilotKit helper skill = agentic architecture and UI pattern
```

The combined process is:

1. Read `architecture_decisions.md`.
2. Read the project goal-engineering skill.
3. Read the CopilotKit helper skill.
4. Read only the CopilotKit reference docs relevant to the requested feature.
5. Choose the smallest working pattern.
6. Create goal+rider files.
7. Include agent/runtime/UI/visual contracts in the rider.
8. Write tests before implementation.
9. For UI, write Playwright tests under `.test/playwright/`.
10. Update `architecture_decisions.md`.

## 5. Artifacts created by the loop

### 5.1 Goal file

The goal is short, ideally under 4000 characters. It is the spine of the round.

It should include:

- One-sentence goal.
- Files to read first.
- Engineering posture.
- High-level phases.
- Verification commands.
- Stop/done condition.

Template:

```markdown
GOAL: <one sentence>. <current pain -> desired change -> expected result>.

**Read first.**
- `<absolute path>/architecture_decisions.md`
- `<absolute path>/docs/goals/<same timestamp>-<topic>-rider.md`
- `<important source files>`
- `<important test files/scripts>`

**Posture.** Keep scope small. No unrelated refactors. No git push. Prefer tests before implementation.

**Phases.** The rider defines the implementation phases. Each non-doc phase writes or updates a named test first.

**Verification.**
- `<unit test command>`
- `<integration/smoke command>`
- If UI exists: `<Playwright command>`

**Stop when** tests pass, the user-visible behavior works, and `architecture_decisions.md` records the new behavior.
```

### 5.2 Rider file

The rider is the detailed mechanics document. It can be long.

It should include:

- Posture and boundaries.
- Touch points.
- Test contract.
- Phase-by-phase plan.
- Named tests.
- Verification commands.
- Out-of-scope items.
- Done criteria.

For agentic/CopilotKit features, the rider must also include:

- `## CopilotKit pattern`
- `## Agent/runtime contract`
- `## UI contract`
- `## Visual verification`

Template:

```markdown
# <Project> — <Topic> Rider

This rider implements `<path to goal>`.

## Posture

- Keep scope small.
- No unrelated refactors.
- No git push.
- Write tests first.

## Touch points

- `<file>` — <why it may change>.
- `<test file>` — <test to add/update>.

## Test contract

Each non-doc phase names a test and writes it before implementation.
Generated scripts, screenshots, traces, logs, and reports go under `.test/`.

## CopilotKit pattern

<Only for agentic UI features. Name the selected pattern and why.>

## Agent/runtime contract

<Endpoint, tools, state, prompts/config, ownership, error behavior.>

## UI contract

<Components, selectors, loading/error/done states.>

## Visual verification

<Playwright screenshots, polling interval, timeout, dedupe method, final assertion.>

## Phases

### P1 — Baseline and failing test
- Add the narrowest named test.
- Run it and observe failure when feasible.
Depth tests:
- `<test_name>`

### P2 — Smallest implementation
- Implement only enough to satisfy P1.
- Run the named test.
Depth tests:
- `<test_name>`

### P3 — Edge cases / integration / UI as applicable
- Add coverage across boundaries.
Depth tests:
- `<test_name>`

### P4 — Visual verification if applicable
- Write Playwright test under `.test/playwright/`.
- Capture screenshots under `.test/playwright/screenshots/`.
- Write screenshot analysis report.
Depth tests:
- `<playwright_test_name>`

### P5 — Architecture closure
- Update `architecture_decisions.md`.

## Verification

```bash
<commands>
```

## Out of scope

- <explicitly excluded work>.

## Done criteria

- Goal and rider exist.
- Tests pass.
- UI visual evidence exists when applicable.
- `architecture_decisions.md` is updated.
```

## 6. CopilotKit/agentic pattern selection

The CopilotKit helper skill keeps local reference docs beside it and selects the smallest architecture that proves the feature.

Common pattern choices:

| User need | Preferred pattern |
|---|---|
| Simple assistant conversation | `CopilotChat` or `CopilotSidebar` |
| Existing app plus docked assistant | `CopilotSidebar` |
| Agent behavior depends on knobs/persona/domain settings | Agent config |
| UI and agent co-edit the same data | Shared state |
| Agent calls backend tools and needs branded output | Tool rendering / render-tool pattern |
| Agent renders known React components | Components as tools |
| Agent state streams into the UI | State rendering |
| Reasoning/progress should be visible | Reasoning rendering |
| Known visual layout, dynamic data only | Fixed-schema A2UI |
| Agent composes layout from approved catalog | Dynamic-schema A2UI |
| Human approval/edit required | HITL / generative UI |
| Specialist delegation or multi-step workflow | Sub-agents / LangGraph |

Biases:

1. Prefer controlled UI before dynamic UI.
2. Prefer fixed-schema A2UI when the layout is known.
3. Use dynamic A2UI only when the agent genuinely needs layout composition freedom.
4. Keep shared state typed.
5. Keep runtime endpoints explicit.

## 7. Runtime and UI contracts

For every agentic feature, write the contracts before coding.

### Agent/runtime contract should define

- Runtime endpoint and protocol.
- Whether the flow calls a local proxy, a CopilotKit runtime, LangGraph, or another backend.
- Agent names and roles.
- Tool names and schemas.
- Prompt/config inputs.
- State keys and ownership:
  - UI-owned.
  - Agent-owned.
  - Shared.
- Error/refusal behavior.
- Streaming/progress behavior.

### UI contract should define

- Chat surface: `CopilotChat`, `CopilotSidebar`, custom shell, or direct proxy chat.
- Registered tools/components/renderers.
- Loading states.
- Error states.
- Completion states.
- Deterministic `data-testid` selectors.
- Where agent metadata appears.

Important lesson from this repo: agent metadata must appear in the conversation surface the user is watching, not in a disconnected panel. For multi-agent flows, render ordered cards/messages with:

- Agent name.
- Role/purpose.
- Status.
- Tools used.
- Output/artifact.

## 8. Testing by code

The loop prefers the fastest reliable test first.

Recommended order:

1. Pure unit tests for functions and transforms.
2. Integration tests for API handlers or runtime boundaries.
3. Smoke scripts for user-visible workflows.
4. Browser tests for UI behavior.
5. Visual screenshot verification for browser-visible agentic flows.

Rules:

- Each implementation phase names at least one test.
- Write or update tests before implementation.
- Watch the test fail when practical.
- Do not declare done without running relevant verification.
- If a network/API test is expensive or blocked, run local tests and state exactly what was not run.

## 9. Testing visually with Playwright

Use Playwright when the feature has visible browser behavior.

Generated Playwright files live under `.test/playwright/`:

```text
.test/playwright/
  playwright.config.mjs
  tests/<feature>.spec.mjs
  screenshots/
  screenshot-analysis.json
```

Minimum browser test shape:

1. Open the app.
2. Trigger the default user path.
3. Poll screenshots while the UI is loading or while the LLM/API call is pending.
4. Skip duplicate/effectively-identical waiting screenshots.
5. Assert final visible behavior.
6. Write `screenshot-analysis.json`.

Visual testing rules:

- Assert user-visible outcomes, not only implementation selectors.
- Use `data-testid` selectors for deterministic targeting.
- For slow LLM/API-backed screens, capture multiple screenshots over time.
- Deduplicate identical waiting-state screenshots using file hash, pixel diff, or screenshot comparison.
- Pair screenshots with DOM assertions.
- Test the real default user path, not only a manually configured happy path.

Example generated test responsibilities:

```text
open app
confirm default mode
submit request
wait/poll for agent card or result
capture screenshots
assert final visible text/cards/output
write screenshot-analysis.json
```

## 10. Example Playwright skeleton

```js
// .test/playwright/tests/feature.spec.mjs
import { test, expect } from '@playwright/test';
import crypto from 'node:crypto';
import fs from 'node:fs/promises';
import path from 'node:path';

const screenshotsDir = path.resolve('.test/playwright/screenshots');

async function fileHash(file) {
  const buf = await fs.readFile(file);
  return crypto.createHash('sha256').update(buf).digest('hex');
}

test('feature is visibly working', async ({ page }) => {
  await fs.mkdir(screenshotsDir, { recursive: true });
  const kept = [];
  const seen = new Set();

  await page.goto('/');
  await page.getByTestId('message-input').fill('Create a plan for onboarding');
  await page.getByTestId('submit-button').click();

  for (let i = 0; i < 20; i += 1) {
    const file = path.join(screenshotsDir, `feature-${i}.png`);
    await page.screenshot({ path: file, fullPage: true });
    const hash = await fileHash(file);

    if (!seen.has(hash)) {
      seen.add(hash);
      kept.push(file);
    }

    const done = await page.getByTestId('final-output').isVisible().catch(() => false);
    if (done) break;
    await page.waitForTimeout(1000);
  }

  await expect(page.getByTestId('final-output')).toBeVisible();
  await expect(page.getByText(/plan/i)).toBeVisible();

  await fs.writeFile(
    path.resolve('.test/playwright/screenshot-analysis.json'),
    JSON.stringify({ kept, assertion: 'final output is visible' }, null, 2)
  );
});
```

## 11. Setting this up in a new empty project

### Step 1 — Create directories

```bash
mkdir -p .pi/skills/project-goal-engineering
mkdir -p .pi/skills/agentic-helper
mkdir -p docs/goals
mkdir -p .test
printf '.test/\n' >> .gitignore
```

### Step 2 — Create `architecture_decisions.md`

Start with a minimal map:

```markdown
# Architecture Decisions

This is the living map for the project. Every feature round updates this file before completion.

## Project shape

- Main app: `<path>`
- Main tests: `<path>`
- Local runner: `<path>`
- Skills: `.pi/skills/`
- Goal/rider history: `docs/goals/`
- Disposable artifacts: `.test/`

## Runtime boundaries

- <browser/backend/worker/API details>

## Testing and verification

```bash
<unit test command>
<integration command>
<browser command if applicable>
```

## Current decisions

- <decision>

## Known gaps

- <gap>
```

### Step 3 — Create a project goal-engineering skill

Create `.pi/skills/project-goal-engineering/SKILL.md`:

```markdown
---
name: project-goal-engineering
description: Use for every feature, bug fix, UI change, endpoint change, testability improvement, or behavior change in this project. Creates goal+rider artifacts, writes tests first, verifies, and updates architecture_decisions.md.
license: Apache-2.0
---

# Project Goal Engineering

Use this skill automatically for any new feature, bug fix, endpoint, UI change, or behavior change.

## Required workflow

1. Read `architecture_decisions.md` first.
2. Read recent files in `docs/goals/`.
3. Identify files/functions likely to change.
4. Identify the fastest meaningful test.
5. Run:
   ```bash
   rm -rf .test && mkdir -p .test
   ```
6. Create a goal+rider pair in `docs/goals/`.
7. Write a failing test first when feasible.
8. Implement the smallest passing slice.
9. Run verification.
10. Update `architecture_decisions.md`.

## Goal/rider naming

`docs/goals/<YYYY-MM-DD>-<HHMM>-<project>-<topic>-goal.md`
`docs/goals/<YYYY-MM-DD>-<HHMM>-<project>-<topic>-rider.md`

## Done criteria

- Goal exists and is concise.
- Rider names phases and tests.
- Relevant tests pass.
- UI has Playwright verification if visible behavior changed.
- `architecture_decisions.md` is updated.
```

### Step 4 — Add an agentic helper skill if the project uses agents

Create `.pi/skills/agentic-helper/SKILL.md` and adapt it to your stack:

```markdown
---
name: agentic-helper
description: Use when designing or implementing agentic UI/backend solutions, including chat, shared state, agent config, tool rendering, HITL, A2UI, sub-agents, LangGraph, and Playwright visual verification.
license: Apache-2.0
---

# Agentic Helper

Use with the project goal-engineering skill for agentic features.

## Pattern selection

| User need | Preferred pattern |
|---|---|
| Simple assistant conversation | Chat or sidebar |
| Agent depends on knobs/config | Agent config |
| UI and agent co-edit data | Shared state |
| Backend tool with visible result | Tool rendering |
| Known layout with dynamic data | Fixed-schema UI |
| Agent-composed layout | Dynamic UI |
| Human approval needed | HITL |
| Specialist delegation | Sub-agents / graph |

## Required rider sections

- `## Agentic pattern`
- `## Agent/runtime contract`
- `## UI contract`
- `## Visual verification`

## Visual verification

For browser-visible agent features, write Playwright tests under `.test/playwright/`, capture screenshots, deduplicate waiting states, assert final visible outcomes, and write a screenshot analysis report.
```

### Step 5 — Add test scripts

Add project-specific commands to package scripts, Makefile, shell scripts, or CI. The important part is that the rider can name exact commands.

Examples:

```bash
npm test
npm run build
npx playwright test --config=.test/playwright/playwright.config.mjs
cargo test
pytest
```

### Step 6 — Run every new feature through the loop

For each feature request:

```text
read architecture_decisions.md
load relevant skill(s)
create goal+rider
clean .test
write tests
implement
run verification
update architecture_decisions.md
summarize changed files and test results
```

## 12. Completion checklist

Before saying a round is done, confirm:

- [ ] `architecture_decisions.md` was read at the start.
- [ ] Relevant skill files were read.
- [ ] Goal file exists in `docs/goals/`.
- [ ] Rider file exists in `docs/goals/`.
- [ ] `.test/` was reset for generated artifacts.
- [ ] Named tests were written/updated before implementation.
- [ ] Code tests pass.
- [ ] Browser tests pass if UI changed.
- [ ] Screenshots and screenshot analysis exist if UI/agentic visible behavior changed.
- [ ] `architecture_decisions.md` was updated at the end.
- [ ] Final response lists changed files and verification results.

## 13. Why this works

The harness makes each agent loop auditable:

- Skills encode repeatable operating rules.
- Goals keep scope small.
- Riders prevent vague implementation.
- Tests prove behavior by code.
- Playwright proves visible behavior in the browser.
- `.test/` separates generated evidence from durable source.
- `architecture_decisions.md` preserves memory for the next round.

Port these pieces into a new project and adapt the skill names, verification commands, runtime contracts, and UI patterns to that project's stack.

## 14. PRD-driven loop engineering

The loop can start from either:

1. A formal initial PRD.
2. A rough idea or user request with no PRD.
3. An existing app with only code and architecture notes.

When a PRD exists, treat the PRD as the **product source of truth** and the loop as the **execution engine**.

Recommended flow:

```text
PRD -> implementation plan -> task ledger -> goal/rider per task -> tests -> implementation -> verification -> status update
```

### PRD mode artifacts

Add these files when the project starts from a PRD:

```text
docs/prd.md
docs/implementation_plan.md
docs/tasks.md
docs/goals/
architecture_decisions.md
```

### `docs/prd.md`

The PRD should hold product intent, not implementation details.

Recommended sections:

```markdown
# Product Requirements Document

## Problem

## Users / Personas

## Goals

## Non-goals

## Core workflows

## Requirements

- PRD-001: <stable requirement>
- PRD-002: <stable requirement>

## Acceptance criteria

## Non-functional requirements

## Open questions

## Change log

| Change ID | Date | Summary | Affected Requirements | Impact |
|---|---|---|---|---|
| CHG-001 | YYYY-MM-DD | <summary> | PRD-001 | <impact> |
```

Every requirement should have a stable ID such as `PRD-001`. Never rely only on section titles because titles change. Goal/rider files, tasks, tests, and architecture notes should reference these stable IDs.

### `docs/implementation_plan.md`

This file translates product requirements into milestones and engineering slices.

Example:

```markdown
# Implementation Plan

## Milestone 1 — Project creation

Covers:
- PRD-001
- PRD-003

Tasks:
- TASK-001: Create project data model
- TASK-002: Add project creation endpoint
- TASK-003: Add project list UI
- TASK-004: Add project status badges

## Milestone 2 — Collaboration

Covers:
- PRD-002

Tasks:
- TASK-005: Invite collaborator domain model
- TASK-006: Invite collaborator endpoint
- TASK-007: Invite collaborator UI
```

### `docs/tasks.md`

This is the execution control board. It answers what is done, what is pending, what changed, and what is blocked.

Recommended format:

```markdown
# Task Ledger

| ID | Status | PRD Refs | Title | Goal/Rider | Tests | Notes |
|---|---|---|---|---|---|---|
| TASK-001 | Done | PRD-001 | Create project model | docs/goals/... | npm test model | Implemented |
| TASK-002 | In Progress | PRD-001 | Create project endpoint | docs/goals/... | npm test api | Edge cases pending |
| TASK-003 | Ready | PRD-003 | Project list UI | - | - | API exists |
| TASK-004 | Blocked | PRD-002 | Invite collaborators | - | - | Auth model unresolved |
```

Useful statuses:

```text
Pending      = identified but not ready
Ready        = can be picked up by the next loop
In Progress  = current loop is executing it
Blocked      = cannot proceed until dependency/question is resolved
Done         = task loop completed and tests passed
Verified     = done and still valid against latest PRD/milestone verification
Changed      = task was affected by PRD or architecture change and needs review
Deprecated   = no longer needed
```

A task is **Done** only when:

- Goal/rider exists.
- Named tests pass.
- Acceptance criteria are covered.
- UI/visual verification exists when applicable.
- `architecture_decisions.md` is updated.
- `docs/tasks.md` is updated.

A task is **Verified** when it has also survived a broader milestone/product check against the latest PRD.

## 15. Product/backlog orchestration skill

Yes: for PRD-driven work, add another skill. It does not replace the goal-engineering skill. It sits one level above it.

Recommended skill stack:

```text
product-backlog-orchestrator skill
  controls PRD, implementation_plan.md, tasks.md, requirement/task status, change impact

goal-engineering skill
  controls one implementation loop for one task

frontend/agentic helper skill
  controls UI/agent architecture and visual verification for frontend tasks
```

The product/backlog skill is responsible for:

- Creating `docs/prd.md` when the user has a PRD or asks for one.
- Creating `docs/implementation_plan.md` from the PRD.
- Creating and maintaining `docs/tasks.md`.
- Assigning stable IDs:
  - `PRD-001` for requirements.
  - `TASK-001` for tasks.
  - `CHG-001` for PRD changes.
- Selecting the next `Ready` task.
- Running impact analysis when the PRD changes.
- Marking tasks as `Changed`, `Deprecated`, `Blocked`, `Done`, or `Verified`.
- Ensuring each implementation task runs through the normal goal/rider loop.

### Product/backlog orchestrator skill template

Create this in a new project as `.pi/skills/product-backlog-orchestrator/SKILL.md`:

```markdown
---
name: product-backlog-orchestrator
description: Use when the project has or needs a PRD, implementation plan, task ledger, backlog control, milestone planning, PRD change impact analysis, or selection of the next task to run through goal engineering.
license: Apache-2.0
---

# Product Backlog Orchestrator

This skill controls product-to-task planning. It does not implement features directly. It prepares and maintains the PRD, implementation plan, task ledger, and change impact analysis, then hands one task at a time to the project goal-engineering skill.

## When to use

Use this skill when the user asks to:

- Start from a PRD.
- Create a PRD.
- Turn a PRD into tasks.
- Know what is done/pending/blocked.
- Change the PRD.
- Re-plan milestones.
- Pick the next task.
- Audit progress against product requirements.

If the user asks for direct implementation of a task, compose this skill with the project goal-engineering skill.

## Required files

- `docs/prd.md` — product source of truth, optional if no PRD exists.
- `docs/implementation_plan.md` — milestone and dependency plan.
- `docs/tasks.md` — task ledger/control board.
- `architecture_decisions.md` — technical source of truth.
- `docs/goals/` — implementation loop history.

## Workflow: PRD exists

1. Read `docs/prd.md` completely.
2. Ensure each requirement has a stable `PRD-###` ID.
3. Create or update `docs/implementation_plan.md`.
4. Create or update `docs/tasks.md`.
5. Each task must reference one or more PRD IDs.
6. Mark tasks as `Pending`, `Ready`, `Blocked`, `Done`, `Verified`, `Changed`, or `Deprecated`.
7. Select one small `Ready` task for implementation.
8. Hand that task to the project goal-engineering skill.

## Workflow: no PRD exists

If the user has no PRD, do not block implementation. Use one of these modes:

### Lightweight request mode

For a small change, create only a goal/rider and optionally add a minimal task entry.

### Mini-PRD mode

For a feature with product ambiguity, create a short `docs/prd.md` or an inline mini-PRD in the rider with:

- Problem
- User
- Desired behavior
- Acceptance criteria
- Out of scope

### Discovery mode

If requirements are unclear, create `docs/prd.md` with open questions and mark implementation tasks as `Blocked` until the user answers.

## PRD change workflow

When `docs/prd.md` changes:

1. Assign a `CHG-###` ID in the PRD change log.
2. Identify affected `PRD-###` requirements.
3. Identify affected `TASK-###` rows.
4. Mark affected tasks:
   - `Changed` if implementation may need updates.
   - `Deprecated` if no longer needed.
   - `Blocked` if requirements are unclear.
   - `Ready` if new work can start.
5. Update `docs/implementation_plan.md`.
6. Update `docs/tasks.md`.
7. Add an architecture note if runtime/design assumptions changed.
8. For any code changes, create a normal goal/rider implementation loop.

## Done criteria

- PRD requirements have stable IDs.
- Implementation plan maps milestones to PRD IDs.
- Task ledger maps tasks to PRD IDs and statuses.
- PRD changes have change IDs and impact analysis.
- The next implementation task is small enough for one goal/rider loop.
```

## 16. Working without a PRD

Sometimes there will be no PRD. That is fine. Do not force heavy process for a tiny fix.

Use three modes:

### 16.1 Lightweight loop mode

Use for:

- Small bug fixes.
- Small refactors.
- Test improvements.
- Minor UI copy changes.

Artifacts:

```text
architecture_decisions.md
docs/goals/<timestamp>-<topic>-goal.md
docs/goals/<timestamp>-<topic>-rider.md
.test/
```

No `docs/prd.md`, `docs/implementation_plan.md`, or `docs/tasks.md` is required.

### 16.2 Mini-PRD mode

Use when the feature is product-facing but still small.

Put a short mini-PRD inside the rider:

```markdown
## Mini-PRD

Problem: <what hurts>
User: <who uses this>
Desired behavior: <what should happen>
Acceptance criteria:
- <criterion>
Out of scope:
- <excluded work>
```

If this mini-PRD grows, promote it into `docs/prd.md` and create `docs/tasks.md`.

### 16.3 Full PRD/backlog mode

Use when:

- The feature spans multiple tasks.
- There are multiple users/personas.
- There are dependencies/milestones.
- You need progress tracking.
- You expect PRD changes.

Artifacts:

```text
docs/prd.md
docs/implementation_plan.md
docs/tasks.md
docs/goals/
architecture_decisions.md
```

## 17. Safely changing the PRD

Changing the initial PRD should be a controlled loop, not an invisible edit.

Use this process:

```text
PRD edit -> CHG ID -> impact analysis -> task ledger updates -> implementation loops for affected tasks
```

### PRD change checklist

- [ ] Add a `CHG-###` row to the PRD change log.
- [ ] List affected `PRD-###` IDs.
- [ ] Search `docs/tasks.md` for tasks referencing those PRD IDs.
- [ ] Search `docs/goals/` for riders referencing those PRD IDs.
- [ ] Mark affected tasks as `Changed`, `Deprecated`, `Blocked`, or `Ready`.
- [ ] Add new tasks if required.
- [ ] Update milestone ordering in `docs/implementation_plan.md`.
- [ ] Update `architecture_decisions.md` if technical assumptions changed.
- [ ] Run a normal goal/rider loop for any code changes.

### Do not rewrite history

Do not edit old goal/rider files to pretend they described the new plan. Old goals/riders are historical evidence. If the PRD changes, create a new change/reconciliation rider or a new implementation rider.

Recommended reconciliation task:

```text
TASK-PRD-CHANGE-001: Reconcile CHG-001 into implementation plan and task ledger
```

That task may be documentation-only if no code changes are needed.

## 18. Frontend and agentic skills: CopilotKit today, replaceable tomorrow

This repository currently uses a CopilotKit-oriented frontend/agentic helper skill:

```text
.pi/skills/copilotkit_helper/SKILL.md
```

It is the right default **for this repo today** because the current frontend uses:

- Vite + React.
- CopilotKit provider wiring.
- CopilotKit packages.
- A chat/agent surface.
- Browser-local LangGraph-style coordinator/specialist routing.
- Playwright visual verification.

But the loop-engineering harness should not be tied forever to CopilotKit. The reusable principle is:

```text
Use a frontend/domain helper skill for the UI framework and agent runtime you are actually using.
```

### Current frontend helper responsibilities

The current CopilotKit helper skill decides:

- Whether to use chat, sidebar, custom shell, direct proxy chat, or CopilotKit runtime.
- Whether to use agent config.
- Whether to use shared UI/agent state.
- Whether to use component/tool rendering.
- Whether to use state rendering or reasoning rendering.
- Whether to use fixed-schema A2UI or dynamic-schema A2UI.
- Whether HITL is needed.
- Whether a LangGraph/sub-agent workflow is appropriate.
- What Playwright visual verification is required.

### If tomorrow the frontend is not CopilotKit

Replace or supplement the CopilotKit helper with a framework-specific skill, for example:

```text
.pi/skills/nextjs-frontend-helper/SKILL.md
.pi/skills/remix-frontend-helper/SKILL.md
.pi/skills/sveltekit-frontend-helper/SKILL.md
.pi/skills/vue-frontend-helper/SKILL.md
.pi/skills/flutter-frontend-helper/SKILL.md
.pi/skills/native-mobile-helper/SKILL.md
.pi/skills/langgraph-backend-helper/SKILL.md
```

The replacement skill should still require the same kinds of contracts:

- Frontend architecture pattern.
- Runtime/API contract.
- State ownership.
- Component contract.
- Loading/error/done states.
- Deterministic selectors or accessibility locators.
- Browser/device visual verification.
- Screenshot/artifact policy.

### Framework-neutral frontend rider sections

For any frontend stack, include these rider sections:

```markdown
## Frontend pattern

<React/CopilotKit, Next.js route/server action, Remix loader/action, Vue composable, SvelteKit form action, native screen, etc.>

## Runtime contract

<API endpoints, payloads, streaming, auth, agent tools, backend boundaries.>

## State contract

<Local state, server state, shared agent state, cache keys, ownership.>

## UI contract

<Components/screens, accessibility labels, deterministic test selectors, loading/error/done states.>

## Visual verification

<Playwright/Appium/screenshot flow, polling, dedupe, final visible assertion.>
```

### CopilotKit-specific rider sections for now

When using the current CopilotKit helper, keep these sections:

```markdown
## CopilotKit pattern

## Agent/runtime contract

## UI contract

## Visual verification
```

If you switch away from CopilotKit, rename `## CopilotKit pattern` to `## Frontend pattern` or `## Agentic pattern`, but preserve the same level of explicitness.

## 19. MiniMax proxy foundation

For MiniMax-powered projects, the loop should include a durable **MiniMax Rust OAuth proxy** as the local LLM boundary. This proxy is what makes the app/framework independent: React, CopilotKit, Pydantic AI, LangGraph, Temporal workers, Python services, or any other client can call the same local OpenAI-compatible endpoint instead of each app owning MiniMax authentication.

### 19.1 Required proxy shape

The proxy should be a small Rust binary with:

```text
Cargo.toml
src/main.rs
first.sh
run_app.sh
```

Current reference shape in this repo:

- Rust binary crate: `minimax_proxy`.
- HTTP framework: Axum.
- Async runtime: Tokio.
- OAuth credential storage: MiniMax CLI-compatible `~/.mmx/config.json` under the `oauth` key.
- Browser-safe CORS when a frontend will call it directly.
- Local default port: `8080`.

Core endpoints:

```text
GET  /health
GET  /auth/status
POST /auth/refresh
POST /auth/token
POST /v1/chat/completions
```

The most important endpoint is:

```text
POST http://localhost:8080/v1/chat/completions
```

It should accept OpenAI-style chat payloads and translate them to MiniMax upstream calls using the authenticated OAuth token.

### 19.2 Authentication requirement

Before launching the app, the proxy must be authenticated or able to complete authentication.

Expected behavior:

1. Start the Rust proxy.
2. Load existing OAuth credentials from `~/.mmx/config.json`.
3. If credentials are missing or expiring, start MiniMax OAuth device-code/browser login.
4. Do not launch the application until the proxy responds successfully to `/health`.
5. Optionally also check `/auth/status` and require an authenticated state before starting expensive app workers.

This gives every frontend/backend framework the same invariant:

```text
If the app starts, a local MiniMax-compatible LLM endpoint is already available.
```

### 19.3 Framework compatibility contract

Every framework integration should prove it can call the proxy. Add a rider section like this for any framework task:

```markdown
## MiniMax proxy contract

- Proxy base URL: `http://localhost:8080` or `$MINIMAX_PROXY_BASE_URL`.
- Chat endpoint: `POST /v1/chat/completions`.
- Request format: OpenAI-compatible chat completion payload.
- Response format: OpenAI-compatible chat completion response.
- Auth is handled by the Rust proxy, not by the app/framework.
- The framework must not require a MiniMax API key in browser/client code.
- Local launcher starts the proxy first and launches the app only after proxy health/auth succeeds.
```

Compatibility examples:

| Framework/runtime | How it should use the proxy |
|---|---|
| React/Vite direct chat | Browser posts to `${VITE_PROXY_BASE_URL}/v1/chat/completions`; CORS must allow the app origin. |
| CopilotKit | CopilotKit UI/runtime calls a route/tool that ultimately uses the proxy as the LLM boundary. |
| LangGraph through CopilotKit | Graph nodes/specialist agents call the proxy directly or through the CopilotKit runtime; graph events are rendered in the UI. |
| Pydantic AI | Configure an OpenAI-compatible provider/client with `base_url=http://localhost:8080/v1` and a placeholder API key if the library requires one. |
| LangGraph Python/backend | Use an OpenAI-compatible chat client pointed at `http://localhost:8080/v1`; keep MiniMax OAuth out of graph node code. |
| Temporal.io | Activities/workflows should call an activity wrapper that uses the proxy; avoid direct LLM calls inside deterministic workflow code. |
| Next.js/Remix/SvelteKit | Server routes/actions call the proxy; browser code can call app routes instead of the proxy if you want to hide local topology. |
| CLI/batch jobs | Use `http://localhost:8080/v1/chat/completions` directly or through an OpenAI-compatible SDK. |

The goal is not to make every framework special. The goal is to keep the MiniMax boundary stable and make every framework prove that it works with that boundary.

### 19.4 Proxy verification

Minimum proxy verification commands:

```bash
cargo check
cargo test
./first.sh
```

`first.sh` should:

1. Stop stale listeners on the proxy port.
2. Start `cargo run` for the proxy.
3. Wait for `GET /health`.
4. Run a smoke client against `/v1/chat/completions`.
5. Stop the proxy on exit.

For each new framework integration, add at least one smoke or automated test that proves the framework can reach the proxy. Examples:

```bash
# React/CopilotKit
cd app && npm test && npm run build

# Browser visual check
cd app && npx playwright test --config=../.test/playwright/playwright.config.mjs

# Python/Pydantic AI or LangGraph Python
pytest

# Temporal workers
cargo test
npm test
pytest
# plus a local worker smoke if the project has one
```

### 19.5 Preferred `run_app.sh` launcher pattern

At the end of each app setup, create a shell script that launches the MiniMax proxy first and then launches the app only when the proxy is ready/authenticated.

Use this behavior:

1. Stop stale proxy/app listeners.
2. Start MiniMax proxy with `cargo run`.
3. Wait for `GET /health`.
4. Optionally verify `GET /auth/status` says authenticated.
5. Install app dependencies if needed.
6. Start the app with the correct proxy base URL environment variable.
7. Print the app URL.
8. Clean up both proxy and app processes on Ctrl+C/exit.

Generic template:

```bash
#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PORT="${PORT:-8080}"
APP_PORT="${APP_PORT:-5173}"
LOG_DIR="${LOG_DIR:-/tmp/minimax_app}"
PROXY_LOG="$LOG_DIR/proxy-${PORT}.log"
APP_LOG="$LOG_DIR/app-${APP_PORT}.log"

mkdir -p "$LOG_DIR"
cd "$ROOT_DIR"

cleanup() {
  trap - EXIT INT TERM
  [[ -z "${APP_PID:-}" ]] || kill "$APP_PID" 2>/dev/null || true
  [[ -z "${PROXY_PID:-}" ]] || kill "$PROXY_PID" 2>/dev/null || true

  local app_port_pids proxy_port_pids
  app_port_pids="$(lsof -tiTCP:"$APP_PORT" -sTCP:LISTEN || true)"
  proxy_port_pids="$(lsof -tiTCP:"$PORT" -sTCP:LISTEN || true)"
  [[ -z "$app_port_pids" ]] || kill $app_port_pids 2>/dev/null || true
  [[ -z "$proxy_port_pids" ]] || kill $proxy_port_pids 2>/dev/null || true
}
trap cleanup EXIT INT TERM

stop_port() {
  local port="$1"
  local pid
  pid="$(lsof -tiTCP:"$port" -sTCP:LISTEN || true)"
  [[ -z "$pid" ]] || kill $pid || true
}

stop_port "$PORT"
stop_port "$APP_PORT"

printf 'Starting MiniMax proxy on http://localhost:%s ...\n' "$PORT"
PORT="$PORT" cargo run >"$PROXY_LOG" 2>&1 &
PROXY_PID=$!

for _ in {1..90}; do
  if curl -fsS "http://localhost:$PORT/health" >/dev/null 2>&1; then
    echo "Proxy health ready."
    break
  fi
  if ! kill -0 "$PROXY_PID" 2>/dev/null; then
    echo "Proxy exited early. Log output:"
    tail -100 "$PROXY_LOG" || true
    exit 1
  fi
  sleep 1
done

if ! curl -fsS "http://localhost:$PORT/health" >/dev/null 2>&1; then
  echo "Timed out waiting for proxy. Log output:"
  tail -100 "$PROXY_LOG" || true
  exit 1
fi

# Optional stricter auth check. Adapt JSON parsing to your /auth/status shape.
# if ! curl -fsS "http://localhost:$PORT/auth/status" | grep -q 'true'; then
#   echo "Proxy is healthy but not authenticated. Check MiniMax OAuth login."
#   tail -100 "$PROXY_LOG" || true
#   exit 1
# fi

# Framework-specific dependency install and launch.
# React/Vite example:
if [[ -d "$ROOT_DIR/app" && ! -d "$ROOT_DIR/app/node_modules" ]]; then
  (cd "$ROOT_DIR/app" && npm install)
fi

printf 'Starting app on http://localhost:%s ...\n' "$APP_PORT"
(
  cd "$ROOT_DIR/app"
  APP_PORT="$APP_PORT" \
  VITE_PROXY_BASE_URL="http://localhost:$PORT" \
  MINIMAX_PROXY_BASE_URL="http://localhost:$PORT" \
  npm run dev -- --port "$APP_PORT"
) >"$APP_LOG" 2>&1 &
APP_PID=$!

for _ in {1..60}; do
  if curl -fsS "http://localhost:$APP_PORT" >/dev/null 2>&1; then
    echo "App ready."
    echo "Open: http://localhost:$APP_PORT"
    wait "$APP_PID"
    exit $?
  fi
  if ! kill -0 "$APP_PID" 2>/dev/null; then
    echo "App exited early. Log output:"
    tail -100 "$APP_LOG" || true
    exit 1
  fi
  sleep 1
done

echo "Timed out waiting for app. Log output:"
tail -100 "$APP_LOG" || true
exit 1
```

For non-React apps, keep the same proxy-first/auth-first structure and replace only the dependency/install/app launch block.

### 19.6 Add MiniMax proxy to planning artifacts

When using a PRD/backlog, the initial plan should include proxy setup as explicit early tasks unless the project already has a working proxy.

Example tasks:

```markdown
| ID | Status | PRD Refs | Title | Goal/Rider | Tests | Notes |
|---|---|---|---|---|---|---|
| TASK-001 | Ready | INFRA-001 | Set up MiniMax Rust OAuth proxy | - | cargo test, ./first.sh | Required before LLM app work |
| TASK-002 | Pending | INFRA-002 | Add framework adapter to proxy | - | framework smoke | Pydantic AI/LangGraph/CopilotKit/etc. |
| TASK-003 | Pending | INFRA-003 | Add run_app.sh proxy-first launcher | - | shell smoke | Launch proxy then app |
```

Add an architecture section like:

```markdown
## MiniMax proxy boundary

- Local proxy URL: `http://localhost:8080`.
- Apps call the proxy, not MiniMax directly.
- OAuth credentials live in `~/.mmx/config.json`.
- Framework adapters must prove compatibility with `/v1/chat/completions`.
- `run_app.sh` starts proxy first and app second.
```

## 20. Recommended operating modes

Use the lightest mode that gives enough control.

| Situation | Use |
|---|---|
| Tiny bug fix | Goal-engineering skill only |
| Small feature, no PRD | Goal-engineering skill with mini-PRD in rider |
| Multi-task product feature | Product/backlog skill + goal-engineering skill |
| CopilotKit/agentic UI | Product/backlog if needed + goal-engineering + CopilotKit helper |
| Non-CopilotKit frontend | Product/backlog if needed + goal-engineering + framework frontend helper |
| PRD changed | Product/backlog skill, impact analysis, then goal-engineering for affected tasks |

The important invariant is that code changes always happen through the implementation loop, while product/backlog changes happen through the orchestration layer.

## 21. Empty-project bootstrap recipe

This section is the practical starting point for a brand-new empty project. The desired behavior is:

```text
empty project + PRD + bootstrap prompt
  -> creates skills
  -> creates architecture_decisions.md
  -> creates docs/prd.md, implementation_plan.md, tasks.md
  -> creates MiniMax Rust proxy
  -> creates first.sh and run_app.sh
  -> creates framework/app skeleton
  -> verifies proxy and app
  -> leaves the project ready to implement TASK-001 through the loop
```

### 21.1 One-shot bootstrap prompt

Use a prompt like this in a new empty project:

```text
Set up this empty project with the loop-engineering harness from loop_engineering.md.

Inputs:
- I will provide an initial PRD below. If the PRD is incomplete, create docs/prd.md with Open Questions and mark blocked tasks.
- Use MiniMax Rust OAuth proxy as the local LLM boundary.
- Use <FRAMEWORK> for the app: <React/CopilotKit | Next.js | Pydantic AI | LangGraph Python | Temporal.io | other>.

Required output in the repo:
1. Create .pi/skills/project-goal-engineering/SKILL.md.
2. Create .pi/skills/product-backlog-orchestrator/SKILL.md.
3. Create a frontend/framework helper skill for <FRAMEWORK>.
4. Create architecture_decisions.md.
5. Create docs/prd.md with stable PRD-### IDs.
6. Create docs/implementation_plan.md.
7. Create docs/tasks.md with statuses and PRD refs.
8. Create a Rust MiniMax proxy: Cargo.toml and src/main.rs.
9. Create first.sh to smoke-test the proxy.
10. Create run_app.sh that launches the proxy first, waits for health/auth, then launches the app.
11. Create the minimal app skeleton for <FRAMEWORK> and make it call the proxy.
12. Add .test/ to .gitignore.
13. Run the fastest verification that is possible locally.

Rules:
- Do not implement the whole PRD at once.
- Create the backlog and make TASK-001 the smallest infrastructure task.
- For each implementation task, use goal+rider under docs/goals/.
- Put generated artifacts under .test/.
- Update architecture_decisions.md before declaring bootstrap done.

PRD:
<paste PRD here>
```

### 21.2 Bootstrap task ordering

In the generated `docs/tasks.md`, prefer this order:

```markdown
| ID | Status | PRD Refs | Title | Goal/Rider | Tests | Notes |
|---|---|---|---|---|---|---|
| TASK-001 | Ready | INFRA-001 | Create loop-engineering skills and architecture docs | - | file checks | Harness foundation |
| TASK-002 | Ready | INFRA-002 | Create MiniMax Rust OAuth proxy | - | cargo check, cargo test | LLM boundary |
| TASK-003 | Ready | INFRA-003 | Create proxy smoke script first.sh | - | ./first.sh | Proves proxy works |
| TASK-004 | Ready | INFRA-004 | Create app skeleton for selected framework | - | framework build/test | App foundation |
| TASK-005 | Ready | INFRA-005 | Create run_app.sh proxy-first launcher | - | shell smoke | Preferred local startup |
| TASK-006 | Pending | PRD-001 | First product feature slice | - | TBD | Starts product work |
```

The bootstrap itself may create these infrastructure tasks and complete some of them immediately. Product tasks should remain `Pending` or `Ready`, not silently implemented.

## 22. Copy-paste skill templates for new projects

These templates are intentionally generic. Replace `<PROJECT_NAME>`, commands, and framework details.

### 22.1 `.pi/skills/project-goal-engineering/SKILL.md`

```markdown
---
name: project-goal-engineering
description: Use for every feature, bug fix, endpoint, UI change, agent workflow, testability improvement, or behavior change. Creates goal+rider artifacts, writes tests first, verifies, and updates architecture_decisions.md.
license: Apache-2.0
---

# Project Goal Engineering

Use this skill for every implementation round.

## Read first

1. `architecture_decisions.md`
2. `docs/tasks.md` if it exists
3. `docs/prd.md` if it exists
4. Recent files in `docs/goals/`
5. Source and tests likely to change

## Round workflow

1. Understand the requested task and its PRD/task IDs.
2. Run `rm -rf .test && mkdir -p .test` before generated artifacts.
3. Create one goal and one rider in `docs/goals/`.
4. Write or update a named failing test first when feasible.
5. Implement the smallest passing slice.
6. Run verification.
7. Update `architecture_decisions.md`.
8. Update `docs/tasks.md` if it exists.

## Goal/rider naming

`docs/goals/<YYYY-MM-DD>-<HHMM>-<project>-<topic>-goal.md`
`docs/goals/<YYYY-MM-DD>-<HHMM>-<project>-<topic>-rider.md`

## Required rider sections

- Posture
- PRD/task coverage, if applicable
- Touch points
- Test contract
- Phases
- Verification
- Out of scope
- Done criteria

For frontend/agentic work, add:

- Frontend/agentic pattern
- Runtime contract
- State contract
- UI contract
- Visual verification

For MiniMax-powered work, add:

- MiniMax proxy contract

## Verification defaults

```bash
cargo check
cargo test
./first.sh
```

Add framework-specific commands, for example:

```bash
cd app && npm test && npm run build
pytest
npx playwright test --config=.test/playwright/playwright.config.mjs
```

## Done criteria

- Goal/rider exist.
- Named tests pass or blocked tests are explicitly explained.
- MiniMax proxy compatibility is preserved for LLM work.
- UI has visual verification when visible behavior changed.
- `architecture_decisions.md` is updated.
- `docs/tasks.md` status is updated when present.
```

### 22.2 `.pi/skills/product-backlog-orchestrator/SKILL.md`

```markdown
---
name: product-backlog-orchestrator
description: Use when the project has or needs a PRD, implementation plan, task ledger, progress tracking, PRD change control, or next-task selection.
license: Apache-2.0
---

# Product Backlog Orchestrator

This skill owns product-to-engineering planning. It does not directly implement features; it creates and maintains the PRD, implementation plan, and task ledger, then hands one small task to the goal-engineering skill.

## Files

- `docs/prd.md`
- `docs/implementation_plan.md`
- `docs/tasks.md`
- `architecture_decisions.md`
- `docs/goals/`

## Workflows

### Bootstrap from PRD

1. Read the user's PRD.
2. Create `docs/prd.md` with stable `PRD-###` requirement IDs.
3. Add `INFRA-###` requirements for necessary infrastructure such as MiniMax proxy, launcher, and app skeleton.
4. Create `docs/implementation_plan.md` with milestones.
5. Create `docs/tasks.md` with task IDs, statuses, PRD refs, tests, and notes.
6. Mark the smallest infrastructure task `Ready`.
7. Do not implement all product tasks in one pass.

### No PRD

Use lightweight mode for small fixes. For ambiguous product work, create a mini-PRD in the rider or create `docs/prd.md` with Open Questions.

### PRD change

1. Add `CHG-###` to the PRD change log.
2. Identify affected `PRD-###` IDs.
3. Find affected tasks and riders.
4. Mark tasks `Changed`, `Deprecated`, `Blocked`, or `Ready`.
5. Update the implementation plan and task ledger.
6. Create implementation goal/rider only for actual code changes.

## Status rules

- Pending: known but not ready.
- Ready: next loop can start.
- In Progress: actively being implemented.
- Blocked: waiting for information/dependency.
- Done: task tests passed and architecture was updated.
- Verified: still valid against latest PRD/milestone check.
- Changed: affected by PRD/architecture change.
- Deprecated: no longer needed.
```

### 22.3 Framework helper skill template

Create one helper for the current frontend/runtime. For this repo today, that is CopilotKit. Tomorrow it may be Next.js, Pydantic AI, LangGraph Python, Temporal.io, or another framework.

```markdown
---
name: framework-helper
description: Use for framework-specific architecture, runtime wiring, state contracts, UI contracts, and visual verification.
license: Apache-2.0
---

# Framework Helper

Use with project-goal-engineering for tasks involving the selected app/runtime framework.

## Required contracts in riders

- Frontend/runtime pattern
- MiniMax proxy contract
- State contract
- UI/API contract
- Loading/error/done behavior
- Verification command
- Visual verification for browser/mobile UI

## MiniMax proxy invariant

The framework must call the local MiniMax proxy instead of handling MiniMax credentials itself.

- Browser code uses `VITE_PROXY_BASE_URL`, `NEXT_PUBLIC_PROXY_BASE_URL`, or an app route.
- Server/worker code uses `MINIMAX_PROXY_BASE_URL`.
- OpenAI-compatible clients should use `base_url=http://localhost:8080/v1`.
- Use a placeholder API key only if the SDK requires one; authentication is handled by the proxy.

## Visual verification

Browser-visible changes require Playwright screenshots under `.test/playwright/screenshots/` and a screenshot analysis report.
```

## 23. MiniMax Rust proxy starter code

Use this starter to create a working proxy quickly. It is intentionally compact. Production rounds can harden streaming, retries, token refresh, structured error mapping, and provider-specific edge cases.

### 23.1 `Cargo.toml`

```toml
[package]
name = "minimax_proxy"
version = "0.1.0"
edition = "2021"
description = "Local MiniMax OAuth proxy with OpenAI-compatible chat endpoint"

[dependencies]
axum = { version = "0.8", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
reqwest = { version = "0.12", features = ["json"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
anyhow = "1"
dirs = "5"
base64 = "0.22"
sha2 = "0.10"
url = "2"
rand = "0.8"
chrono = { version = "0.4", default-features = false, features = ["clock"] }
tower-http = { version = "0.6", features = ["cors"] }
```

### 23.2 `src/main.rs` starter

This version supports:

- `GET /health`
- `GET /auth/status`
- `POST /v1/chat/completions`
- Loading OAuth token from `~/.mmx/config.json` or `$MMX_CONFIG_DIR/config.json`
- OpenAI-compatible request/response at the app boundary
- MiniMax Anthropic-compatible upstream call
- CORS for browser apps

```rust
use axum::{
    extract::State,
    http::{HeaderMap, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    client: reqwest::Client,
    api_base: String,
    default_model: String,
    token: Arc<Option<String>>,
}

#[derive(Debug, Deserialize)]
struct ChatRequest {
    messages: Vec<ChatMessage>,
    model: Option<String>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    stream: Option<bool>,
    system: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ChatMessage {
    role: String,
    content: Value,
}

#[derive(Debug, Deserialize)]
struct MmxConfig {
    oauth: Option<MmxOAuth>,
}

#[derive(Debug, Deserialize)]
struct MmxOAuth {
    access_token: Option<String>,
    expires_at: Option<i64>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8080);

    let api_base = std::env::var("MINIMAX_API_BASE")
        .unwrap_or_else(|_| "https://api.minimax.io".to_string());
    let default_model = std::env::var("MINIMAX_MODEL")
        .unwrap_or_else(|_| "MiniMax-M2.7".to_string());

    let token = load_access_token().ok();
    if token.is_none() {
        tracing::warn!("No MiniMax OAuth token found. Run MiniMax login or use the full OAuth implementation from the reference repo.");
    }

    let state = AppState {
        client: reqwest::Client::new(),
        api_base,
        default_model,
        token: Arc::new(token),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/auth/status", get(auth_status))
        .route("/v1/chat/completions", post(chat_completions))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("MiniMax proxy listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!({
        "ok": true,
        "authenticated": state.token.is_some(),
        "chat_endpoint": "/v1/chat/completions"
    }))
}

async fn auth_status(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!({
        "authenticated": state.token.is_some()
    }))
}

async fn chat_completions(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<ChatRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let token = state
        .token
        .as_ref()
        .clone()
        .or_else(|| bearer_from_headers(&headers))
        .ok_or_else(|| json_error(StatusCode::UNAUTHORIZED, "missing MiniMax OAuth token"))?;

    let model = req.model.unwrap_or_else(|| state.default_model.clone());
    let max_tokens = req.max_tokens.unwrap_or(1024);
    let mut system_parts: Vec<String> = Vec::new();
    if let Some(system) = req.system {
        system_parts.push(system);
    }

    let mut upstream_messages = Vec::new();
    for msg in req.messages {
        if msg.role == "system" {
            system_parts.push(content_to_text(&msg.content));
        } else {
            upstream_messages.push(json!({
                "role": msg.role,
                "content": content_to_text(&msg.content)
            }));
        }
    }

    let mut body = json!({
        "model": model,
        "max_tokens": max_tokens,
        "stream": false,
        "messages": upstream_messages
    });

    if !system_parts.is_empty() {
        body["system"] = Value::String(system_parts.join("\n\n"));
    }
    if let Some(t) = req.temperature {
        body["temperature"] = json!(t);
    }
    if let Some(top_p) = req.top_p {
        body["top_p"] = json!(top_p);
    }
    if req.stream.unwrap_or(false) {
        tracing::warn!("stream=true requested; starter proxy returns non-streaming responses");
    }

    let url = format!("{}/anthropic/v1/messages", state.api_base.trim_end_matches('/'));
    let upstream = state
        .client
        .post(url)
        .header("x-api-key", token)
        .json(&body)
        .send()
        .await
        .map_err(|e| json_error(StatusCode::BAD_GATEWAY, &format!("MiniMax request failed: {e}")))?;

    let status = upstream.status();
    let value: Value = upstream
        .json()
        .await
        .map_err(|e| json_error(StatusCode::BAD_GATEWAY, &format!("MiniMax response decode failed: {e}")))?;

    if !status.is_success() {
        return Err(json_error(
            StatusCode::BAD_GATEWAY,
            &format!("MiniMax upstream error: {value}"),
        ));
    }

    let text = extract_anthropic_text(&value);
    let response = json!({
        "id": value.get("id").and_then(Value::as_str).unwrap_or("chatcmpl-minimax"),
        "object": "chat.completion",
        "created": chrono::Utc::now().timestamp(),
        "model": model,
        "choices": [{
            "index": 0,
            "message": { "role": "assistant", "content": text },
            "finish_reason": "stop"
        }]
    });

    Ok(Json(response))
}

fn json_error(status: StatusCode, message: &str) -> (StatusCode, Json<Value>) {
    (status, Json(json!({ "error": { "message": message } })))
}

fn bearer_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(ToString::to_string)
}

fn content_to_text(content: &Value) -> String {
    if let Some(s) = content.as_str() {
        return s.to_string();
    }
    if let Some(parts) = content.as_array() {
        return parts
            .iter()
            .filter_map(|p| p.get("text").and_then(Value::as_str))
            .collect::<Vec<_>>()
            .join("\n");
    }
    content.to_string()
}

fn extract_anthropic_text(value: &Value) -> String {
    value
        .get("content")
        .and_then(Value::as_array)
        .map(|parts| {
            parts
                .iter()
                .filter_map(|p| p.get("text").and_then(Value::as_str))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| value.to_string())
}

fn config_path() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("MMX_CONFIG_DIR") {
        return Some(PathBuf::from(dir).join("config.json"));
    }
    dirs::home_dir().map(|home| home.join(".mmx").join("config.json"))
}

fn load_access_token() -> anyhow::Result<String> {
    let path = config_path().ok_or_else(|| anyhow::anyhow!("could not resolve home directory"))?;
    let raw = std::fs::read_to_string(&path)?;
    let config: MmxConfig = serde_json::from_str(&raw)?;
    let oauth = config.oauth.ok_or_else(|| anyhow::anyhow!("missing oauth in {path:?}"))?;

    if let Some(expires_at) = oauth.expires_at {
        let now = chrono::Utc::now().timestamp();
        if expires_at < now {
            tracing::warn!("MiniMax OAuth token appears expired; use full OAuth refresh implementation or re-login");
        }
    }

    oauth
        .access_token
        .ok_or_else(|| anyhow::anyhow!("missing access_token in {path:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_string_content() {
        assert_eq!(content_to_text(&json!("hello")), "hello");
    }

    #[test]
    fn extracts_array_text_content() {
        let value = json!([{ "type": "text", "text": "a" }, { "type": "text", "text": "b" }]);
        assert_eq!(content_to_text(&value), "a\nb");
    }
}
```

Important note: the starter above loads an existing token. If you want fully automatic device-code OAuth inside the proxy, copy the complete OAuth implementation from this repository's `src/main.rs` rather than only the starter. In a bootstrap round, create a task named `Implement full MiniMax device-code OAuth` if the empty project does not already inherit it.

## 24. Smoke scripts and app launcher code

### 24.1 `test_client.py`

```python
#!/usr/bin/env python3
import json
import os
import sys
import urllib.request

base = os.environ.get("MINIMAX_PROXY_BASE_URL", "http://localhost:8080")

payload = {
    "model": os.environ.get("MINIMAX_MODEL", "MiniMax-M2.7"),
    "messages": [
        {"role": "system", "content": "You are a concise smoke-test assistant."},
        {"role": "user", "content": "Reply with exactly: proxy ok"},
    ],
    "max_tokens": 32,
}

req = urllib.request.Request(
    f"{base}/v1/chat/completions",
    data=json.dumps(payload).encode("utf-8"),
    headers={"content-type": "application/json"},
    method="POST",
)

try:
    with urllib.request.urlopen(req, timeout=60) as resp:
        body = json.loads(resp.read().decode("utf-8"))
except Exception as exc:
    print(f"Proxy smoke failed: {exc}", file=sys.stderr)
    sys.exit(1)

content = body.get("choices", [{}])[0].get("message", {}).get("content", "")
print(content)
if not content:
    print("Proxy smoke failed: empty assistant content", file=sys.stderr)
    sys.exit(1)
```

### 24.2 `first.sh`

```bash
#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PORT="${PORT:-8080}"
PYTHON="${PYTHON:-python3}"
LOG_FILE="${LOG_FILE:-/tmp/minimax_proxy_${PORT}.log}"

cd "$ROOT_DIR"

EXISTING_PID="$(lsof -tiTCP:"$PORT" -sTCP:LISTEN || true)"
if [[ -n "$EXISTING_PID" ]]; then
  echo "Stopping existing process on port $PORT: $EXISTING_PID"
  kill $EXISTING_PID || true
  sleep 1
fi

echo "Starting MiniMax proxy on port $PORT..."
PORT="$PORT" cargo run >"$LOG_FILE" 2>&1 &
PROXY_PID=$!

cleanup() {
  if kill -0 "$PROXY_PID" 2>/dev/null; then
    echo "Stopping proxy (pid $PROXY_PID)..."
    kill "$PROXY_PID" || true
  fi
}
trap cleanup EXIT INT TERM

echo "Waiting for proxy health check..."
for _ in {1..90}; do
  if curl -fsS "http://localhost:$PORT/health" >/dev/null 2>&1; then
    echo "Proxy is ready. Log: $LOG_FILE"
    break
  fi
  if ! kill -0 "$PROXY_PID" 2>/dev/null; then
    echo "Proxy exited early. Log output:"
    tail -100 "$LOG_FILE" || true
    exit 1
  fi
  sleep 1
done

if ! curl -fsS "http://localhost:$PORT/health" >/dev/null 2>&1; then
  echo "Timed out waiting for proxy. Log output:"
  tail -100 "$LOG_FILE" || true
  exit 1
fi

MINIMAX_PROXY_BASE_URL="http://localhost:$PORT" "$PYTHON" "$ROOT_DIR/test_client.py" "$@"
```

### 24.3 `run_app.sh`

Use the `run_app.sh` template in section 19.5 and adapt only the app launch block. Examples:

React/Vite:

```bash
(
  cd "$ROOT_DIR/app"
  APP_PORT="$APP_PORT" \
  VITE_PROXY_BASE_URL="http://localhost:$PORT" \
  npm run dev -- --port "$APP_PORT"
) >"$APP_LOG" 2>&1 &
```

Next.js:

```bash
(
  cd "$ROOT_DIR/app"
  PORT="$APP_PORT" \
  MINIMAX_PROXY_BASE_URL="http://localhost:$PORT" \
  NEXT_PUBLIC_PROXY_BASE_URL="http://localhost:$PORT" \
  npm run dev
) >"$APP_LOG" 2>&1 &
```

Python/Pydantic AI API:

```bash
(
  cd "$ROOT_DIR"
  MINIMAX_PROXY_BASE_URL="http://localhost:$PORT" \
  uvicorn app.main:app --host 127.0.0.1 --port "$APP_PORT"
) >"$APP_LOG" 2>&1 &
```

Temporal worker plus web app:

```bash
(
  cd "$ROOT_DIR"
  MINIMAX_PROXY_BASE_URL="http://localhost:$PORT" \
  npm run worker
) >"$LOG_DIR/worker.log" 2>&1 &
WORKER_PID=$!

(
  cd "$ROOT_DIR/app"
  PORT="$APP_PORT" npm run dev
) >"$APP_LOG" 2>&1 &
APP_PID=$!
```

If the launcher starts extra workers, extend `cleanup()` to stop those PIDs too.

## 25. Minimal framework adapter samples

### 25.1 OpenAI-compatible JavaScript call

```js
export async function callMiniMaxProxy(messages, options = {}) {
  const baseUrl = process.env.MINIMAX_PROXY_BASE_URL || import.meta.env?.VITE_PROXY_BASE_URL || 'http://localhost:8080';
  const res = await fetch(`${baseUrl}/v1/chat/completions`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({
      model: options.model || 'MiniMax-M2.7',
      messages,
      max_tokens: options.maxTokens || 1024,
      temperature: options.temperature ?? 0.2,
    }),
  });
  if (!res.ok) throw new Error(`MiniMax proxy failed: ${res.status} ${await res.text()}`);
  const data = await res.json();
  return data.choices?.[0]?.message?.content || '';
}
```

### 25.2 Pydantic AI / OpenAI-compatible Python pattern

Exact imports may vary by Pydantic AI version, but the invariant is the same: configure an OpenAI-compatible client/model with the proxy base URL.

```python
import os
from openai import AsyncOpenAI

client = AsyncOpenAI(
    base_url=os.environ.get("MINIMAX_PROXY_BASE_URL", "http://localhost:8080") + "/v1",
    api_key="not-needed-auth-is-in-proxy",
)

async def ask_llm(prompt: str) -> str:
    resp = await client.chat.completions.create(
        model=os.environ.get("MINIMAX_MODEL", "MiniMax-M2.7"),
        messages=[{"role": "user", "content": prompt}],
        max_tokens=512,
    )
    return resp.choices[0].message.content or ""
```

### 25.3 LangGraph Python node pattern

```python
from typing import TypedDict
from openai import AsyncOpenAI
import os

client = AsyncOpenAI(
    base_url=os.environ.get("MINIMAX_PROXY_BASE_URL", "http://localhost:8080") + "/v1",
    api_key="not-needed-auth-is-in-proxy",
)

class GraphState(TypedDict):
    user_request: str
    answer: str

async def specialist_node(state: GraphState) -> GraphState:
    resp = await client.chat.completions.create(
        model=os.environ.get("MINIMAX_MODEL", "MiniMax-M2.7"),
        messages=[{"role": "user", "content": state["user_request"]}],
    )
    return {**state, "answer": resp.choices[0].message.content or ""}
```

### 25.4 Temporal.io activity pattern

Do not put non-deterministic LLM calls directly in deterministic workflow code. Put them in activities.

```ts
// activities.ts
export async function callLlmActivity(prompt: string): Promise<string> {
  const baseUrl = process.env.MINIMAX_PROXY_BASE_URL || 'http://localhost:8080';
  const res = await fetch(`${baseUrl}/v1/chat/completions`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({
      model: process.env.MINIMAX_MODEL || 'MiniMax-M2.7',
      messages: [{ role: 'user', content: prompt }],
    }),
  });
  if (!res.ok) throw new Error(await res.text());
  const data = await res.json();
  return data.choices?.[0]?.message?.content || '';
}
```

```ts
// workflow.ts
import { proxyActivities } from '@temporalio/workflow';
import type * as activities from './activities';

const { callLlmActivity } = proxyActivities<typeof activities>({
  startToCloseTimeout: '2 minutes',
});

export async function llmWorkflow(prompt: string): Promise<string> {
  return await callLlmActivity(prompt);
}
```

### 25.5 CopilotKit / LangGraph through CopilotKit note

For CopilotKit, keep the UI skill in charge of selecting the pattern. The proxy remains the LLM boundary.

Acceptable shapes:

```text
Browser CopilotKit UI -> app route/tool -> MiniMax proxy
Browser CopilotKit UI -> CopilotKit runtime -> MiniMax proxy
Browser UI -> browser-local LangGraph node -> MiniMax proxy
Browser UI -> backend LangGraph -> MiniMax proxy
```

The rider must say which shape is used, what state is shared, what tools/actions exist, and what Playwright screenshots prove the visible result.

## 26. Bootstrap acceptance checklist

A new empty project is ready to work when this checklist passes:

- [ ] `.pi/skills/project-goal-engineering/SKILL.md` exists.
- [ ] `.pi/skills/product-backlog-orchestrator/SKILL.md` exists if a PRD/backlog is used.
- [ ] A framework helper skill exists for the selected frontend/runtime.
- [ ] `architecture_decisions.md` exists and documents the proxy boundary, app shape, commands, and known gaps.
- [ ] `docs/prd.md` exists if the user provided a PRD, with stable `PRD-###` IDs.
- [ ] `docs/implementation_plan.md` maps milestones to PRD IDs.
- [ ] `docs/tasks.md` maps tasks to PRD IDs and statuses.
- [ ] `.test/` is ignored by git.
- [ ] `Cargo.toml` and `src/main.rs` create the MiniMax proxy.
- [ ] `cargo check` passes.
- [ ] `cargo test` passes.
- [ ] `first.sh` exists and can smoke the proxy, or clearly documents missing OAuth/login prerequisites.
- [ ] `run_app.sh` exists and starts proxy first, then app.
- [ ] The selected framework has a minimal app/worker/route that can call the proxy.
- [ ] The first product task is small and marked `Ready`.

After this, normal work should be:

```text
Pick next Ready TASK -> create goal/rider -> write tests -> implement -> verify -> update architecture/tasks
```


## 27. Full tested MiniMax Rust proxy source from this repository

The starter in section 23 is useful for explanation, but for fastest reuse in a new project use this **full tested proxy source** from the current repository. This is the code path that has been used with:

```bash
cargo check
cargo test
./first.sh
./run_app.sh
```

In a new empty project, create `src/main.rs` with the following content and use the `Cargo.toml` dependencies from section 23.1.

```rust
use anyhow::{anyhow, Context, Result};
use axum::{
    extract::State,
    http::{Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

const CLIENT_ID: &str = "659cf4c1-615c-45f6-a5f6-4bf15eb476e5";
const DEFAULT_PORT: u16 = 8080;

struct AppState {
    credentials: Mutex<Option<Credentials>>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            credentials: Mutex::new(self.credentials.lock().unwrap().clone()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Credentials {
    access_token: String,
    refresh_token: String,
    expires_at: u64,
    region: String,
    resource_url: Option<String>,
    account: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    messages: Vec<ChatMessage>,
    model: Option<String>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    stream: Option<bool>,
    system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicRequest {
    model: String,
    messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
    index: u32,
    finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicResponse {
    id: String,
    #[serde(rename = "type")]
    msg_type: String,
    role: String,
    content: Vec<AnthropicContent>,
    model: String,
    #[serde(rename = "stop_reason")]
    stop_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
    thinking: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorDetail {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
    code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceCodeResponse {
    #[serde(rename = "user_code")]
    user_code: String,
    #[serde(rename = "verification_uri")]
    verification_uri: String,
    interval: u64,
    expires_in: u64,
    code_verifier: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenResponse {
    status: String,
    #[serde(rename = "access_token")]
    access_token: Option<String>,
    #[serde(rename = "refresh_token")]
    refresh_token: Option<String>,
    #[serde(rename = "expired_in")]
    expired_in: Option<u64>,
    #[serde(rename = "resource_url")]
    resource_url: Option<String>,
}

fn auth_error(message: &str, code: Option<&str>) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse {
            error: ErrorDetail {
                message: message.to_string(),
                error_type: "authentication_required".to_string(),
                code: code.map(|s| s.to_string()),
            },
        }),
    )
}

fn server_error(message: &str) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            error: ErrorDetail {
                message: message.to_string(),
                error_type: "internal_error".to_string(),
                code: None,
            },
        }),
    )
}

fn bad_request(message: &str, code: Option<&str>) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: ErrorDetail {
                message: message.to_string(),
                error_type: "bad_request".to_string(),
                code: code.map(|s| s.to_string()),
            },
        }),
    )
}

fn get_config_path() -> std::path::PathBuf {
    if let Ok(dir) = env::var("MMX_CONFIG_DIR") {
        return std::path::PathBuf::from(dir).join("config.json");
    }
    dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".mmx")
        .join("config.json")
}

fn get_legacy_credentials_path() -> std::path::PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".mmx")
        .join("credentials.json")
}

fn parse_expires_at_iso(value: &str) -> Option<u64> {
    chrono::DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|dt| dt.timestamp().max(0) as u64)
}

fn expires_to_iso(expires_at: u64) -> String {
    chrono::DateTime::<chrono::Utc>::from_timestamp(expires_at as i64, 0)
        .unwrap_or_else(chrono::Utc::now)
        .to_rfc3339()
}

fn token_expiry_to_epoch_seconds(expired_in: Option<u64>) -> u64 {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    match expired_in {
        // MiniMax OAuth returns absolute epoch milliseconds.
        Some(v) if v > 10_000_000_000 => v / 1000,
        // Be tolerant of absolute epoch seconds.
        Some(v) if v > 1_000_000_000 => v,
        // Also tolerate duration seconds.
        Some(v) => now + v,
        None => now + 3600,
    }
}

fn load_credentials() -> Option<Credentials> {
    let config_path = get_config_path();
    if let Ok(content) = fs::read_to_string(&config_path) {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(oauth) = value.get("oauth") {
                let access_token = oauth.get("access_token")?.as_str()?.to_string();
                let refresh_token = oauth
                    .get("refresh_token")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let expires_at = oauth
                    .get("expires_at")
                    .and_then(|v| v.as_str())
                    .and_then(parse_expires_at_iso)?;
                let region = oauth
                    .get("region")
                    .and_then(|v| v.as_str())
                    .unwrap_or("global")
                    .to_string();
                let resource_url = oauth
                    .get("resource_url")
                    .and_then(|v| v.as_str())
                    .map(str::to_string);
                let account = oauth
                    .get("account")
                    .and_then(|v| v.as_str())
                    .map(str::to_string);
                return Some(Credentials {
                    access_token,
                    refresh_token,
                    expires_at,
                    region,
                    resource_url,
                    account,
                });
            }
        }
    }

    // Backward compatibility with the old guide's ~/.mmx/credentials.json format.
    let legacy_path = get_legacy_credentials_path();
    fs::read_to_string(&legacy_path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
}

fn save_credentials(creds: &Credentials) -> Result<()> {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("Failed to create .mmx directory")?;
    }

    let mut config = fs::read_to_string(&path)
        .ok()
        .and_then(|content| serde_json::from_str::<serde_json::Value>(&content).ok())
        .unwrap_or_else(|| serde_json::json!({}));

    config["oauth"] = serde_json::json!({
        "access_token": creds.access_token,
        "refresh_token": creds.refresh_token,
        "expires_at": expires_to_iso(creds.expires_at),
        "region": creds.region,
        "resource_url": creds.resource_url,
        "account": creds.account,
    });
    config["region"] = serde_json::json!(creds.region);

    let content =
        serde_json::to_string_pretty(&config).context("Failed to serialize credentials")?;
    fs::write(&path, content).context("Failed to write credentials")?;
    Ok(())
}

fn oauth_host(region: &str) -> String {
    match region {
        "cn" => "https://account.minimaxi.com".to_string(),
        _ => "https://account.minimax.io".to_string(),
    }
}

fn api_host(region: &str, resource_url: Option<&str>) -> String {
    if let Some(url) = resource_url {
        url.to_string()
    } else {
        match region {
            "cn" => "https://api.minimaxi.com".to_string(),
            _ => "https://api.minimax.io".to_string(),
        }
    }
}

async fn start_device_code(region: String) -> Result<DeviceCodeResponse> {
    let code_verifier = generate_pkce_verifier();
    let code_challenge = generate_pkce_challenge(&code_verifier);

    let client = reqwest::Client::new();
    let state = generate_pkce_verifier();
    let resp = client
        .post(format!("{}/oauth2/device/code", oauth_host(&region)))
        .form(&[
            ("client_id", CLIENT_ID),
            ("scope", "openid profile coding_plan"),
            ("code_challenge", &code_challenge),
            ("code_challenge_method", "S256"),
            ("state", &state),
        ])
        .send()
        .await
        .context("Failed to request device code")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(anyhow!("Device code request failed ({}): {}", status, text));
    }

    #[derive(Deserialize)]
    struct RawDeviceCodeResponse {
        #[serde(rename = "user_code")]
        user_code: Option<String>,
        #[serde(rename = "verification_uri_complete")]
        verification_uri_complete: Option<String>,
        #[serde(rename = "verification_uri")]
        verification_uri: Option<String>,
        interval: Option<u64>,
        #[serde(rename = "expired_in")]
        expired_in: Option<u64>,
        state: Option<String>,
        #[serde(rename = "base_resp")]
        base_resp: Option<BaseResp>,
    }

    #[derive(Deserialize)]
    struct BaseResp {
        #[serde(rename = "status_code")]
        status_code: Option<i32>,
    }

    let raw: RawDeviceCodeResponse = resp
        .json()
        .await
        .context("Failed to parse device code response")?;

    if let Some(br) = raw.base_resp {
        if br.status_code != Some(0) {
            return Err(anyhow!(
                "Device code request failed: status_code={}",
                br.status_code.unwrap_or(-1)
            ));
        }
    }

    if raw.state.as_deref() != Some(&state) {
        return Err(anyhow!("OAuth state mismatch"));
    }

    Ok(DeviceCodeResponse {
        user_code: raw.user_code.unwrap_or_default(),
        verification_uri: raw
            .verification_uri_complete
            .or(raw.verification_uri)
            .unwrap_or_default(),
        interval: raw.interval.unwrap_or(3000),
        expires_in: raw.expired_in.unwrap_or(0),
        code_verifier,
        state,
    })
}

fn generate_pkce_verifier() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(&bytes)
}

fn generate_pkce_challenge(verifier: &str) -> String {
    // Proper SHA256 hash of the verifier
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let result = hasher.finalize();
    URL_SAFE_NO_PAD.encode(&result)
}

async fn poll_for_token(
    region: &str,
    user_code: &str,
    code_verifier: &str,
) -> Result<TokenResponse> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/oauth2/token", oauth_host(region)))
        .form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
            ("client_id", CLIENT_ID),
            ("user_code", user_code),
            ("code_verifier", code_verifier),
        ])
        .send()
        .await
        .context("Failed to poll for token")?;

    let status = resp.status();
    let text = resp.text().await.unwrap_or_default();

    // Debug: print what we got
    info!("Poll response status: {}, body: {}", status, text);

    // Check if it's not JSON
    if !text.starts_with('{') && !text.starts_with('[') {
        // Not JSON - might be error page or something else
        if text.contains("rate") || text.contains("limit") || text.contains("error") {
            return Ok(TokenResponse {
                status: "pending".to_string(),
                access_token: None,
                refresh_token: None,
                expired_in: None,
                resource_url: None,
            });
        }
        return Err(anyhow!("Non-JSON response ({}): {}", status, text));
    }

    let token: TokenResponse =
        serde_json::from_str(&text).context("Failed to parse token response")?;
    Ok(token)
}

async fn refresh_token(region: &str, refresh_token: &str) -> Result<TokenResponse> {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/oauth2/token", oauth_host(region)))
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", CLIENT_ID),
        ])
        .send()
        .await
        .context("Failed to refresh token")?;

    let token: TokenResponse = resp
        .json()
        .await
        .context("Failed to parse refresh response")?;
    Ok(token)
}

fn get_access_token(state: &AppState) -> Result<String> {
    let creds = state.credentials.lock().unwrap();

    let creds = match creds.as_ref() {
        Some(c) => c,
        None => {
            return Err(anyhow!(
                "Not authenticated. Use POST /auth/login to start OAuth flow."
            ))
        }
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if creds.expires_at <= now + 300 {
        return Err(anyhow!("Token expired. Use POST /auth/refresh to refresh."));
    }

    Ok(creds.access_token.clone())
}

async fn chat_handler(
    State(state): State<AppState>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, Json<ErrorResponse>)> {
    info!("Chat request: {:?}", request);

    let access_token = match get_access_token(&state) {
        Ok(token) => token,
        Err(e) => {
            let err_msg = e.to_string();
            if err_msg.contains("Not authenticated") {
                return Err(auth_error(
                    "MiniMax OAuth authentication required. Use POST /auth/login to start OAuth flow.",
                    Some("AUTH_REQUIRED"),
                ));
            }
            if err_msg.contains("expired") {
                return Err(auth_error(
                    "MiniMax OAuth token expired. Use POST /auth/refresh",
                    Some("TOKEN_EXPIRED"),
                ));
            }
            return Err(server_error(&format!("Auth error: {}", e)));
        }
    };

    let creds = {
        let c = state.credentials.lock().unwrap();
        c.clone()
    };

    let creds = match creds {
        Some(c) => c,
        None => return Err(auth_error("No credentials", Some("NO_CREDENTIALS"))),
    };

    let model = request.model.unwrap_or_else(|| "MiniMax-M2.7".to_string());
    let api_base = api_host(&creds.region, creds.resource_url.as_deref());

    let mut system = request.system;
    let mut messages = Vec::new();
    for msg in request.messages {
        if msg.role == "system" {
            if system.is_none() {
                system = Some(msg.content);
            }
        } else {
            messages.push(msg);
        }
    }

    let anthropic_req = AnthropicRequest {
        model: model.clone(),
        messages,
        max_tokens: request.max_tokens.or(Some(4096)),
        temperature: request.temperature,
        top_p: request.top_p,
        // OpenAI streaming translation is not implemented yet; request a normal JSON response.
        stream: Some(false),
        system,
    };

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/anthropic/v1/messages", api_base))
        .header("x-api-key", access_token)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&anthropic_req)
        .send()
        .await
        .map_err(|e| {
            error!("Failed to send request: {}", e);
            server_error(&format!("Request failed: {}", e))
        })?;

    let status = resp.status();

    if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
        return Err(auth_error(
            "MiniMax OAuth token invalid or expired. Use POST /auth/refresh",
            Some("TOKEN_INVALID"),
        ));
    }

    if status.as_u16() == 402 || status.as_u16() >= 500 {
        let error_text = resp.text().await.unwrap_or_default();
        error!("API error ({}): {}", status, error_text);
        return Err(bad_request(
            &format!("API error: {}", error_text),
            Some(&status.as_u16().to_string()),
        ));
    }

    if !status.is_success() {
        let error_text = resp.text().await.unwrap_or_default();
        error!("Request failed ({}): {}", status, error_text);
        return Err(bad_request(
            &format!("Request failed: {}", error_text),
            None,
        ));
    }

    let anthropic_resp: AnthropicResponse = resp.json().await.map_err(|e| {
        error!("Failed to parse response: {}", e);
        server_error(&format!("Failed to parse response: {}", e))
    })?;

    let content = anthropic_resp
        .content
        .iter()
        .filter_map(|c| c.text.clone())
        .collect::<Vec<_>>()
        .join("")
        .trim()
        .to_string();

    let response = ChatResponse {
        id: anthropic_resp.id,
        object: "chat.completion".to_string(),
        created: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        model,
        choices: vec![ChatChoice {
            message: ChatMessage {
                role: anthropic_resp.role,
                content,
            },
            index: 0,
            finish_reason: anthropic_resp
                .stop_reason
                .unwrap_or_else(|| "stop".to_string()),
        }],
    };

    info!("Chat response: {:?}", response);
    Ok(Json(response))
}

async fn health_handler(State(state): State<AppState>) -> Json<serde_json::Value> {
    let authenticated = {
        let creds = state.credentials.lock().unwrap();
        creds.is_some()
    };

    Json(serde_json::json!({
        "status": if authenticated { "ok" } else { "auth_required" },
        "authenticated": authenticated
    }))
}

async fn auth_status_handler(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let creds = state.credentials.lock().unwrap();

    match creds.as_ref() {
        Some(c) => {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let expires_in = if c.expires_at > now {
                c.expires_at - now
            } else {
                0
            };

            Ok(Json(serde_json::json!({
                "authenticated": true,
                "region": c.region,
                "expires_in_seconds": expires_in,
                "has_refresh_token": !c.refresh_token.is_empty()
            })))
        }
        None => Err(auth_error(
            "Not authenticated. Use POST /auth/login to start OAuth flow.",
            Some("AUTH_REQUIRED"),
        )),
    }
}

#[derive(Deserialize)]
struct LoginRequest {}

async fn auth_login_handler(
    State(_state): State<AppState>,
    Json(_body): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    Err(bad_request(
        "OAuth is automatically started on server startup. No manual login needed.",
        Some("AUTO_AUTH"),
    ))
}

#[derive(Deserialize)]
struct TokenRequest {
    user_code: String,
    code_verifier: String,
    region: Option<String>,
}

async fn auth_token_handler(
    State(state): State<AppState>,
    Json(body): Json<TokenRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let region = body.region.unwrap_or_else(|| "global".to_string());

    info!("Polling for token with user_code... (will retry until authorized)");

    let interval_ms = 3000;

    let token_resp = loop {
        match poll_for_token(&region, &body.user_code, &body.code_verifier).await {
            Ok(resp) => {
                if resp.status != "pending" {
                    break resp;
                }
            }
            Err(e) => {
                error!("Poll failed: {}", e);
                return Err(server_error(&format!("Token poll failed: {}", e)));
            }
        };

        info!("Authorization pending, waiting...");
        tokio::time::sleep(tokio::time::Duration::from_millis(interval_ms)).await;
    };

    if token_resp.status != "success" || token_resp.access_token.is_none() {
        return Err(bad_request(
            &format!("Authorization failed: status={}", token_resp.status),
            Some("AUTH_FAILED"),
        ));
    }

    let token = token_resp;

    let expires_at = token_expiry_to_epoch_seconds(token.expired_in);

    let credentials = Credentials {
        access_token: token.access_token.unwrap(),
        refresh_token: token.refresh_token.unwrap_or_default(),
        expires_at,
        region: region.clone(),
        resource_url: token.resource_url,
        account: None,
    };

    if let Err(e) = save_credentials(&credentials) {
        error!("Failed to save credentials: {}", e);
    }

    {
        let mut creds = state.credentials.lock().unwrap();
        *creds = Some(credentials.clone());
    }

    info!("OAuth authentication successful!");

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Successfully authenticated with MiniMax OAuth",
        "region": credentials.region,
        "expires_in": credentials.expires_at - std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    })))
}

async fn auth_refresh_handler(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let (region, refresh_token_str, old_resource_url, old_account) = {
        let creds = state.credentials.lock().unwrap();
        match creds.as_ref() {
            Some(c) => (
                c.region.clone(),
                c.refresh_token.clone(),
                c.resource_url.clone(),
                c.account.clone(),
            ),
            None => return Err(auth_error("Not authenticated", Some("AUTH_REQUIRED"))),
        }
    };

    if refresh_token_str.is_empty() {
        return Err(bad_request(
            "No refresh token available. Please re-authenticate with POST /auth/login",
            Some("NO_REFRESH_TOKEN"),
        ));
    }

    info!("Refreshing OAuth token...");

    let token_resp = match refresh_token(&region, &refresh_token_str).await {
        Ok(resp) => resp,
        Err(e) => {
            error!("Token refresh failed: {}", e);
            return Err(server_error(&format!("Token refresh failed: {}", e)));
        }
    };

    if token_resp.status != "success" || token_resp.access_token.is_none() {
        return Err(auth_error(
            "Token refresh failed. Please re-authenticate with POST /auth/login",
            Some("REFRESH_FAILED"),
        ));
    }

    let token = token_resp;

    let expires_at = token_expiry_to_epoch_seconds(token.expired_in);

    let credentials = Credentials {
        access_token: token.access_token.unwrap(),
        refresh_token: token.refresh_token.unwrap_or(refresh_token_str),
        expires_at,
        region: region.clone(),
        resource_url: token.resource_url.or(old_resource_url),
        account: old_account,
    };

    if let Err(e) = save_credentials(&credentials) {
        error!("Failed to save credentials: {}", e);
    }

    {
        let mut creds = state.credentials.lock().unwrap();
        *creds = Some(credentials.clone());
    }

    info!("Token refreshed successfully!");

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Token refreshed successfully",
        "expires_in": credentials.expires_at - std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    })))
}

fn open_browser(url: &str) {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(url).spawn().ok();
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open").arg(url).spawn().ok();
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/c", "start", "", url])
            .spawn()
            .ok();
    }
}

async fn auto_auth(state: &AppState) -> Result<()> {
    info!("Starting automatic OAuth authentication...");

    let region = "global";
    let device_resp = start_device_code(region.to_string()).await?;

    println!();
    println!("==============================================");
    println!("  🔐 MiniMax OAuth Authentication Required");
    println!("==============================================");
    println!();
    println!("  User Code: {}", device_resp.user_code);
    println!();
    println!("  Opening browser for authorization...");
    println!();
    println!("  If browser doesn't open, visit:");
    println!("  {}", device_resp.verification_uri);
    println!();
    println!("  Enter the user code: {}", device_resp.user_code);
    println!("  Then click Authorize");
    println!();
    println!("  Waiting for authentication...");
    println!("  (Press Ctrl+C to exit and try again)");
    println!();

    // Open browser
    open_browser(&device_resp.verification_uri);

    // Poll for token - MiniMax's interval appears to be in milliseconds (3000 = 3 seconds)
    let interval_ms = device_resp.interval;

    info!("Polling every {}ms... (interval from server)", interval_ms);

    let token_resp = loop {
        match poll_for_token(region, &device_resp.user_code, &device_resp.code_verifier).await {
            Ok(resp) => {
                if resp.status != "pending" {
                    break resp;
                }
            }
            Err(e) => {
                error!("Poll error: {}", e);
            }
        };

        print!(".");
        std::io::Write::flush(&mut std::io::stdout()).ok();
        tokio::time::sleep(tokio::time::Duration::from_millis(interval_ms)).await;
    };

    println!();

    if token_resp.status != "success" || token_resp.access_token.is_none() {
        return Err(anyhow!(
            "Authorization failed: status={}",
            token_resp.status
        ));
    }

    let expires_at = token_expiry_to_epoch_seconds(token_resp.expired_in);

    let credentials = Credentials {
        access_token: token_resp.access_token.unwrap(),
        refresh_token: token_resp.refresh_token.unwrap_or_default(),
        expires_at,
        region: region.to_string(),
        resource_url: token_resp.resource_url,
        account: None,
    };

    if let Err(e) = save_credentials(&credentials) {
        error!("Failed to save credentials: {}", e);
    }

    {
        let mut creds = state.credentials.lock().unwrap();
        *creds = Some(credentials.clone());
    }

    println!();
    println!("  ✅ Authentication successful!");
    println!();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| DEFAULT_PORT.to_string())
        .parse()
        .unwrap_or(DEFAULT_PORT);

    // Load existing credentials
    let credentials = load_credentials();

    let state = AppState {
        credentials: Mutex::new(credentials),
    };

    // Check if we have valid credentials
    let needs_auth = {
        let creds = state.credentials.lock().unwrap();
        match creds.as_ref() {
            Some(c) => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                c.expires_at <= now + 300 // Expires within 5 minutes
            }
            None => true,
        }
    };

    if needs_auth {
        println!();
        println!("==============================================");
        println!("  MiniMax OAuth Proxy");
        println!("==============================================");
        println!();

        if let Err(e) = auto_auth(&state).await {
            error!("Auto-auth failed: {}", e);
            println!();
            println!("  ✗ Authentication failed: {}", e);
            println!("  Please try running again or check your internet connection.");
            std::process::exit(1);
        }
    } else {
        info!("Using existing OAuth credentials");
    }

    println!();
    println!("==============================================");
    println!("  MiniMax Proxy Server (OAuth)");
    println!("  port: http://0.0.0.0:{}", port);
    println!("==============================================");
    println!();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/auth/status", get(auth_status_handler))
        .route("/auth/login", post(auth_login_handler))
        .route("/auth/token", post(auth_token_handler))
        .route("/auth/refresh", post(auth_refresh_handler))
        .route("/v1/chat/completions", post(chat_handler))
        .layer(cors)
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    info!("Starting server on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

```

### 27.1 Bootstrap instruction

When bootstrapping a new project, prefer this instruction over recreating the proxy from memory:

```text
Create the MiniMax Rust proxy by copying the full tested `src/main.rs` from loop_engineering.md section 27 and the `Cargo.toml` from section 23.1. Do not simplify OAuth. Preserve automatic device-code auth, credential loading/saving, refresh, CORS, health/auth routes, and `/v1/chat/completions` OpenAI-compatible translation.
```

### 27.2 What not to remove

Do not remove these tested behaviors unless a later rider explicitly replaces them:

- Automatic MiniMax OAuth device-code login on startup when credentials are missing/expiring.
- Credential loading from `~/.mmx/config.json` and `$MMX_CONFIG_DIR/config.json`.
- Legacy fallback to `~/.mmx/credentials.json`.
- Saving OAuth credentials back into MiniMax CLI-compatible config.
- `GET /health`.
- `GET /auth/status`.
- `POST /auth/login` disabled with `AUTO_AUTH` explanation.
- `POST /auth/token` manual token polling helper.
- `POST /auth/refresh`.
- `POST /v1/chat/completions` OpenAI-compatible endpoint.
- Browser CORS for app frameworks.
- OpenAI system-message lifting into Anthropic `system`.
- Non-streaming upstream behavior until streaming is implemented by a dedicated task.
