use coffee_shared::Db;
use crate::error::{ApiError, ApiResult};
use crate::models::{CreateProductRequest, Product, UpdateProductRequest};
use axum::{
    extract::{Path, State},
    response::Json,
};
use chrono::Utc;
use serde_json::Value;

// Helper function to get table name
fn table_name() -> String {
    "product".to_string()
}

// Helper function to create SurrealDB record ID
fn make_record_id(id: &str) -> (String, String) {
    (table_name(), id.to_string())
}

// GET /products - List all products
pub async fn list_products(State(db): State<Db>) -> ApiResult<Json<Vec<Product>>> {
    let products: Vec<Product> = db.select("product").await?;

    Ok(Json(products))
}

// GET /products/:id - Get specific product
pub async fn get_product(
    State(db): State<Db>,
    Path(id): Path<String>,
) -> ApiResult<Json<Product>> {
    match db.select(make_record_id(&id)).await? {
        Some(product) => Ok(Json(product)),
        None => Err(ApiError::NotFound {
            message: "Failed to get product record".to_string(),
        }),
    }
}

// POST /products - Create new product
pub async fn create_product(
    State(db): State<Db>,
    Json(payload): Json<CreateProductRequest>,
) -> ApiResult<Json<Product>> {
    let product: Product = payload.into();

    let created: Option<Product> = db.create(table_name()).content(product).await?;

    match created {
        Some(product) => Ok(Json(product)),
        None => Err(ApiError::Internal {
            message: "Failed to create product record".to_string(),
        }),
    }
}

// PUT /products/:id - Update product
pub async fn update_product(
    State(db): State<Db>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProductRequest>,
) -> ApiResult<Json<Product>> {
    // First check if the record exists
    let existing: Option<Product> = db.select(make_record_id(&id)).await?;

    let mut product = existing.ok_or_else(|| ApiError::NotFound {
        message: format!("Product with id '{}' not found", id),
    })?;

    // Update fields if provided
    if let Some(roast) = payload.roast {
        product.roast = Some(roast);
    }
    if let Some(name) = payload.name {
        product.name = name;
    }
    if let Some(description) = payload.description {
        product.description = Some(description);
    }
    if let Some(package_size_grams) = payload.package_size_grams {
        product.package_size_grams = package_size_grams;
    }
    if let Some(price) = payload.price {
        product.price = price;
    }
    if let Some(price_currency) = payload.price_currency {
        product.price_currency = Some(price_currency);
    }
    if let Some(stock_units) = payload.stock_units {
        product.stock_units = stock_units;
    }
    product.updated_at = Some(Utc::now());

    let updated: Option<Product> = db.update(make_record_id(&id)).content(product).await?;

    match updated {
        Some(product) => Ok(Json(product)),
        None => Err(ApiError::Internal {
            message: "Failed to update product record".to_string(),
        }),
    }
}

// DELETE /products/:id - Delete product
pub async fn delete_product(State(db): State<Db>, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    let deleted: Option<Product> = db.delete(make_record_id(&id)).await?;

    match deleted {
        Some(_) => Ok(Json(
            serde_json::json!({"message": "Product deleted successfully"}),
        )),
        None => Err(ApiError::NotFound {
            message: format!("Product with id '{}' not found", id),
        }),
    }
}
