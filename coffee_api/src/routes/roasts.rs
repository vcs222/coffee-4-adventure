use crate::db::Db;
use crate::error::{ApiError, ApiResult};
use crate::models::{CreateRoastRequest, Roast, UpdateRoastRequest};
use axum::{
    extract::{Path, State},
    response::Json,
};
use chrono::Utc;
use serde_json::Value;

// Helper function to get table name
fn table_name() -> String {
    "roast".to_string()
}

// Helper function to create SurrealDB record ID
fn make_record_id(id: &str) -> (String, String) {
    (table_name(), id.to_string())
}

// GET /roasts - List all roasts
pub async fn list_roasts(State(db): State<Db>) -> ApiResult<Json<Vec<Roast>>> {
    let roasts: Vec<Roast> = db.select("roast").await?;

    Ok(Json(roasts))
}

// GET /roasts/:id - Get specific roast
pub async fn get_roast(
    State(db): State<Db>,
    Path(id): Path<String>,
) -> ApiResult<Json<Roast>> {
    match db.select(make_record_id(&id)).await? {
        Some(roast) => Ok(Json(roast)),
        None => Err(ApiError::NotFound {
            message: "Failed to get roast record".to_string(),
        }),
    }
}

// POST /roasts - Create new roast
pub async fn create_roast(
    State(db): State<Db>,
    Json(payload): Json<CreateRoastRequest>,
) -> ApiResult<Json<Roast>> {
    let roast: Roast = payload.into();

    let created: Option<Roast> = db.create(table_name()).content(roast).await?;

    match created {
        Some(roast) => Ok(Json(roast)),
        None => Err(ApiError::Internal {
            message: "Failed to create roast record".to_string(),
        }),
    }
}

// PUT /roasts/:id - Update roast
pub async fn update_roast(
    State(db): State<Db>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateRoastRequest>,
) -> ApiResult<Json<Roast>> {
    // First check if the record exists
    let existing: Option<Roast> = db.select(make_record_id(&id)).await?;

    let mut roast = existing.ok_or_else(|| ApiError::NotFound {
        message: format!("Roast with id '{}' not found", id),
    })?;

    // Update fields if provided
    if let Some(name) = payload.name {
        roast.name = name;
    }
    if let Some(green_coffee) = payload.green_coffee {
        roast.green_coffee = Some(green_coffee);
    }
    if let Some(date_roasted) = payload.date_roasted {
        roast.date_roasted = Some(date_roasted);
    }
    if let Some(roast_level) = payload.roast_level {
        roast.roast_level = roast_level;
    }
    if let Some(batch_size_grams) = payload.batch_size_grams {
        roast.batch_size_grams = batch_size_grams;
    }
    if let Some(yield_grams) = payload.yield_grams {
        roast.yield_grams = yield_grams;
    }
    if let Some(notes) = payload.notes {
        roast.notes = Some(notes);
    }
    roast.updated_at = Some(Utc::now());

    let updated: Option<Roast> = db.update(make_record_id(&id)).content(roast).await?;

    match updated {
        Some(roast) => Ok(Json(roast)),
        None => Err(ApiError::Internal {
            message: "Failed to update roast record".to_string(),
        }),
    }
}

// DELETE /roasts/:id - Delete roast
pub async fn delete_roast(State(db): State<Db>, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    let deleted: Option<Roast> = db.delete(make_record_id(&id)).await?;

    match deleted {
        Some(_) => Ok(Json(
            serde_json::json!({"message": "Roast deleted successfully"}),
        )),
        None => Err(ApiError::NotFound {
            message: format!("Roast with id '{}' not found", id),
        }),
    }
}
