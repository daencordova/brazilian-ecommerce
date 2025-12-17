use crate::handlers::*;
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Customers
        .route(
            "/customers",
            post(create_customer_handler).get(get_customers_handler),
        )
        .route(
            "/customers/{id}",
            get(get_customer_by_id_handler)
                .put(update_customer_handler)
                .delete(delete_customer_handler),
        )
        .route("/customers/{id}/orders", get(get_customer_orders_handler))
        // Sellers
        .route(
            "/sellers",
            post(create_seller_handler).get(get_sellers_handler),
        )
        .route("/sellers/{id}", get(get_seller_by_id_handler))
        // Orders
        .route(
            "/orders",
            post(create_order_handler).get(get_orders_handler),
        )
        .route("/orders/{id}", get(get_order_by_id_handler))
        .route("/orders/{id}/items", post(add_item_to_order_by_id_handler))
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
        .route(
            "/products",
            post(create_product_handler).get(get_products_handler),
        )
        .route("/products/{id}", get(get_product_by_id_handler))
        // Data Loading
        .route("/load-data", post(load_data_from_csv_handler))
        .with_state(state)
}
