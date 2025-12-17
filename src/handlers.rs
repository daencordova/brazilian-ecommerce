use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::de::DeserializeOwned;
use tracing::{error, info};

use crate::error::{AppError, AppResult};
use crate::models::{
    AddItemToOrderDto, CreateCustomerDto, CreateOrderDto, CreateProductDto, CreateSellerDto,
    LocationSearchQuery, OrderSearchQuery, PaginationParams, ProductSearchQuery, UpdateCustomerDto,
};
use crate::state::AppState;

// --- Customer Handlers ---

pub async fn create_customer_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateCustomerDto>,
) -> AppResult<impl IntoResponse> {
    let customer = state.customer_service.create_customer(payload).await?;
    Ok((StatusCode::CREATED, Json(customer)))
}

pub async fn get_customers_handler(
    State(state): State<AppState>,
    Query(query): Query<LocationSearchQuery>,
) -> AppResult<impl IntoResponse> {
    let response = state.customer_service.get_customers(query).await?;
    Ok(Json(response))
}

pub async fn get_customer_by_id_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let customer = state.customer_service.get_customer_by_id(&id).await?;
    Ok(Json(customer))
}

pub async fn update_customer_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateCustomerDto>,
) -> AppResult<impl IntoResponse> {
    let customer = state.customer_service.update_customer(&id, payload).await?;
    Ok((StatusCode::OK, Json(customer)))
}

pub async fn delete_customer_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    state.customer_service.delete_customer(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_customer_orders_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
) -> AppResult<impl IntoResponse> {
    let response = state
        .order_service
        .get_orders_by_customer(&id, &pagination)
        .await?;
    Ok(Json(response))
}

// --- Seller Handlers ---

pub async fn create_seller_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateSellerDto>,
) -> AppResult<impl IntoResponse> {
    let seller = state.seller_service.create_seller(payload).await?;
    Ok((StatusCode::CREATED, Json(seller)))
}

pub async fn get_sellers_handler(
    State(state): State<AppState>,
    Query(query): Query<LocationSearchQuery>,
) -> AppResult<impl IntoResponse> {
    let response = state.seller_service.get_sellers(query).await?;
    Ok(Json(response))
}

pub async fn get_seller_by_id_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let seller = state.seller_service.get_seller_by_id(&id).await?;
    Ok(Json(seller))
}

// --- Order Handlers ---

pub async fn create_order_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrderDto>,
) -> AppResult<impl IntoResponse> {
    let order = state.order_service.create_order(payload).await?;
    Ok((StatusCode::CREATED, Json(order)))
}

pub async fn get_orders_handler(
    State(state): State<AppState>,
    Query(query): Query<OrderSearchQuery>,
) -> AppResult<impl IntoResponse> {
    let response = state.order_service.get_orders(query).await?;
    Ok(Json(response))
}

pub async fn get_order_by_id_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let response = state.order_service.get_order_by_id(&id).await?;
    Ok(Json(response))
}

pub async fn add_item_to_order_by_id_handler(
    State(state): State<AppState>,
    Path(order_id): Path<String>,
    Json(payload): Json<AddItemToOrderDto>,
) -> AppResult<impl IntoResponse> {
    let order_item = state
        .order_service
        .add_item_to_order(&order_id, payload)
        .await?;
    Ok((StatusCode::CREATED, Json(order_item)))
}

pub async fn get_products_by_order_id_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let response = state.order_service.get_products_by_order_id(&id).await?;
    Ok(Json(response))
}

pub async fn get_payments_by_order_id_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let response = state.order_service.get_payments_by_order_id(&id).await?;
    Ok(Json(response))
}

pub async fn get_reviews_by_order_id_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let response = state.order_service.get_reviews_by_order_id(&id).await?;
    Ok(Json(response))
}

// --- Product Handlers ---

pub async fn create_product_handler(
    State(state): State<AppState>,
    Json(dto): Json<CreateProductDto>,
) -> AppResult<impl IntoResponse> {
    let product = state.product_service.create_product(dto).await?;
    Ok((StatusCode::CREATED, Json(product)))
}

pub async fn get_products_handler(
    State(state): State<AppState>,
    Query(query): Query<ProductSearchQuery>,
) -> AppResult<impl IntoResponse> {
    let response = state.product_service.get_products(query).await?;
    Ok(Json(response))
}

pub async fn get_product_by_id_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    let product = state.product_service.get_product_by_id(&id).await?;
    Ok(Json(product))
}

// --- Data Loader Handler (Optimized) ---

pub async fn load_data_from_csv_handler(
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    // Note: In a real world scenario, file paths should be configurable or uploaded via Multipart

    let mut total_success = 0;
    let mut total_error = 0;

    info!("Starting Customer Import...");
    let (s, e) = load_csv_data(
        "data/olist_customers_dataset.csv",
        |record: CreateCustomerDto| {
            let service = state.customer_service.clone();
            async move { service.create_customer(record).await.map(|_| ()) }
        },
    )
    .await?;
    total_success += s;
    total_error += e;

    info!("Starting Seller Import...");
    let (s, e) = load_csv_data(
        "data/olist_sellers_dataset.csv",
        |record: CreateSellerDto| {
            let service = state.seller_service.clone();
            async move { service.create_seller(record).await.map(|_| ()) }
        },
    )
    .await?;
    total_success += s;
    total_error += e;

    info!("Starting Order Import...");
    let (s, e) = load_csv_data("data/olist_orders_dataset.csv", |record: CreateOrderDto| {
        let service = state.order_service.clone();
        async move { service.create_order(record).await.map(|_| ()) }
    })
    .await?;
    total_success += s;
    total_error += e;

    Ok(Json(serde_json::json!({
        "message": "Data load processed",
        "success_count": total_success,
        "error_count": total_error
    })))
}

// Generic CSV loader that takes a closure to execute the logic
// This removes the HTTP roundtrip overhead completely.
async fn load_csv_data<T, F, Fut>(file_path: &str, process_fn: F) -> AppResult<(usize, usize)>
where
    T: DeserializeOwned + Send + 'static,
    F: Fn(T) -> Fut + Send + Sync + Copy,
    Fut: std::future::Future<Output = AppResult<()>> + Send,
{
    let mut rdr = csv::Reader::from_path(file_path).map_err(|e| {
        error!("Failed to open CSV file {}: {}", file_path, e);
        AppError::ConfigError(format!("Failed to open CSV file: {}", e))
    })?;

    let mut success_count = 0;
    let mut error_count = 0;

    // Optional: You could use tokio::spawn here to process in parallel chunks
    // But for now, sequential processing via service is infinitely better than HTTP loop.
    for result in rdr.deserialize() {
        let record: T = match result {
            Ok(r) => r,
            Err(e) => {
                error!("CSV Parse Error in {}: {}", file_path, e);
                error_count += 1;
                continue;
            }
        };

        match process_fn(record).await {
            Ok(_) => success_count += 1,
            Err(e) => {
                error!("Failed to process record from {}: {:?}", file_path, e);
                error_count += 1;
            }
        }
    }

    Ok((success_count, error_count))
}
