# Rust MiniMax Proxy — UNJobNet Artificial Keyword Source Rider

## Posture

- Extend only the Python scraper/service unless tests show frontend changes are required.
- Preserve existing API compatibility: keep `source` and `jobs`; add `sources`.
- Use fixture tests for multiple sources and deduplication.

## Agent/runtime contract

- Existing endpoint remains `GET /jobs/search?q=<query>&limit=<n>`.
- Scraper sources now include:
  - `https://www.unjobnet.org/countries/Spain`
  - `https://www.unjobnet.org/jobs?keywords=artificial`
- Results are merged and deduplicated by normalized job URL.
- `source` remains the Spain URL for compatibility.
- `sources` lists all scraped URLs.

## Phases

### P1 — Failing multi-source tests
- Add parser/service tests proving two fixture pages merge and duplicate URLs dedupe.
Depth tests:
- `.venv/bin/python -m pytest jobs_scraper/tests`

### P2 — Scraper implementation
- Add configured source list, multi-page search helper, and multi-source fetcher.
Depth tests:
- `.venv/bin/python -m pytest jobs_scraper/tests`

### P3 — App compatibility and closure
- Run app tests/build and update architecture decisions.
Depth tests:
- `cd app && npm test`
- `cd app && npm run build`

## Out of scope

- Pagination crawl.
- User-selectable source filters.
- UI redesign for source badges.
