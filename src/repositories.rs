use crate::models::{
    AddItemToOrderDto, CreateCustomerDto, CreateOrderDto, CreateProductDto, CreateSellerDto,
    Customer, CustomerFilter, Order, OrderFilter, OrderItem, OrderProduct, PaginationParams,
    Payment, Product, ProductFilter, Review, Seller, SellerFilter, UpdateCustomerDto,
};

use async_trait::async_trait;
use sqlx::{PgPool, Result as SqlxResult};
use tracing::{error, info, instrument};

#[async_trait]
pub trait CustomerRepository: Send + Sync {
    async fn create(&self, dto: CreateCustomerDto) -> SqlxResult<Customer>;
    async fn find_all(
        &self,
        filter: &CustomerFilter,
        pagination: &PaginationParams,
    ) -> SqlxResult<(Vec<Customer>, i64)>;
    async fn find_by_id(&self, id: &str) -> SqlxResult<Option<Customer>>;
    async fn update(&self, id: &str, dto: UpdateCustomerDto) -> SqlxResult<Option<Customer>>;
    async fn delete(&self, id: &str) -> SqlxResult<u64>;
}

#[derive(Clone)]
pub struct PgCustomerRepository {
    pool: PgPool,
}

impl PgCustomerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CustomerRepository for PgCustomerRepository {
    async fn create(&self, dto: CreateCustomerDto) -> SqlxResult<Customer> {
        sqlx::query_as::<_, Customer>(
            r#"
            INSERT INTO customers (
                customer_id, customer_unique_id, customer_zip_code_prefix,
                customer_city, customer_state
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                customer_id, customer_unique_id, customer_zip_code_prefix,
                customer_city, customer_state
            "#,
        )
        .bind(dto.customer_id)
        .bind(dto.customer_unique_id)
        .bind(dto.customer_zip_code_prefix)
        .bind(dto.customer_city)
        .bind(dto.customer_state)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Error creating customer: {:?}", e);
            e
        })
    }

    async fn find_all(
        &self,
        filter: &CustomerFilter,
        pagination: &PaginationParams,
    ) -> SqlxResult<(Vec<Customer>, i64)> {
        let (limit, offset, _, _) = pagination.normalize();

        let count_row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM customers
            WHERE ($1::text IS NULL OR customer_city = $1)
              AND ($2::text IS NULL OR customer_state = $2)
            "#,
        )
        .bind(&filter.city)
        .bind(&filter.state)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Error counting customers: {:?}", e);
            e
        })?;
        let total_count = count_row.0;

        let customers = sqlx::query_as::<_, Customer>(
            r#"
            SELECT
                customer_id, customer_unique_id, customer_zip_code_prefix,
                customer_city, customer_state
            FROM customers
            WHERE ($1::text IS NULL OR customer_city = $1)
              AND ($2::text IS NULL OR customer_state = $2)
            ORDER BY customer_zip_code_prefix DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(&filter.city)
        .bind(&filter.state)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!("Error fetching customers: {:?}", e);
            e
        })?;

        Ok((customers, total_count))
    }

    async fn find_by_id(&self, id: &str) -> SqlxResult<Option<Customer>> {
        sqlx::query_as::<_, Customer>(
            r#"
            SELECT
                customer_id, customer_unique_id, customer_zip_code_prefix,
                customer_city, customer_state
            FROM customers WHERE customer_id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("Error fetching customer by id: {:?}", e);
            e
        })
    }

    #[instrument(skip(self, dto), fields(customer_id = id))]
    async fn update(&self, id: &str, dto: UpdateCustomerDto) -> SqlxResult<Option<Customer>> {
        let result = sqlx::query_as::<_, Customer>(
            r#"
            UPDATE customers
            SET
                customer_unique_id = COALESCE($2, customer_unique_id),
                customer_zip_code_prefix = COALESCE($3, customer_zip_code_prefix),
                customer_city = COALESCE($4, customer_city),
                customer_state = COALESCE($5, customer_state)
            WHERE customer_id = $1
            RETURNING
                customer_id, customer_unique_id, customer_zip_code_prefix,
                customer_city, customer_state
            "#,
        )
        .bind(id)
        .bind(dto.customer_unique_id)
        .bind(dto.customer_zip_code_prefix)
        .bind(dto.customer_city)
        .bind(dto.customer_state)
        .fetch_optional(&self.pool)
        .await;

        match &result {
            Ok(Some(_)) => info!("Customer updated successfully"),
            Ok(None) => info!("Customer not found for update"),
            Err(e) => error!("Error updating customer: {:?}", e),
        }

        result
    }

    #[instrument(skip(self), fields(customer_id = id))]
    async fn delete(&self, id: &str) -> SqlxResult<u64> {
        let result = sqlx::query(
            r#"
            DELETE FROM customers WHERE customer_id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map(|r| r.rows_affected());

        match result {
            Ok(rows) if rows > 0 => info!("Customer deleted successfully. Rows affected: {}", rows),
            Ok(0) => info!("Customer not found for deletion"),
            Err(ref e) => error!("Error deleting customer: {:?}", e),
            _ => (),
        }

        result
    }
}

#[async_trait]
pub trait SellerRepository: Send + Sync {
    async fn create(&self, dto: CreateSellerDto) -> SqlxResult<Seller>;
    async fn find_all(
        &self,
        filter: &SellerFilter,
        pagination: &PaginationParams,
    ) -> SqlxResult<(Vec<Seller>, i64)>;
    async fn find_by_id(&self, id: &str) -> SqlxResult<Option<Seller>>;
}

#[derive(Clone)]
pub struct PgSellerRepository {
    pool: PgPool,
}

impl PgSellerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SellerRepository for PgSellerRepository {
    async fn create(&self, dto: CreateSellerDto) -> SqlxResult<Seller> {
        sqlx::query_as::<_, Seller>(
            r#"
            INSERT INTO sellers (
                seller_id, seller_zip_code_prefix,
                seller_city, seller_state
            )
            VALUES ($1, $2, $3, $4)
            RETURNING
                seller_id, seller_zip_code_prefix,
                seller_city, seller_state
            "#,
        )
        .bind(dto.seller_id)
        .bind(dto.seller_zip_code_prefix)
        .bind(dto.seller_city)
        .bind(dto.seller_state)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Error creating seller: {:?}", e);
            e
        })
    }

    async fn find_all(
        &self,
        filter: &SellerFilter,
        pagination: &PaginationParams,
    ) -> SqlxResult<(Vec<Seller>, i64)> {
        let (limit, offset, _, _) = pagination.normalize();

        let count_row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM sellers
            WHERE ($1::text IS NULL OR seller_city = $1)
              AND ($2::text IS NULL OR seller_state = $2)
            "#,
        )
        .bind(&filter.city)
        .bind(&filter.state)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Error counting sellers: {:?}", e);
            e
        })?;
        let total_count = count_row.0;

        let sellers = sqlx::query_as::<_, Seller>(
            r#"
            SELECT
                seller_id,
                seller_zip_code_prefix,
                seller_city,
                seller_state
            FROM sellers
            WHERE ($1::text IS NULL OR seller_city = $1)
              AND ($2::text IS NULL OR seller_state = $2)
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(&filter.city)
        .bind(&filter.state)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!("Error fetching sellers: {:?}", e);
            e
        })?;

        Ok((sellers, total_count))
    }

    async fn find_by_id(&self, id: &str) -> SqlxResult<Option<Seller>> {
        sqlx::query_as::<_, Seller>(
            r#"
            SELECT
                seller_id, seller_zip_code_prefix,
                seller_city, seller_state
            FROM sellers WHERE seller_id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("Error fetching seller by id: {:?}", e);
            e
        })
    }
}

#[async_trait]
pub trait OrderRepository: Send + Sync {
    async fn create(&self, dto: CreateOrderDto) -> SqlxResult<Order>;
    async fn add_item(&self, order_id: &str, dto: AddItemToOrderDto) -> SqlxResult<OrderItem>;
    async fn find_all(
        &self,
        filter: &OrderFilter,
        pagination: &PaginationParams,
    ) -> SqlxResult<(Vec<Order>, i64)>;
    async fn find_by_id(&self, id: &str) -> SqlxResult<Option<Order>>;
    async fn find_products_by_order_id(&self, id: &str) -> SqlxResult<Vec<OrderProduct>>;
    async fn find_payments_by_order_id(&self, id: &str) -> SqlxResult<Vec<Payment>>;
    async fn find_reviews_by_order_id(&self, id: &str) -> SqlxResult<Vec<Review>>;
    async fn find_by_customer_id(
        &self,
        customer_id: &str,
        pagination: &PaginationParams,
    ) -> SqlxResult<(Vec<Order>, i64)>;
}

#[derive(Clone)]
pub struct PgOrderRepository {
    pool: PgPool,
}

impl PgOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrderRepository for PgOrderRepository {
    async fn create(&self, dto: CreateOrderDto) -> SqlxResult<Order> {
        sqlx::query_as::<_, Order>(
            r#"
            INSERT INTO orders (
                order_id, customer_id, order_status,
                order_purchase_timestamp, order_approved_at,
                order_delivered_carrier_date, order_delivered_customer_date,
                order_estimated_delivery_date
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING
                order_id, customer_id, order_status,
                order_purchase_timestamp, order_approved_at,
                order_delivered_carrier_date, order_delivered_customer_date,
                order_estimated_delivery_date
            "#,
        )
        .bind(dto.order_id)
        .bind(dto.customer_id)
        .bind(dto.order_status)
        .bind(dto.order_purchase_timestamp)
        .bind(dto.order_approved_at)
        .bind(dto.order_delivered_carrier_date)
        .bind(dto.order_delivered_customer_date)
        .bind(dto.order_estimated_delivery_date)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error creating order: {:?}", e);
            e
        })
    }

    async fn add_item(&self, order_id: &str, dto: AddItemToOrderDto) -> SqlxResult<OrderItem> {
        sqlx::query_as::<_, OrderItem>(
            r#"
            INSERT INTO order_items (
                order_item_id, order_id, product_id, seller_id,
                shipping_limit_date, price, freight_value
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING
                order_item_id, order_id, product_id, seller_id,
                shipping_limit_date, price, freight_value
            "#,
        )
        .bind(dto.order_item_id)
        .bind(order_id)
        .bind(dto.product_id)
        .bind(dto.seller_id)
        .bind(dto.shipping_limit_date)
        .bind(dto.price)
        .bind(dto.freight_value)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error adding item to order: {:?}", e);
            e
        })
    }

    async fn find_all(
        &self,
        filter: &OrderFilter,
        pagination: &PaginationParams,
    ) -> SqlxResult<(Vec<Order>, i64)> {
        let (limit, offset, _, _) = pagination.normalize();

        let count_row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM orders
            WHERE ($1::text IS NULL OR order_status = $1)
            "#,
        )
        .bind(&filter.status)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error counting orders: {:?}", e);
            e
        })?;
        let total_count = count_row.0;

        let orders = sqlx::query_as::<_, Order>(
            r#"
            SELECT
                order_id, customer_id, order_status,
                order_purchase_timestamp, order_approved_at,
                order_delivered_carrier_date, order_delivered_customer_date,
                order_estimated_delivery_date
            FROM orders
            WHERE ($1::text IS NULL OR order_status = $1)
            ORDER BY order_purchase_timestamp DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(&filter.status)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error fetching orders: {:?}", e);
            e
        })?;

        Ok((orders, total_count))
    }

    async fn find_by_id(&self, id: &str) -> SqlxResult<Option<Order>> {
        sqlx::query_as::<_, Order>(
            r#"
            SELECT
                order_id, customer_id, order_status,
                order_purchase_timestamp, order_approved_at,
                order_delivered_carrier_date, order_delivered_customer_date,
                order_estimated_delivery_date
            FROM orders WHERE order_id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error fetching order by id: {:?}", e);
            e
        })
    }

    async fn find_by_customer_id(
        &self,
        customer_id: &str,
        pagination: &PaginationParams,
    ) -> SqlxResult<(Vec<Order>, i64)> {
        let (limit, offset, _, _) = pagination.normalize();

        let count_row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM orders
            WHERE customer_id = $1
            "#,
        )
        .bind(customer_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Error counting orders for customer: {:?}", e);
            e
        })?;
        let total_count = count_row.0;

        let orders = sqlx::query_as::<_, Order>(
            r#"
            SELECT
                order_id, customer_id, order_status,
                order_purchase_timestamp, order_approved_at,
                order_delivered_carrier_date, order_delivered_customer_date,
                order_estimated_delivery_date
            FROM orders
            WHERE customer_id = $1
            ORDER BY order_purchase_timestamp DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(customer_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            error!("Error fetching orders for customer: {:?}", e);
            e
        })?;

        Ok((orders, total_count))
    }

    async fn find_products_by_order_id(&self, id: &str) -> SqlxResult<Vec<OrderProduct>> {
        sqlx::query_as::<_, OrderProduct>(
            r#"
            SELECT
                p.product_id,
                p.product_category_name,
                p.product_name_lenght,
                p.product_description_lenght,
                p.product_photos_qty,
                p.product_weight_g,
                p.product_length_cm,
                p.product_height_cm,
                p.product_width_cm,
                oi.shipping_limit_date,
                oi.price,
                oi.freight_value
            FROM products p
            INNER JOIN order_items oi ON p.product_id = oi.product_id
            WHERE oi.order_id = $1
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error fetching products for order: {:?}", e);
            e
        })
    }

    async fn find_payments_by_order_id(&self, id: &str) -> SqlxResult<Vec<Payment>> {
        sqlx::query_as::<_, Payment>(
            r#"
            SELECT
                order_id,
                payment_sequential,
                payment_type,
                payment_installments,
                payment_value
            FROM payments
            WHERE order_id = $1
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error fetching payments for order: {:?}", e);
            e
        })
    }

    async fn find_reviews_by_order_id(&self, id: &str) -> SqlxResult<Vec<Review>> {
        sqlx::query_as::<_, Review>(
            r#"
            SELECT
                review_id,
                order_id,
                review_score,
                review_comment_title,
                review_comment_message,
                review_creation_date,
                review_answer_timestamp
            FROM reviews
            WHERE order_id = $1
            "#,
        )
        .bind(id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error fetching reviews for order: {:?}", e);
            e
        })
    }
}

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn create(&self, dto: CreateProductDto) -> SqlxResult<Product>;
    async fn find_all(
        &self,
        filter: &ProductFilter,
        pagination: &PaginationParams,
    ) -> SqlxResult<(Vec<Product>, i64)>;
    async fn find_by_id(&self, id: &str) -> SqlxResult<Option<Product>>;
}

#[derive(Clone)]
pub struct PgProductRepository {
    pool: PgPool,
}

impl PgProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepository for PgProductRepository {
    async fn create(&self, dto: CreateProductDto) -> SqlxResult<Product> {
        sqlx::query_as::<_, Product>(
            r#"
            INSERT INTO products (
                product_id, product_category_name, product_name_lenght,
                product_description_lenght, product_photos_qty, product_weight_g,
                product_length_cm, product_height_cm, product_width_cm
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(dto.product_id)
        .bind(dto.product_category_name)
        .bind(dto.product_name_lenght)
        .bind(dto.product_description_lenght)
        .bind(dto.product_photos_qty)
        .bind(dto.product_weight_g)
        .bind(dto.product_length_cm)
        .bind(dto.product_height_cm)
        .bind(dto.product_width_cm)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Error creating product: {:?}", e);
            e
        })
    }

    async fn find_all(
        &self,
        filter: &ProductFilter,
        pagination: &PaginationParams,
    ) -> SqlxResult<(Vec<Product>, i64)> {
        let (limit, offset, _, _) = pagination.normalize();

        let count_row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM products
            WHERE ($1::text IS NULL OR product_category_name = $1)
            "#,
        )
        .bind(&filter.category_name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Error counting products: {:?}", e);
            e
        })?;
        let total_count = count_row.0;

        let products = sqlx::query_as::<_, Product>(
            r#"
            SELECT
                product_id, product_category_name, product_name_lenght,
                product_description_lenght, product_photos_qty, product_weight_g,
                product_length_cm, product_height_cm, product_width_cm
            FROM products
            WHERE ($1::text IS NULL OR product_category_name = $1)
            ORDER BY product_id DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(&filter.category_name)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error fetching products: {:?}", e);
            e
        })?;

        Ok((products, total_count))
    }

    async fn find_by_id(&self, id: &str) -> SqlxResult<Option<Product>> {
        sqlx::query_as::<_, Product>("SELECT * FROM products WHERE product_id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Error fetching product by id: {:?}", e);
                e
            })
    }
}
