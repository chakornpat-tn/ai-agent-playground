use crate::{
    application::{AppError, CatalogPort},
    domain::{Brand, Category, Product, ProductDetail, ProductFilters, ProductImage, ProductList},
};
use async_trait::async_trait;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
    sql_query,
    sql_types::{BigInt, Double, Integer, Nullable, Text},
};
pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub struct DieselCatalogAdapter {
    pub pool: DbPool,
}
#[derive(QueryableByName)]
struct CountRow {
    #[diesel(sql_type = BigInt)]
    count: i64,
}
#[derive(QueryableByName)]
struct DetailRow {
    #[diesel(sql_type = Nullable<Text>)]
    weight: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    origin: Option<String>,
    #[diesel(sql_type = Nullable<Integer>)]
    best_before_days: Option<i32>,
    #[diesel(sql_type = Nullable<Text>)]
    ingredients: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    nutrition: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    shipping_returns: Option<String>,
}
#[derive(QueryableByName)]
struct ImageRow {
    #[diesel(sql_type = Text)]
    url: String,
    #[diesel(sql_type = Integer)]
    sort_order: i32,
}
#[derive(QueryableByName)]
struct CategoryRow {
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type = Text)]
    name: String,
    #[diesel(sql_type = Text)]
    slug: String,
    #[diesel(sql_type = Nullable<Text>)]
    image_url: Option<String>,
}
#[derive(QueryableByName)]
struct BrandRow {
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type = Text)]
    name: String,
    #[diesel(sql_type = Nullable<Text>)]
    description: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    image_url: Option<String>,
}
#[derive(QueryableByName)]
struct ProductRow {
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type = Text)]
    name: String,
    #[diesel(sql_type = Text)]
    slug: String,
    #[diesel(sql_type = Integer)]
    category_id: i32,
    #[diesel(sql_type = Text)]
    category_name: String,
    #[diesel(sql_type = Integer)]
    brand_id: i32,
    #[diesel(sql_type = Text)]
    brand_name: String,
    #[diesel(sql_type = Double)]
    price: f64,
    #[diesel(sql_type = Nullable<Double>)]
    compare_at_price: Option<f64>,
    #[diesel(sql_type = Double)]
    rating: f64,
    #[diesel(sql_type = Integer)]
    review_count: i32,
    #[diesel(sql_type = Integer)]
    stock: i32,
    #[diesel(sql_type = Text)]
    image_url: String,
    #[diesel(sql_type = Nullable<Text>)]
    description: Option<String>,
}
impl From<CategoryRow> for Category {
    fn from(x: CategoryRow) -> Self {
        Self { id: x.id, name: x.name, slug: x.slug, image_url: x.image_url }
    }
}
impl From<BrandRow> for Brand {
    fn from(x: BrandRow) -> Self {
        Self { id: x.id, name: x.name, description: x.description, image_url: x.image_url }
    }
}
impl From<ProductRow> for Product {
    fn from(x: ProductRow) -> Self {
        Self {
            id: x.id,
            name: x.name,
            slug: x.slug,
            category_id: x.category_id,
            category_name: x.category_name,
            brand_id: x.brand_id,
            brand_name: x.brand_name,
            price: x.price,
            compare_at_price: x.compare_at_price,
            rating: x.rating,
            review_count: x.review_count,
            stock: x.stock,
            image_url: x.image_url,
            description: x.description,
        }
    }
}
fn connection(
    pool: &DbPool,
) -> Result<diesel::r2d2::PooledConnection<ConnectionManager<SqliteConnection>>, AppError> {
    pool.get().map_err(|e| AppError::BadRequest(e.to_string()))
}
const PRODUCT_SELECT: &str = "SELECT p.id,p.name,p.slug,p.category_id,c.name AS category_name,p.brand_id,b.name AS brand_name,p.price,p.compare_at_price,p.rating,p.review_count,p.stock,p.image_url,p.description FROM products p JOIN categories c ON c.id=p.category_id JOIN brands b ON b.id=p.brand_id";
#[async_trait]
impl CatalogPort for DieselCatalogAdapter {
    async fn categories(&self) -> Result<Vec<Category>, AppError> {
        let mut c = connection(&self.pool)?;
        Ok(sql_query("SELECT id,name,slug,image_url FROM categories ORDER BY name")
            .load::<CategoryRow>(&mut c)?
            .into_iter()
            .map(Into::into)
            .collect())
    }
    async fn brands(&self) -> Result<Vec<Brand>, AppError> {
        let mut c = connection(&self.pool)?;
        Ok(sql_query("SELECT id,name,description,image_url FROM brands ORDER BY name")
            .load::<BrandRow>(&mut c)?
            .into_iter()
            .map(Into::into)
            .collect())
    }
    async fn products(&self, f: &ProductFilters) -> Result<ProductList, AppError> {
        let mut c = connection(&self.pool)?;
        let page = f.page.unwrap_or(1).max(1);
        let per_page = f.per_page.unwrap_or(20).clamp(1, 100);
        let sort = match f.sort.as_deref() {
            Some("price_asc") => "p.price ASC",
            Some("price_desc") => "p.price DESC",
            Some("rating") => "p.rating DESC",
            _ => "p.created_at DESC",
        };
        let sql = format!(
            "{} WHERE p.active=1 AND (? IS NULL OR (lower(p.name) LIKE ? OR lower(b.name) LIKE ?)) AND (? IS NULL OR p.category_id=?) ORDER BY {} LIMIT ? OFFSET ?",
            PRODUCT_SELECT, sort
        );
        let q = f.q.as_ref().map(|x| format!("%{}%", x.to_lowercase()));
        let rows = sql_query(sql)
            .bind::<Nullable<Text>, _>(q.clone())
            .bind::<Nullable<Text>, _>(q.clone())
            .bind::<Nullable<Text>, _>(q)
            .bind::<Nullable<Integer>, _>(f.category_id)
            .bind::<Nullable<Integer>, _>(f.category_id)
            .bind::<BigInt, _>(per_page)
            .bind::<BigInt, _>((page - 1) * per_page)
            .load::<ProductRow>(&mut c)?;
        let total_sql = "SELECT count(*) AS count FROM products p JOIN categories c ON c.id=p.category_id JOIN brands b ON b.id=p.brand_id WHERE p.active=1 AND (? IS NULL OR (lower(p.name) LIKE ? OR lower(b.name) LIKE ?)) AND (? IS NULL OR p.category_id=?)";
        let total = sql_query(total_sql)
            .bind::<Nullable<Text>, _>(f.q.as_ref().map(|x| format!("%{}%", x.to_lowercase())))
            .bind::<Nullable<Text>, _>(f.q.as_ref().map(|x| format!("%{}%", x.to_lowercase())))
            .bind::<Nullable<Text>, _>(f.q.as_ref().map(|x| format!("%{}%", x.to_lowercase())))
            .bind::<Nullable<Integer>, _>(f.category_id)
            .bind::<Nullable<Integer>, _>(f.category_id)
            .get_result::<CountRow>(&mut c)?
            .count;
        Ok(ProductList { items: rows.into_iter().map(Into::into).collect(), page, per_page, total })
    }
    async fn product(&self, id: i32) -> Result<Product, AppError> {
        let mut c = connection(&self.pool)?;
        sql_query(format!("{} WHERE p.id=? AND p.active=1", PRODUCT_SELECT))
            .bind::<Integer, _>(id)
            .get_result::<ProductRow>(&mut c)
            .map(Into::into)
            .map_err(|e| {
                if e == diesel::result::Error::NotFound {
                    AppError::NotFound
                } else {
                    AppError::Database(e)
                }
            })
    }
    async fn product_detail(&self, id: i32) -> Result<ProductDetail, AppError> {
        let product = self.product(id).await?;
        let mut c = connection(&self.pool)?;
        let detail = sql_query("SELECT weight,origin,best_before_days,ingredients,nutrition,shipping_returns FROM product_details WHERE product_id=?").bind::<Integer,_>(id).get_result::<DetailRow>(&mut c).optional()?.unwrap_or(DetailRow { weight: None, origin: None, best_before_days: None, ingredients: None, nutrition: None, shipping_returns: None });
        let images = sql_query(
            "SELECT url,sort_order FROM product_images WHERE product_id=? ORDER BY sort_order",
        )
        .bind::<Integer, _>(id)
        .load::<ImageRow>(&mut c)?
        .into_iter()
        .map(|x| ProductImage { url: x.url, sort_order: x.sort_order })
        .collect();
        Ok(ProductDetail {
            product,
            weight: detail.weight,
            origin: detail.origin,
            best_before_days: detail.best_before_days,
            ingredients: detail.ingredients,
            nutrition: detail.nutrition,
            shipping_returns: detail.shipping_returns,
            images,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::sql_query;

    fn adapter() -> DieselCatalogAdapter {
        let pool = Pool::builder()
            .max_size(1)
            .build(ConnectionManager::<SqliteConnection>::new(":memory:"))
            .expect("in-memory pool");
        let mut c = pool.get().expect("connection");
        for sql in [
            "CREATE TABLE categories (id INTEGER PRIMARY KEY, name TEXT NOT NULL, slug TEXT NOT NULL, image_url TEXT)",
            "CREATE TABLE brands (id INTEGER PRIMARY KEY, name TEXT NOT NULL, description TEXT, image_url TEXT)",
            "CREATE TABLE products (id INTEGER PRIMARY KEY, name TEXT NOT NULL, slug TEXT NOT NULL, category_id INTEGER NOT NULL, brand_id INTEGER NOT NULL, price REAL NOT NULL, compare_at_price REAL, rating REAL NOT NULL, review_count INTEGER NOT NULL, stock INTEGER NOT NULL, image_url TEXT NOT NULL, description TEXT, active INTEGER NOT NULL, created_at TEXT NOT NULL)",
            "CREATE TABLE product_details (product_id INTEGER PRIMARY KEY, weight TEXT, origin TEXT, best_before_days INTEGER, ingredients TEXT, nutrition TEXT, shipping_returns TEXT)",
            "CREATE TABLE product_images (id INTEGER PRIMARY KEY, product_id INTEGER NOT NULL, url TEXT NOT NULL, sort_order INTEGER NOT NULL)",
            "INSERT INTO categories VALUES (1, 'Fruit', 'fruit', 'fruit.png'), (2, 'Vegetables', 'vegetables', NULL)",
            "INSERT INTO brands VALUES (1, 'Farmart', 'Local produce', NULL), (2, 'Other', NULL, 'other.png')",
            "INSERT INTO products VALUES (1, 'Apple', 'apple', 1, 1, 3.0, 4.0, 4.5, 10, 20, 'apple.png', 'Crisp apple', 1, '2024-01-01'), (2, 'Carrot', 'carrot', 2, 2, 1.0, NULL, 4.9, 5, 30, 'carrot.png', NULL, 1, '2024-01-02'), (3, 'Hidden', 'hidden', 1, 1, 2.0, NULL, 1.0, 0, 0, 'hidden.png', NULL, 0, '2024-01-03')",
            "INSERT INTO product_details VALUES (1, '1kg', 'UK', 7, 'Apples', 'Healthy', 'Keep cool')",
            "INSERT INTO product_images VALUES (1, 1, 'apple-1.png', 0), (2, 1, 'apple-2.png', 1)",
        ] {
            sql_query(sql).execute(&mut c).expect("seed database");
        }
        DieselCatalogAdapter { pool }
    }

    fn filters() -> ProductFilters {
        ProductFilters { q: None, category_id: None, sort: None, page: None, per_page: None }
    }

    #[tokio::test]
    async fn reads_categories_brands_and_filtered_products() {
        let adapter = adapter();
        let categories = adapter.categories().await.expect("categories");
        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0].name, "Fruit");
        assert_eq!(categories[0].image_url.as_deref(), Some("fruit.png"));

        let brands = adapter.brands().await.expect("brands");
        assert_eq!(brands.len(), 2);
        assert_eq!(brands[0].description.as_deref(), Some("Local produce"));
        assert_eq!(brands[1].image_url.as_deref(), Some("other.png"));

        let mut f = filters();
        f.q = Some("FARMART".into());
        f.category_id = Some(1);
        f.sort = Some("price_asc".into());
        f.page = Some(0);
        f.per_page = Some(0);
        let result = adapter.products(&f).await.expect("products");
        assert_eq!(result.page, 1);
        assert_eq!(result.per_page, 1);
        assert_eq!(result.total, 1);
        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].name, "Apple");
        assert_eq!(result.items[0].compare_at_price, Some(4.0));
    }

    #[tokio::test]
    async fn supports_each_sort_and_pagination_defaults() {
        let adapter = adapter();
        for sort in ["price_desc", "rating", "unknown"] {
            let mut f = filters();
            f.sort = Some(sort.into());
            f.per_page = Some(200);
            let result = adapter.products(&f).await.expect("products");
            assert_eq!(result.page, 1);
            assert_eq!(result.per_page, 100);
            assert_eq!(result.items.len(), 2);
        }
        let mut f = filters();
        f.q = Some("apple".into());
        f.page = Some(2);
        let result = adapter.products(&f).await.expect("paged products");
        assert!(result.items.is_empty());
    }

    #[tokio::test]
    async fn reads_product_detail_and_images() {
        let adapter = adapter();
        let product = adapter.product(1).await.expect("product");
        assert_eq!(product.name, "Apple");
        assert_eq!(product.brand_name, "Farmart");
        assert_eq!(product.description.as_deref(), Some("Crisp apple"));

        let detail = adapter.product_detail(1).await.expect("detail");
        assert_eq!(detail.weight.as_deref(), Some("1kg"));
        assert_eq!(detail.best_before_days, Some(7));
        assert_eq!(detail.images.len(), 2);
        assert_eq!(detail.images[1].sort_order, 1);

        let missing_detail = adapter.product_detail(2).await.expect("detail fallback");
        assert!(missing_detail.weight.is_none());
        assert!(missing_detail.images.is_empty());
    }

    #[tokio::test]
    async fn returns_not_found_for_missing_or_inactive_products() {
        let adapter = adapter();
        assert!(matches!(adapter.product(999).await, Err(AppError::NotFound)));
        assert!(matches!(adapter.product(3).await, Err(AppError::NotFound)));
        assert!(matches!(adapter.product_detail(999).await, Err(AppError::NotFound)));
    }
}
