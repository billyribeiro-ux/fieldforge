use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
    pub auth: AuthSettings,
    pub stripe: StripeSettings,
    pub twilio: TwilioSettings,
    pub sendgrid: SendGridSettings,
    pub storage: StorageSettings,
    pub meilisearch: MeilisearchSettings,
    pub ai: AiSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub env: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisSettings {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthSettings {
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub refresh_token_expiry_days: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StripeSettings {
    pub secret_key: String,
    pub publishable_key: String,
    pub webhook_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TwilioSettings {
    pub account_sid: String,
    pub auth_token: String,
    pub phone_number: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendGridSettings {
    pub api_key: String,
    pub from_email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageSettings {
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeilisearchSettings {
    pub url: String,
    pub api_key: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiSettings {
    pub service_url: String,
}

impl Settings {
    pub fn from_env() -> Result<Self> {
        let settings = Self {
            server: ServerSettings {
                host: std::env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
                port: std::env::var("API_PORT")
                    .unwrap_or_else(|_| "8080".into())
                    .parse()?,
                env: std::env::var("API_ENV").unwrap_or_else(|_| "development".into()),
            },
            database: DatabaseSettings {
                url: std::env::var("DATABASE_URL")?,
                max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "20".into())
                    .parse()?,
                min_connections: std::env::var("DATABASE_MIN_CONNECTIONS")
                    .unwrap_or_else(|_| "5".into())
                    .parse()?,
            },
            redis: RedisSettings {
                url: std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".into()),
            },
            auth: AuthSettings {
                jwt_secret: std::env::var("JWT_SECRET")?,
                jwt_expiry_hours: std::env::var("JWT_EXPIRY_HOURS")
                    .unwrap_or_else(|_| "24".into())
                    .parse()?,
                refresh_token_expiry_days: std::env::var("REFRESH_TOKEN_EXPIRY_DAYS")
                    .unwrap_or_else(|_| "30".into())
                    .parse()?,
            },
            stripe: StripeSettings {
                secret_key: std::env::var("STRIPE_SECRET_KEY").unwrap_or_default(),
                publishable_key: std::env::var("STRIPE_PUBLISHABLE_KEY").unwrap_or_default(),
                webhook_secret: std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default(),
            },
            twilio: TwilioSettings {
                account_sid: std::env::var("TWILIO_ACCOUNT_SID").unwrap_or_default(),
                auth_token: std::env::var("TWILIO_AUTH_TOKEN").unwrap_or_default(),
                phone_number: std::env::var("TWILIO_PHONE_NUMBER").unwrap_or_default(),
            },
            sendgrid: SendGridSettings {
                api_key: std::env::var("SENDGRID_API_KEY").unwrap_or_default(),
                from_email: std::env::var("SENDGRID_FROM_EMAIL")
                    .unwrap_or_else(|_| "noreply@fieldforge.com".into()),
            },
            storage: StorageSettings {
                bucket: std::env::var("S3_BUCKET").unwrap_or_else(|_| "fieldforge-uploads".into()),
                region: std::env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".into()),
                endpoint: std::env::var("S3_ENDPOINT")
                    .unwrap_or_else(|_| "http://localhost:9000".into()),
                access_key: std::env::var("S3_ACCESS_KEY").unwrap_or_default(),
                secret_key: std::env::var("S3_SECRET_KEY").unwrap_or_default(),
            },
            meilisearch: MeilisearchSettings {
                url: std::env::var("MEILISEARCH_URL")
                    .unwrap_or_else(|_| "http://localhost:7700".into()),
                api_key: std::env::var("MEILISEARCH_API_KEY")
                    .unwrap_or_else(|_| "masterKey".into()),
            },
            ai: AiSettings {
                service_url: std::env::var("AI_SERVICE_URL")
                    .unwrap_or_else(|_| "http://localhost:8000".into()),
            },
        };

        Ok(settings)
    }

    pub fn is_production(&self) -> bool {
        self.server.env == "production"
    }
}
