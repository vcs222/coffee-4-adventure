use crate::error::ApiResult;
use axum::response::Json;
use serde_json::{Value, json};

pub async fn health_check() -> ApiResult<Json<Value>> {
    Ok(Json(json!({"status": "ok"})))
}
