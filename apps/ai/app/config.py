from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    ENV: str = "development"
    ALLOWED_ORIGIN: str = "https://app.fieldforge.com"

    # Database
    DATABASE_URL: str = "postgresql+asyncpg://fieldforge:fieldforge@localhost:5432/fieldforge"

    # Redis
    REDIS_URL: str = "redis://localhost:6379"

    # S3 Storage
    S3_BUCKET: str = "fieldforge-uploads"
    S3_REGION: str = "us-east-1"
    S3_ENDPOINT: str = "http://localhost:9000"
    S3_ACCESS_KEY: str = ""
    S3_SECRET_KEY: str = ""

    # OpenAI
    OPENAI_API_KEY: str = ""
    OPENAI_MODEL: str = "gpt-4o"

    # ML Models
    PHOTO_MODEL_PATH: str = "models/photo_estimation"
    SCHEDULING_MODEL_PATH: str = "models/scheduling"

    # API Auth
    API_SECRET_KEY: str = "change-me"

    model_config = {"env_file": ".env", "extra": "ignore"}


settings = Settings()
