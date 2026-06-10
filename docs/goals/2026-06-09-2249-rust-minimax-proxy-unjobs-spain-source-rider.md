# Rust MiniMax Proxy — UNJobs Spain Source Rider

## Posture

- Add one source URL: `https://unjobs.org/duty_stations/spain`.
- Keep the Jobs Agent API unchanged: `GET /jobs/search?q=<query>&limit=<n>`.
- Preserve `source`, `sources`, `jobs`, CV match metadata, and current card UI.

## Runtime contract

- Scraper fetches three pages:
  - `https://www.unjobnet.org/countries/Spain`
  - `https://www.unjobnet.org/jobs?keywords=artificial`
  - `https://unjobs.org/duty_stations/spain`
- Parser supports UNJobs relative links and normalizes them against `https://unjobs.org`.
- Results merge, dedupe by URL, then pass through search-term and CV-fit filtering.

## Phases

### P1 — Failing tests
- Add fixture test for UNJobs source, relative URL normalization, source list, and CV-fit metadata.

### P2 — Implement source support
- Add constants, base URL handling, broader job link selectors, and source-aware parsing.

### P3 — Verify and close
- Run Python/app checks and update architecture decisions.

## Out of scope

- Pagination.
- Separate UI source badges.
- Live-network durable tests.
