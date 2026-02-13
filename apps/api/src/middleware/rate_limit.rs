use axum::{extract::Request, middleware::Next, response::Response};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Simple in-memory rate limiter (production should use Redis)
#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window: Duration::from_secs(window_secs),
        }
    }

    async fn check(&self, key: &str) -> bool {
        let mut map = self.requests.lock().await;
        let now = Instant::now();
        let cutoff = now - self.window;

        let entries = map.entry(key.to_string()).or_default();
        entries.retain(|t| *t > cutoff);

        if entries.len() >= self.max_requests {
            return false;
        }

        entries.push(now);
        true
    }
}

pub async fn rate_limit_middleware(req: Request, next: Next) -> Response {
    // Extract client IP from headers or connection info
    let client_ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("unknown").trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Use a static rate limiter (100 requests per 60 seconds per IP)
    static LIMITER: std::sync::OnceLock<RateLimiter> = std::sync::OnceLock::new();
    let limiter = LIMITER.get_or_init(|| RateLimiter::new(100, 60));

    if !limiter.check(&client_ip).await {
        tracing::warn!(client_ip = %client_ip, "Rate limit exceeded");

        return Response::builder()
            .status(429)
            .header("content-type", "application/json")
            .header("retry-after", "60")
            .body(axum::body::Body::from(
                r#"{"errors":[{"code":"RATE_LIMITED","message":"Too many requests. Please try again later."}]}"#,
            ))
            .unwrap();
    }

    next.run(req).await
}
