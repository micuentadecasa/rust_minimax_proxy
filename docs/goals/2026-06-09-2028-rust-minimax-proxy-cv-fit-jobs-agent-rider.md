# Rust MiniMax Proxy — CV-fit Jobs Agent Rider

## Posture

- Use deterministic CV/job text matching; no LLM ranking in this round.
- Preserve `GET /jobs/search?q=<query>&limit=<n>` and existing job card UI.
- Add match metadata fields to each returned job: `cvMatchedTerms`, `cvMatchScore`, `cvMatchReason`.
- Durable tests use fixtures, not live UNJobNet.

## Runtime contract

- CV file: `jobs_scraper/cv.json`.
- If CV file exists, Jobs Agent returns only jobs with enough overlap with Luis's CV.
- If CV file is missing/unreadable, service falls back to curated AI/data/technology terms so the service remains available.
- Filtering prioritizes responsible AI, AI governance/compliance, AI architecture, cloud, data/AI engineering, Python, RAG/agentic AI, product leadership, security, GDPR, EU AI Act, and UN/international organization experience.

## Phases

### P1 — Failing CV-fit tests
- Add fixture jobs that match search terms but not the CV and assert exclusion.
- Assert included jobs carry CV match metadata.

### P2 — Implement CV profile loading and scoring
- Add CV JSON loader, term extraction/curation, scoring, and filtering.

### P3 — Verification and architecture closure
- Run Python/app checks and update architecture decisions.

## Out of scope

- LLM semantic ranking.
- User-editable CV in the UI.
- Persisted saved jobs.
