use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreenCoffee {
    pub id: Option<Thing>,
    pub name: String,
    pub origin_country: String,
    pub region: Option<String>,
    pub variety: Option<String>,
    pub processing_method: Option<String>,
    pub altitude_masl: Option<i32>,
    pub harvest_year: Option<i32>,
    pub stock_grams: f64,
    pub price_per_kg: Option<f64>,
    pub price_currency: Option<String>,
    pub supplier: Option<String>,
    pub cupping_notes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGreenCoffeeRequest {
    pub name: String,
    pub origin_country: String,
    pub region: Option<String>,
    pub variety: Option<String>,
    pub processing_method: Option<String>,
    pub altitude_masl: Option<i32>,
    pub harvest_year: Option<i32>,
    pub stock_grams: f64,
    pub price_per_kg: Option<f64>,
    pub price_currency: Option<String>,
    pub supplier: Option<String>,
    pub cupping_notes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGreenCoffeeRequest {
    pub name: Option<String>,
    pub origin_country: Option<String>,
    pub region: Option<String>,
    pub variety: Option<String>,
    pub processing_method: Option<String>,
    pub altitude_masl: Option<i32>,
    pub harvest_year: Option<i32>,
    pub stock_grams: Option<f64>,
    pub price_per_kg: Option<f64>,
    pub price_currency: Option<String>,
    pub supplier: Option<String>,
    pub cupping_notes: Option<Vec<String>>,
}

impl From<CreateGreenCoffeeRequest> for GreenCoffee {
    fn from(req: CreateGreenCoffeeRequest) -> Self {
        Self {
            id: None,
            name: req.name,
            origin_country: req.origin_country,
            region: req.region,
            variety: req.variety,
            processing_method: req.processing_method,
            altitude_masl: req.altitude_masl,
            harvest_year: req.harvest_year,
            stock_grams: req.stock_grams,
            price_per_kg: req.price_per_kg,
            price_currency: req.price_currency,
            supplier: req.supplier,
            cupping_notes: req.cupping_notes,
            created_at: None,
            updated_at: None,
        }
    }
}
