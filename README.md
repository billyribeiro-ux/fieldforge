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

## API Routes (35 modules)

| Module | Endpoints |
|--------|-----------|
| `health` | `GET /health`, `GET /health/ready` (DB + Redis checks) |
| `auth` | Register, login, `/auth/me` (JWT + Argon2) |
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
| `expenses` | CRUD, filters by category/date/billable |
| `messages` | CRUD, conversations |
| `reviews` | CRUD, customer reviews |
| `service_plans` | CRUD, maintenance agreements |
| `search` | Global search across jobs, customers, estimates, invoices |
| `audit` | Paginated audit log with entity/user/action filters |
| `webhooks` | CRUD, HMAC secret generation, test endpoint |
| `notifications` | List, mark read, mark all read, unread count |
| `payments` | List payments, get payment, refund with reason |
| `automation_rules` | CRUD, toggle active/inactive |
| `documents` | CRUD, signatures |
| `licenses` | CRUD licenses + insurance policies |
| `recurring_rules` | CRUD, toggle, generate next occurrence |
| `tags` | CRUD tags |
| `fuel_logs` | CRUD fuel logs per vehicle |
| `purchase_orders` | CRUD purchase orders + line items |
| `gps` | GPS location tracking, history |
| `portal` | Public estimate/invoice portal (token-based) |
| `stripe` | Stripe webhook handler, checkout sessions |
| `ws` | WebSocket real-time events |

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

### UI Components (24)
Avatar, Badge, Breadcrumb, Button, Card, CommandPalette (⌘K), ConfirmDialog, DataTable, Dropdown, EmptyState, ErrorBoundary, FileUpload, Input, Modal, Pagination, Progress, Select, Skeleton, StatusBadge, Switch, Tabs, Textarea, Toast, Tooltip

### Middleware (4)
Auth (JWT), CORS, Request ID, Rate Limiting (100 req/60s per IP)

### Server-Side Features
- `hooks.server.ts` — auth token extraction from cookies, user profile fetch
- `+layout.server.ts` — dashboard auth guard, user data propagation
- `+page.server.ts` — SSR load functions for ALL dashboard pages (parallel fetch with `Promise.allSettled`)
- **Form Actions** — server-side mutations for jobs, customers, estimates, invoices (create, update, delete, status transitions, notes, properties, payments)
- **Server API Client** — `serverFetch`, `serverPost`, `serverPatch`, `serverDelete` helpers

### Reactive Stores (Svelte 5)
- `auth.svelte.ts` — JWT token, user state
- `toast.svelte.ts` — toast notification system
- `notifications.svelte.ts` — fetch, mark read, polling (30s)
- `loading.svelte.ts` — ref-counted loading state

### Utilities
`format.ts` (currency, date, phone, address), `validate.ts`, `debounce.ts`, `storage.ts`, `theme.ts`, `cn.ts`

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
- [x] **Phase 8** — Responsive mobile sidebar, form validation utility
- [x] **Phase 9** — Request ID middleware, formatting utilities (currency, date, phone, address)
- [x] **Phase 10** — EmptyState, Avatar, Breadcrumb components, enhanced health check
- [x] **Phase 11** — Dropdown, Tabs, Pagination, StatusBadge components
- [x] **Phase 12** — Tooltip, Progress, Switch components, barrel export (21 components)
- [x] **Phase 13** — Rate limiting middleware, global error page
- [x] **Phase 14** — Server-side auth (hooks.server.ts, layout guard, app.d.ts types)
- [x] **Phase 15** — SSR load functions (jobs, customers, estimates, invoices, dashboard)
- [x] **Phase 16** — /auth/me endpoint, verify_token, server API client
- [x] **Phase 17** — Final types (AuditLog, Webhook, SearchResults), rate limiting wired, README update
- [x] **Phase 18** — DataTable, FileUpload, ConfirmDialog components (24 UI components)
- [x] **Phase 19** — Debounce/throttle, localStorage utility, database seed script
- [x] **Phase 20** — SEO meta tags, Dockerfiles (already existed)
- [x] **Phase 21** — SSR load functions for schedule, inventory, fleet, reports + theme constants
- [x] **Phase 22** — SSR load for estimate/invoice detail, settings, marketing @const fix
- [x] **Phase 23** — Dashboard nested error page, estimate builder SSR load
- [x] **Phase 24** — Payments route wired (24 API modules), fixes
- [x] **Phase 25** — NotificationStore, LoadingStore (Svelte 5 reactive classes), polling
- [x] **Phase 26** — Web .env.example, README final update
- [x] **Phase 27** — SvelteKit form actions, serverPost/serverPatch helpers
- [x] **Phase 28** — Form actions for estimates (line items), job status transitions, notes
- [x] **Phase 29** — Comprehensive form actions: customer update/property/note, estimate send/approve/decline/convert, invoice send/void/payment
- [x] **Phase 30** — serverDelete helper, invoice creation form action, complete server API client
- [x] **Phase 31** — README final architecture update (24 components, stores, utilities)
- [x] **Phase 32** — robots.txt SEO, OpenAPI 3.1 spec, `<svelte:head>` page titles
- [x] **Phase 33** — (merged into Phase 32)
- [x] **Phase 34** — TypeScript types for form actions and page data (~849 lines)
- [x] **Phase 35** — Keyboard shortcut utility, README status update
- [x] **Phase 36** — `<svelte:head>` titles for all detail pages (14 total)
- [x] **Phase 37** — Keyboard shortcuts wired into layout, navigation loading bar
- [x] **Phase 38** — documents, recurring_rules, tags routes wired (30 → 35 API modules)
- [x] **Phase 39** — README updated with 35 API route modules
- [x] **Phase 40** — Skip-to-content accessibility link, final polish
- [ ] **Phase 41** — Mobile apps (iOS + Android)
- [ ] **Phase 42** — AI/ML features (photo estimation, smart scheduling)
- [ ] **Phase 43** — Production deployment, performance tuning, security audit

## License

Proprietary — All rights reserved.
