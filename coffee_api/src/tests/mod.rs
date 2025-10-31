pub mod products;
pub mod roasts;
pub mod greens;

use crate::routes::*;
use axum::{routing::get, Router};
use coffee_shared::db;

pub async fn app() -> Router {
    let db = db::connect().await.unwrap();
    Router::new()
        .route("/roasts", get(list_roasts).post(create_roast))
        .route(
            "/roasts/{id}",
            get(get_roast).put(update_roast).delete(delete_roast),
        )
        .route("/products", get(list_products).post(create_product))
        .route(
            "/products/{id}",
            get(get_product).put(update_product).delete(delete_product),
        )
        .route("/greens", get(list_greens).post(create_green))
        .route(
            "/greens/{id}",
            get(get_green).put(update_green).delete(delete_green),
        )
        .with_state(db)
}