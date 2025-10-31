DEFINE TABLE green_coffee SCHEMAFULL;

DEFINE FIELD name ON green_coffee TYPE string ASSERT $value != NONE;
DEFINE FIELD origin_country ON green_coffee TYPE string ASSERT $value != NONE;
DEFINE FIELD region ON green_coffee TYPE string;
DEFINE FIELD variety ON green_coffee TYPE string;
DEFINE FIELD processing_method ON green_coffee TYPE string;
DEFINE FIELD altitude_masl ON green_coffee TYPE int;
DEFINE FIELD harvest_year ON green_coffee TYPE int;
DEFINE FIELD stock_grams ON green_coffee TYPE float ASSERT $value >= 0;
DEFINE FIELD price_per_kg ON green_coffee TYPE float ASSERT $value >= 0;
DEFINE FIELD price_currency ON green_coffee TYPE string;
DEFINE FIELD supplier ON green_coffee TYPE string;
DEFINE FIELD cupping_notes ON green_coffee TYPE array<string>;
DEFINE FIELD created_at ON green_coffee TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON green_coffee TYPE datetime DEFAULT time::now();
