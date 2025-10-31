# coffee-4-adventure

Project: Coffee 4 Adventure — Backend API (coffee-api)
1. 🎯 Overview
Coffee 4 Adventure is a project that combines coffee roasting and outdoor adventure content.
The coffee-api backend will serve as the foundation for managing all coffee-related data — green bean inventory, roast batches, and eventually user interactions.

The goal of this first milestone is to build a functional backend API that:
	•	Stores coffee bean and roast data
	•	Provides a clean REST interface
	•	Is deployable in a serverless environment (AWS Lambda)
	•	Uses surrealdb
	•	Allows easy integration later with CLI and mobile/web apps

2. 🧭 Problem Statement
Currently, there is no structured way to:
	•	Track the inventory of green beans used for roasting
	•	Store or query details of roast batches
	•	Expose consistent data for other frontends (CLI, mobile, or web)
	•	Manage data via a simple admin API

3. 🎯 Goals & Non-Goals

✅ Goals
	•	Build a minimal, clean REST API for coffee data.
	•	Focus on admin-level CRUD for green coffee inventory.
	•	Deploy as an AWS Lambda (using cargo-lambda).
	•	Store data in surrealdb
	•	Maintain a structure that allows extension toward roast management, authentication, and user-facing features later.

❌ Non-Goals (for this milestone)
	•	No Auth0 authentication or roles yet.
	•	No payments, orders, or e-commerce integration.
	•	No frontend (CLI, web, or mobile) implementation.
	•	No images or media upload features.
	•	No CI/CD automation.

4. 🧩 Product Scope

4.1 Core Features

1. Health Check
	•	Purpose: Allow monitoring and uptime verification.
	•	Endpoint: GET /health
	•	Response: { "status": "ok" }


2. Green Coffee Management

Admin CRUD operations for managing green coffee bean batches.

Feature	Description
Create	Add a new green coffee batch
Read (List)	List all stored coffees
Read (Single)	Retrieve one by ID
Update	Modify fields like name, origin, quantity
Delete	Remove a record

Data Model:

pub struct GreenCoffee {
    pub id: Option<Thing>, // SurrealDB record ID
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Roast {
    pub id: Option<Thing>, // SurrealDB record ID
    pub name: String,
    pub roast_level: String,  // Light, Medium, Dark
    pub batch_size_grams: f64,
    pub yield_grams: f64,
    pub notes: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Product {
    pub id: Option<Thing>, // SurrealDB record ID
    pub roast: Option<Thing>, // "roast:id"
    pub name: String,
    pub description: Option<String>,
    pub package_size_grams: f64,
    pub price: f64,
	pub price_currency: Option<String>,
    pub stock_units: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

4.2 Future Expansion (Not in this milestone)

Roast batches: Batch profiles, roast logs, and cupping notes.

User authentication: Auth0 integration for admin/public separation.

CLI and mobile client: Built using Rust and/or Dioxus.

Adventure content module: Outdoor coffee brewing content.

Image uploads: Coffee bean photos (S3 integration).

Metrics: Consumption analytics and roasting history.