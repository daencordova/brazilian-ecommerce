use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};

use serde::{Serialize, de::DeserializeOwned};
use tracing::error;

use crate::error::{AppError, AppResult};
use crate::models::{
    AddItemToOrderDto, CreateCustomerDto, CreateOrderDto, CreateProductDto, CreateSellerDto,
    Customer, LocationSearchQuery, Order, OrderProductResponse, OrderSearchQuery,
    PaginatedResponse, PaginationParams, Payment, Product, ProductSearchQuery, Review, Seller,
    UpdateCustomerDto,
};
use crate::state::AppState;

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
) -> AppResult<Json<PaginatedResponse<Customer>>> {
    let response = state.customer_service.get_customers(query).await?;
    Ok(Json(response))
}

pub async fn get_customer_by_id_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Customer>> {
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
) -> AppResult<Json<PaginatedResponse<Order>>> {
    let response = state
        .order_service
        .get_orders_by_customer(&id, &pagination)
        .await?;
    Ok(Json(response))
}

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
) -> AppResult<Json<PaginatedResponse<Seller>>> {
    let response = state.seller_service.get_sellers(query).await?;
    Ok(Json(response))
}

pub async fn get_seller_by_id_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Seller>> {
    let seller = state.seller_service.get_seller_by_id(&id).await?;
    Ok(Json(seller))
}

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
) -> AppResult<Json<PaginatedResponse<Order>>> {
    let response = state.order_service.get_orders(query).await?;
    Ok(Json(response))
}

pub async fn get_order_by_id_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Order>> {
    let response = state.order_service.get_order_by_id(&id).await?;
    Ok(Json(response))
}

pub async fn add_item_to_order_by_id_handler(
    State(state): State<AppState>,
    Path(order_id): Path<String>,
    Json(payload): Json<AddItemToOrderDto>,
) -> AppResult<impl IntoResponse> {
    println!("{:?}", payload);
    let order_item = state
        .order_service
        .add_item_to_order(&order_id, payload)
        .await?;
    Ok((StatusCode::CREATED, Json(order_item)))
}

pub async fn get_products_by_order_id_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<OrderProductResponse>> {
    let response = state.order_service.get_products_by_order_id(&id).await?;
    Ok(Json(response))
}

pub async fn get_payments_by_order_id_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<Payment>>> {
    let response = state.order_service.get_payments_by_order_id(&id).await?;
    Ok(Json(response))
}

pub async fn get_reviews_by_order_id_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<Review>>> {
    let response = state.order_service.get_reviews_by_order_id(&id).await?;
    Ok(Json(response))
}

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
) -> AppResult<Json<PaginatedResponse<Product>>> {
    let response = state.product_service.get_products(query).await?;
    Ok(Json(response))
}

pub async fn get_product_by_id_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<Product>> {
    let product = state.product_service.get_product_by_id(&id).await?;
    Ok(Json(product))
}

pub async fn load_data_from_csv_handler() -> AppResult<impl IntoResponse> {
    let client = reqwest::Client::new();
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let base_url = format!("http://localhost:{}", port);

    let mut total_success = 0;
    let mut total_error = 0;

    // Load Customers
    let (success, error) = load_csv_data::<CreateCustomerDto>(
        &client,
        &format!("{}/customers", base_url),
        "data/olist_customers_dataset.csv",
    )
    .await?;
    total_success += success;
    total_error += error;

    // Load Sellers
    let (success, error) = load_csv_data::<CreateSellerDto>(
        &client,
        &format!("{}/sellers", base_url),
        "data/olist_sellers_dataset.csv",
    )
    .await?;
    total_success += success;
    total_error += error;

    // Load Orders
    let (success, error) = load_csv_data::<CreateOrderDto>(
        &client,
        &format!("{}/orders", base_url),
        "data/olist_orders_dataset.csv",
    )
    .await?;
    total_success += success;
    total_error += error;

    Ok(Json(serde_json::json!({
        "message": "Data load processed",
        "success_count": total_success,
        "error_count": total_error
    })))
}

async fn load_csv_data<T>(
    client: &reqwest::Client,
    url: &str,
    file_path: &str,
) -> AppResult<(usize, usize)>
where
    T: DeserializeOwned + Serialize,
{
    let mut rdr = csv::Reader::from_path(file_path).map_err(|e| {
        error!("Failed to open CSV file {}: {}", file_path, e);
        AppError::ConfigError(format!("Failed to open CSV file: {}", e))
    })?;

    let mut success_count = 0;
    let mut error_count = 0;

    for result in rdr.deserialize() {
        let record: T = match result {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to parse CSV record in {}: {}", file_path, e);
                error_count += 1;
                continue;
            }
        };

        let res = client.post(url).json(&record).send().await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    success_count += 1;
                } else {
                    error!(
                        "Failed to create record from {}: status={}",
                        file_path,
                        response.status()
                    );
                    error_count += 1;
                }
            }
            Err(e) => {
                error!(
                    "Failed to send request for record from {}: {}",
                    file_path, e
                );
                error_count += 1;
            }
        }
    }
    Ok((success_count, error_count))
}
