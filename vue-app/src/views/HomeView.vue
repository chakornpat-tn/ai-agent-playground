<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import AppIcon from '../components/AppIcon.vue'
import { getHome, searchProducts, type Brand, type Category, type Product } from '../services/api'
import { useCartStore } from '../stores/cart'

const route = useRoute()
const cart = useCartStore()
const categories = ref<Array<Category & { img: string; active: boolean }>>([])
const brands = ref<Array<Brand & { img: string; desc: string }>>([])
const topSaverProducts = ref<Product[]>([])
const bestSellerProducts = ref<Product[]>([])
const justLandingProducts = ref<Product[]>([])
const loading = ref(true)
const error = ref('')
const isSearch = ref(false)
const searchTerm = ref('')
const fallbackImage =
  'https://images.unsplash.com/photo-1542838132-92c53300491e?auto=format&fit=crop&w=500&q=80'
const brandImages = [
  'https://images.unsplash.com/photo-1599490659213-e2b9527bd087?auto=format&fit=crop&w=300&q=80',
  'https://images.unsplash.com/photo-1576092762791-dd9e2220abd4?auto=format&fit=crop&w=300&q=80',
]

const money = (value: number) => `$${value.toFixed(2)}`
const loadHome = async () => {
  loading.value = true
  error.value = ''
  try {
    const query = String(route.query.q ?? '').trim()
    searchTerm.value = query
    isSearch.value = Boolean(query)
    if (query) {
      const result = await searchProducts(query)
      topSaverProducts.value = result.items
      bestSellerProducts.value = []
      justLandingProducts.value = []
      categories.value = []
      brands.value = []
    } else {
      const result = await getHome()
      categories.value = result.categories.map((item, index) => ({
        ...item,
        img: item.image_url ?? fallbackImage,
        active: index === 1,
      }))
      brands.value = result.brands.map((item, index) => ({
        ...item,
        img: item.image_url ?? brandImages[index % brandImages.length] ?? fallbackImage,
        desc: item.description ?? 'Fresh Farmart selection',
      }))
      topSaverProducts.value = result.top_savers
      bestSellerProducts.value = result.best_sellers
      justLandingProducts.value = result.just_landed
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Unable to load products'
  } finally {
    loading.value = false
  }
}

onMounted(loadHome)
watch(() => route.query.q, loadHome)

const bestSellerTabs = [
  'All',
  'Fruits & Vegetables',
  'Frozen Seafoods',
  'Raw Meats',
  'Coffee & Teas',
  'Pet Foods',
  'Milks & Dairies',
]
const activeTab = ref('All')
const quantities = ref<Record<number, number>>({})

const quantityFor = (product: Product) => quantities.value[product.id] ?? 1
const setQuantity = (product: Product, value: number) => {
  quantities.value[product.id] = Number.isFinite(value) ? Math.max(1, Math.floor(value)) : 1
}
const changeQuantity = (product: Product, amount: number) => {
  setQuantity(product, quantityFor(product) + amount)
}
const addToCart = (product: Product, withQuantity = false) => {
  if (product.stock < 1) return
  cart.add(product, withQuantity ? quantityFor(product) : 1)
}

const renderStars = (rating: number) => {
  return { full: Math.round(rating), empty: 5 - Math.round(rating) }
}
</script>

<template>
  <main>
    <!-- Hero Section -->
    <section class="hero-section">
      <div class="container hero-grid">
        <div class="hero-banner banner-left">
          <div class="banner-content">
            <h2>Active Summer With<br />Juice Milk 300ml</h2>
            <p>New arrivals with natural fruits, juice<br />milks, essential for summer.</p>
            <button class="btn btn-white">Shop Now</button>
          </div>
        </div>
        <div class="hero-banner banner-right">
          <div class="banner-content">
            <h2>20% SALE OFF</h2>
            <p>Synthetic seeds<br />Net 2.0 OZ</p>
            <button class="btn btn-white">Shop Now</button>
          </div>
        </div>
      </div>
    </section>

    <section v-if="isSearch" class="container search-results">
      <div class="section-header">
        <div>
          <h3>Search results</h3>
          <p class="search-caption">{{ searchTerm }}</p>
        </div>
      </div>
      <p v-if="loading" class="state-message">Searching products...</p>
      <p v-else-if="error" class="state-message error-message">{{ error }}</p>
      <p v-else-if="topSaverProducts.length === 0" class="state-message">
        No products found for “{{ searchTerm }}”.
      </p>
      <div v-else class="product-grid-4">
        <div
          v-for="product in topSaverProducts"
          :key="product.id"
          class="product-card"
          :class="{ 'product-card-disabled': product.stock < 1 }"
        >
          <div class="product-img"><img :src="product.image_url" :alt="product.name" /></div>
          <div class="product-brand">{{ product.brand_name }}</div>
          <div class="product-title">{{ product.name }}</div>
          <div class="product-rating">
            <span
              v-for="i in renderStars(product.rating).full"
              :key="`search-full-${product.id}-${i}`"
              >★</span
            ><span
              v-for="i in renderStars(product.rating).empty"
              :key="`search-empty-${product.id}-${i}`"
              class="empty-star"
              >★</span
            ><span>({{ product.review_count }})</span>
          </div>
          <div class="product-price">
            <span v-if="product.compare_at_price" class="old-price">{{
              money(product.compare_at_price)
            }}</span
            ><span class="new-price">{{ money(product.price) }}</span>
          </div>
          <button class="btn btn-full" :disabled="product.stock < 1" @click="addToCart(product)">
            {{ product.stock < 1 ? 'Unavailable' : 'Add To Cart' }}
          </button>
        </div>
      </div>
    </section>

    <div v-if="!isSearch" class="container">
      <!-- Browse by Category -->
      <div class="section-header">
        <h3>Browse by Category</h3>
        <div class="section-nav">
          <span>All Categories <AppIcon name="chevron-right" :size="10" /></span>
          <div class="nav-arrows">
            <AppIcon name="chevron-left" />
            <AppIcon name="chevron-right" />
          </div>
        </div>
      </div>
      <div class="category-grid">
        <div
          v-for="cat in categories"
          :key="cat.name"
          class="cat-card"
          :class="{ active: cat.active }"
        >
          <img :src="cat.img" :alt="cat.name" />
          <p v-html="cat.name.replace('\n', '<br>')"></p>
        </div>
      </div>

      <!-- Featured Brands -->
      <div class="section-header">
        <h3>Featured Brands</h3>
        <div class="section-nav">
          <span>All Offers <AppIcon name="chevron-right" :size="10" /></span>
          <div class="nav-arrows">
            <AppIcon name="chevron-left" /><AppIcon name="chevron-right" />
          </div>
        </div>
      </div>
      <div class="brands-grid">
        <div v-for="brand in brands" :key="brand.name" class="brand-card">
          <div class="brand-img"><img :src="brand.img" :alt="brand.name" /></div>
          <div class="brand-info">
            <h4>{{ brand.name }}</h4>
            <p>{{ brand.desc }}</p>
          </div>
        </div>
      </div>

      <!-- Top Saver Today -->
      <div class="section-header">
        <div class="section-title-group">
          <h3>Top Saver Today</h3>
          <span class="all-offers">All Offers <AppIcon name="chevron-right" :size="10" /></span>
          <div class="expires-badge">Expires in: 02 : 15 : 37</div>
        </div>
        <div class="section-nav">
          <div class="nav-arrows">
            <AppIcon name="chevron-left" /><AppIcon name="chevron-right" />
          </div>
        </div>
      </div>

      <div class="layout-with-sidebar">
        <div class="product-grid-4">
          <div v-for="product in topSaverProducts" :key="product.id" class="product-card">
            <div class="product-img"><img :src="product.image_url" :alt="product.name" /></div>
            <div class="product-brand">{{ product.brand_name }}</div>
            <div class="product-title">{{ product.name }}</div>
            <div class="product-rating">
              <span v-for="i in renderStars(product.rating).full" :key="'full-' + i">★</span>
              <span
                v-for="i in renderStars(product.rating).empty"
                :key="'empty-' + i"
                class="empty-star"
                >★</span
              >
              <span>({{ product.review_count }})</span>
            </div>
            <div class="product-price">
              <span v-if="product.compare_at_price" class="old-price">{{
                money(product.compare_at_price)
              }}</span>
              <span class="new-price">{{ money(product.price) }}</span>
            </div>
            <div class="qty-add">
              <div class="qty-box">
                <button type="button" @click="changeQuantity(product, -1)">-</button
                ><input
                  type="number"
                  min="1"
                  :value="quantityFor(product)"
                  @input="setQuantity(product, Number(($event.target as HTMLInputElement).value))"
                /><button type="button" @click="changeQuantity(product, 1)">+</button>
              </div>
              <span class="sold-text">{{ product.stock }} in stock</span>
            </div>
            <button class="btn btn-full" @click="addToCart(product, true)">Add To Cart</button>
          </div>
        </div>

        <!-- Sidebar Form -->
        <div class="sidebar-widget">
          <h3>15% OFF</h3>
          <p>For new member sign up at<br />the first time</p>
          <div class="sidebar-form">
            <div class="input-icon">
              <AppIcon name="envelope" />
              <input type="email" placeholder="youremail@gmail.com" />
            </div>
            <div class="input-icon">
              <AppIcon name="lock" />
              <input type="password" placeholder="Password" />
            </div>
            <div class="input-icon">
              <AppIcon name="lock" />
              <input type="password" placeholder="Re-enter Password" />
            </div>
            <button class="btn btn-full">Register Now</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Background split for lower sections -->
    <div v-if="!isSearch" class="lower-sections">
      <div class="container">
        <!-- Best Seller -->
        <div class="section-header section-header-top">
          <div class="section-title-group">
            <h3>Best Seller</h3>
            <div class="cat-tabs">
              <span
                v-for="tab in bestSellerTabs"
                :key="tab"
                :class="{ active: activeTab === tab }"
                @click="activeTab = tab"
                >{{ tab }}</span
              >
            </div>
          </div>
          <div class="nav-arrows nav-arrows-white">
            <AppIcon name="chevron-left" /><AppIcon name="chevron-right" />
          </div>
        </div>

        <div class="product-grid-5">
          <div v-for="product in bestSellerProducts" :key="product.id" class="product-card">
            <div class="product-img"><img :src="product.image_url" :alt="product.name" /></div>
            <div class="product-brand">{{ product.brand_name }}</div>
            <div class="product-title">{{ product.name }}</div>
            <div class="product-rating">
              <span v-for="i in renderStars(product.rating).full" :key="'full-' + i">★</span>
              <span
                v-for="i in renderStars(product.rating).empty"
                :key="'empty-' + i"
                class="empty-star"
                >★</span
              >
            </div>
            <div class="product-price">
              <span v-if="product.compare_at_price" class="old-price">{{
                money(product.compare_at_price)
              }}</span>
              <span class="new-price">{{ money(product.price) }}</span>
            </div>
            <div class="qty-add">
              <div class="qty-box">
                <button type="button" @click="changeQuantity(product, -1)">-</button
                ><input
                  type="number"
                  min="1"
                  :value="quantityFor(product)"
                  @input="setQuantity(product, Number(($event.target as HTMLInputElement).value))"
                /><button type="button" @click="changeQuantity(product, 1)">+</button>
              </div>
            </div>
            <button class="btn btn-full" @click="addToCart(product, true)">Add To Cart</button>
          </div>
        </div>

        <!-- Just Landing -->
        <div class="section-header">
          <div class="section-title-group">
            <h3>Just Landing</h3>
            <div class="cat-tabs">
              <span
                v-for="tab in bestSellerTabs"
                :key="tab"
                :class="{ active: activeTab === tab }"
                @click="activeTab = tab"
                >{{ tab }}</span
              >
            </div>
          </div>
        </div>

        <div class="product-grid-5">
          <div v-for="product in justLandingProducts" :key="product.id" class="product-card">
            <div class="product-img"><img :src="product.image_url" :alt="product.name" /></div>
            <div class="product-brand">{{ product.brand_name }}</div>
            <div class="product-title">{{ product.name }}</div>
            <div class="product-price">
              <span class="new-price">{{ money(product.price) }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
main {
  flex: 1;
}

/* Hero Section */
.hero-section {
  background-color: var(--bg-gray);
  padding: 30px 0;
}

.hero-grid {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 20px;
  height: 350px;
}

.hero-banner {
  border-radius: 8px;
  padding: 40px;
  position: relative;
  overflow: hidden;
  display: flex;
  align-items: center;
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
}

.banner-left {
  background-image:
    linear-gradient(to right, rgba(230, 240, 249, 0.95) 40%, rgba(230, 240, 249, 0) 100%),
    url('https://images.unsplash.com/photo-1622483767028-3f66f32aef97?auto=format&fit=crop&w=800&q=80');
  background-color: #e6f0f9;
}

.banner-right {
  background-image:
    linear-gradient(to top, rgba(255, 223, 140, 0.95) 40%, rgba(255, 223, 140, 0) 100%),
    url('https://images.unsplash.com/photo-1590779033100-9f60a05a013d?auto=format&fit=crop&w=500&q=80');
  background-color: #ffdf8c;
  align-items: flex-end;
}

.banner-content {
  position: relative;
  z-index: 2;
  max-width: 55%;
}

.banner-right .banner-content {
  max-width: 100%;
  width: 100%;
  text-align: center;
}

.banner-content h2 {
  font-size: 32px;
  margin-bottom: 15px;
  color: #111;
}

.banner-right .banner-content h2 {
  font-size: 24px;
}

.banner-content p {
  margin-bottom: 20px;
  color: #444;
  font-size: 13px;
  line-height: 1.5;
}

.btn-white {
  background: #fff;
  border: 1px solid #ccc;
}

/* Section Titles */
.section-header {
  margin: 40px 0 20px;
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
}

.section-header h3 {
  font-size: 20px;
  font-weight: 700;
}

.section-nav {
  display: flex;
  gap: 15px;
  color: var(--text-light);
  align-items: center;
}

.section-title-group {
  display: flex;
  align-items: center;
  gap: 20px;
}

.all-offers {
  font-size: 12px;
  color: var(--text-light);
}

.expires-badge {
  background: #e74c3c;
  color: #fff;
  padding: 5px 10px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: bold;
}

.nav-arrows i {
  background: var(--bg-gray);
  padding: 8px;
  border-radius: 50%;
  margin-left: 5px;
  cursor: pointer;
}

.nav-arrows-white i {
  background: #fff;
}

/* Category Grid */
.category-grid {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 15px;
}

.cat-card {
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 20px 10px;
  text-align: center;
  background: #fff;
  transition: box-shadow 0.3s;
  cursor: pointer;
}

.cat-card:hover,
.cat-card.active {
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.08);
}

.cat-card.active {
  border-color: transparent;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
}

.cat-card img {
  width: 60px;
  height: 60px;
  border-radius: 50%;
  object-fit: cover;
  margin-bottom: 12px;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
}

.cat-card p {
  font-weight: 600;
  font-size: 13px;
  line-height: 1.4;
}

/* Brands Grid */
.brands-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 20px;
}

.brand-card {
  border: 1px solid var(--border-color);
  border-radius: 8px;
  overflow: hidden;
}

.brand-img {
  height: 160px;
  width: 100%;
}

.brand-img img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.brand-info {
  padding: 15px;
  background: #fff;
}

.brand-info h4 {
  font-size: 14px;
  margin-bottom: 5px;
  text-transform: uppercase;
}

.brand-info p {
  font-size: 12px;
  color: var(--text-light);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

/* Product Grid */
.product-grid-4 {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 15px;
}

.product-grid-5 {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 15px;
}

.product-card {
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 15px;
  position: relative;
  display: flex;
  flex-direction: column;
  background: #fff;
}

.badge {
  position: absolute;
  top: 10px;
  left: 10px;
  color: white;
  padding: 3px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: bold;
  z-index: 2;
}

.product-img {
  height: 160px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 15px;
  border-radius: 4px;
  overflow: hidden;
  background: #fdfdfd;
}

.product-img img {
  height: 100%;
  width: 100%;
  object-fit: cover;
  transition: transform 0.3s;
}

.product-card:hover .product-img img {
  transform: scale(1.05);
}

.product-brand {
  font-size: 11px;
  color: var(--text-light);
  margin-bottom: 5px;
}

.product-title {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 10px;
  flex-grow: 1;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  line-height: 1.4;
}

.product-rating {
  color: #f1c40f;
  font-size: 11px;
  margin-bottom: 10px;
}

.product-rating span {
  color: #f1c40f;
}

.product-rating .empty-star {
  color: var(--border-color);
}

.product-price {
  margin-bottom: 15px;
}

.old-price {
  text-decoration: line-through;
  color: var(--text-light);
  font-size: 12px;
  margin-right: 5px;
}

.new-price {
  font-weight: 700;
  color: #e74c3c;
  font-size: 16px;
}

.qty-add {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 10px;
}

.qty-box {
  display: flex;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  overflow: hidden;
}

.qty-box button {
  background: #f5f5f5;
  border: none;
  padding: 5px 10px;
  cursor: pointer;
}

.qty-box input {
  width: 30px;
  text-align: center;
  border: none;
  border-left: 1px solid var(--border-color);
  border-right: 1px solid var(--border-color);
  font-size: 12px;
}

.sold-text {
  font-size: 12px;
  color: #555;
}

.btn-full {
  width: 100%;
}

/* Layout with Sidebar */
.layout-with-sidebar {
  display: grid;
  grid-template-columns: 3fr 1fr;
  gap: 20px;
}

.sidebar-widget {
  background: #fff8f0;
  border-radius: 8px;
  padding: 30px 20px;
  text-align: center;
  height: 100%;
  border: 1px solid #fae5d3;
}

.sidebar-widget h3 {
  color: #d35400;
  font-size: 24px;
  margin-bottom: 10px;
}

.sidebar-widget p {
  font-size: 13px;
  margin-bottom: 20px;
  color: #666;
  line-height: 1.5;
}

.sidebar-form {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.input-icon {
  position: relative;
}

.input-icon .app-icon {
  position: absolute;
  left: 12px;
  top: 14px;
  color: #aaa;
}

.input-icon input {
  width: 100%;
  padding: 12px;
  padding-left: 35px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-family: inherit;
}

.sidebar-form .btn {
  margin-top: 10px;
  font-size: 15px;
  padding: 12px;
}

/* Lower Sections */
.lower-sections {
  background-color: var(--bg-gray);
  padding-top: 20px;
  padding-bottom: 50px;
  margin-top: 50px;
}

.section-header-top {
  margin-top: 0;
}

/* Category Tabs */
.cat-tabs {
  display: flex;
  gap: 15px;
  font-size: 13px;
  font-weight: 500;
}

.cat-tabs span {
  cursor: pointer;
  color: var(--text-light);
}

.cat-tabs span.active {
  color: var(--text-main);
  font-weight: 700;
}
.search-results {
  min-height: 420px;
  padding-top: 8px;
  padding-bottom: 60px;
}
.product-card-disabled {
  opacity: 0.55;
  filter: grayscale(0.8);
}
.btn:disabled {
  cursor: not-allowed;
  opacity: 0.55;
}
.search-caption {
  margin-top: 6px;
  color: var(--text-light);
  font-size: 13px;
}
.state-message {
  padding: 48px 0;
  color: var(--text-light);
  text-align: center;
}
.error-message {
  color: #c0392b;
}
</style>
