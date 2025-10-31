use super::app;
use axum::body::to_bytes;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use coffee_shared::models::GreenCoffee;
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn create_green_coffee_test() {
    let app = app().await;

    let green_coffee_data = json!({
        "name": "Test Green Coffee",
        "origin_country": "Ethiopia",
        "region": "Sidama",
        "variety": "Heirloom",
        "processing_method": "Washed",
        "altitude_masl": 2000,
        "harvest_year": 2023,
        "stock_grams": 1000.0,
        "price_per_kg": 25.0,
        "price_currency": "USD",
        "supplier": "Test Supplier",
        "cupping_notes": ["Floral", "Citrus"]
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/greens")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(green_coffee_data.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1_000_000).await.unwrap();
    let green_coffee: GreenCoffee = serde_json::from_slice(&body).unwrap();
    assert_eq!(green_coffee.name, "Test Green Coffee");
    assert_eq!(green_coffee.origin_country, "Ethiopia");
}
