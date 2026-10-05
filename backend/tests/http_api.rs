use async_trait::async_trait;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use farmart_backend::{
    adapters::inbound::http::{HttpState, router},
    application::{AppError, CatalogPort},
    domain::{Brand, Category, Product, ProductDetail, ProductFilters, ProductList},
};
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt;

struct StubCatalog;

#[async_trait]
impl CatalogPort for StubCatalog {
    async fn categories(&self) -> Result<Vec<Category>, AppError> {
        Ok(vec![Category { id: 1, name: "Fruit".into(), slug: "fruit".into(), image_url: None }])
    }
    async fn brands(&self) -> Result<Vec<Brand>, AppError> {
        Ok(vec![Brand { id: 1, name: "Farmart".into(), description: None, image_url: None }])
    }
    async fn products(&self, _: &ProductFilters) -> Result<ProductList, AppError> {
        Ok(ProductList { items: vec![product()], page: 1, per_page: 20, total: 1 })
    }
    async fn product(&self, id: i32) -> Result<Product, AppError> {
        if id == 1 { Ok(product()) } else { Err(AppError::NotFound) }
    }
    async fn product_detail(&self, id: i32) -> Result<ProductDetail, AppError> {
        if id != 1 {
            return Err(AppError::NotFound);
        }
        Ok(ProductDetail {
            product: product(),
            weight: Some("1kg".into()),
            origin: None,
            best_before_days: Some(7),
            ingredients: None,
            nutrition: None,
            shipping_returns: None,
            images: vec![],
        })
    }
}

fn product() -> Product {
    Product {
        id: 1,
        name: "Apple".into(),
        slug: "apple".into(),
        category_id: 1,
        category_name: "Fruit".into(),
        brand_id: 1,
        brand_name: "Farmart".into(),
        price: 3.0,
        compare_at_price: Some(4.0),
        rating: 4.5,
        review_count: 10,
        stock: 20,
        image_url: "apple.png".into(),
        description: Some("Crisp apple".into()),
    }
}

fn app() -> axum::Router {
    router(HttpState::new(Arc::new(StubCatalog)))
}

async fn json_response(response: axum::response::Response) -> Value {
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.expect("response body");
    serde_json::from_slice(&bytes).expect("valid JSON")
}

#[tokio::test]
async fn health_endpoint_returns_service_status() {
    let response =
        app().oneshot(Request::get("/api/v1/health").body(Body::empty()).unwrap()).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        json_response(response).await,
        serde_json::json!({
            "status": "ok",
            "service": "farmart-backend"
        })
    );
}

#[tokio::test]
async fn register_endpoint_validates_and_returns_member() {
    let request = Request::post("/api/v1/auth/register")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name":"Asha Patel","email":"asha@example.com","password":"correct-horse","phone":null}"#))
        .unwrap();

    let response = app().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(json_response(response).await["email"], "asha@example.com");
}

#[tokio::test]
async fn cart_endpoint_returns_bad_request_for_invalid_quantity() {
    let request = Request::post("/api/v1/cart/items")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"product_id":1,"quantity":0}"#))
        .unwrap();

    let response = app().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        json_response(response).await["error"],
        "invalid request: quantity must be greater than zero"
    );
}

#[tokio::test]
async fn cart_endpoint_adds_product_and_returns_totals() {
    let request = Request::post("/api/v1/cart/items")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"product_id":1,"quantity":2}"#))
        .unwrap();

    let response = app().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = json_response(response).await;
    assert!(body["session_id"].as_str().is_some_and(|id| id.len() == 64));
    assert_eq!(body["item_count"], 2);
    assert_eq!(body["subtotal"], 6.0);
    assert_eq!(body["items"][0]["product"]["id"], 1);
    assert_eq!(body["items"][0]["line_total"], 6.0);
}

#[tokio::test]
async fn cart_endpoint_does_not_accept_unknown_client_session_ids() {
    let request = Request::post("/api/v1/cart/items")
        .header("content-type", "application/json")
        .header("x-session-id", "another-users-session")
        .body(Body::from(r#"{"product_id":1,"quantity":1}"#))
        .unwrap();

    let response = app().oneshot(request).await.unwrap();
    let body = json_response(response).await;
    assert_eq!(body["item_count"], 1);
    assert_ne!(body["session_id"], "another-users-session");
}

#[tokio::test]
async fn cart_endpoint_preserves_session_for_get_update_and_delete() {
    let app = app();
    let add = Request::post("/api/v1/cart/items")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"product_id":1,"quantity":2}"#))
        .unwrap();
    let response = app.clone().oneshot(add).await.unwrap();
    let session = response.headers().get("x-session-id").unwrap().clone();

    let update = Request::patch("/api/v1/cart/items/1")
        .header("content-type", "application/json")
        .header("x-session-id", session.clone())
        .body(Body::from(r#"{"quantity":4}"#))
        .unwrap();
    let response = app.clone().oneshot(update).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let get = Request::get("/api/v1/cart")
        .header("x-session-id", session.clone())
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(get).await.unwrap();
    assert_eq!(json_response(response).await["item_count"], 4);

    let delete = Request::delete("/api/v1/cart/items/1")
        .header("x-session-id", session)
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(delete).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(json_response(response).await["item_count"], 0);
}

#[tokio::test]
async fn cart_endpoint_rejects_unknown_or_over_stock_products() {
    let unknown = Request::post("/api/v1/cart/items")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"product_id":99,"quantity":1}"#))
        .unwrap();
    assert_eq!(app().oneshot(unknown).await.unwrap().status(), StatusCode::NOT_FOUND);

    let over_stock = Request::post("/api/v1/cart/items")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"product_id":1,"quantity":21}"#))
        .unwrap();
    let response = app().oneshot(over_stock).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        json_response(response).await["error"],
        "invalid request: quantity must not exceed stock (20)"
    );
}

#[tokio::test]
async fn catalog_endpoints_return_catalog_data() {
    for path in ["/api/v1/categories", "/api/v1/brands"] {
        let response =
            app().oneshot(Request::get(path).body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK, "{path}");
        assert!(json_response(response).await.as_array().is_some(), "{path}");
    }
    let response =
        app().oneshot(Request::get("/api/v1/products").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(json_response(response).await["items"].as_array().is_some());
    for (path, expected) in [
        ("/api/v1/products/1", "name"),
        ("/api/v1/products/1/detail", "weight"),
        ("/api/v1/home", "top_savers"),
    ] {
        let response =
            app().oneshot(Request::get(path).body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK, "{path}");
        assert!(json_response(response).await.get(expected).is_some(), "{path}");
    }
}

#[tokio::test]
async fn product_endpoint_returns_not_found_for_unknown_product() {
    let response = app()
        .oneshot(Request::get("/api/v1/products/999").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_eq!(json_response(response).await["error"], "resource not found");
}

#[tokio::test]
async fn documentation_endpoints_are_available() {
    let openapi =
        app().oneshot(Request::get("/openapi.json").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(openapi.status(), StatusCode::OK);

    let diagram =
        app().oneshot(Request::get("/mermaid.html").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(diagram.status(), StatusCode::OK);
}
