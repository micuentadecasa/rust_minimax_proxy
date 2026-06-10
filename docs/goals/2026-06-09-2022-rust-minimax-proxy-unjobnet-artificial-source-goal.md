GOAL: Extend the Jobs Agent scraper to also scrape `https://www.unjobnet.org/jobs?keywords=artificial` in addition to the Spain country page. The Jobs Agent should merge and deduplicate results from both UNJobNet sources so artificial-intelligence job prompts can include matches from the keyword search page as well as Spain-specific listings.

**Read first.**
- `/Users/luis/projects/rust_minimax_proxy/architecture_decisions.md` — current Jobs Agent architecture.
- `/Users/luis/projects/rust_minimax_proxy/docs/goals/2026-06-09-2022-rust-minimax-proxy-unjobnet-artificial-source-rider.md` — implementation phases and tests.
- `/Users/luis/projects/rust_minimax_proxy/jobs_scraper/unjobnet.py` and `jobs_scraper/service.py` — Scrapy parser and service.
- `/Users/luis/projects/rust_minimax_proxy/jobs_scraper/tests/test_unjobnet.py` — scraper/service tests.

**Posture.** Keep this a small scraper extension. No UI changes unless the API shape breaks existing cards. Preserve fixture-based tests and avoid durable live-network assertions.

**Verification.**
- `.venv/bin/python -m pytest jobs_scraper/tests`
- `cd app && npm test`
- `cd app && npm run build`

**Stop when** the service fetches/parses both configured sources, deduplicates jobs by URL, returns a `sources` list, preserves the existing `source` field for compatibility, and architecture decisions document the new artificial-keyword source.
