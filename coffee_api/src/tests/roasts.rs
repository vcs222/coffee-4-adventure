use super::app;
use axum::body::to_bytes;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use coffee_shared::models::{GreenCoffee, Roast};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn create_roast_test() {
    let app = app().await;

    // Create a green coffee entry first
    let green_coffee_data = json!({
        "name": "Test Green Coffee for Roast",
        "origin_country": "Colombia",
        "region": "Huila",
        "variety": "Caturra",
        "processing_method": "Washed",
        "altitude_masl": 1700,
        "harvest_year": 2022,
        "stock_grams": 1000.0,
        "price_per_kg": 20.0,
        "price_currency": "USD",
        "supplier": "Cafe Imports",
        "cupping_notes": ["Chocolate", "Caramel"]
    });

    let response = app
        .clone()
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
    let green_coffee_id = green_coffee.id.unwrap();

    let roast_data = json!({
        "name": "Test Roast",
        "green_coffee": green_coffee_id,
        "roast_level": "Medium",
        "batch_size_grams": 500.0,
        "yield_grams": 450.0,
        "date_roasted": null,
        "notes": [],
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/roasts")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(roast_data.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1_000_000).await.unwrap();
    let roast: Roast = serde_json::from_slice(&body).unwrap();
    assert_eq!(roast.name, "Test Roast");
    assert_eq!(roast.green_coffee.unwrap(), green_coffee_id);
    assert_eq!(roast.yield_grams, 450.0);
}
