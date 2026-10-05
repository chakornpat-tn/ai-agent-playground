use crate::{
    application::{AppError, CartStore, CatalogPort},
    domain::*,
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    routing::{get, patch, post},
};
use getrandom::fill as fill_random;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
#[derive(Clone)]
pub struct HttpState {
    pub catalog: Arc<dyn CatalogPort>,
    pub carts: Arc<CartStore>,
}

impl HttpState {
    pub fn new(catalog: Arc<dyn CatalogPort>) -> Self {
        Self { catalog, carts: Arc::new(CartStore::default()) }
    }

    fn session_id(&self, headers: &HeaderMap) -> String {
        if let Some(session_id) = headers
            .get("x-session-id")
            .and_then(|value| value.to_str().ok())
            .filter(|value| !value.is_empty())
            .filter(|value| self.carts.has_session(value))
        {
            return session_id.to_owned();
        }

        let mut bytes = [0_u8; 32];
        fill_random(&mut bytes).expect("OS randomness unavailable");
        let session_id: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
        session_id
    }
}
#[utoipa::path(get,path="/api/v1/health",responses((status=200,body=HealthResponse)))]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok".into(), service: "farmart-backend".into() })
}
#[utoipa::path(get,path="/api/v1/categories",responses((status=200,body=[Category])))]
pub async fn categories(State(s): State<HttpState>) -> Result<Json<Vec<Category>>, AppError> {
    Ok(Json(s.catalog.categories().await?))
}
#[utoipa::path(get,path="/api/v1/brands",responses((status=200,body=[Brand])))]
pub async fn brands(State(s): State<HttpState>) -> Result<Json<Vec<Brand>>, AppError> {
    Ok(Json(s.catalog.brands().await?))
}
#[utoipa::path(get,path="/api/v1/products",params(ProductFilters),responses((status=200,body=ProductList)))]
pub async fn products(
    State(s): State<HttpState>,
    Query(f): Query<ProductFilters>,
) -> Result<Json<ProductList>, AppError> {
    Ok(Json(s.catalog.products(&f).await?))
}
#[utoipa::path(get,path="/api/v1/products/{id}",params(("id"=i32,Path)),responses((status=200,body=Product),(status=404,body=ErrorResponse)))]
pub async fn product(
    State(s): State<HttpState>,
    Path(id): Path<i32>,
) -> Result<Json<Product>, AppError> {
    Ok(Json(s.catalog.product(id).await?))
}
#[utoipa::path(get,path="/api/v1/products/{id}/detail",params(("id"=i32,Path)),responses((status=200,body=ProductDetail),(status=404,body=ErrorResponse)))]
pub async fn product_detail(
    State(s): State<HttpState>,
    Path(id): Path<i32>,
) -> Result<Json<ProductDetail>, AppError> {
    Ok(Json(s.catalog.product_detail(id).await?))
}
#[utoipa::path(get,path="/api/v1/home",responses((status=200,body=HomePayload)))]
pub async fn home(State(s): State<HttpState>) -> Result<Json<HomePayload>, AppError> {
    let f = ProductFilters {
        q: None,
        category_id: None,
        sort: None,
        page: Some(1),
        per_page: Some(20),
    };
    let products = s.catalog.products(&f).await?.items;
    let top = products.iter().filter(|p| p.compare_at_price.is_some()).take(4).cloned().collect();
    let best = products.iter().take(5).cloned().collect();
    let just = products.iter().take(5).cloned().collect();
    Ok(Json(HomePayload {
        categories: s.catalog.categories().await?,
        brands: s.catalog.brands().await?,
        top_savers: top,
        best_sellers: best,
        just_landed: just,
    }))
}
#[utoipa::path(post,path="/api/v1/auth/register",request_body=RegisterRequest,responses((status=201,body=Member)))]
pub async fn register(
    Json(r): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<Member>), AppError> {
    if r.password.len() < 8 {
        return Err(AppError::BadRequest("password must be at least 8 characters".into()));
    }
    Ok((StatusCode::CREATED, Json(Member { id: 0, name: r.name, email: r.email, phone: r.phone })))
}
async fn cart_response(state: &HttpState, session_id: String) -> Result<CartResponse, AppError> {
    let stored = state.carts.items(&session_id);
    let mut items = Vec::with_capacity(stored.len());
    for (product_id, quantity) in stored {
        let product = state.catalog.product(product_id).await?;
        items.push(CartItem { line_total: product.price * f64::from(quantity), product, quantity });
    }
    items.sort_by_key(|item| item.product.id);
    let subtotal = items.iter().map(|item| item.line_total).sum();
    let item_count = items.iter().map(|item| item.quantity).sum();
    Ok(CartResponse { session_id, items, subtotal, item_count })
}

fn session_header(session_id: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-session-id",
        HeaderValue::try_from(session_id).expect("session ids are valid header values"),
    );
    headers
}

#[utoipa::path(get,path="/api/v1/cart",params(("x-session-id"=String,Header)),responses((status=200,body=CartResponse),(status=404,body=ErrorResponse)))]
pub async fn get_cart(
    State(state): State<HttpState>,
    headers: HeaderMap,
) -> Result<(HeaderMap, Json<CartResponse>), AppError> {
    let session_id = state.session_id(&headers);
    let response = cart_response(&state, session_id.clone()).await?;
    Ok((session_header(&session_id), Json(response)))
}

#[utoipa::path(post,path="/api/v1/cart/items",request_body=AddCartRequest,responses((status=200,body=CartResponse),(status=400,body=ErrorResponse),(status=404,body=ErrorResponse)))]
pub async fn add_cart(
    State(state): State<HttpState>,
    headers: HeaderMap,
    Json(r): Json<AddCartRequest>,
) -> Result<(HeaderMap, Json<CartResponse>), AppError> {
    if r.quantity < 1 {
        return Err(AppError::BadRequest("quantity must be greater than zero".into()));
    }
    let product = state.catalog.product(r.product_id).await?;
    let session_id = state.session_id(&headers);
    let existing = state.carts.items(&session_id).get(&r.product_id).copied().unwrap_or(0);
    let quantity = existing
        .checked_add(r.quantity)
        .ok_or_else(|| AppError::BadRequest("quantity is too large".into()))?;
    if quantity > product.stock {
        return Err(AppError::BadRequest(format!(
            "quantity must not exceed stock ({})",
            product.stock
        )));
    }
    state.carts.set_quantity(&session_id, r.product_id, quantity);
    let response = cart_response(&state, session_id.clone()).await?;
    Ok((session_header(&session_id), Json(response)))
}

#[utoipa::path(patch,path="/api/v1/cart/items/{product_id}",params(("product_id"=i32,Path),("x-session-id"=String,Header)),request_body=UpdateCartRequest,responses((status=200,body=CartResponse),(status=400,body=ErrorResponse),(status=404,body=ErrorResponse)))]
pub async fn update_cart(
    State(state): State<HttpState>,
    Path(product_id): Path<i32>,
    headers: HeaderMap,
    Json(r): Json<UpdateCartRequest>,
) -> Result<(HeaderMap, Json<CartResponse>), AppError> {
    if r.quantity < 1 {
        return Err(AppError::BadRequest("quantity must be greater than zero".into()));
    }
    let product = state.catalog.product(product_id).await?;
    if r.quantity > product.stock {
        return Err(AppError::BadRequest(format!(
            "quantity must not exceed stock ({})",
            product.stock
        )));
    }
    let session_id = state.session_id(&headers);
    state.carts.set_quantity(&session_id, product_id, r.quantity);
    let response = cart_response(&state, session_id.clone()).await?;
    Ok((session_header(&session_id), Json(response)))
}

#[utoipa::path(delete,path="/api/v1/cart/items/{product_id}",params(("product_id"=i32,Path),("x-session-id"=String,Header)),responses((status=200,body=CartResponse)))]
pub async fn delete_cart(
    State(state): State<HttpState>,
    Path(product_id): Path<i32>,
    headers: HeaderMap,
) -> Result<(HeaderMap, Json<CartResponse>), AppError> {
    let session_id = state.session_id(&headers);
    state.carts.remove(&session_id, product_id);
    let response = cart_response(&state, session_id.clone()).await?;
    Ok((session_header(&session_id), Json(response)))
}
pub async fn mermaid_diagram() -> axum::response::Html<&'static str> {
    axum::response::Html(include_str!("../../../mermaid.html"))
}
pub fn router(state: HttpState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
        .route("/mermaid.html", get(mermaid_diagram))
        .route("/api/v1/health", get(health))
        .route("/api/v1/home", get(home))
        .route("/api/v1/categories", get(categories))
        .route("/api/v1/brands", get(brands))
        .route("/api/v1/products", get(products))
        .route("/api/v1/products/{id}", get(product))
        .route("/api/v1/products/{id}/detail", get(product_detail))
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/cart", get(get_cart))
        .route("/api/v1/cart/items", post(add_cart))
        .route("/api/v1/cart/items/{product_id}", patch(update_cart).delete(delete_cart))
        .with_state(state)
}
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Farmart API",
        version = "1.0.0",
        description = "API contract for Farmart grocery marketplace"
    ),
    paths(
        health,
        home,
        categories,
        brands,
        products,
        product,
        product_detail,
        register,
        get_cart,
        add_cart,
        update_cart,
        delete_cart
    ),
    components(schemas(
        ErrorResponse,
        HealthResponse,
        Category,
        Brand,
        Product,
        ProductImage,
        ProductDetail,
        ProductList,
        HomePayload,
        ProductFilters,
        RegisterRequest,
        Member,
        AddCartRequest,
        UpdateCartRequest,
        CartResponse,
        CartItem
    ))
)]
pub struct ApiDoc;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_returns_ok_service_payload() {
        let Json(response) = health().await;

        assert_eq!(response.status, "ok");
        assert_eq!(response.service, "farmart-backend");
    }

    #[tokio::test]
    async fn register_rejects_passwords_shorter_than_eight_characters() {
        let result = register(Json(RegisterRequest {
            name: "Asha Patel".into(),
            email: "asha@example.com".into(),
            password: "short".into(),
            phone: None,
        }))
        .await;

        assert!(
            matches!(result, Err(AppError::BadRequest(message)) if message == "password must be at least 8 characters")
        );
    }

    #[tokio::test]
    async fn register_returns_created_member_without_password() {
        let (status, Json(member)) = register(Json(RegisterRequest {
            name: "Asha Patel".into(),
            email: "asha@example.com".into(),
            password: "correct-horse".into(),
            phone: Some("+441234567890".into()),
        }))
        .await
        .expect("valid registration");

        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(member.name, "Asha Patel");
        assert_eq!(member.email, "asha@example.com");
        assert_eq!(member.phone.as_deref(), Some("+441234567890"));
    }
}
