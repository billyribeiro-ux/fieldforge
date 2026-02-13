from fastapi import APIRouter

router = APIRouter()


@router.get("/health")
async def health_check():
    return {
        "status": "healthy",
        "service": "fieldforge-ai",
        "version": "0.1.0",
    }
