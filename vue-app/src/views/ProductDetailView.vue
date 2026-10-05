<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import AppDialog from '../components/AppDialog.vue'
import AppIcon from '../components/AppIcon.vue'
import DsButton from '../components/DsButton.vue'
import ProductRating from '../components/ProductRating.vue'
import QuantityControl from '../components/QuantityControl.vue'
import { useCartStore } from '../stores/cart'
import type { Product } from '../services/api'

const router = useRouter()
const cart = useCartStore()
const quantity = ref(1)
const selectedImage = ref(0)
const activeTab = ref('Description')
const addedDialogOpen = ref(false)

const images = [
  'https://images.unsplash.com/photo-1603048297172-c92544798d5a?auto=format&fit=crop&w=900&q=85',
  'https://images.unsplash.com/photo-1588347818036-558601350947?auto=format&fit=crop&w=900&q=85',
  'https://images.unsplash.com/photo-1551024506-0bccd828d307?auto=format&fit=crop&w=900&q=85',
]

// This view is intentionally static for now; keep a complete Product fallback so
// it can participate in the same cart flow as API-backed product cards.
const detailProduct: Product = {
  id: 1,
  name: 'British Beef Mince (20% Fat)',
  slug: 'british-beef-mince-20-fat',
  category_id: 1,
  category_name: 'Raw Meats',
  brand_id: 1,
  brand_name: 'Farmart',
  price: 18.5,
  compare_at_price: 22,
  rating: 5,
  review_count: 12,
  stock: 20,
  image_url: images[0] ?? '',
  description:
    'Rich, finely ground British beef with just the right amount of fat for juicy burgers, hearty sauces, and weeknight comfort food.',
}

const tabs = ['Description', 'Nutrition', 'Shipping & Returns']
const relatedProducts = [
  {
    title: 'Premium Beef Steak Cuts',
    brand: 'Farmart',
    price: '$24.50',
    image:
      'https://images.unsplash.com/photo-1544025162-d76694265947?auto=format&fit=crop&w=480&q=80',
  },
  {
    title: 'Farm Fresh Chicken Breast',
    brand: 'Meat Brand',
    price: '$12.70',
    image:
      'https://images.unsplash.com/photo-1604503468506-a8da13d82791?auto=format&fit=crop&w=480&q=80',
  },
  {
    title: 'Organic Minced Lamb',
    brand: 'Goodness Co.',
    price: '$21.90',
    image:
      'https://images.unsplash.com/photo-1603048297172-c92544798d5a?auto=format&fit=crop&w=480&q=80',
  },
  {
    title: 'Free Range Pork Fillet',
    brand: 'Farmart',
    price: '$16.00',
    image:
      'https://images.unsplash.com/photo-1432139509613-5c4255815697?auto=format&fit=crop&w=480&q=80',
  },
]

const addToCart = () => {
  cart.add(detailProduct, quantity.value)
  addedDialogOpen.value = true
}
const viewCart = () => {
  addedDialogOpen.value = false
  router.push('/cart')
}
</script>

<template>
  <main class="product-detail">
    <div class="container">
      <nav class="breadcrumbs" aria-label="Breadcrumb">
        <RouterLink to="/">Home</RouterLink><AppIcon name="chevron-right" :size="11" /><RouterLink
          to="/"
          >Raw Meats</RouterLink
        ><AppIcon name="chevron-right" :size="11" /><span>British Beef Mince</span>
      </nav>

      <section class="product-layout">
        <div class="gallery-panel">
          <div class="gallery-main">
            <span class="sale-badge">SAVE 15%</span
            ><img :src="images[selectedImage]" alt="British Beef Mince product photo" /><button
              class="gallery-arrow previous"
              aria-label="Previous image"
              @click="selectedImage = (selectedImage + images.length - 1) % images.length"
            >
              <AppIcon name="chevron-left" /></button
            ><button
              class="gallery-arrow next"
              aria-label="Next image"
              @click="selectedImage = (selectedImage + 1) % images.length"
            >
              <AppIcon name="chevron-right" />
            </button>
          </div>
          <div class="thumbnail-row">
            <button
              v-for="(image, index) in images"
              :key="image"
              class="thumbnail"
              :class="{ active: selectedImage === index }"
              @click="selectedImage = index"
            >
              <img :src="image" :alt="`Product view ${index + 1}`" />
            </button>
          </div>
        </div>

        <article class="product-info">
          <p class="product-brand">FARMART / FRESH CUTS</p>
          <h1>British Beef Mince<br /><span>(20% Fat)</span></h1>
          <div class="review-line">
            <ProductRating :rating="5" :reviews="12" /><span class="divider"></span
            ><span>In stock</span>
          </div>
          <div class="price-line">
            <del>$22.00</del><strong>$18.50</strong><span>Save $3.50</span>
          </div>
          <p class="product-summary">
            Rich, finely ground British beef with just the right amount of fat for juicy burgers,
            hearty sauces, and weeknight comfort food.
          </p>
          <div class="meta-list">
            <div><span>Weight</span><b>500g</b></div>
            <div><span>Origin</span><b>British Isles</b></div>
            <div><span>Best before</span><b>5 days</b></div>
          </div>
          <div class="buy-row">
            <QuantityControl v-model="quantity" /><DsButton block @click="addToCart"
              ><AppIcon name="cart" :size="16" /> Add To Cart</DsButton
            >
          </div>
          <DsButton variant="outline" block @click="addToCart"
            >Buy it now <AppIcon name="arrow-right" :size="15"
          /></DsButton>
          <div class="delivery-note">
            <AppIcon name="leaf" :size="18" /><span
              ><b>Freshness guaranteed</b
              ><small>Order before 2pm for next-day delivery</small></span
            >
          </div>
        </article>
      </section>

      <section class="service-strip">
        <div>
          <AppIcon name="basket" :size="20" /><span
            ><b>Freshly packed</b><small>Prepared with care</small></span
          >
        </div>
        <div>
          <AppIcon name="clock" :size="20" /><span
            ><b>Fast delivery</b><small>Next day available</small></span
          >
        </div>
        <div>
          <AppIcon name="store" :size="20" /><span
            ><b>Farmart quality</b><small>Always hand selected</small></span
          >
        </div>
        <div>
          <AppIcon name="xmark" :size="20" /><span
            ><b>Easy returns</b><small>30 day guarantee</small></span
          >
        </div>
      </section>

      <section class="details-section">
        <div class="detail-tabs">
          <button
            v-for="tab in tabs"
            :key="tab"
            :class="{ active: activeTab === tab }"
            @click="activeTab = tab"
          >
            {{ tab }}
          </button>
        </div>
        <div class="detail-copy" v-if="activeTab === 'Description'">
          <div>
            <p class="section-kicker">THE GOOD STUFF</p>
            <h2>Simple ingredients.<br />Big comfort.</h2>
          </div>
          <div>
            <p>
              Our British Beef Mince is prepared from carefully selected cuts, ground fresh in small
              batches for a tender texture and rich, natural flavour. It is a versatile fridge
              staple made for the meals you already love.
            </p>
            <ul>
              <li>100% British beef</li>
              <li>No added preservatives</li>
              <li>Perfect for burgers, meatballs and sauces</li>
            </ul>
          </div>
        </div>
        <div v-else class="detail-placeholder">
          <p>
            {{ activeTab }} information is prepared with the same clear, practical Farmart language.
          </p>
        </div>
      </section>

      <section class="related-section">
        <div class="section-heading">
          <div>
            <p class="section-kicker">YOU MAY ALSO LIKE</p>
            <h2>More from the butcher</h2>
          </div>
          <RouterLink to="/" class="see-all"
            >View all <AppIcon name="arrow-right" :size="14"
          /></RouterLink>
        </div>
        <div class="related-grid">
          <article v-for="item in relatedProducts" :key="item.title" class="related-card">
            <div class="related-image"><img :src="item.image" :alt="item.title" /></div>
            <span>{{ item.brand }}</span>
            <h3>{{ item.title }}</h3>
            <strong>{{ item.price }}</strong>
          </article>
        </div>
      </section>
    </div>

    <AppDialog
      v-model:open="addedDialogOpen"
      title="Added to cart"
      description="Your fresh cut is on its way."
      ><div class="added-content">
        <div class="added-icon"><AppIcon name="basket" :size="28" /></div>
        <p>
          <b>{{ quantity }} x {{ detailProduct.name }}</b
          ><span>Subtotal ${{ (detailProduct.price * quantity).toFixed(2) }}</span>
        </p>
      </div>
      <template #footer
        ><DsButton variant="outline" @click="addedDialogOpen = false">Continue shopping</DsButton
        ><DsButton @click="viewCart">View cart</DsButton></template
      ></AppDialog
    >
  </main>
</template>

<style scoped>
.product-detail {
  --ds-primary: #f8b82a;
  --ds-primary-hover: #d4a017;
  --ds-ink: #333;
  --ds-muted: #777;
  --ds-border: #eaeaea;
  --ds-surface-soft: #f9f9f9;
  --ds-badge: #c0392b;
  --ds-star: #f1c40f;
  --ds-sidebar: #fff8f0;
  background: #fff;
  color: var(--ds-ink);
  font-family: Inter, sans-serif;
}
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 15px;
}
.breadcrumbs {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 23px 0;
  color: var(--ds-muted);
  font-size: 12px;
}
.breadcrumbs a {
  color: var(--ds-ink);
  text-decoration: none;
}
.breadcrumbs a:hover,
.see-all:hover {
  color: var(--ds-badge);
}
.product-layout {
  display: grid;
  grid-template-columns: minmax(0, 1.05fr) minmax(360px, 0.95fr);
  gap: 64px;
  padding: 26px 0 72px;
}
.gallery-main {
  position: relative;
  height: 480px;
  overflow: hidden;
  border-radius: 8px;
  background: var(--ds-surface-soft);
}
.gallery-main > img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}
.sale-badge {
  position: absolute;
  top: 20px;
  left: 20px;
  z-index: 1;
  padding: 5px 10px;
  border-radius: 4px;
  background: var(--ds-badge);
  color: #fff;
  font-size: 11px;
  font-weight: 700;
}
.gallery-arrow {
  position: absolute;
  top: 50%;
  display: grid;
  width: 36px;
  height: 36px;
  place-items: center;
  border: 0;
  border-radius: 50%;
  background: #fff;
  color: var(--ds-ink);
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
  cursor: pointer;
  transform: translateY(-50%);
}
.gallery-arrow.previous {
  left: 16px;
}
.gallery-arrow.next {
  right: 16px;
}
.thumbnail-row {
  display: flex;
  gap: 12px;
  margin-top: 16px;
}
.thumbnail {
  width: 76px;
  height: 76px;
  padding: 0;
  overflow: hidden;
  border: 2px solid transparent;
  border-radius: 4px;
  background: var(--ds-surface-soft);
  cursor: pointer;
}
.thumbnail.active {
  border-color: var(--ds-primary);
}
.thumbnail img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}
.product-info {
  padding: 18px 0;
}
.product-brand,
.section-kicker {
  color: var(--ds-badge);
  font:
    500 11px/1.2 'DM Mono',
    monospace;
  letter-spacing: 0.08em;
}
.product-info h1 {
  margin: 17px 0 16px;
  font-size: 40px;
  line-height: 1.1;
  letter-spacing: -0.04em;
}
.product-info h1 span {
  font-weight: 400;
}
.review-line {
  display: flex;
  align-items: center;
  gap: 12px;
}
.divider {
  width: 1px;
  height: 14px;
  background: var(--ds-border);
}
.review-line > span:last-child {
  color: #3f8a57;
  font-size: 12px;
}
.price-line {
  display: flex;
  align-items: center;
  gap: 10px;
  margin: 25px 0 18px;
}
.price-line del {
  color: var(--ds-muted);
  font-size: 14px;
}
.price-line strong {
  color: var(--ds-badge);
  font-size: 25px;
}
.price-line > span {
  padding: 4px 8px;
  border-radius: 4px;
  background: #fff0ee;
  color: var(--ds-badge);
  font-size: 11px;
  font-weight: 600;
}
.product-summary {
  max-width: 470px;
  color: var(--ds-muted);
  font-size: 14px;
  line-height: 1.7;
}
.meta-list {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
  margin: 28px 0;
  border-top: 1px solid var(--ds-border);
  border-bottom: 1px solid var(--ds-border);
  padding: 17px 0;
}
.meta-list div {
  display: flex;
  flex-direction: column;
  gap: 5px;
}
.meta-list span {
  color: var(--ds-muted);
  font-size: 11px;
}
.meta-list b {
  font-size: 12px;
}
.buy-row {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: 12px;
  margin-bottom: 12px;
}
.buy-row .ds-button {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 8px;
}
.product-info > .ds-button {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 8px;
}
.delivery-note {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-top: 28px;
  padding: 14px;
  background: var(--ds-sidebar);
  color: var(--ds-primary-hover);
}
.delivery-note span {
  display: flex;
  flex-direction: column;
  gap: 3px;
}
.delivery-note b {
  color: var(--ds-ink);
  font-size: 12px;
}
.delivery-note small {
  color: var(--ds-muted);
  font-size: 11px;
}
.service-strip {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  padding: 23px 0;
  border-top: 1px solid var(--ds-border);
  border-bottom: 1px solid var(--ds-border);
}
.service-strip > div {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 10px;
  border-right: 1px solid var(--ds-border);
  color: var(--ds-primary-hover);
}
.service-strip > div:last-child {
  border: 0;
}
.service-strip span {
  display: flex;
  flex-direction: column;
  gap: 3px;
}
.service-strip b {
  color: var(--ds-ink);
  font-size: 12px;
}
.service-strip small {
  color: var(--ds-muted);
  font-size: 11px;
}
.details-section {
  padding: 72px 0;
  border-bottom: 1px solid var(--ds-border);
}
.detail-tabs {
  display: flex;
  gap: 32px;
  border-bottom: 1px solid var(--ds-border);
}
.detail-tabs button {
  position: relative;
  padding: 0 0 14px;
  border: 0;
  background: transparent;
  color: var(--ds-muted);
  cursor: pointer;
  font:
    600 14px Inter,
    sans-serif;
}
.detail-tabs button.active {
  color: var(--ds-ink);
}
.detail-tabs button.active:after {
  position: absolute;
  right: 0;
  bottom: -1px;
  left: 0;
  height: 2px;
  background: var(--ds-primary);
  content: '';
}
.detail-copy {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 80px;
  padding-top: 42px;
}
.detail-copy h2,
.section-heading h2 {
  margin-top: 15px;
  font-size: 30px;
  line-height: 1.15;
  letter-spacing: -0.04em;
}
.detail-copy > div:last-child {
  color: var(--ds-muted);
  font-size: 14px;
  line-height: 1.7;
}
.detail-copy ul {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
  margin: 25px 0 0;
  padding: 0;
  list-style: none;
  color: var(--ds-ink);
  font-size: 12px;
}
.detail-copy li {
  padding: 8px 10px;
  border: 1px solid var(--ds-border);
  border-radius: 4px;
}
.detail-placeholder {
  padding: 40px 0;
  color: var(--ds-muted);
  font-size: 14px;
}
.related-section {
  padding: 72px 0 88px;
}
.section-heading {
  display: flex;
  align-items: end;
  justify-content: space-between;
  margin-bottom: 26px;
}
.see-all {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--ds-muted);
  font-size: 12px;
  text-decoration: none;
}
.related-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
}
.related-card {
  padding: 15px;
  border: 1px solid var(--ds-border);
  border-radius: 8px;
  background: #fff;
  transition:
    box-shadow 0.2s,
    transform 0.2s;
}
.related-card:hover {
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.08);
  transform: translateY(-2px);
}
.related-image {
  height: 180px;
  margin-bottom: 14px;
  overflow: hidden;
  border-radius: 4px;
  background: var(--ds-surface-soft);
}
.related-image img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}
.related-card > span {
  color: var(--ds-muted);
  font-size: 11px;
}
.related-card h3 {
  min-height: 38px;
  margin: 7px 0 12px;
  font-size: 14px;
  font-weight: 600;
  line-height: 1.4;
}
.related-card > strong {
  color: var(--ds-badge);
  font-size: 16px;
}
.added-content {
  display: flex;
  align-items: center;
  gap: 14px;
}
.added-icon {
  display: grid;
  width: 56px;
  height: 56px;
  place-items: center;
  border-radius: 8px;
  background: var(--ds-sidebar);
  color: var(--ds-primary-hover);
}
.added-content p {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-size: 14px;
}
.added-content span {
  color: var(--ds-badge);
  font-weight: 700;
}
@media (max-width: 900px) {
  .product-layout {
    grid-template-columns: 1fr;
    gap: 28px;
  }
  .gallery-main {
    height: 420px;
  }
  .product-info {
    padding: 0;
  }
  .related-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  .service-strip {
    grid-template-columns: repeat(2, 1fr);
  }
  .service-strip > div:nth-child(2) {
    border: 0;
  }
  .detail-copy {
    gap: 30px;
  }
}
@media (max-width: 767px) {
  .breadcrumbs {
    padding: 16px 0;
  }
  .product-layout {
    padding: 10px 0 48px;
  }
  .gallery-main {
    height: 340px;
  }
  .product-info h1 {
    font-size: 32px;
  }
  .meta-list {
    gap: 8px;
  }
  .buy-row {
    grid-template-columns: auto 1fr;
  }
  .service-strip {
    grid-template-columns: 1fr 1fr;
    gap: 0;
  }
  .service-strip > div {
    padding: 14px 6px;
  }
  .service-strip b {
    font-size: 11px;
  }
  .detail-tabs {
    gap: 20px;
    overflow: auto;
    white-space: nowrap;
  }
  .details-section,
  .related-section {
    padding: 48px 0;
  }
  .detail-copy {
    grid-template-columns: 1fr;
    gap: 22px;
    padding-top: 30px;
  }
  .detail-copy h2,
  .section-heading h2 {
    font-size: 26px;
  }
  .section-heading {
    align-items: flex-start;
    gap: 18px;
    flex-direction: column;
  }
  .related-grid {
    grid-template-columns: 1fr 1fr;
    gap: 10px;
  }
  .related-card {
    padding: 10px;
  }
  .related-image {
    height: 130px;
  }
  .related-card h3 {
    font-size: 12px;
  }
  .dialog-footer .ds-button {
    font-size: 12px;
    padding: 8px 10px;
  }
}
</style>
