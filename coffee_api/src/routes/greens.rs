// use crate::db::Db;
use coffee_shared::Db;
use crate::error::{ApiError, ApiResult};
use crate::models::{CreateGreenCoffeeRequest, GreenCoffee, UpdateGreenCoffeeRequest};
use axum::{
    extract::{Path, State},
    response::Json,
};
use chrono::Utc;
use serde_json::Value;

// Helper function to get table name
fn table_name() -> String {
    "green_coffee".to_string()
}

// Helper function to create SurrealDB record ID
fn make_record_id(id: &str) -> (String, String) {
    (table_name(), id.to_string())
}

// GET /greens - List all green coffees
pub async fn list_greens(State(db): State<Db>) -> ApiResult<Json<Vec<GreenCoffee>>> {
    let greens: Vec<GreenCoffee> = db.select("green_coffee").await?;

    Ok(Json(greens))
}

// GET /greens/:id - Get specific green coffee
pub async fn get_green(
    State(db): State<Db>,
    Path(id): Path<String>,
) -> ApiResult<Json<GreenCoffee>> {
    match db.select(make_record_id(&id)).await? {
        Some(green) => Ok(Json(green)),
        None => Err(ApiError::NotFound {
            message: "Failed to get green coffee record".to_string(),
        }),
    }
}

// POST /greens - Create new green coffee
pub async fn create_green(
    State(db): State<Db>,
    Json(payload): Json<CreateGreenCoffeeRequest>,
) -> ApiResult<Json<GreenCoffee>> {
    let green_coffee: GreenCoffee = payload.into();

    let created: Option<GreenCoffee> = db.create(table_name()).content(green_coffee).await?;

    match created {
        Some(green) => Ok(Json(green)),
        None => Err(ApiError::Internal {
            message: "Failed to create green coffee record".to_string(),
        }),
    }
}

// PUT /greens/:id - Update green coffee
pub async fn update_green(
    State(db): State<Db>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateGreenCoffeeRequest>,
) -> ApiResult<Json<GreenCoffee>> {
    // First check if the record exists
    let existing: Option<GreenCoffee> = db.select(make_record_id(&id)).await?;

    let mut green = existing.ok_or_else(|| ApiError::NotFound {
        message: format!("Green coffee with id '{}' not found", id),
    })?;

    // Update fields if provided
    if let Some(name) = payload.name {
        green.name = name;
    }
    if let Some(origin_country) = payload.origin_country {
        green.origin_country = origin_country;
    }
    if let Some(region) = payload.region {
        green.region = Some(region);
    }
    if let Some(variety) = payload.variety {
        green.variety = Some(variety);
    }
    if let Some(processing_method) = payload.processing_method {
        green.processing_method = Some(processing_method);
    }
    if let Some(altitude_masl) = payload.altitude_masl {
        green.altitude_masl = Some(altitude_masl);
    }
    if let Some(harvest_year) = payload.harvest_year {
        green.harvest_year = Some(harvest_year);
    }
    if let Some(stock_grams) = payload.stock_grams {
        green.stock_grams = stock_grams;
    }
    if let Some(price_per_kg) = payload.price_per_kg {
        green.price_per_kg = Some(price_per_kg);
    }
    if let Some(price_currency) = payload.price_currency {
        green.price_currency = Some(price_currency);
    }
    if let Some(supplier) = payload.supplier {
        green.supplier = Some(supplier);
    }
    if let Some(cupping_notes) = payload.cupping_notes {
        green.cupping_notes = Some(cupping_notes);
    }
    green.updated_at = Some(Utc::now());

    let updated: Option<GreenCoffee> = db.update(make_record_id(&id)).content(green).await?;

    match updated {
        Some(green) => Ok(Json(green)),
        None => Err(ApiError::Internal {
            message: "Failed to update green coffee record".to_string(),
        }),
    }
}

// DELETE /greens/:id - Delete green coffee
pub async fn delete_green(State(db): State<Db>, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    let deleted: Option<GreenCoffee> = db.delete(make_record_id(&id)).await?;

    match deleted {
        Some(_) => Ok(Json(
            serde_json::json!({"message": "Green coffee deleted successfully"}),
        )),
        None => Err(ApiError::NotFound {
            message: format!("Green coffee with id '{}' not found", id),
        }),
    }
}
