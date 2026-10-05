use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}
#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}
#[derive(Clone, Serialize, ToSchema)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub image_url: Option<String>,
}
#[derive(Clone, Serialize, ToSchema)]
pub struct Brand {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}
#[derive(Clone, Serialize, ToSchema)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub category_id: i32,
    pub category_name: String,
    pub brand_id: i32,
    pub brand_name: String,
    pub price: f64,
    pub compare_at_price: Option<f64>,
    pub rating: f64,
    pub review_count: i32,
    pub stock: i32,
    pub image_url: String,
    pub description: Option<String>,
}
#[derive(Serialize, ToSchema)]
pub struct ProductImage {
    pub url: String,
    pub sort_order: i32,
}
#[derive(Serialize, ToSchema)]
pub struct ProductDetail {
    pub product: Product,
    pub weight: Option<String>,
    pub origin: Option<String>,
    pub best_before_days: Option<i32>,
    pub ingredients: Option<String>,
    pub nutrition: Option<String>,
    pub shipping_returns: Option<String>,
    pub images: Vec<ProductImage>,
}
#[derive(Serialize, ToSchema)]
pub struct ProductList {
    pub items: Vec<Product>,
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}
#[derive(Serialize, ToSchema)]
pub struct HomePayload {
    pub categories: Vec<Category>,
    pub brands: Vec<Brand>,
    pub top_savers: Vec<Product>,
    pub best_sellers: Vec<Product>,
    pub just_landed: Vec<Product>,
}
#[derive(Deserialize, IntoParams, ToSchema)]
pub struct ProductFilters {
    pub q: Option<String>,
    pub category_id: Option<i32>,
    pub sort: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
#[derive(Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
}
#[derive(Serialize, ToSchema)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}
#[derive(Deserialize, ToSchema)]
pub struct AddCartRequest {
    pub product_id: i32,
    pub quantity: i32,
}
#[derive(Deserialize, ToSchema)]
pub struct UpdateCartRequest {
    pub quantity: i32,
}
#[derive(Serialize, ToSchema)]
pub struct CartResponse {
    pub session_id: String,
    pub items: Vec<CartItem>,
    pub subtotal: f64,
    pub item_count: i32,
}
#[derive(Serialize, ToSchema)]
pub struct CartItem {
    pub product: Product,
    pub quantity: i32,
    pub line_total: f64,
}
