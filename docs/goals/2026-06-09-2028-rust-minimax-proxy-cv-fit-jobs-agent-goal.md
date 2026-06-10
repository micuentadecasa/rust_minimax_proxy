GOAL: Make the UNJobNet Jobs Agent select only jobs that fit `jobs_scraper/cv.json`. The scraper should still search the configured UNJobNet sources, but returned job cards must be filtered by CV-relevant skills/roles such as AI governance, Responsible AI, product/technical leadership, AI architecture, data/AI engineering, cloud, Python, RAG, LangGraph, GDPR, and EU AI Act readiness.

**Read first.**
- `/Users/luis/projects/rust_minimax_proxy/architecture_decisions.md` — current Jobs Agent boundary.
- `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-09-2028-rust-minimax-proxy-cv-fit-jobs-agent-rider.md` — implementation phases.
- `/Users/luis/projects/rust_minimax_proxy/jobs_scraper/cv.json` — CV source for fit filtering.
- `/Users/luis/projects/rust_minimax_proxy/jobs_scraper/unjobnet.py` and tests — scraper/filtering implementation.

**Posture.** Keep filtering deterministic and testable. Do not call an LLM for CV matching in this round. Preserve existing endpoint shape and add match metadata to returned jobs.

**Verification.**
- `.venv/bin/python -m pytest jobs_scraper/tests`
- `cd app && npm test`
- `cd app && npm run build`

**Stop when** jobs that only match generic search terms but not the CV are excluded, returned jobs include CV match terms/score/reason, and architecture decisions document CV-fit filtering.
