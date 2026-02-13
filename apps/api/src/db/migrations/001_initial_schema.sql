-- FieldForge Complete Database Schema
-- PostgreSQL 16 — Normalized, multi-tenant, production-grade
-- Every table: id (UUID v7), created_at, updated_at, soft delete where applicable

-- ============================================================
-- EXTENSIONS
-- ============================================================
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- ============================================================
-- CUSTOM TYPES
-- ============================================================
CREATE TYPE user_role AS ENUM ('owner', 'admin', 'technician', 'apprentice', 'office_manager');
CREATE TYPE plan_tier AS ENUM ('free_trial', 'solo', 'crew', 'business', 'enterprise');
CREATE TYPE job_status AS ENUM ('lead', 'estimated', 'approved', 'scheduled', 'en_route', 'in_progress', 'paused', 'completed', 'invoiced', 'paid', 'closed', 'cancelled');
CREATE TYPE job_priority AS ENUM ('emergency', 'high', 'normal', 'low');
CREATE TYPE property_type AS ENUM ('residential', 'commercial', 'industrial', 'government', 'hoa');
CREATE TYPE estimate_status AS ENUM ('draft', 'sent', 'viewed', 'approved', 'declined', 'expired', 'converted');
CREATE TYPE invoice_status AS ENUM ('draft', 'sent', 'viewed', 'partially_paid', 'paid', 'overdue', 'void');
CREATE TYPE payment_status AS ENUM ('pending', 'processing', 'succeeded', 'failed', 'refunded', 'partially_refunded', 'disputed');
CREATE TYPE payment_method AS ENUM ('card', 'ach', 'apple_pay', 'google_pay', 'cash', 'check', 'venmo', 'zelle', 'financing', 'other');
CREATE TYPE line_item_category AS ENUM ('labor', 'materials', 'equipment', 'permits', 'disposal', 'overhead', 'other');
CREATE TYPE time_entry_type AS ENUM ('work', 'travel', 'break', 'overtime');
CREATE TYPE photo_category AS ENUM ('before', 'during', 'after', 'damage', 'measurement', 'general');
CREATE TYPE note_type AS ENUM ('text', 'voice', 'system');
CREATE TYPE checklist_type AS ENUM ('safety', 'quality', 'inspection', 'custom');
CREATE TYPE contact_method AS ENUM ('sms', 'email', 'phone', 'any');
CREATE TYPE credit_terms AS ENUM ('cod', 'net_15', 'net_30', 'net_45', 'net_60');
CREATE TYPE sync_status AS ENUM ('pending', 'syncing', 'synced', 'conflict');
CREATE TYPE vehicle_status AS ENUM ('active', 'maintenance', 'retired');
CREATE TYPE inventory_txn_type AS ENUM ('purchase', 'usage', 'transfer_in', 'transfer_out', 'adjustment', 'return', 'shrinkage');
CREATE TYPE po_status AS ENUM ('draft', 'sent', 'partially_received', 'received', 'closed', 'cancelled');
CREATE TYPE equipment_condition AS ENUM ('good', 'fair', 'needs_repair', 'out_of_service');
CREATE TYPE recurring_frequency AS ENUM ('daily', 'weekly', 'biweekly', 'monthly', 'quarterly', 'semi_annual', 'annual', 'custom');
CREATE TYPE message_channel AS ENUM ('sms', 'email', 'push', 'in_app');
CREATE TYPE message_status AS ENUM ('queued', 'sent', 'delivered', 'failed', 'bounced', 'opened', 'clicked');

-- ============================================================
-- TEAMS (multi-tenant root)
-- ============================================================
CREATE TABLE teams (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            TEXT NOT NULL,
    slug            TEXT NOT NULL UNIQUE,
    owner_id        UUID, -- FK set after users table
    logo_url        TEXT,
    phone           TEXT,
    email           TEXT,
    website         TEXT,
    address_line1   TEXT,
    address_line2   TEXT,
    city            TEXT,
    state           TEXT,
    zip_code        TEXT,
    country         TEXT NOT NULL DEFAULT 'US',
    timezone        TEXT NOT NULL DEFAULT 'America/New_York',
    default_hourly_rate   NUMERIC(10,2),
    default_markup_pct    NUMERIC(5,2),
    tax_rate              NUMERIC(5,4),
    primary_trade         TEXT,
    service_radius_miles  INT,
    plan_tier             plan_tier NOT NULL DEFAULT 'free_trial',
    trial_ends_at         TIMESTAMPTZ,
    stripe_customer_id    TEXT,
    stripe_subscription_id TEXT,
    invoice_prefix        TEXT DEFAULT 'FF',
    invoice_next_number   INT NOT NULL DEFAULT 1,
    estimate_prefix       TEXT DEFAULT 'EST',
    estimate_next_number  INT NOT NULL DEFAULT 1,
    working_hours_start   TIME DEFAULT '08:00',
    working_hours_end     TIME DEFAULT '17:00',
    working_days          INT[] DEFAULT '{1,2,3,4,5}', -- 0=Sun, 6=Sat
    is_active             BOOLEAN NOT NULL DEFAULT true,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_teams_slug ON teams(slug);
CREATE INDEX idx_teams_owner ON teams(owner_id);

-- ============================================================
-- USERS
-- ============================================================
CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID REFERENCES teams(id) ON DELETE SET NULL,
    email           TEXT,
    phone           TEXT,
    password_hash   TEXT NOT NULL,
    first_name      TEXT NOT NULL,
    last_name       TEXT NOT NULL,
    avatar_url      TEXT,
    role            user_role NOT NULL DEFAULT 'owner',
    trade           TEXT,
    hourly_rate     NUMERIC(10,2),
    skills          TEXT[],
    certifications  TEXT[],
    languages       TEXT[] DEFAULT '{en}',
    is_active       BOOLEAN NOT NULL DEFAULT true,
    email_verified  BOOLEAN NOT NULL DEFAULT false,
    phone_verified  BOOLEAN NOT NULL DEFAULT false,
    last_login_at   TIMESTAMPTZ,
    last_location_lat   DOUBLE PRECISION,
    last_location_lng   DOUBLE PRECISION,
    last_location_at    TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(email),
    UNIQUE(phone)
);

ALTER TABLE teams ADD CONSTRAINT fk_teams_owner FOREIGN KEY (owner_id) REFERENCES users(id);

CREATE INDEX idx_users_team ON users(team_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_phone ON users(phone);

-- ============================================================
-- REFRESH TOKENS
-- ============================================================
CREATE TABLE refresh_tokens (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash  TEXT NOT NULL UNIQUE,
    expires_at  TIMESTAMPTZ NOT NULL,
    revoked_at  TIMESTAMPTZ,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_refresh_tokens_user ON refresh_tokens(user_id);

-- ============================================================
-- CUSTOMERS
-- ============================================================
CREATE TABLE customers (
    id                      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id                 UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    first_name              TEXT NOT NULL,
    last_name               TEXT NOT NULL,
    email                   TEXT,
    phone                   TEXT,
    phone_secondary         TEXT,
    company_name            TEXT,
    preferred_contact_method contact_method NOT NULL DEFAULT 'sms',
    preferred_technician_id UUID REFERENCES users(id) ON DELETE SET NULL,
    credit_terms            credit_terms NOT NULL DEFAULT 'cod',
    tax_exempt              BOOLEAN NOT NULL DEFAULT false,
    tax_exempt_number       TEXT,
    do_not_service          BOOLEAN NOT NULL DEFAULT false,
    do_not_service_reason   TEXT,
    do_not_contact          BOOLEAN NOT NULL DEFAULT false,
    referral_source         TEXT,
    referred_by_customer_id UUID REFERENCES customers(id) ON DELETE SET NULL,
    notes_pinned            TEXT,
    tags                    TEXT[] DEFAULT '{}',
    portal_token            TEXT UNIQUE,
    lifetime_value          NUMERIC(12,2) NOT NULL DEFAULT 0,
    outstanding_balance     NUMERIC(12,2) NOT NULL DEFAULT 0,
    total_jobs              INT NOT NULL DEFAULT 0,
    last_service_date       DATE,
    is_active               BOOLEAN NOT NULL DEFAULT true,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at              TIMESTAMPTZ
);

CREATE INDEX idx_customers_team ON customers(team_id);
CREATE INDEX idx_customers_phone ON customers(team_id, phone);
CREATE INDEX idx_customers_email ON customers(team_id, email);
CREATE INDEX idx_customers_name ON customers(team_id, last_name, first_name);
CREATE INDEX idx_customers_search ON customers USING gin (
    (first_name || ' ' || last_name || ' ' || COALESCE(company_name, '') || ' ' || COALESCE(email, '') || ' ' || COALESCE(phone, ''))
    gin_trgm_ops
);
CREATE INDEX idx_customers_portal_token ON customers(portal_token) WHERE portal_token IS NOT NULL;

-- ============================================================
-- PROPERTIES
-- ============================================================
CREATE TABLE properties (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id         UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    team_id             UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    address_line1       TEXT NOT NULL,
    address_line2       TEXT,
    city                TEXT NOT NULL,
    state               TEXT NOT NULL,
    zip_code            TEXT NOT NULL,
    country             TEXT NOT NULL DEFAULT 'US',
    latitude            DOUBLE PRECISION,
    longitude           DOUBLE PRECISION,
    property_type       property_type NOT NULL DEFAULT 'residential',
    square_footage      INT,
    year_built          INT,
    stories             INT,
    access_notes        TEXT,
    gate_code_encrypted TEXT,
    lockbox_code_encrypted TEXT,
    alarm_code_encrypted   TEXT,
    pet_info            TEXT,
    is_primary          BOOLEAN NOT NULL DEFAULT false,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_properties_customer ON properties(customer_id);
CREATE INDEX idx_properties_team ON properties(team_id);
CREATE INDEX idx_properties_location ON properties(latitude, longitude) WHERE latitude IS NOT NULL;

-- ============================================================
-- EQUIPMENT REGISTRY (per property)
-- ============================================================
CREATE TABLE property_equipment (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    property_id     UUID NOT NULL REFERENCES properties(id) ON DELETE CASCADE,
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    brand           TEXT,
    model           TEXT,
    serial_number   TEXT,
    install_date    DATE,
    warranty_expiry DATE,
    last_service_date DATE,
    next_service_date DATE,
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_property_equipment_property ON property_equipment(property_id);

-- ============================================================
-- RECURRING RULES
-- ============================================================
CREATE TABLE recurring_rules (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    customer_id     UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    frequency       recurring_frequency NOT NULL,
    interval_value  INT NOT NULL DEFAULT 1,
    day_of_week     INT,
    day_of_month    INT,
    month_of_year   INT,
    start_date      DATE NOT NULL,
    end_date        DATE,
    max_occurrences INT,
    occurrences_created INT NOT NULL DEFAULT 0,
    job_template    JSONB NOT NULL,
    assigned_to     UUID REFERENCES users(id) ON DELETE SET NULL,
    is_active       BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_recurring_rules_team ON recurring_rules(team_id);

-- ============================================================
-- JOBS
-- ============================================================
CREATE TABLE jobs (
    id                      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id                 UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    customer_id             UUID NOT NULL REFERENCES customers(id) ON DELETE RESTRICT,
    property_id             UUID REFERENCES properties(id) ON DELETE SET NULL,
    assigned_to             UUID REFERENCES users(id) ON DELETE SET NULL,
    parent_job_id           UUID REFERENCES jobs(id) ON DELETE SET NULL,
    recurring_rule_id       UUID REFERENCES recurring_rules(id) ON DELETE SET NULL,
    title                   TEXT NOT NULL,
    description             TEXT,
    status                  job_status NOT NULL DEFAULT 'lead',
    priority                job_priority NOT NULL DEFAULT 'normal',
    job_type                TEXT,
    trade                   TEXT,
    source                  TEXT,
    scheduled_date          DATE,
    scheduled_start_time    TIME,
    scheduled_end_time      TIME,
    arrival_window_start    TIME,
    arrival_window_end      TIME,
    estimated_duration_minutes INT,
    actual_duration_minutes    INT,
    started_at              TIMESTAMPTZ,
    completed_at            TIMESTAMPTZ,
    budget_amount           NUMERIC(12,2),
    total_amount            NUMERIC(12,2),
    internal_notes          TEXT,
    access_instructions     TEXT,
    permit_required         BOOLEAN NOT NULL DEFAULT false,
    warranty_job            BOOLEAN NOT NULL DEFAULT false,
    po_number               TEXT,
    customer_signature      TEXT,
    customer_signed_at      TIMESTAMPTZ,
    customer_rating         SMALLINT CHECK (customer_rating BETWEEN 1 AND 5),
    customer_feedback       TEXT,
    tags                    TEXT[] DEFAULT '{}',
    custom_fields           JSONB DEFAULT '{}',
    version                 INT NOT NULL DEFAULT 1,
    sync_status             sync_status NOT NULL DEFAULT 'synced',
    synced_at               TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at              TIMESTAMPTZ
);

CREATE INDEX idx_jobs_team ON jobs(team_id);
CREATE INDEX idx_jobs_customer ON jobs(customer_id);
CREATE INDEX idx_jobs_assigned ON jobs(assigned_to);
CREATE INDEX idx_jobs_status ON jobs(team_id, status);
CREATE INDEX idx_jobs_scheduled ON jobs(team_id, scheduled_date) WHERE scheduled_date IS NOT NULL;
CREATE INDEX idx_jobs_date_status ON jobs(team_id, scheduled_date, status);
CREATE INDEX idx_jobs_parent ON jobs(parent_job_id) WHERE parent_job_id IS NOT NULL;
CREATE INDEX idx_jobs_search ON jobs USING gin (
    (title || ' ' || COALESCE(description, '') || ' ' || COALESCE(po_number, ''))
    gin_trgm_ops
);
CREATE INDEX idx_jobs_created ON jobs(team_id, created_at DESC);

-- ============================================================
-- JOB STATUS AUDIT TRAIL
-- ============================================================
CREATE TABLE job_status_history (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id          UUID NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,
    from_status     job_status,
    to_status       job_status NOT NULL,
    changed_by      UUID REFERENCES users(id) ON DELETE SET NULL,
    latitude        DOUBLE PRECISION,
    longitude       DOUBLE PRECISION,
    note            TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_job_status_history_job ON job_status_history(job_id);

-- ============================================================
-- ESTIMATES
-- ============================================================
CREATE TABLE estimates (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id             UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    job_id              UUID REFERENCES jobs(id) ON DELETE SET NULL,
    customer_id         UUID NOT NULL REFERENCES customers(id) ON DELETE RESTRICT,
    property_id         UUID REFERENCES properties(id) ON DELETE SET NULL,
    estimate_number     TEXT NOT NULL,
    version             INT NOT NULL DEFAULT 1,
    status              estimate_status NOT NULL DEFAULT 'draft',
    title               TEXT,
    scope_of_work       TEXT,
    subtotal            NUMERIC(12,2) NOT NULL DEFAULT 0,
    discount_amount     NUMERIC(12,2) NOT NULL DEFAULT 0,
    discount_pct        NUMERIC(5,2),
    tax_amount          NUMERIC(12,2) NOT NULL DEFAULT 0,
    tax_rate            NUMERIC(5,4),
    total               NUMERIC(12,2) NOT NULL DEFAULT 0,
    deposit_required_pct NUMERIC(5,2),
    deposit_amount      NUMERIC(12,2),
    margin_pct          NUMERIC(5,2),
    internal_cost       NUMERIC(12,2),
    valid_until         DATE,
    payment_terms       TEXT,
    warranty_terms      TEXT,
    terms_and_conditions TEXT,
    customer_signature  TEXT,
    signed_at           TIMESTAMPTZ,
    sent_at             TIMESTAMPTZ,
    viewed_at           TIMESTAMPTZ,
    approved_at         TIMESTAMPTZ,
    declined_at         TIMESTAMPTZ,
    decline_reason      TEXT,
    portal_token        TEXT NOT NULL UNIQUE DEFAULT encode(gen_random_bytes(32), 'hex'),
    pdf_url             TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at          TIMESTAMPTZ
);

CREATE INDEX idx_estimates_team ON estimates(team_id);
CREATE INDEX idx_estimates_customer ON estimates(customer_id);
CREATE INDEX idx_estimates_job ON estimates(job_id) WHERE job_id IS NOT NULL;
CREATE INDEX idx_estimates_status ON estimates(team_id, status);
CREATE INDEX idx_estimates_number ON estimates(team_id, estimate_number);
CREATE INDEX idx_estimates_portal ON estimates(portal_token);

-- ============================================================
-- INVOICES
-- ============================================================
CREATE TABLE invoices (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id             UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    job_id              UUID REFERENCES jobs(id) ON DELETE SET NULL,
    estimate_id         UUID REFERENCES estimates(id) ON DELETE SET NULL,
    customer_id         UUID NOT NULL REFERENCES customers(id) ON DELETE RESTRICT,
    property_id         UUID REFERENCES properties(id) ON DELETE SET NULL,
    invoice_number      TEXT NOT NULL,
    status              invoice_status NOT NULL DEFAULT 'draft',
    subtotal            NUMERIC(12,2) NOT NULL DEFAULT 0,
    discount_amount     NUMERIC(12,2) NOT NULL DEFAULT 0,
    tax_amount          NUMERIC(12,2) NOT NULL DEFAULT 0,
    tax_rate            NUMERIC(5,4),
    total               NUMERIC(12,2) NOT NULL DEFAULT 0,
    amount_paid         NUMERIC(12,2) NOT NULL DEFAULT 0,
    amount_due          NUMERIC(12,2) NOT NULL DEFAULT 0,
    due_date            DATE,
    payment_terms       TEXT,
    late_fee_rate       NUMERIC(5,2),
    late_fee_amount     NUMERIC(12,2),
    late_fee_applied    BOOLEAN NOT NULL DEFAULT false,
    notes               TEXT,
    terms_and_conditions TEXT,
    po_number           TEXT,
    portal_token        TEXT NOT NULL UNIQUE DEFAULT encode(gen_random_bytes(32), 'hex'),
    pdf_url             TEXT,
    sent_at             TIMESTAMPTZ,
    viewed_at           TIMESTAMPTZ,
    paid_at             TIMESTAMPTZ,
    voided_at           TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    deleted_at          TIMESTAMPTZ
);

CREATE INDEX idx_invoices_team ON invoices(team_id);
CREATE INDEX idx_invoices_customer ON invoices(customer_id);
CREATE INDEX idx_invoices_job ON invoices(job_id) WHERE job_id IS NOT NULL;
CREATE INDEX idx_invoices_status ON invoices(team_id, status);
CREATE INDEX idx_invoices_number ON invoices(team_id, invoice_number);
CREATE INDEX idx_invoices_due ON invoices(team_id, due_date) WHERE status NOT IN ('paid', 'void');
CREATE INDEX idx_invoices_portal ON invoices(portal_token);
CREATE INDEX idx_invoices_overdue ON invoices(team_id, due_date, status) WHERE status IN ('sent', 'viewed', 'partially_paid');

-- ============================================================
-- LINE ITEMS (shared by estimates + invoices)
-- ============================================================
CREATE TABLE line_items (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    estimate_id     UUID REFERENCES estimates(id) ON DELETE CASCADE,
    invoice_id      UUID REFERENCES invoices(id) ON DELETE CASCADE,
    description     TEXT NOT NULL,
    category        line_item_category NOT NULL DEFAULT 'other',
    quantity        NUMERIC(10,3) NOT NULL DEFAULT 1,
    unit            TEXT NOT NULL DEFAULT 'each',
    unit_price      NUMERIC(12,2) NOT NULL DEFAULT 0,
    total           NUMERIC(12,2) NOT NULL DEFAULT 0,
    cost_price      NUMERIC(12,2),
    taxable         BOOLEAN NOT NULL DEFAULT true,
    sort_order      INT NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT chk_line_item_parent CHECK (
        (estimate_id IS NOT NULL AND invoice_id IS NULL) OR
        (estimate_id IS NULL AND invoice_id IS NOT NULL)
    )
);

CREATE INDEX idx_line_items_estimate ON line_items(estimate_id) WHERE estimate_id IS NOT NULL;
CREATE INDEX idx_line_items_invoice ON line_items(invoice_id) WHERE invoice_id IS NOT NULL;

-- ============================================================
-- PAYMENTS
-- ============================================================
CREATE TABLE payments (
    id                      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id                 UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    invoice_id              UUID NOT NULL REFERENCES invoices(id) ON DELETE RESTRICT,
    customer_id             UUID NOT NULL REFERENCES customers(id) ON DELETE RESTRICT,
    amount                  NUMERIC(12,2) NOT NULL,
    tip_amount              NUMERIC(12,2) NOT NULL DEFAULT 0,
    processing_fee          NUMERIC(12,2) NOT NULL DEFAULT 0,
    net_amount              NUMERIC(12,2) NOT NULL DEFAULT 0,
    payment_method          payment_method NOT NULL,
    status                  payment_status NOT NULL DEFAULT 'pending',
    stripe_payment_intent_id TEXT,
    stripe_charge_id        TEXT,
    square_payment_id       TEXT,
    check_number            TEXT,
    reference_number        TEXT,
    notes                   TEXT,
    refunded_amount         NUMERIC(12,2) NOT NULL DEFAULT 0,
    refund_reason           TEXT,
    collected_by            UUID REFERENCES users(id) ON DELETE SET NULL,
    collected_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_payments_team ON payments(team_id);
CREATE INDEX idx_payments_invoice ON payments(invoice_id);
CREATE INDEX idx_payments_customer ON payments(customer_id);
CREATE INDEX idx_payments_stripe ON payments(stripe_payment_intent_id) WHERE stripe_payment_intent_id IS NOT NULL;
CREATE INDEX idx_payments_date ON payments(team_id, collected_at DESC);

-- ============================================================
-- TIME ENTRIES
-- ============================================================
CREATE TABLE time_entries (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    job_id          UUID NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    entry_type      time_entry_type NOT NULL DEFAULT 'work',
    started_at      TIMESTAMPTZ NOT NULL,
    ended_at        TIMESTAMPTZ,
    duration_minutes INT,
    hourly_rate     NUMERIC(10,2),
    total_cost      NUMERIC(12,2),
    notes           TEXT,
    latitude_start  DOUBLE PRECISION,
    longitude_start DOUBLE PRECISION,
    latitude_end    DOUBLE PRECISION,
    longitude_end   DOUBLE PRECISION,
    sync_status     sync_status NOT NULL DEFAULT 'synced',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_time_entries_job ON time_entries(job_id);
CREATE INDEX idx_time_entries_user ON time_entries(user_id);
CREATE INDEX idx_time_entries_date ON time_entries(team_id, started_at DESC);

-- ============================================================
-- MATERIALS USED (on jobs)
-- ============================================================
CREATE TABLE materials_used (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    job_id          UUID NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,
    inventory_item_id UUID,
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    quantity        NUMERIC(10,3) NOT NULL,
    unit            TEXT NOT NULL DEFAULT 'each',
    unit_cost       NUMERIC(12,2),
    total_cost      NUMERIC(12,2),
    billable        BOOLEAN NOT NULL DEFAULT true,
    barcode         TEXT,
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_materials_used_job ON materials_used(job_id);

-- ============================================================
-- PHOTOS
-- ============================================================
CREATE TABLE photos (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    job_id          UUID REFERENCES jobs(id) ON DELETE SET NULL,
    customer_id     UUID REFERENCES customers(id) ON DELETE SET NULL,
    uploaded_by     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    original_url    TEXT NOT NULL,
    thumbnail_url   TEXT,
    medium_url      TEXT,
    large_url       TEXT,
    category        photo_category NOT NULL DEFAULT 'general',
    caption         TEXT,
    latitude        DOUBLE PRECISION,
    longitude       DOUBLE PRECISION,
    taken_at        TIMESTAMPTZ,
    file_size_bytes BIGINT NOT NULL DEFAULT 0,
    mime_type       TEXT NOT NULL DEFAULT 'image/jpeg',
    width           INT,
    height          INT,
    sort_order      INT NOT NULL DEFAULT 0,
    sync_status     sync_status NOT NULL DEFAULT 'synced',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_photos_job ON photos(job_id) WHERE job_id IS NOT NULL;
CREATE INDEX idx_photos_customer ON photos(customer_id) WHERE customer_id IS NOT NULL;
CREATE INDEX idx_photos_team ON photos(team_id, created_at DESC);

-- ============================================================
-- NOTES
-- ============================================================
CREATE TABLE notes (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    job_id          UUID REFERENCES jobs(id) ON DELETE CASCADE,
    customer_id     UUID REFERENCES customers(id) ON DELETE CASCADE,
    author_id       UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content         TEXT NOT NULL,
    note_type       note_type NOT NULL DEFAULT 'text',
    is_internal     BOOLEAN NOT NULL DEFAULT true,
    audio_url       TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_notes_job ON notes(job_id) WHERE job_id IS NOT NULL;
CREATE INDEX idx_notes_customer ON notes(customer_id) WHERE customer_id IS NOT NULL;

-- ============================================================
-- CHECKLISTS
-- ============================================================
CREATE TABLE checklists (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    job_id          UUID REFERENCES jobs(id) ON DELETE CASCADE,
    template_id     UUID,
    title           TEXT NOT NULL,
    checklist_type  checklist_type NOT NULL DEFAULT 'custom',
    is_required     BOOLEAN NOT NULL DEFAULT false,
    completed_at    TIMESTAMPTZ,
    completed_by    UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE checklist_items (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    checklist_id    UUID NOT NULL REFERENCES checklists(id) ON DELETE CASCADE,
    description     TEXT NOT NULL,
    is_completed    BOOLEAN NOT NULL DEFAULT false,
    completed_at    TIMESTAMPTZ,
    completed_by    UUID REFERENCES users(id) ON DELETE SET NULL,
    notes           TEXT,
    photo_id        UUID REFERENCES photos(id) ON DELETE SET NULL,
    sort_order      INT NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_checklists_job ON checklists(job_id) WHERE job_id IS NOT NULL;
CREATE INDEX idx_checklist_items_checklist ON checklist_items(checklist_id);

-- ============================================================
-- SIGNATURES
-- ============================================================
CREATE TABLE signatures (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    job_id          UUID REFERENCES jobs(id) ON DELETE SET NULL,
    estimate_id     UUID REFERENCES estimates(id) ON DELETE SET NULL,
    invoice_id      UUID REFERENCES invoices(id) ON DELETE SET NULL,
    signer_name     TEXT NOT NULL,
    signer_role     TEXT,
    signature_data  TEXT NOT NULL,
    signature_url   TEXT,
    ip_address      TEXT,
    signed_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_signatures_job ON signatures(job_id) WHERE job_id IS NOT NULL;

-- ============================================================
-- DOCUMENTS
-- ============================================================
CREATE TABLE documents (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    job_id          UUID REFERENCES jobs(id) ON DELETE SET NULL,
    customer_id     UUID REFERENCES customers(id) ON DELETE SET NULL,
    uploaded_by     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    document_type   TEXT NOT NULL,
    file_url        TEXT NOT NULL,
    file_size_bytes BIGINT NOT NULL DEFAULT 0,
    mime_type       TEXT NOT NULL,
    expires_at      DATE,
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_documents_team ON documents(team_id);
CREATE INDEX idx_documents_job ON documents(job_id) WHERE job_id IS NOT NULL;
CREATE INDEX idx_documents_customer ON documents(customer_id) WHERE customer_id IS NOT NULL;
CREATE INDEX idx_documents_expiry ON documents(expires_at) WHERE expires_at IS NOT NULL;

-- ============================================================
-- TAGS
-- ============================================================
CREATE TABLE tags (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id     UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    color       TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(team_id, name)
);

CREATE INDEX idx_tags_team ON tags(team_id);

-- ============================================================
-- INVENTORY
-- ============================================================
CREATE TABLE inventory_locations (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id     UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    location_type TEXT NOT NULL DEFAULT 'warehouse',
    vehicle_id  UUID,
    address     TEXT,
    is_active   BOOLEAN NOT NULL DEFAULT true,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE inventory_items (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    sku             TEXT,
    barcode         TEXT,
    description     TEXT,
    category        TEXT,
    subcategory     TEXT,
    unit_of_measure TEXT NOT NULL DEFAULT 'each',
    min_stock_level NUMERIC(10,3) DEFAULT 0,
    reorder_quantity NUMERIC(10,3),
    cost_price      NUMERIC(12,2),
    markup_pct      NUMERIC(5,2),
    sell_price      NUMERIC(12,2),
    preferred_supplier TEXT,
    photo_url       TEXT,
    is_active       BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE inventory_stock (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    item_id         UUID NOT NULL REFERENCES inventory_items(id) ON DELETE CASCADE,
    location_id     UUID NOT NULL REFERENCES inventory_locations(id) ON DELETE CASCADE,
    quantity        NUMERIC(10,3) NOT NULL DEFAULT 0,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(item_id, location_id)
);

CREATE TABLE inventory_transactions (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    item_id         UUID NOT NULL REFERENCES inventory_items(id) ON DELETE CASCADE,
    location_id     UUID NOT NULL REFERENCES inventory_locations(id) ON DELETE CASCADE,
    txn_type        inventory_txn_type NOT NULL,
    quantity        NUMERIC(10,3) NOT NULL,
    unit_cost       NUMERIC(12,2),
    job_id          UUID REFERENCES jobs(id) ON DELETE SET NULL,
    po_id           UUID,
    reference       TEXT,
    notes           TEXT,
    performed_by    UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_inventory_items_team ON inventory_items(team_id);
CREATE INDEX idx_inventory_items_sku ON inventory_items(team_id, sku) WHERE sku IS NOT NULL;
CREATE INDEX idx_inventory_items_barcode ON inventory_items(barcode) WHERE barcode IS NOT NULL;
CREATE INDEX idx_inventory_stock_item ON inventory_stock(item_id);
CREATE INDEX idx_inventory_txn_item ON inventory_transactions(item_id);
CREATE INDEX idx_inventory_txn_date ON inventory_transactions(team_id, created_at DESC);

-- ============================================================
-- PURCHASE ORDERS
-- ============================================================
CREATE TABLE purchase_orders (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    po_number       TEXT NOT NULL,
    supplier_name   TEXT NOT NULL,
    supplier_email  TEXT,
    supplier_phone  TEXT,
    status          po_status NOT NULL DEFAULT 'draft',
    subtotal        NUMERIC(12,2) NOT NULL DEFAULT 0,
    tax_amount      NUMERIC(12,2) NOT NULL DEFAULT 0,
    shipping_amount NUMERIC(12,2) NOT NULL DEFAULT 0,
    total           NUMERIC(12,2) NOT NULL DEFAULT 0,
    expected_delivery DATE,
    notes           TEXT,
    created_by      UUID REFERENCES users(id) ON DELETE SET NULL,
    approved_by     UUID REFERENCES users(id) ON DELETE SET NULL,
    approved_at     TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE purchase_order_items (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    po_id           UUID NOT NULL REFERENCES purchase_orders(id) ON DELETE CASCADE,
    inventory_item_id UUID REFERENCES inventory_items(id) ON DELETE SET NULL,
    description     TEXT NOT NULL,
    quantity_ordered NUMERIC(10,3) NOT NULL,
    quantity_received NUMERIC(10,3) NOT NULL DEFAULT 0,
    unit_cost       NUMERIC(12,2) NOT NULL,
    total           NUMERIC(12,2) NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_po_team ON purchase_orders(team_id);
CREATE INDEX idx_po_items_po ON purchase_order_items(po_id);

-- ============================================================
-- VEHICLES (Fleet)
-- ============================================================
CREATE TABLE vehicles (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    assigned_to     UUID REFERENCES users(id) ON DELETE SET NULL,
    make            TEXT NOT NULL,
    model           TEXT NOT NULL,
    year            INT NOT NULL,
    vin             TEXT,
    license_plate   TEXT,
    color           TEXT,
    status          vehicle_status NOT NULL DEFAULT 'active',
    odometer        INT,
    registration_expiry DATE,
    insurance_policy TEXT,
    insurance_expiry DATE,
    photo_url       TEXT,
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE vehicle_maintenance (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vehicle_id      UUID NOT NULL REFERENCES vehicles(id) ON DELETE CASCADE,
    maintenance_type TEXT NOT NULL,
    description     TEXT,
    provider        TEXT,
    cost            NUMERIC(12,2),
    odometer        INT,
    performed_at    DATE NOT NULL,
    next_due_date   DATE,
    next_due_odometer INT,
    receipt_url     TEXT,
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE fuel_logs (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vehicle_id      UUID NOT NULL REFERENCES vehicles(id) ON DELETE CASCADE,
    gallons         NUMERIC(8,3) NOT NULL,
    cost_per_gallon NUMERIC(6,3) NOT NULL,
    total_cost      NUMERIC(10,2) NOT NULL,
    odometer        INT,
    station         TEXT,
    logged_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_vehicles_team ON vehicles(team_id);
CREATE INDEX idx_vehicle_maintenance_vehicle ON vehicle_maintenance(vehicle_id);
CREATE INDEX idx_fuel_logs_vehicle ON fuel_logs(vehicle_id);

-- ============================================================
-- TOOLS & EQUIPMENT
-- ============================================================
CREATE TABLE equipment (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    category        TEXT,
    brand           TEXT,
    model           TEXT,
    serial_number   TEXT,
    purchase_date   DATE,
    purchase_price  NUMERIC(12,2),
    warranty_expiry DATE,
    assigned_to     UUID REFERENCES users(id) ON DELETE SET NULL,
    location_id     UUID REFERENCES inventory_locations(id) ON DELETE SET NULL,
    condition       equipment_condition NOT NULL DEFAULT 'good',
    replacement_value NUMERIC(12,2),
    photo_url       TEXT,
    qr_code         TEXT,
    notes           TEXT,
    is_active       BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_equipment_team ON equipment(team_id);
CREATE INDEX idx_equipment_assigned ON equipment(assigned_to) WHERE assigned_to IS NOT NULL;

-- ============================================================
-- MESSAGES / COMMUNICATION LOG
-- ============================================================
CREATE TABLE messages (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    customer_id     UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    job_id          UUID REFERENCES jobs(id) ON DELETE SET NULL,
    direction       TEXT NOT NULL CHECK (direction IN ('inbound', 'outbound')),
    channel         message_channel NOT NULL,
    status          message_status NOT NULL DEFAULT 'queued',
    from_number     TEXT,
    to_number       TEXT,
    from_email      TEXT,
    to_email        TEXT,
    subject         TEXT,
    body            TEXT NOT NULL,
    template_id     TEXT,
    external_id     TEXT,
    error_message   TEXT,
    sent_at         TIMESTAMPTZ,
    delivered_at    TIMESTAMPTZ,
    opened_at       TIMESTAMPTZ,
    clicked_at      TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_messages_customer ON messages(customer_id);
CREATE INDEX idx_messages_job ON messages(job_id) WHERE job_id IS NOT NULL;
CREATE INDEX idx_messages_team_date ON messages(team_id, created_at DESC);
CREATE INDEX idx_messages_external ON messages(external_id) WHERE external_id IS NOT NULL;

-- ============================================================
-- AUTOMATION RULES
-- ============================================================
CREATE TABLE automation_rules (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    trigger_event   TEXT NOT NULL,
    conditions      JSONB DEFAULT '{}',
    actions         JSONB NOT NULL,
    delay_minutes   INT DEFAULT 0,
    is_active       BOOLEAN NOT NULL DEFAULT true,
    last_triggered_at TIMESTAMPTZ,
    trigger_count   INT NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_automation_rules_team ON automation_rules(team_id);
CREATE INDEX idx_automation_rules_trigger ON automation_rules(trigger_event) WHERE is_active = true;

-- ============================================================
-- REVIEWS
-- ============================================================
CREATE TABLE reviews (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    customer_id     UUID REFERENCES customers(id) ON DELETE SET NULL,
    job_id          UUID REFERENCES jobs(id) ON DELETE SET NULL,
    platform        TEXT NOT NULL,
    external_id     TEXT,
    rating          SMALLINT NOT NULL CHECK (rating BETWEEN 1 AND 5),
    content         TEXT,
    reviewer_name   TEXT,
    response        TEXT,
    responded_at    TIMESTAMPTZ,
    published_at    TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_reviews_team ON reviews(team_id);

-- ============================================================
-- LICENSES & CERTIFICATIONS
-- ============================================================
CREATE TABLE licenses (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id         UUID REFERENCES users(id) ON DELETE CASCADE,
    license_type    TEXT NOT NULL,
    license_number  TEXT NOT NULL,
    issuing_state   TEXT,
    issuing_authority TEXT,
    issued_date     DATE,
    expiry_date     DATE,
    status          TEXT NOT NULL DEFAULT 'active',
    document_url    TEXT,
    reminder_sent_90 BOOLEAN NOT NULL DEFAULT false,
    reminder_sent_60 BOOLEAN NOT NULL DEFAULT false,
    reminder_sent_30 BOOLEAN NOT NULL DEFAULT false,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_licenses_team ON licenses(team_id);
CREATE INDEX idx_licenses_expiry ON licenses(expiry_date) WHERE status = 'active';

-- ============================================================
-- INSURANCE POLICIES
-- ============================================================
CREATE TABLE insurance_policies (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    policy_type     TEXT NOT NULL,
    provider        TEXT NOT NULL,
    policy_number   TEXT NOT NULL,
    coverage_amount NUMERIC(14,2),
    premium_amount  NUMERIC(12,2),
    effective_date  DATE NOT NULL,
    expiry_date     DATE NOT NULL,
    document_url    TEXT,
    auto_renew      BOOLEAN NOT NULL DEFAULT false,
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_insurance_team ON insurance_policies(team_id);
CREATE INDEX idx_insurance_expiry ON insurance_policies(expiry_date);

-- ============================================================
-- SERVICE PLANS (Maintenance Agreements)
-- ============================================================
CREATE TABLE service_plans (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    description     TEXT,
    price_monthly   NUMERIC(10,2),
    price_quarterly NUMERIC(10,2),
    price_annual    NUMERIC(10,2),
    visits_per_year INT NOT NULL DEFAULT 1,
    included_services TEXT[],
    discount_pct    NUMERIC(5,2),
    priority_scheduling BOOLEAN NOT NULL DEFAULT true,
    is_active       BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE customer_service_plans (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_plan_id UUID NOT NULL REFERENCES service_plans(id) ON DELETE RESTRICT,
    customer_id     UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    billing_frequency TEXT NOT NULL DEFAULT 'monthly',
    start_date      DATE NOT NULL,
    end_date        DATE,
    auto_renew      BOOLEAN NOT NULL DEFAULT true,
    stripe_subscription_id TEXT,
    status          TEXT NOT NULL DEFAULT 'active',
    visits_used     INT NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_service_plans_team ON service_plans(team_id);
CREATE INDEX idx_customer_service_plans_customer ON customer_service_plans(customer_id);

-- ============================================================
-- EXPENSES
-- ============================================================
CREATE TABLE expenses (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id         UUID REFERENCES users(id) ON DELETE SET NULL,
    job_id          UUID REFERENCES jobs(id) ON DELETE SET NULL,
    vehicle_id      UUID REFERENCES vehicles(id) ON DELETE SET NULL,
    category        TEXT NOT NULL,
    description     TEXT NOT NULL,
    amount          NUMERIC(12,2) NOT NULL,
    tax_amount      NUMERIC(12,2) DEFAULT 0,
    vendor          TEXT,
    receipt_url     TEXT,
    expense_date    DATE NOT NULL,
    is_billable     BOOLEAN NOT NULL DEFAULT false,
    is_reimbursable BOOLEAN NOT NULL DEFAULT false,
    reimbursed      BOOLEAN NOT NULL DEFAULT false,
    mileage         NUMERIC(8,1),
    mileage_rate    NUMERIC(6,4),
    notes           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_expenses_team ON expenses(team_id);
CREATE INDEX idx_expenses_date ON expenses(team_id, expense_date DESC);
CREATE INDEX idx_expenses_category ON expenses(team_id, category);

-- ============================================================
-- AUDIT LOG
-- ============================================================
CREATE TABLE audit_log (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL,
    user_id         UUID,
    action          TEXT NOT NULL,
    resource_type   TEXT NOT NULL,
    resource_id     UUID NOT NULL,
    changes         JSONB,
    ip_address      TEXT,
    user_agent      TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_audit_log_team ON audit_log(team_id, created_at DESC);
CREATE INDEX idx_audit_log_resource ON audit_log(resource_type, resource_id);

-- ============================================================
-- WEBHOOKS
-- ============================================================
CREATE TABLE webhooks (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id         UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    url             TEXT NOT NULL,
    events          TEXT[] NOT NULL,
    secret          TEXT NOT NULL,
    is_active       BOOLEAN NOT NULL DEFAULT true,
    last_triggered_at TIMESTAMPTZ,
    failure_count   INT NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_webhooks_team ON webhooks(team_id);

-- ============================================================
-- UPDATED_AT TRIGGER FUNCTION
-- ============================================================
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply to all tables with updated_at
DO $$
DECLARE
    t TEXT;
BEGIN
    FOR t IN
        SELECT table_name FROM information_schema.columns
        WHERE column_name = 'updated_at'
        AND table_schema = 'public'
        AND table_name NOT IN ('audit_log')
    LOOP
        EXECUTE format('
            CREATE TRIGGER set_updated_at
            BEFORE UPDATE ON %I
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at_column()',
            t
        );
    END LOOP;
END;
$$;
