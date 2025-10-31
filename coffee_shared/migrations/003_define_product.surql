DEFINE TABLE product SCHEMAFULL;

DEFINE FIELD roast ON product TYPE record<roast>;
DEFINE FIELD name ON product TYPE string ASSERT $value != NONE;
DEFINE FIELD description ON product TYPE string;
DEFINE FIELD package_size_grams ON product TYPE float ASSERT $value >= 0;
DEFINE FIELD price ON product TYPE float ASSERT $value >= 0;
DEFINE FIELD price_currency ON product TYPE string;
DEFINE FIELD stock_units ON product TYPE int ASSERT $value >= 0;
DEFINE FIELD created_at ON product TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON product TYPE datetime DEFAULT time::now();
