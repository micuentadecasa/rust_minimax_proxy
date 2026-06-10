GOAL: Add `https://unjobs.org/duty_stations/spain` as a third source for the Jobs Agent scraper. The service should fetch UNJobNet Spain, UNJobNet artificial keyword search, and UNJobs Spain duty-station listings, merge and deduplicate jobs, then keep returning only CV-fit matches.

**Read first.**
- `/Users/luis/projects/rust_minimax_proxy/architecture_decisions.md`
- `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-09-2249-rust-minimax-proxy-unjobs-spain-source-rider.md`
- `/Users/luis/projects/rust_minimax_proxy/jobs_scraper/unjobnet.py`
- `/Users/luis/projects/rust_minimax_proxy/jobs_scraper/tests/test_unjobnet.py`

**Posture.** Keep this a scraper-source extension. Preserve existing endpoint and UI shape. Use fixture tests; do not require live UNJobs availability in durable tests.

**Verification.**
- `.venv/bin/python -m pytest jobs_scraper/tests`
- `cd app && npm test`
- `cd app && npm run build`

**Stop when** UNJobs Spain appears in `sources`, relative UNJobs links normalize to `https://unjobs.org`, jobs dedupe with existing sources, CV-fit filtering still applies, and architecture decisions document the new source.
