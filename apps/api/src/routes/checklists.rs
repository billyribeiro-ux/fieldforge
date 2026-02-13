use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::errors::{ApiError, ApiResult};
use crate::models::checklist::{Checklist, ChecklistItem};
use crate::AppState;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/jobs/{job_id}/checklists", get(list_job_checklists).post(create_checklist))
        .route("/checklists/{id}", get(get_checklist).delete(delete_checklist))
        .route("/checklists/{id}/items", axum::routing::post(add_item))
        .route("/checklists/{checklist_id}/items/{item_id}/toggle", axum::routing::post(toggle_item))
}

#[derive(Deserialize)]
struct CreateChecklistRequest {
    title: String,
    checklist_type: Option<String>,
    is_required: Option<bool>,
    items: Option<Vec<CreateChecklistItemInput>>,
}

#[derive(Deserialize)]
struct CreateChecklistItemInput {
    description: String,
    sort_order: Option<i32>,
}

async fn create_checklist(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<Uuid>,
    Json(req): Json<CreateChecklistRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let team_id = Uuid::nil();
    let checklist_type = req.checklist_type.as_deref().unwrap_or("custom");
    let is_required = req.is_required.unwrap_or(false);

    let mut tx = state.db.begin().await?;

    let checklist = sqlx::query_as::<_, Checklist>(
        r#"
        INSERT INTO checklists (team_id, job_id, title, checklist_type, is_required)
        VALUES ($1, $2, $3, $4::checklist_type, $5)
        RETURNING *
        "#,
    )
    .bind(team_id)
    .bind(job_id)
    .bind(&req.title)
    .bind(checklist_type)
    .bind(is_required)
    .fetch_one(&mut *tx)
    .await?;

    if let Some(items) = &req.items {
        for (i, item) in items.iter().enumerate() {
            let order = item.sort_order.unwrap_or(i as i32);
            sqlx::query(
                r#"
                INSERT INTO checklist_items (checklist_id, description, sort_order)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(checklist.id)
            .bind(&item.description)
            .bind(order)
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;

    tracing::info!(checklist_id = %checklist.id, job_id = %job_id, "Checklist created");

    Ok(Json(json!({
        "data": checklist,
        "meta": null,
        "errors": null,
    })))
}

async fn list_job_checklists(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let checklists = sqlx::query_as::<_, Checklist>(
        "SELECT * FROM checklists WHERE job_id = $1 ORDER BY created_at",
    )
    .bind(job_id)
    .fetch_all(&state.db)
    .await?;

    // Fetch items for all checklists
    let checklist_ids: Vec<Uuid> = checklists.iter().map(|c| c.id).collect();

    let items = if !checklist_ids.is_empty() {
        sqlx::query_as::<_, ChecklistItem>(
            "SELECT * FROM checklist_items WHERE checklist_id = ANY($1) ORDER BY sort_order",
        )
        .bind(&checklist_ids)
        .fetch_all(&state.db)
        .await?
    } else {
        vec![]
    };

    Ok(Json(json!({
        "data": {
            "checklists": checklists,
            "items": items,
        },
        "meta": null,
        "errors": null,
    })))
}

async fn get_checklist(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let checklist = sqlx::query_as::<_, Checklist>(
        "SELECT * FROM checklists WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Checklist".into()))?;

    let items = sqlx::query_as::<_, ChecklistItem>(
        "SELECT * FROM checklist_items WHERE checklist_id = $1 ORDER BY sort_order",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    let completed = items.iter().filter(|i| i.is_completed).count();

    Ok(Json(json!({
        "data": {
            "checklist": checklist,
            "items": items,
        },
        "meta": {
            "total_items": items.len(),
            "completed": completed,
        },
        "errors": null,
    })))
}

async fn delete_checklist(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    sqlx::query("DELETE FROM checklists WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    Ok(Json(json!({
        "data": null,
        "meta": { "message": "Checklist deleted" },
        "errors": null,
    })))
}

#[derive(Deserialize)]
struct AddItemRequest {
    description: String,
    sort_order: Option<i32>,
}

async fn add_item(
    State(state): State<Arc<AppState>>,
    Path(checklist_id): Path<Uuid>,
    Json(req): Json<AddItemRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let order = match req.sort_order {
        Some(o) => o,
        None => {
            let max: Option<i32> = sqlx::query_scalar(
                "SELECT MAX(sort_order) FROM checklist_items WHERE checklist_id = $1",
            )
            .bind(checklist_id)
            .fetch_one(&state.db)
            .await?;
            max.unwrap_or(0) + 1
        }
    };

    let item = sqlx::query_as::<_, ChecklistItem>(
        r#"
        INSERT INTO checklist_items (checklist_id, description, sort_order)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(checklist_id)
    .bind(&req.description)
    .bind(order)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(json!({
        "data": item,
        "meta": null,
        "errors": null,
    })))
}

async fn toggle_item(
    State(state): State<Arc<AppState>>,
    Path((checklist_id, item_id)): Path<(Uuid, Uuid)>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = Uuid::nil();

    let item = sqlx::query_as::<_, ChecklistItem>(
        r#"
        UPDATE checklist_items SET
            is_completed = NOT is_completed,
            completed_at = CASE WHEN NOT is_completed THEN now() ELSE NULL END,
            completed_by = CASE WHEN NOT is_completed THEN $3 ELSE NULL END,
            updated_at = now()
        WHERE id = $1 AND checklist_id = $2
        RETURNING *
        "#,
    )
    .bind(item_id)
    .bind(checklist_id)
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Checklist item".into()))?;

    // Check if all items are completed → mark checklist as completed
    let remaining = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM checklist_items WHERE checklist_id = $1 AND is_completed = false",
    )
    .bind(checklist_id)
    .fetch_one(&state.db)
    .await?;

    if remaining == 0 {
        sqlx::query(
            "UPDATE checklists SET completed_at = now(), completed_by = $2, updated_at = now() WHERE id = $1",
        )
        .bind(checklist_id)
        .bind(user_id)
        .execute(&state.db)
        .await?;
    } else {
        sqlx::query(
            "UPDATE checklists SET completed_at = NULL, completed_by = NULL, updated_at = now() WHERE id = $1",
        )
        .bind(checklist_id)
        .execute(&state.db)
        .await?;
    }

    Ok(Json(json!({
        "data": item,
        "meta": { "remaining_items": remaining },
        "errors": null,
    })))
}
