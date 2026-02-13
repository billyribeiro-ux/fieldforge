"""
FieldForge AI Service — FastAPI application for AI/ML features.

Provides:
- Photo estimation (damage assessment, material detection, measurement)
- Smart scheduling optimization
- Predictive analytics (revenue, churn, demand)
- NLP (voice-to-text notes, smart search)
- AI business assistant
"""

import structlog
from contextlib import asynccontextmanager
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.config import settings
from app.routes import health, photo_estimation, scheduling, analytics, assistant

logger = structlog.get_logger()


@asynccontextmanager
async def lifespan(app: FastAPI):
    logger.info("FieldForge AI Service starting", env=settings.ENV)
    # Startup: load ML models, connect to services
    yield
    # Shutdown: cleanup
    logger.info("FieldForge AI Service shutting down")


app = FastAPI(
    title="FieldForge AI Service",
    description="AI/ML microservice for photo estimation, smart scheduling, and predictive analytics",
    version="0.1.0",
    lifespan=lifespan,
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"] if settings.ENV == "development" else [settings.ALLOWED_ORIGIN],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.include_router(health.router, tags=["Health"])
app.include_router(photo_estimation.router, prefix="/api/v1/ai", tags=["Photo Estimation"])
app.include_router(scheduling.router, prefix="/api/v1/ai", tags=["Smart Scheduling"])
app.include_router(analytics.router, prefix="/api/v1/ai", tags=["Predictive Analytics"])
app.include_router(assistant.router, prefix="/api/v1/ai", tags=["AI Assistant"])


if __name__ == "__main__":
    import uvicorn

    uvicorn.run("main:app", host="0.0.0.0", port=8000, reload=True)
