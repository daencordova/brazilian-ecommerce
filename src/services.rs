use bigdecimal::{BigDecimal, Zero};
use std::sync::Arc;
use tracing::instrument;
use validator::Validate;

use crate::error::{AppError, AppResult, map_db_error};
use crate::models::{
    AddItemToOrderDto, CreateCustomerDto, CreateOrderDto, CreateProductDto, CreateSellerDto,
    Customer, LocationSearchQuery, Order, OrderItem, OrderProductResponse, OrderSearchQuery,
    PaginatedResponse, PaginationParams, Payment, Product, ProductSearchQuery, Review, Seller,
    UpdateCustomerDto,
};
use crate::repositories::{
    CustomerRepository, OrderRepository, ProductRepository, SellerRepository,
};

#[derive(Clone)]
pub struct CustomerService {
    repository: Arc<dyn CustomerRepository>,
}

impl CustomerService {
    pub fn new(repository: Arc<dyn CustomerRepository>) -> Self {
        Self { repository }
    }

    #[instrument(skip(self))]
    pub async fn create_customer(&self, dto: CreateCustomerDto) -> AppResult<Customer> {
        dto.validate()?;
        Ok(self
            .repository
            .create(dto)
            .await
            .map_err(|e| map_db_error(e, "Customer"))?)
    }

    #[instrument(skip(self))]
    pub async fn get_customer_by_id(&self, id: &str) -> AppResult<Customer> {
        match self.repository.find_by_id(id).await? {
            Some(customer) => Ok(customer),
            None => Err(AppError::NotFound),
        }
    }

    #[instrument(skip(self, dto), fields(customer_id = id))]
    pub async fn update_customer(&self, id: &str, dto: UpdateCustomerDto) -> AppResult<Customer> {
        dto.validate()?;

        if dto.customer_unique_id.is_none()
            && dto.customer_zip_code_prefix.is_none()
            && dto.customer_city.is_none()
            && dto.customer_state.is_none()
        {
            return Err(AppError::NoChangesToUpdate);
        }

        match self.repository.update(id, dto).await? {
            Some(customer) => Ok(customer),
            None => Err(AppError::NotFound),
        }
    }

    #[instrument(skip(self), fields(customer_id = id))]
    pub async fn delete_customer(&self, id: &str) -> AppResult<()> {
        let rows_affected = self.repository.delete(id).await?;
        if rows_affected == 0 {
            Err(AppError::NotFound)
        } else {
            Ok(())
        }
    }

    #[instrument(skip(self))]
    pub async fn get_customers(
        &self,
        query: LocationSearchQuery,
    ) -> AppResult<PaginatedResponse<Customer>> {
        let pagination = query.pagination();
        let filter = query.filter();

        let (_, _, page, page_size) = pagination.normalize();

        let (customers, total_records) = self.repository.find_all(&filter, &pagination).await?;

        Ok(PaginatedResponse::new(
            customers,
            total_records,
            page,
            page_size,
        ))
    }
}

#[derive(Clone)]
pub struct SellerService {
    repository: Arc<dyn SellerRepository>,
}

impl SellerService {
    pub fn new(repository: Arc<dyn SellerRepository>) -> Self {
        Self { repository }
    }

    #[instrument(skip(self))]
    pub async fn create_seller(&self, dto: CreateSellerDto) -> AppResult<Seller> {
        dto.validate()?;
        Ok(self
            .repository
            .create(dto)
            .await
            .map_err(|e| map_db_error(e, "Seller"))?)
    }

    #[instrument(skip(self))]
    pub async fn get_seller_by_id(&self, id: &str) -> AppResult<Seller> {
        match self.repository.find_by_id(id).await? {
            Some(seller) => Ok(seller),
            None => Err(AppError::NotFound),
        }
    }

    #[instrument(skip(self))]
    pub async fn get_sellers(
        &self,
        query: LocationSearchQuery,
    ) -> AppResult<PaginatedResponse<Seller>> {
        let pagination = query.pagination();
        let filter = query.filter();
        let (_, _, page, page_size) = pagination.normalize();

        let (sellers, total_records) = self.repository.find_all(&filter, &pagination).await?;

        Ok(PaginatedResponse::new(
            sellers,
            total_records,
            page,
            page_size,
        ))
    }
}

#[derive(Clone)]
pub struct OrderService {
    repository: Arc<dyn OrderRepository>,
}

impl OrderService {
    pub fn new(repository: Arc<dyn OrderRepository>) -> Self {
        Self { repository }
    }

    #[instrument(skip(self))]
    pub async fn create_order(&self, dto: CreateOrderDto) -> AppResult<Order> {
        dto.validate()?;
        Ok(self
            .repository
            .create(dto)
            .await
            .map_err(|e| map_db_error(e, "Order"))?)
    }

    #[instrument(skip(self))]
    pub async fn add_item_to_order(
        &self,
        order_id: &str,
        dto: AddItemToOrderDto,
    ) -> AppResult<OrderItem> {
        dto.validate()?;
        Ok(self.repository.add_item(order_id, dto).await?)
    }

    #[instrument(skip(self))]
    pub async fn get_order_by_id(&self, id: &str) -> AppResult<Order> {
        match self.repository.find_by_id(id).await? {
            Some(order) => Ok(order),
            None => Err(AppError::NotFound),
        }
    }

    #[instrument(skip(self))]
    pub async fn get_products_by_order_id(&self, id: &str) -> AppResult<OrderProductResponse> {
        let products = self.repository.find_products_by_order_id(id).await?;
        let total_value: BigDecimal = products.iter().fold(BigDecimal::zero(), |acc, product| {
            acc + &product.price + &product.freight_value
        });

        Ok(OrderProductResponse {
            products,
            total_value,
        })
    }

    #[instrument(skip(self))]
    pub async fn get_payments_by_order_id(&self, id: &str) -> AppResult<Vec<Payment>> {
        let payments = self.repository.find_payments_by_order_id(id).await?;
        Ok(payments)
    }

    #[instrument(skip(self))]
    pub async fn get_reviews_by_order_id(&self, id: &str) -> AppResult<Vec<Review>> {
        let reviews = self.repository.find_reviews_by_order_id(id).await?;
        Ok(reviews)
    }

    #[instrument(skip(self))]
    pub async fn get_orders(&self, query: OrderSearchQuery) -> AppResult<PaginatedResponse<Order>> {
        let pagination = query.pagination();
        let filter = query.filter();
        let (_, _, page, page_size) = pagination.normalize();

        let (orders, total_records) = self.repository.find_all(&filter, &pagination).await?;

        Ok(PaginatedResponse::new(
            orders,
            total_records,
            page,
            page_size,
        ))
    }

    #[instrument(skip(self))]
    pub async fn get_orders_by_customer(
        &self,
        customer_id: &str,
        pagination: &PaginationParams,
    ) -> AppResult<PaginatedResponse<Order>> {
        let (_, _, page, page_size) = pagination.normalize();
        let (orders, count) = self
            .repository
            .find_by_customer_id(customer_id, pagination)
            .await?;

        Ok(PaginatedResponse::new(orders, count, page, page_size))
    }
}

#[derive(Clone)]
pub struct ProductService {
    repository: Arc<dyn ProductRepository>,
}

impl ProductService {
    pub fn new(repository: Arc<dyn ProductRepository>) -> Self {
        Self { repository }
    }

    #[instrument(skip(self))]
    pub async fn create_product(&self, dto: CreateProductDto) -> AppResult<Product> {
        dto.validate()?;
        Ok(self
            .repository
            .create(dto)
            .await
            .map_err(|e| map_db_error(e, "Product"))?)
    }

    #[instrument(skip(self))]
    pub async fn get_product_by_id(&self, id: &str) -> AppResult<Product> {
        match self.repository.find_by_id(id).await? {
            Some(product) => Ok(product),
            None => Err(AppError::NotFound),
        }
    }

    #[instrument(skip(self))]
    pub async fn get_products(
        &self,
        query: ProductSearchQuery,
    ) -> AppResult<PaginatedResponse<Product>> {
        let pagination = query.pagination();
        let filter = query.filter();
        let (_, _, page, page_size) = pagination.normalize();

        let (products, total_records) = self.repository.find_all(&filter, &pagination).await?;

        Ok(PaginatedResponse::new(
            products,
            total_records,
            page,
            page_size,
        ))
    }
}
