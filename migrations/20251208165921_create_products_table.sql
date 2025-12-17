-- Migration: Create products table
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS products (
    product_id VARCHAR(32) PRIMARY KEY,
    product_category_name VARCHAR(100) NOT NULL,
    product_name_lenght INTEGER NOT NULL,
    product_description_lenght INTEGER NOT NULL,
    product_photos_qty INTEGER NOT NULL,
    product_weight_g INTEGER NOT NULL,
    product_length_cm INTEGER NOT NULL,
    product_height_cm INTEGER NOT NULL,
    product_width_cm INTEGER NOT NULL
);

CREATE INDEX idx_products_category_name ON products(product_category_name);
