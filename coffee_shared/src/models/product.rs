use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Option<Thing>,
    pub roast: Option<Thing>,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub colours: Option<Vec<String>>,
    pub details: Option<Vec<String>>,
    pub package_size_grams: f64,
    pub price: f64,
    pub price_currency: Option<String>,
    pub stock_units: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub roast: Option<Thing>,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub colours: Option<Vec<String>>,
    pub details: Option<Vec<String>>,
    pub package_size_grams: f64,
    pub price: f64,
    pub price_currency: Option<String>,
    pub stock_units: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub roast: Option<Thing>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub colours: Option<Vec<String>>,
    pub details: Option<Vec<String>>,
    pub package_size_grams: Option<f64>,
    pub price: Option<f64>,
    pub price_currency: Option<String>,
    pub stock_units: Option<i32>,
}

impl From<CreateProductRequest> for Product {
    fn from(req: CreateProductRequest) -> Self {
        Self {
            id: None,
            roast: req.roast,
            name: req.name,
            description: req.description,
            category: req.category,
            colours: req.colours,
            details: req.details,
            package_size_grams: req.package_size_grams,
            price: req.price,
            price_currency: req.price_currency,
            stock_units: req.stock_units,
            created_at: None,
            updated_at: None,
        }
    }
}
