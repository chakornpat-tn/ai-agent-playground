# Farmart Seed Reference

This reference is based on the current Farmart backend and frontend contract. Update it when the domain schema or displayed flows change.

## Runtime

- Backend: Rust Axum service.
- Database: SQLite, initialized by `backend/src/main.rs`.
- Default database: `backend/farmart.db`.
- Default API: `http://127.0.0.1:8080`.
- Startup: `cd backend; cargo run`.
- The API serves `/api/v1/home`, `/api/v1/categories`, `/api/v1/brands`, `/api/v1/products`, and `/api/v1/products/{id}/detail`.

## Current entities

Seed in dependency order:

1. `categories`: Fruits & Vegetables, Breads & Sweets, Raw Meats.
2. `brands`: Farmart and Meat Brand, with descriptions and optional images.
3. `products`: include fresh produce, breads/sweets, and multiple meat products. At minimum retain:
   - British Beef Mince (20% Fat), with a compare-at price and full detail data.
   - Avia Green Bananas.
   - Premium Beef Steak Cuts, with a compare-at price and full detail data.
   - Farm Fresh Chicken Breast, with a compare-at price and full detail data.
   - Organic Minced Lamb.
   - Free Range Pork Fillet.
   - Mixed Nuts & Berries.
4. `product_details`: weight, origin, best-before days, ingredients, nutrition, and shipping/returns for the product-detail flow.
5. `product_images`: at least one ordered image for products shown in the detail flow.

## Data behavior to preserve

- Products used by the API must have valid category and brand references.
- Active products should cover search by product name and brand, category filtering, price ascending/descending, rating sorting, pagination, and compare-at-price promotional sections.
- Keep some nullable values (`compare_at_price`, descriptions, image URLs, and detail fields) so optional-field rendering remains exercised.
- Product detail requests for a valid active product should return the product, optional detail fields, and ordered images.
- Re-running initialization must not duplicate categories, products, product details, or images.
- Keep the seed local and deterministic. Existing image URLs may be retained as display data, but seeding must not fetch them.
