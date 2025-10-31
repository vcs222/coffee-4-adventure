mod routes;
#[cfg(test)]
mod tests;

use coffee_shared::{db, error, models};
use routes::*;

use axum::{Router, routing::get};
use std::env;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database connection
    let db = db::connect().await.map_err(|e| {
        eprintln!("Failed to connect to database: {}", e);
        e
    })?;

    db::apply_migrations(&db).await?;

    // Get port from environment or default to 8080
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/greens", get(list_greens).post(create_green))
        .route(
            "/greens/{id}",
            get(get_green).put(update_green).delete(delete_green),
        )
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
        .layer(CorsLayer::permissive())
        .with_state(db);

    // Run our app with hyper
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("🚀 Server running on http://0.0.0.0:{}", port);

    axum::serve(listener, app).await?;

    Ok(())
}
