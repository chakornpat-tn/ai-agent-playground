use axum::http::{HeaderName, HeaderValue, Method};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
    sql_query,
};
use farmart_backend::adapters::inbound::http;
use farmart_backend::adapters::outbound::sqlite::{DbPool, DieselCatalogAdapter};
use std::{env, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;
fn init_db(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut c = pool.get()?;
    for sql in [
        "CREATE TABLE IF NOT EXISTS categories (id INTEGER PRIMARY KEY AUTOINCREMENT,name TEXT NOT NULL,slug TEXT UNIQUE NOT NULL,image_url TEXT)",
        "CREATE TABLE IF NOT EXISTS brands (id INTEGER PRIMARY KEY AUTOINCREMENT,name TEXT NOT NULL,description TEXT,image_url TEXT)",
        "CREATE TABLE IF NOT EXISTS products (id INTEGER PRIMARY KEY AUTOINCREMENT,name TEXT NOT NULL,slug TEXT UNIQUE NOT NULL,category_id INTEGER NOT NULL,brand_id INTEGER NOT NULL,price REAL NOT NULL,compare_at_price REAL,rating REAL NOT NULL DEFAULT 0,review_count INTEGER NOT NULL DEFAULT 0,stock INTEGER NOT NULL DEFAULT 0,image_url TEXT NOT NULL,description TEXT,active INTEGER NOT NULL DEFAULT 1,created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP)",
        "CREATE TABLE IF NOT EXISTS product_details (product_id INTEGER PRIMARY KEY,weight TEXT,origin TEXT,best_before_days INTEGER,ingredients TEXT,nutrition TEXT,shipping_returns TEXT,FOREIGN KEY(product_id) REFERENCES products(id))",
        "CREATE TABLE IF NOT EXISTS product_images (id INTEGER PRIMARY KEY AUTOINCREMENT,product_id INTEGER NOT NULL,url TEXT NOT NULL,sort_order INTEGER NOT NULL DEFAULT 0,FOREIGN KEY(product_id) REFERENCES products(id))",
    ] {
        sql_query(sql).execute(&mut c)?;
    }
    if sql_query("SELECT count(*) AS count FROM categories").get_result::<Count>(&mut c)?.count == 0
    {
        sql_query("INSERT INTO categories(name,slug) VALUES('Fruits & Vegetables','fruits-vegetables'),('Breads & Sweets','breads-sweets'),('Raw Meats','raw-meats')").execute(&mut c)?;
        sql_query("INSERT INTO brands(name,description) VALUES('Farmart','Fresh food for every table'),('Meat Brand','Quality cuts, prepared fresh')").execute(&mut c)?;
        sql_query("INSERT INTO products(name,slug,category_id,brand_id,price,compare_at_price,rating,review_count,stock,image_url,description) VALUES('British Beef Mince (20% Fat)','british-beef-mince',3,2,18.5,22,5,12,74,'https://images.unsplash.com/photo-1588168333986-5078d3ae3976?auto=format&fit=crop&w=800&q=80','Rich, finely ground British beef.'),('Avia Green Bananas','avia-green-bananas',1,1,18.5,NULL,4,8,120,'https://images.unsplash.com/photo-1603833665858-e61d17a86224?auto=format&fit=crop&w=800&q=80','Fresh green bananas.')").execute(&mut c)?;
    }
    sql_query("INSERT OR IGNORE INTO products(name,slug,category_id,brand_id,price,compare_at_price,rating,review_count,stock,image_url,description) VALUES ('Premium Beef Steak Cuts','premium-beef-steak-cuts',3,2,24.50,29.00,4.8,24,35,'https://images.unsplash.com/photo-1544025162-d76694265947?auto=format&fit=crop&w=800&q=80','Tender premium British beef steak cuts, prepared fresh.'),('Farm Fresh Chicken Breast','farm-fresh-chicken-breast',3,1,12.70,15.00,4.7,18,60,'https://images.unsplash.com/photo-1604503468506-a8da13d82791?auto=format&fit=crop&w=800&q=80','Lean, skinless chicken breast for everyday meals.'),('Organic Minced Lamb','organic-minced-lamb',3,1,21.90,NULL,4.6,9,28,'https://images.unsplash.com/photo-1603048297172-c92544798d5a?auto=format&fit=crop&w=800&q=80','Rich organic lamb mince with a tender texture.'),('Free Range Pork Fillet','free-range-pork-fillet',3,2,16.00,NULL,4.5,7,42,'https://images.unsplash.com/photo-1432139509613-5c4255815697?auto=format&fit=crop&w=800&q=80','Free-range pork fillet, trimmed and ready to cook.'),('Mixed Nuts & Berries','mixed-nuts-berries',2,1,9.99,NULL,4.4,16,80,'https://images.unsplash.com/photo-1536584756858-f94625d97f26?auto=format&fit=crop&w=800&q=80','A crunchy blend of roasted nuts and dried berries.')").execute(&mut c)?;
    sql_query("INSERT OR IGNORE INTO product_details(product_id,weight,origin,best_before_days,ingredients,nutrition,shipping_returns) SELECT id,'500g','British Isles',5,'100% British beef. No added preservatives.','Per 100g: 210 kcal, protein 20g, fat 14g, carbohydrates 0g.','Keep chilled at 0–4°C. Order before 2pm for next-day delivery. 30-day quality guarantee.' FROM products WHERE slug='british-beef-mince'").execute(&mut c)?;
    sql_query("INSERT OR IGNORE INTO product_details(product_id,weight,origin,best_before_days,ingredients,nutrition,shipping_returns) SELECT id,'500g','British Isles',4,'100% premium beef steak.','Per 100g: 250 kcal, protein 26g, fat 16g, carbohydrates 0g.','Keep chilled at 0–4°C. Prepared fresh and packed on the day of dispatch.' FROM products WHERE slug='premium-beef-steak-cuts'").execute(&mut c)?;
    sql_query("INSERT OR IGNORE INTO product_details(product_id,weight,origin,best_before_days,ingredients,nutrition,shipping_returns) SELECT id,'500g','Farmart local farms',3,'Chicken breast 100%.','Per 100g: 165 kcal, protein 31g, fat 3.6g, carbohydrates 0g.','Keep chilled. Suitable for next-day delivery. Return if packaging is damaged.' FROM products WHERE slug='farm-fresh-chicken-breast'").execute(&mut c)?;
    sql_query("INSERT OR IGNORE INTO product_images(product_id,url,sort_order) SELECT id,image_url,0 FROM products WHERE slug IN ('british-beef-mince','premium-beef-steak-cuts','farm-fresh-chicken-breast')").execute(&mut c)?;
    Ok(())
}
#[derive(diesel::QueryableByName)]
struct Count {
    #[diesel(sql_type=diesel::sql_types::BigInt)]
    count: i64,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let url = env::var("DATABASE_URL").unwrap_or_else(|_| "farmart.db".into());
    let pool =
        Pool::builder().max_size(8).build(ConnectionManager::<SqliteConnection>::new(url))?;
    init_db(&pool)?;
    let state = http::HttpState::new(Arc::new(DieselCatalogAdapter { pool }));
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080);
    let addr: SocketAddr = format!("{host}:{port}").parse()?;
    let frontend_origin = env::var("FRONTEND_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173".into())
        .parse::<HeaderValue>()?;
    let cors = CorsLayer::new()
        .allow_origin(frontend_origin)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([
            HeaderName::from_static("content-type"),
            HeaderName::from_static("x-session-id"),
        ]);
    info!("Farmart API listening at http://{addr}/docs");
    axum::serve(TcpListener::bind(addr).await?, http::router(state).layer(cors)).await?;
    Ok(())
}
