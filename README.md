# FieldForge

**The #1 Universal Job Management Platform for Tradespeople**

## Architecture

```
fieldforge/
├── apps/
│   ├── api/          # Rust/Axum backend API
│   ├── web/          # SvelteKit 5 admin dashboard + customer portal
│   └── ai/           # Python/FastAPI AI/ML services
├── services/
│   ├── scheduler/    # Smart scheduling microservice
│   └── notifications/# Push, SMS, email notification service
├── packages/
│   ├── shared-types/ # Shared TypeScript types
│   └── db-schema/    # Database migrations (SQLx)
├── infrastructure/
│   ├── terraform/    # AWS infrastructure as code
│   └── docker/       # Docker configurations
├── mobile/
│   ├── ios/          # Swift/SwiftUI iOS app
│   └── android/      # Kotlin/Compose Android app
├── docs/             # Documentation
└── .github/workflows/# CI/CD pipelines
```

## Tech Stack

| Layer | Technology |
|-------|-----------|
| **Backend API** | Rust 1.75+ / Axum / SQLx / PostgreSQL |
| **Web Dashboard** | SvelteKit 5 / Svelte 5 runes / TypeScript strict / Tailwind CSS 4 |
| **AI/ML Services** | Python 3.12+ / FastAPI / PyTorch / ONNX |
| **iOS** | Swift 6 / SwiftUI / SQLite (offline) |
| **Android** | Kotlin / Jetpack Compose / Room (offline) |
| **Database** | PostgreSQL 16 / Redis 7 |
| **Storage** | S3-compatible (Cloudflare R2 / AWS S3) |
| **Search** | Meilisearch |
| **Payments** | Stripe (primary) / Square |
| **SMS** | Twilio |
| **Email** | SendGrid |
| **Infrastructure** | AWS (ECS Fargate, RDS, ElastiCache, S3, CloudFront) |
| **CI/CD** | GitHub Actions |
| **Monitoring** | Grafana / Sentry / OpenTelemetry |

## Getting Started

### Prerequisites

- Rust 1.75+ (`rustup`)
- Node.js 20+ / pnpm 9+
- Python 3.12+
- Docker & Docker Compose
- PostgreSQL 16 (or use Docker)

### Local Development

```bash
# Start infrastructure (Postgres, Redis, Meilisearch)
docker compose up -d

# Run API server
cd apps/api && cargo run

# Run web dashboard
cd apps/web && pnpm dev

# Run AI services
cd apps/ai && python -m uvicorn main:app --reload
```

## Implementation Phases

1. **Foundation** — Infrastructure, DB schema, API core with auth
2. **Core Product** — Jobs, customers, estimates, invoices
3. **Mobile** — iOS + Android (parallel)
4. **Web** — Admin dashboard + customer portal
5. **AI & Intelligence** — Photo estimation, smart scheduling
6. **Advanced** — Inventory, fleet, analytics
7. **Polish & Launch** — Performance, security audit, beta

## License

Proprietary — All rights reserved.
