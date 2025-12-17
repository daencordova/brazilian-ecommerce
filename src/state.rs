use crate::services::{CustomerService, OrderService, ProductService, SellerService};

#[derive(Clone)]
pub struct AppState {
    pub customer_service: CustomerService,
    pub seller_service: SellerService,
    pub order_service: OrderService,
    pub product_service: ProductService,
}
