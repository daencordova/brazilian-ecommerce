use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::handlers::{
    add_item_to_order_by_id_handler, create_customer_handler, create_order_handler,
    create_product_handler, create_seller_handler, delete_customer_handler,
    get_customer_by_id_handler, get_customer_orders_handler, get_customers_handler,
    get_order_by_id_handler, get_orders_handler, get_payments_by_order_id_handler,
    get_product_by_id_handler, get_products_by_order_id_handler, get_products_handler,
    get_reviews_by_order_id_handler, get_seller_by_id_handler, get_sellers_handler,
    load_data_from_csv_handler, update_customer_handler,
};

use crate::state::AppState;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/load-data", post(load_data_from_csv_handler))
        // Customers
        .route("/customers", post(create_customer_handler))
        .route("/customers", get(get_customers_handler))
        .route("/customers/{id}", get(get_customer_by_id_handler))
        .route("/customers/{id}", put(update_customer_handler))
        .route("/customers/{id}", delete(delete_customer_handler))
        .route("/customers/{id}/orders", get(get_customer_orders_handler))
        // Sellers
        .route("/sellers", post(create_seller_handler))
        .route("/sellers", get(get_sellers_handler))
        .route("/sellers/{id}", get(get_seller_by_id_handler))
        // Orders
        .route("/orders", post(create_order_handler))
        .route("/orders", get(get_orders_handler))
        .route("/orders/{id}", get(get_order_by_id_handler))
        .route(
            "/orders/{id}/add-item",
            post(add_item_to_order_by_id_handler),
        )
        .route(
            "/orders/{id}/products",
            get(get_products_by_order_id_handler),
        )
        .route(
            "/orders/{id}/payments",
            get(get_payments_by_order_id_handler),
        )
        .route("/orders/{id}/reviews", get(get_reviews_by_order_id_handler))
        // Products
        .route("/products", post(create_product_handler))
        .route("/products", get(get_products_handler))
        .route("/products/{id}", get(get_product_by_id_handler))
        .with_state(app_state)
}
