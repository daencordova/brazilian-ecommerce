-- Migration: Create payments table
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS payments (
    order_id VARCHAR(32) PRIMARY KEY,
    payment_sequential INTEGER NOT NULL,
    payment_type VARCHAR(20) NOT NULL,
    payment_installments INTEGER NOT NULL,
    payment_value DECIMAL(10, 2) NOT NULL,
    CONSTRAINT fk_order_payments
        FOREIGN KEY (order_id)
        REFERENCES orders(order_id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION
);

CREATE INDEX idx_payments_type ON payments(payment_type);
