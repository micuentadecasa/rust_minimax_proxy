import pytest

from jobs_scraper.unjobnet import ARTIFICIAL_JOBS_URL, SOURCE_URL, UNJOBS_SPAIN_URL, filter_jobs, load_cv_profile, parse_jobs_from_html, search_jobs_from_html, search_jobs_from_pages


FIXTURE_HTML = """
<html><body>
  <main>
    <article class="job">
      <h2><a href="/jobs/detail/1">Data Analyst</a></h2>
      <div class="organization">UNICEF</div>
      <span class="location">Madrid, Spain</span>
      <time>2026-06-30</time>
      <p>Analyze humanitarian data and dashboards.</p>
    </article>
    <article class="job">
      <h2><a href="https://www.unjobnet.org/jobs/detail/2">Artificial Intelligence Specialist</a></h2>
      <div class="organization">UN Global Pulse</div>
      <span class="location">Barcelona, Spain</span>
      <span class="deadline">Closing date: 2026-07-15</span>
      <p>Machine learning and artificial intelligence role.</p>
    </article>
    <article class="job">
      <h2><a href="/jobs/detail/3">Finance Officer</a></h2>
      <div class="organization">WHO</div>
      <span class="location">Valencia, Spain</span>
      <span>Deadline: 2026-08-01</span>
      <p>Accounting and budget monitoring.</p>
    </article>
    <article class="job">
      <h2><a href="/jobs/detail/5">Artificial Intelligence Communications Intern</a></h2>
      <div class="organization">UNESCO</div>
      <span class="location">Madrid, Spain</span>
      <p>Write social media content about artificial intelligence events and campaigns.</p>
    </article>
  </main>
</body></html>
"""


def test_parse_jobs_from_fixture_html_normalizes_urls_and_fields():
    jobs = parse_jobs_from_html(FIXTURE_HTML)

    assert len(jobs) == 4
    assert jobs[0]["title"] == "Data Analyst"
    assert jobs[0]["organization"] == "UNICEF"
    assert jobs[0]["location"] == "Madrid, Spain"
    assert jobs[0]["deadline"] == "2026-06-30"
    assert jobs[0]["url"] == "https://www.unjobnet.org/jobs/detail/1"


def test_filter_jobs_matches_data_ai_and_informatics_terms():
    jobs = parse_jobs_from_html(FIXTURE_HTML)
    matches = filter_jobs(jobs, "I want data or artificial intelligence jobs", limit=10)

    assert [job["title"] for job in matches] == ["Data Analyst", "Artificial Intelligence Specialist"]
    assert "data" in matches[0]["matchedTerms"]
    assert "artificial intelligence" in matches[1]["matchedTerms"]


def test_search_jobs_from_html_returns_envelope():
    result = search_jobs_from_html(FIXTURE_HTML, query="informatics data AI", limit=2)

    assert result["source"] == SOURCE_URL
    assert result["sources"] == [SOURCE_URL]
    assert result["query"] == "informatics data AI"
    assert len(result["jobs"]) == 2
    assert all(job["cvMatchScore"] > 0 for job in result["jobs"])
    assert all(job["cvMatchedTerms"] for job in result["jobs"])


def test_cv_fit_filter_excludes_search_matches_that_do_not_fit_cv():
    html = """
    <html><body>
      <article class="job">
        <h2><a href="/jobs/detail/10">AI Governance Lead</a></h2>
        <div class="organization">UNDP</div>
        <p>Lead responsible AI, GDPR, EU AI Act readiness, RAG, and model governance.</p>
      </article>
      <article class="job">
        <h2><a href="/jobs/detail/11">Artificial Intelligence Communications Intern</a></h2>
        <div class="organization">UNESCO</div>
        <p>Write social media copy about artificial intelligence campaigns and events.</p>
      </article>
    </body></html>
    """

    result = search_jobs_from_html(html, query="artificial intelligence jobs", limit=10)

    assert [job["title"] for job in result["jobs"]] == ["AI Governance Lead"]
    assert result["jobs"][0]["cvMatchScore"] >= 6
    assert "responsible ai" in result["jobs"][0]["cvMatchedTerms"]
    assert "gdpr" in result["jobs"][0]["cvMatchedTerms"]


def test_load_cv_profile_uses_cv_json_terms():
    profile = load_cv_profile()

    assert profile["name"] == "Luis Molina Martinez"
    assert "responsible ai" in profile["terms"]
    assert "eu ai act" in profile["terms"]


def test_search_jobs_from_pages_merges_sources_and_deduplicates_by_url():
    artificial_html = """
    <html><body>
      <article class="job">
        <h2><a href="/jobs/detail/2">Artificial Intelligence Specialist</a></h2>
        <div class="organization">UN Global Pulse Duplicate</div>
        <p>Duplicate AI role.</p>
      </article>
      <article class="job">
        <h2><a href="/jobs/detail/4">Artificial Intelligence Engineer</a></h2>
        <div class="organization">UNDP</div>
        <span class="location">Remote</span>
        <p>Build artificial intelligence systems.</p>
      </article>
    </body></html>
    """

    result = search_jobs_from_pages(
        [(SOURCE_URL, FIXTURE_HTML), (ARTIFICIAL_JOBS_URL, artificial_html)],
        query="artificial intelligence jobs",
        limit=10,
    )

    assert result["source"] == SOURCE_URL
    assert result["sources"] == [SOURCE_URL, ARTIFICIAL_JOBS_URL]
    assert [job["title"] for job in result["jobs"]] == [
        "Artificial Intelligence Specialist",
        "Artificial Intelligence Engineer",
    ]
    assert len({job["url"] for job in result["jobs"]}) == 2


def test_search_jobs_from_pages_includes_unjobs_spain_and_normalizes_relative_urls():
    unjobs_html = """
    <html><body>
      <div class="job">
        <h3><a href="/vacancies/12345">Responsible AI Governance Specialist</a></h3>
        <span class="organization">UNDP</span>
        <span class="location">Madrid, Spain</span>
        <p>Lead responsible AI, GDPR compliance, EU AI Act readiness, and model governance.</p>
      </div>
    </body></html>
    """

    result = search_jobs_from_pages(
        [(UNJOBS_SPAIN_URL, unjobs_html)],
        query="responsible ai jobs",
        limit=10,
    )

    assert result["sources"] == [UNJOBS_SPAIN_URL]
    assert [job["title"] for job in result["jobs"]] == ["Responsible AI Governance Specialist"]
    assert result["jobs"][0]["url"] == "https://unjobs.org/vacancies/12345"
    assert result["jobs"][0]["cvMatchScore"] >= 10
    assert "responsible ai" in result["jobs"][0]["cvMatchedTerms"]


@pytest.mark.asyncio
async def test_fastapi_jobs_endpoint_uses_injected_fetcher():
    from jobs_scraper.service import create_app
    from fastapi.testclient import TestClient

    async def fake_fetcher():
        return [(SOURCE_URL, FIXTURE_HTML), (ARTIFICIAL_JOBS_URL, FIXTURE_HTML), (UNJOBS_SPAIN_URL, "<html></html>")]

    app = create_app(fetcher=fake_fetcher)
    client = TestClient(app)

    response = client.get("/jobs/search?q=data%20ai&limit=5")

    assert response.status_code == 200
    payload = response.json()
    assert payload["sources"] == [SOURCE_URL, ARTIFICIAL_JOBS_URL, UNJOBS_SPAIN_URL]
    assert [job["title"] for job in payload["jobs"]] == ["Data Analyst", "Artificial Intelligence Specialist"]


def test_fastapi_jobs_endpoint_rejects_empty_query():
    from jobs_scraper.service import create_app
    from fastapi.testclient import TestClient

    client = TestClient(create_app(fetcher=lambda: FIXTURE_HTML))

    response = client.get("/jobs/search?q=   ")

    assert response.status_code == 400
    assert "query is required" in response.json()["detail"].lower()
