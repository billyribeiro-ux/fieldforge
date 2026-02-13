use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::models::customer::{CreateCustomerRequest, Customer, CustomerListItem, UpdateCustomerRequest};
use crate::models::job::{CreateJobRequest, Job, JobFilters, JobListItem, UpdateJobRequest};
use crate::models::user::{CreateUserRequest, User, UserResponse};

// ── Users ──

pub async fn create_user(pool: &PgPool, req: &CreateUserRequest, password_hash: &str) -> ApiResult<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, phone, password_hash, first_name, last_name, trade)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(&req.email)
    .bind(&req.phone)
    .bind(password_hash)
    .bind(&req.first_name)
    .bind(&req.last_name)
    .bind(&req.trade)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> ApiResult<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

pub async fn find_user_by_phone(pool: &PgPool, phone: &str) -> ApiResult<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE phone = $1")
        .bind(phone)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

pub async fn find_user_by_id(pool: &PgPool, id: Uuid) -> ApiResult<Option<UserResponse>> {
    let user = sqlx::query_as::<_, UserResponse>(
        r#"
        SELECT id, email, phone, first_name, last_name, avatar_url, role::text, team_id, trade, is_active, created_at
        FROM users WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

// ── Customers ──

pub async fn create_customer(pool: &PgPool, team_id: Uuid, req: &CreateCustomerRequest) -> ApiResult<Customer> {
    let portal_token: String = format!("{}", Uuid::new_v4());
    let tags = req.tags.clone().unwrap_or_default();

    let customer = sqlx::query_as::<_, Customer>(
        r#"
        INSERT INTO customers (team_id, first_name, last_name, email, phone, company_name, referral_source, tags, portal_token)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(&req.first_name)
    .bind(&req.last_name)
    .bind(&req.email)
    .bind(&req.phone)
    .bind(&req.company_name)
    .bind(&req.referral_source)
    .bind(&tags)
    .bind(&portal_token)
    .fetch_one(pool)
    .await?;

    Ok(customer)
}

pub async fn list_customers(pool: &PgPool, team_id: Uuid, search: Option<&str>, limit: i64, cursor: Option<Uuid>) -> ApiResult<Vec<CustomerListItem>> {
    let customers = if let Some(search_term) = search {
        let pattern = format!("%{}%", search_term);
        sqlx::query_as::<_, CustomerListItem>(
            r#"
            SELECT id, first_name, last_name, email, phone, company_name, lifetime_value, outstanding_balance, tags, created_at
            FROM customers
            WHERE team_id = $1 AND deleted_at IS NULL
              AND (first_name || ' ' || last_name || ' ' || COALESCE(company_name, '') || ' ' || COALESCE(email, '') || ' ' || COALESCE(phone, '')) ILIKE $2
              AND ($3::uuid IS NULL OR id < $3)
            ORDER BY created_at DESC
            LIMIT $4
            "#,
        )
        .bind(team_id)
        .bind(&pattern)
        .bind(cursor)
        .bind(limit)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, CustomerListItem>(
            r#"
            SELECT id, first_name, last_name, email, phone, company_name, lifetime_value, outstanding_balance, tags, created_at
            FROM customers
            WHERE team_id = $1 AND deleted_at IS NULL
              AND ($2::uuid IS NULL OR id < $2)
            ORDER BY created_at DESC
            LIMIT $3
            "#,
        )
        .bind(team_id)
        .bind(cursor)
        .bind(limit)
        .fetch_all(pool)
        .await?
    };

    Ok(customers)
}

pub async fn get_customer(pool: &PgPool, team_id: Uuid, id: Uuid) -> ApiResult<Customer> {
    sqlx::query_as::<_, Customer>(
        "SELECT * FROM customers WHERE id = $1 AND team_id = $2 AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Customer".into()))
}

pub async fn update_customer(pool: &PgPool, team_id: Uuid, id: Uuid, req: &UpdateCustomerRequest) -> ApiResult<Customer> {
    let existing = get_customer(pool, team_id, id).await?;

    let customer = sqlx::query_as::<_, Customer>(
        r#"
        UPDATE customers SET
            first_name = COALESCE($3, first_name),
            last_name = COALESCE($4, last_name),
            email = COALESCE($5, email),
            phone = COALESCE($6, phone),
            phone_secondary = COALESCE($7, phone_secondary),
            company_name = COALESCE($8, company_name),
            notes_pinned = COALESCE($9, notes_pinned)
        WHERE id = $1 AND team_id = $2 AND deleted_at IS NULL
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(team_id)
    .bind(&req.first_name)
    .bind(&req.last_name)
    .bind(&req.email)
    .bind(&req.phone)
    .bind(&req.phone_secondary)
    .bind(&req.company_name)
    .bind(&req.notes_pinned)
    .fetch_one(pool)
    .await?;

    Ok(customer)
}

pub async fn delete_customer(pool: &PgPool, team_id: Uuid, id: Uuid) -> ApiResult<()> {
    let result = sqlx::query(
        "UPDATE customers SET deleted_at = now() WHERE id = $1 AND team_id = $2 AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(team_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Customer".into()));
    }
    Ok(())
}

// ── Jobs ──

pub async fn create_job(pool: &PgPool, team_id: Uuid, req: &CreateJobRequest) -> ApiResult<Job> {
    let tags = req.tags.clone().unwrap_or_default();

    let job = sqlx::query_as::<_, Job>(
        r#"
        INSERT INTO jobs (team_id, customer_id, property_id, assigned_to, title, description, priority, job_type, trade,
                          scheduled_date, scheduled_start_time, estimated_duration_minutes, access_instructions, internal_notes, tags)
        VALUES ($1, $2, $3, $4, $5, $6, COALESCE($7, 'normal')::job_priority, $8, $9, $10, $11, $12, $13, $14, $15)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(req.customer_id)
    .bind(req.property_id)
    .bind(req.assigned_to)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.priority)
    .bind(&req.job_type)
    .bind(&req.trade)
    .bind(req.scheduled_date)
    .bind(req.scheduled_start_time)
    .bind(req.estimated_duration_minutes)
    .bind(&req.access_instructions)
    .bind(&req.internal_notes)
    .bind(&tags)
    .fetch_one(pool)
    .await?;

    Ok(job)
}

pub async fn get_job(pool: &PgPool, team_id: Uuid, id: Uuid) -> ApiResult<Job> {
    sqlx::query_as::<_, Job>(
        "SELECT * FROM jobs WHERE id = $1 AND team_id = $2 AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(team_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Job".into()))
}

pub async fn list_jobs(pool: &PgPool, team_id: Uuid, filters: &JobFilters, limit: i64, cursor: Option<Uuid>) -> ApiResult<Vec<JobListItem>> {
    let jobs = sqlx::query_as::<_, JobListItem>(
        r#"
        SELECT j.id, j.customer_id, c.first_name as customer_first_name, c.last_name as customer_last_name,
               j.title, j.status::text, j.priority::text, j.scheduled_date, j.scheduled_start_time,
               j.assigned_to, j.total_amount, j.tags, j.created_at
        FROM jobs j
        JOIN customers c ON c.id = j.customer_id
        WHERE j.team_id = $1 AND j.deleted_at IS NULL
          AND ($2::text IS NULL OR j.status::text = $2)
          AND ($3::text IS NULL OR j.priority::text = $3)
          AND ($4::uuid IS NULL OR j.assigned_to = $4)
          AND ($5::uuid IS NULL OR j.customer_id = $5)
          AND ($6::date IS NULL OR j.scheduled_date >= $6)
          AND ($7::date IS NULL OR j.scheduled_date <= $7)
          AND ($8::uuid IS NULL OR j.id < $8)
        ORDER BY j.created_at DESC
        LIMIT $9
        "#,
    )
    .bind(team_id)
    .bind(&filters.status)
    .bind(&filters.priority)
    .bind(filters.assigned_to)
    .bind(filters.customer_id)
    .bind(filters.date_from)
    .bind(filters.date_to)
    .bind(cursor)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(jobs)
}

pub async fn update_job_status(pool: &PgPool, team_id: Uuid, job_id: Uuid, new_status: &str, user_id: Uuid, lat: Option<f64>, lng: Option<f64>, note: Option<&str>) -> ApiResult<Job> {
    let job = get_job(pool, team_id, job_id).await?;

    // Record status history
    sqlx::query(
        r#"
        INSERT INTO job_status_history (job_id, from_status, to_status, changed_by, latitude, longitude, note)
        VALUES ($1, $2::job_status, $3::job_status, $4, $5, $6, $7)
        "#,
    )
    .bind(job_id)
    .bind(&job.status)
    .bind(new_status)
    .bind(user_id)
    .bind(lat)
    .bind(lng)
    .bind(note)
    .execute(pool)
    .await?;

    // Update job status
    let updated = sqlx::query_as::<_, Job>(
        r#"
        UPDATE jobs SET
            status = $3::job_status,
            started_at = CASE WHEN $3 = 'in_progress' AND started_at IS NULL THEN now() ELSE started_at END,
            completed_at = CASE WHEN $3 = 'completed' THEN now() ELSE completed_at END,
            version = version + 1
        WHERE id = $1 AND team_id = $2 AND deleted_at IS NULL
        RETURNING *
        "#,
    )
    .bind(job_id)
    .bind(team_id)
    .bind(new_status)
    .fetch_one(pool)
    .await?;

    Ok(updated)
}
