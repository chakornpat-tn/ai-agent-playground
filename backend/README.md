# Farmart Backend

Rust API service for the Vue Farmart grocery marketplace.

## Run

```bash
cargo run
```

The SQLite database is created and seeded on first run. Open Swagger UI at
`http://127.0.0.1:8080/docs`; the raw contract is available at
`http://127.0.0.1:8080/openapi.json`.

Implemented stack: Axum, Tokio, Diesel SQLite, `async-trait`, Serde and Utoipa.

Product detail data is persisted in `product_details` and `product_images` and is available at `GET /api/v1/products/{id}/detail` (also shown in Swagger). The first run adds multiple grocery products plus weight, origin, shelf-life, ingredients, nutrition and shipping/returns information.
The SQLite ER diagram is available at `http://127.0.0.1:8080/mermaid.html`.

## Hexagonal structure

- `src/domain.rs` — API/domain models with no framework orchestration.
- `src/application.rs` — `CatalogPort`, the outbound application port.
- `src/adapters.rs` — Diesel SQLite outbound adapter implementing the port.
- `src/inbound.rs` — Axum inbound adapter, routing and OpenAPI contract.
- `src/main.rs` — composition root and database bootstrap only.
