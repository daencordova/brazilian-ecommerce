-- Migration: Create customers table
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS order_items (
    order_item_id INTEGER NOT NULL,
    order_id VARCHAR(32) NOT NULL,
    product_id VARCHAR(32) NOT NULL,
    seller_id VARCHAR(32) NOT NULL,
    shipping_limit_date TIMESTAMP NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    freight_value DECIMAL(10, 2) NOT NULL,
    PRIMARY KEY (order_item_id, order_id, product_id, seller_id),
    CONSTRAINT fk_order_order_items
        FOREIGN KEY (order_id)
        REFERENCES orders(order_id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,
    CONSTRAINT fk_product_order_items
        FOREIGN KEY (product_id)
        REFERENCES products(product_id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,
    CONSTRAINT fk_seller_order_items
        FOREIGN KEY (seller_id)
        REFERENCES sellers(seller_id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION
);
