-- Migration: Create reviews table
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS reviews (
    review_id VARCHAR(32) PRIMARY KEY,
    order_id VARCHAR(32) NOT NULL,
    review_score INTEGER NOT NULL,
    review_comment_title VARCHAR(30),
    review_comment_message TEXT,
    review_creation_date TIMESTAMP NOT NULL,
    review_answer_timestamp TIMESTAMP NOT NULL,
    CONSTRAINT fk_order_reviews
        FOREIGN KEY (order_id)
        REFERENCES orders(order_id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION
);
