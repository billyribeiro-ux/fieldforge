# FieldForge

**The #1 Universal Job Management Platform for Tradespeople**

Built for HVAC, plumbing, electrical, roofing, painting, landscaping, and general contracting businesses. FieldForge handles everything from lead capture to final payment — jobs, scheduling, estimates, invoices, inventory, fleet, and team management in one platform.

---

## Architecture

```
fieldforge/
├── apps/
│   ├── api/             # Rust/Axum backend — 19 route modules
│   ├── web/             # SvelteKit 5 dashboard + customer portal
│   └── ai/              # Python/FastAPI AI/ML services
├── infrastructure/
│   └── terraform/       # AWS infrastructure (VPC, RDS, ECS, S3, CloudFront)
├── mobile/
│   ├── ios/             # Swift/SwiftUI (placeholder)
│   └── android/         # Kotlin/Compose (placeholder)
└── .github/workflows/   # CI/CD (GitHub Actions)
```

## Tech Stack

| Layer | Technology |
|-------|-----------|
| **Backend API** | Rust 1.75+ / Axum 0.7 / SQLx / PostgreSQL 16 |
| **Web Dashboard** | SvelteKit 5 / Svelte 5 runes / TypeScript / Tailwind CSS 4 |
| **AI/ML Services** | Python 3.12+ / FastAPI / PyTorch / ONNX |
| **Database** | PostgreSQL 16 (~50 tables) / Redis 7 |
| **Storage** | S3-compatible (MinIO local / AWS S3 prod) |
| **Search** | Meilisearch (stub — PostgreSQL ILIKE fallback) |
| **Payments** | Stripe (async-stripe crate) |
| **Infrastructure** | AWS ECS Fargate, RDS, ElastiCache, S3, CloudFront |
| **CI/CD** | GitHub Actions (Rust, Node, Python) |

## API Routes (19 modules)

| Module | Endpoints |
|--------|-----------|
| `health` | `GET /health` |
| `auth` | Register, login (JWT + Argon2) |
| `customers` | CRUD, search, stats |
| `jobs` | CRUD, FSM status transitions (14 states) |
| `estimates` | CRUD, line items, send/approve/decline, convert to invoice, duplicate |
| `invoices` | CRUD, send/void, payment recording |
| `time_entries` | Start/stop timer, manual entry, active timers |
| `photos` | S3 presigned URLs, CRUD, categories |
| `properties` | CRUD per customer, types, access instructions |
| `notes` | CRUD per job/customer, internal/external |
| `teams` | Get/update team, invite/update/deactivate members |
| `inventory` | Items CRUD, locations, stock adjustment, transactions |
| `vehicles` | CRUD, maintenance tracking |
| `checklists` | CRUD, item completion |
| `equipment` | CRUD, assignment |
| `search` | Global search across jobs, customers, estimates, invoices |
| `audit` | Paginated audit log with filters |
| `webhooks` | CRUD, HMAC secret generation, test endpoint |
| `notifications` | List, mark read, mark all read, unread count |

## Web Dashboard Pages

| Page | Features |
|------|----------|
| **Dashboard** | KPI stats, recent jobs, quick actions |
| **Jobs** | List + filters, detail page (status pipeline, checklist, timer, photos, notes), create modal |
| **Customers** | List + search, detail page (tabs: overview, jobs, invoices, communication, properties), create modal |
| **Estimates** | List, detail page (document view, portal link, activity), builder (line items, live totals, discount/tax) |
| **Invoices** | List, detail page (document view, payment recording modal, portal link) |
| **Schedule** | Weekly calendar view |
| **Inventory** | Stock tracking, low stock alerts, category filters |
| **Fleet** | Vehicle cards, service due alerts, mileage tracking |
| **Reports** | KPIs, revenue by trade, top customers, technician performance |
| **Settings** | Company profile, team members, billing, notifications, integrations, templates, appearance, security |

### Customer Portal (public, no auth)
- **Estimate view** — approve/decline with optional reason
- **Invoice view** — online payment (card, ACH, Apple Pay, Google Pay)

### UI Components
Button, Badge, Card, Input, Select, Textarea, Modal, Sidebar, TopBar (with notification dropdown), StatCard, Skeleton, ErrorBoundary, Toast, CommandPalette (⌘K)

## Getting Started

### Prerequisites

- Rust 1.75+ (`rustup`)
- Node.js 20+ / pnpm 9+
- Python 3.12+
- Docker & Docker Compose

### Quick Start

```bash
# 1. Start infrastructure
docker compose up -d   # postgres, redis, meilisearch, minio

# 2. Run database migrations
cd apps/api && sqlx migrate run

# 3. Start API server
cd apps/api && cargo run

# 4. Start web dashboard
cd apps/web && pnpm install && pnpm dev

# 5. Start AI services (optional)
cd apps/ai && pip install -r requirements.txt && uvicorn main:app --reload
```

### Environment Variables

Copy `.env.example` to `.env` and configure:

```env
DATABASE_URL=postgres://fieldforge:fieldforge@localhost:5432/fieldforge
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-secret-key
S3_ENDPOINT=http://localhost:9000
S3_BUCKET=fieldforge-uploads
STRIPE_SECRET_KEY=sk_test_...
```

### Build Verification

```bash
# Rust API
cd apps/api && SQLX_OFFLINE=true cargo check

# SvelteKit
cd apps/web && pnpm build
```

## Database Schema

~50 tables covering: teams, users, customers, properties, jobs (with 14-state FSM), estimates, invoices, payments, line items, time entries, photos, notes, checklists, inventory (items, locations, stock, transactions), vehicles, equipment, messages, automation rules, reviews, licenses, insurance, service plans, expenses, audit log, webhooks, notifications.

## Implementation Status

- [x] **Phase 1** — Foundation (monorepo, DB schema, auth, Docker, Terraform, CI/CD)
- [x] **Phase 2** — Core Product (all CRUD routes, dashboard pages, forms, detail pages)
- [x] **Phase 3** — Estimate detail, TypeScript types, Svelte 5 fixes
- [x] **Phase 4** — Portals, inventory/fleet pages, photo/team routes
- [x] **Phase 5** — Search, audit, webhook routes, notification dropdown
- [x] **Phase 6** — API service layer, toast system, command palette, notification routes
- [x] **Phase 7** — Skeleton loader, error boundary, comprehensive README
- [ ] **Phase 8** — Mobile apps (iOS + Android)
- [ ] **Phase 9** — AI/ML features (photo estimation, smart scheduling)
- [ ] **Phase 10** — Production deployment, performance tuning, security audit

## License

Proprietary — All rights reserved.
