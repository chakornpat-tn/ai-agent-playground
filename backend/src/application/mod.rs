use crate::domain::ErrorResponse;
use crate::domain::{Brand, Category, Product, ProductDetail, ProductFilters, ProductList};
use async_trait::async_trait;
use axum::{Json, http::StatusCode, response::IntoResponse};
use std::{collections::HashMap, sync::RwLock};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] diesel::result::Error),
    #[error("resource not found")]
    NotFound,
    #[error("invalid request: {0}")]
    BadRequest(String),
}
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(ErrorResponse { error: self.to_string() })).into_response()
    }
}

#[async_trait]
pub trait CatalogPort: Send + Sync {
    async fn categories(&self) -> Result<Vec<Category>, AppError>;
    async fn brands(&self) -> Result<Vec<Brand>, AppError>;
    async fn products(&self, filters: &ProductFilters) -> Result<ProductList, AppError>;
    async fn product(&self, id: i32) -> Result<Product, AppError>;
    async fn product_detail(&self, id: i32) -> Result<ProductDetail, AppError>;
}

#[derive(Default)]
pub struct CartStore {
    items: RwLock<HashMap<String, HashMap<i32, i32>>>,
}

impl CartStore {
    pub fn items(&self, session_id: &str) -> HashMap<i32, i32> {
        self.items
            .read()
            .expect("cart store read lock")
            .get(session_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn set_quantity(&self, session_id: &str, product_id: i32, quantity: i32) {
        let mut carts = self.items.write().expect("cart store write lock");
        let cart = carts.entry(session_id.to_owned()).or_default();
        if quantity == 0 {
            cart.remove(&product_id);
        } else {
            cart.insert(product_id, quantity);
        }
        if cart.is_empty() {
            carts.remove(session_id);
        }
    }

    pub fn remove(&self, session_id: &str, product_id: i32) {
        self.set_quantity(session_id, product_id, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::to_bytes, response::IntoResponse};

    #[tokio::test]
    async fn database_errors_are_internal_server_errors() {
        let response = AppError::Database(diesel::result::Error::NotFound).into_response();

        assert_eq!(response.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        let body = to_bytes(response.into_body(), 1024).await.expect("body");
        assert_eq!(body, r#"{"error":"database error: Record not found"}"#);
    }

    #[tokio::test]
    async fn not_found_errors_are_not_found_responses() {
        let response = AppError::NotFound.into_response();

        assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn bad_request_errors_are_bad_request_responses() {
        let response = AppError::BadRequest("invalid".into()).into_response();

        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    }
}
