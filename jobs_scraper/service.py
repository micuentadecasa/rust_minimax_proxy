from __future__ import annotations

import inspect
from typing import Awaitable, Callable

from fastapi import FastAPI, HTTPException, Query
from fastapi.middleware.cors import CORSMiddleware

from .unjobnet import SOURCE_URL, fetch_unjobnet_pages, search_jobs_from_html, search_jobs_from_pages

PagePayload = str | list[tuple[str, str]]
Fetcher = Callable[[], PagePayload | Awaitable[PagePayload]]


async def _call_fetcher(fetcher: Fetcher) -> PagePayload:
    value = fetcher()
    if inspect.isawaitable(value):
        return await value
    return value


def create_app(fetcher: Fetcher = fetch_unjobnet_pages) -> FastAPI:
    app = FastAPI(title="UNJobNet Spain Jobs Scraper", version="0.1.0")
    app.add_middleware(
        CORSMiddleware,
        allow_origins=["*"],
        allow_credentials=False,
        allow_methods=["GET", "OPTIONS"],
        allow_headers=["*"],
    )

    @app.get("/health")
    async def health() -> dict:
        return {"ok": True, "service": "unjobnet-jobs-scraper"}

    @app.get("/jobs/search")
    async def search_jobs(
        q: str = Query(..., description="User job search request"),
        limit: int = Query(10, ge=1, le=50),
    ) -> dict:
        query = q.strip()
        if not query:
            raise HTTPException(status_code=400, detail="query is required")
        try:
            payload = await _call_fetcher(fetcher)
        except Exception as exc:  # pragma: no cover - exercised by runtime smoke, not unit fixture
            raise HTTPException(status_code=502, detail=f"failed to fetch UNJobNet pages: {exc}") from exc
        if isinstance(payload, str):
            return search_jobs_from_html(payload, query=query, limit=limit)
        return search_jobs_from_pages(payload or [(SOURCE_URL, "")], query=query, limit=limit)

    return app


app = create_app()
