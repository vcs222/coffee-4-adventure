use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roast {
    pub id: Option<Thing>,
    pub name: String,
    pub green_coffee: Option<Thing>,
    pub date_roasted: Option<DateTime<Utc>>,
    pub roast_level: String,
    pub batch_size_grams: f64,
    pub yield_grams: f64,
    pub notes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoastRequest {
    pub name: String,
    pub green_coffee: Option<Thing>,
    pub date_roasted: Option<DateTime<Utc>>,
    pub roast_level: String,
    pub batch_size_grams: f64,
    pub yield_grams: f64,
    pub notes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoastRequest {
    pub name: Option<String>,
    pub green_coffee: Option<Thing>,
    pub date_roasted: Option<DateTime<Utc>>,
    pub roast_level: Option<String>,
    pub batch_size_grams: Option<f64>,
    pub yield_grams: Option<f64>,
    pub notes: Option<Vec<String>>,
}

impl From<CreateRoastRequest> for Roast {
    fn from(req: CreateRoastRequest) -> Self {
        Self {
            id: None,
            name: req.name,
            green_coffee: req.green_coffee,
            date_roasted: req.date_roasted,
            roast_level: req.roast_level,
            batch_size_grams: req.batch_size_grams,
            yield_grams: req.yield_grams,
            notes: req.notes,
            created_at: None,
            updated_at: None,
        }
    }
}
