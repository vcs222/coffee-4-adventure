DEFINE TABLE roast SCHEMAFULL;

DEFINE FIELD name ON roast TYPE string ASSERT $value != NONE;
DEFINE FIELD green_coffee ON roast TYPE record<green_coffee>;
DEFINE FIELD date_roasted ON roast TYPE datetime DEFAULT time::now();
DEFINE FIELD roast_level ON roast TYPE string ASSERT $value != NONE;
DEFINE FIELD batch_size_grams ON roast TYPE float ASSERT $value >= 0;
DEFINE FIELD yield_grams ON roast TYPE float ASSERT $value >= 0;
DEFINE FIELD notes ON roast TYPE array<string>;
DEFINE FIELD created_at ON roast TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON roast TYPE datetime DEFAULT time::now();
