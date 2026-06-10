from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Iterable
from urllib.parse import urljoin

import httpx
from scrapy import Selector

SOURCE_URL = "https://www.unjobnet.org/countries/Spain"
ARTIFICIAL_JOBS_URL = "https://www.unjobnet.org/jobs?keywords=artificial"
UNJOBS_SPAIN_URL = "https://unjobs.org/duty_stations/spain"
SOURCE_URLS = [SOURCE_URL, ARTIFICIAL_JOBS_URL, UNJOBS_SPAIN_URL]
BASE_URL = "https://www.unjobnet.org"
UNJOBS_BASE_URL = "https://unjobs.org"
CV_PATH = Path(__file__).with_name("cv.json")

DEFAULT_TERMS = [
    "informatics",
    "information technology",
    "information systems",
    "ict",
    "it ",
    "data",
    "database",
    "analytics",
    "analyst",
    "artificial intelligence",
    "ai",
    "machine learning",
    "ml",
    "digital",
    "software",
    "developer",
    "programmer",
    "computer",
    "cyber",
]

CURATED_CV_TERMS = {
    "responsible ai": 5,
    "ai governance": 5,
    "model governance": 5,
    "eu ai act": 5,
    "gdpr": 4,
    "compliance": 4,
    "audit": 4,
    "risk assessment": 4,
    "human-in-the-loop": 4,
    "ai architecture": 4,
    "solutions architect": 4,
    "solution architecture": 4,
    "enterprise architecture": 4,
    "generative ai": 4,
    "llm": 4,
    "llms": 4,
    "rag": 4,
    "langgraph": 4,
    "agentic ai": 4,
    "machine learning": 4,
    "artificial intelligence": 3,
    "data engineering": 4,
    "data": 2,
    "analytics": 2,
    "analyst": 2,
    "python": 3,
    "cloud": 3,
    "azure": 3,
    "aws": 3,
    "oci": 3,
    "kubernetes": 3,
    "openshift": 3,
    "product ownership": 4,
    "product lead": 4,
    "product manager": 3,
    "roadmap": 2,
    "technical leader": 4,
    "engineering manager": 4,
    "security": 3,
    "cybersecurity": 3,
    "identity": 2,
    "united nations": 3,
    "unops": 3,
}

CV_MATCH_THRESHOLD = 2
CV_NEGATIVE_TERMS = ["communications", "communication", "social media", "campaign", "copywriting", "graphic design"]


def clean_text(value: str | None) -> str:
    if not value:
        return ""
    return re.sub(r"\s+", " ", value).strip(" \t\r\n-–|,")


def normalize_url(href: str | None, base_url: str = BASE_URL) -> str:
    return urljoin(base_url, href or "")


def base_url_for_source(source_url: str) -> str:
    return UNJOBS_BASE_URL if "unjobs.org" in source_url else BASE_URL


def first_text(selector: Selector, css_queries: Iterable[str], xpath_queries: Iterable[str] = ()) -> str:
    for query in css_queries:
        value = clean_text(" ".join(selector.css(query).getall()))
        if value:
            return value
    for query in xpath_queries:
        value = clean_text(" ".join(selector.xpath(query).getall()))
        if value:
            return value
    return ""


def card_text(card: Selector) -> str:
    return clean_text(" ".join(card.xpath(".//text()").getall()))


def extract_deadline(card: Selector) -> str:
    value = first_text(
        card,
        [
            "time::text",
            ".deadline::text",
            "[class*=deadline]::text",
            "[class*=Deadline]::text",
            "[class*=closing]::text",
            "[class*=Closing]::text",
            "[class*=date]::text",
            "[class*=Date]::text",
        ],
    )
    if value:
        return value

    text = card_text(card)
    patterns = [
        r"(?:closing date|deadline|expires|until)[:\s]+([^|•;]+)",
        r"\b(\d{4}-\d{2}-\d{2})\b",
        r"\b(\d{1,2}\s+[A-Za-z]{3,9}\s+\d{4})\b",
    ]
    for pattern in patterns:
        match = re.search(pattern, text, flags=re.IGNORECASE)
        if match:
            return clean_text(match.group(1))
    return ""


def extract_organization(card: Selector, title: str) -> str:
    value = first_text(
        card,
        [
            ".organization::text",
            ".org::text",
            "[class*=organization]::text",
            "[class*=Organization]::text",
            "[class*=agency]::text",
            "[class*=Agency]::text",
            "[class*=company]::text",
            "[class*=Company]::text",
            "[class*=employer]::text",
            "[class*=Employer]::text",
        ],
    )
    if value and value != title:
        return value

    lines = [clean_text(line) for line in card.xpath(".//text()").getall()]
    lines = [line for line in lines if line and line != title]
    for line in lines[:6]:
        if not re.search(r"deadline|closing|spain|madrid|barcelona|valencia", line, re.I):
            return line
    return ""


def extract_location(card: Selector) -> str:
    value = first_text(
        card,
        [
            ".location::text",
            "[class*=location]::text",
            "[class*=Location]::text",
            "[class*=duty]::text",
            "[class*=Duty]::text",
            "[class*=country]::text",
            "[class*=Country]::text",
        ],
    )
    if value:
        return value

    text = card_text(card)
    match = re.search(r"\b(Madrid|Barcelona|Valencia|Seville|Bilbao|Spain|España)(?:[,\w\s]*)", text, re.I)
    return clean_text(match.group(0)) if match else "Spain"


def parse_jobs_from_html(html: str, base_url: str = BASE_URL) -> list[dict]:
    selector = Selector(text=html)
    anchors = selector.css('a[href*="/jobs/detail"], a[href*="/job/"], a[href*="/jobs/"], a[href*="/vacancies/"]')
    jobs: list[dict] = []
    seen: set[str] = set()

    for anchor in anchors:
        href = anchor.attrib.get("href")
        url = normalize_url(href, base_url=base_url)
        if not url or url in seen:
            continue

        title = clean_text(" ".join(anchor.xpath(".//text()").getall()))
        if not title:
            continue

        card = anchor.xpath("ancestor::*[self::article or self::li or self::tr or self::div][1]")
        if not card:
            card = anchor
        else:
            card = card[0]

        organization = extract_organization(card, title)
        location = extract_location(card)
        deadline = extract_deadline(card)
        summary = card_text(card)
        if summary.startswith(title):
            summary = clean_text(summary[len(title):])

        jobs.append(
            {
                "title": title,
                "organization": organization,
                "location": location,
                "deadline": deadline,
                "url": url,
                "summary": summary[:500],
                "matchedTerms": [],
            }
        )
        seen.add(url)

    return jobs


def query_terms(query: str) -> list[str]:
    text = query.lower()
    explicit_terms = [term for term in DEFAULT_TERMS if term.strip() and term.strip() in text]
    if "ai" in explicit_terms and "artificial intelligence" not in explicit_terms:
        explicit_terms.append("artificial intelligence")
    if "artificial intelligence" in explicit_terms and "ai" not in explicit_terms:
        explicit_terms.append("ai")
    # For generic job prompts, keep the domain defaults because this specialist is scoped
    # to informatics/data/AI jobs. For specific prompts like "artificial intelligence",
    # honor the explicit domain terms first instead of broadening to every default term.
    return explicit_terms or DEFAULT_TERMS


def iter_strings(value) -> Iterable[str]:
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for child in value.values():
            yield from iter_strings(child)
    elif isinstance(value, list):
        for child in value:
            yield from iter_strings(child)


def load_cv_profile(path: Path = CV_PATH) -> dict:
    try:
        cv = json.loads(path.read_text(encoding="utf-8"))
    except Exception:
        cv = {}

    corpus = " ".join(iter_strings(cv)).lower()
    terms = {term: weight for term, weight in CURATED_CV_TERMS.items() if term in corpus}
    if not terms:
        terms = dict(CURATED_CV_TERMS)

    name = (
        cv.get("personal_information", {}).get("full_name")
        if isinstance(cv, dict)
        else None
    ) or "CV profile"
    return {"name": name, "terms": terms}


def score_job_against_cv(job: dict, cv_profile: dict | None = None) -> dict:
    profile = cv_profile or load_cv_profile()
    terms: dict[str, int] = profile.get("terms", {})
    haystack = " ".join(
        str(job.get(key, "")) for key in ["title", "organization", "location", "deadline", "summary"]
    ).lower()
    matched: list[str] = []
    score = 0
    for term, weight in terms.items():
        needle = term.strip().lower()
        if not needle:
            continue
        if len(needle) <= 3:
            found = bool(re.search(rf"\b{re.escape(needle)}\b", haystack))
        else:
            found = needle in haystack
        if found:
            matched.append(needle)
            score += int(weight)

    return {
        "score": score,
        "matchedTerms": matched[:10],
        "reason": f"Matched CV terms: {', '.join(matched[:6])}" if matched else "No strong CV overlap found.",
    }


def filter_jobs(jobs: list[dict], query: str, limit: int = 10, cv_profile: dict | None = None) -> list[dict]:
    terms = query_terms(query)
    filtered: list[dict] = []
    profile = cv_profile or load_cv_profile()

    for job in jobs:
        haystack = " ".join(
            str(job.get(key, "")) for key in ["title", "organization", "location", "deadline", "summary"]
        ).lower()
        matched = []
        for term in terms:
            needle = term.strip().lower()
            if not needle:
                continue
            if needle in {"ai", "it", "ml"}:
                if re.search(rf"\b{re.escape(needle)}\b", haystack):
                    matched.append(needle)
            elif needle in haystack:
                matched.append(needle)

        if matched:
            cv_match = score_job_against_cv(job, profile)
            has_negative_role = any(term in haystack for term in CV_NEGATIVE_TERMS)
            if cv_match["score"] < CV_MATCH_THRESHOLD or (has_negative_role and cv_match["score"] < 6):
                continue
            item = dict(job)
            item["matchedTerms"] = matched[:6]
            item["cvMatchedTerms"] = cv_match["matchedTerms"]
            item["cvMatchScore"] = cv_match["score"]
            item["cvMatchReason"] = cv_match["reason"]
            filtered.append(item)

    return filtered[: max(1, min(limit, 50))]


def dedupe_jobs_by_url(jobs: list[dict]) -> list[dict]:
    deduped: list[dict] = []
    seen: set[str] = set()
    for job in jobs:
        url = str(job.get("url") or "")
        key = url or f"{job.get('title', '')}|{job.get('organization', '')}|{job.get('location', '')}"
        if key in seen:
            continue
        seen.add(key)
        deduped.append(job)
    return deduped


def search_jobs_from_pages(pages: list[tuple[str, str]], query: str, limit: int = 10) -> dict:
    all_jobs: list[dict] = []
    sources: list[str] = []
    for source, html in pages:
        sources.append(source)
        all_jobs.extend(parse_jobs_from_html(html, base_url=base_url_for_source(source)))
    jobs = filter_jobs(dedupe_jobs_by_url(all_jobs), query=query, limit=limit)
    return {"source": SOURCE_URL, "sources": sources, "query": query, "jobs": jobs}


def search_jobs_from_html(html: str, query: str, limit: int = 10) -> dict:
    return search_jobs_from_pages([(SOURCE_URL, html)], query=query, limit=limit)


async def fetch_unjobnet_pages() -> list[tuple[str, str]]:
    headers = {
        "user-agent": "Mozilla/5.0 (compatible; MiniMaxProxyJobsAgent/0.1; +http://localhost)",
        "accept": "text/html,application/xhtml+xml",
    }
    pages: list[tuple[str, str]] = []
    async with httpx.AsyncClient(timeout=30, follow_redirects=True, headers=headers) as client:
        for source_url in SOURCE_URLS:
            response = await client.get(source_url)
            response.raise_for_status()
            pages.append((source_url, response.text))
    return pages


async def fetch_unjobnet_spain_html() -> str:
    pages = await fetch_unjobnet_pages()
    return pages[0][1]
