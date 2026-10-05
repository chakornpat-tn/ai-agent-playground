<script setup lang="ts">
import { RouterView } from 'vue-router'
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { storeToRefs } from 'pinia'
import AppIcon from './components/AppIcon.vue'
import { useCartStore } from './stores/cart'

const searchQuery = ref('')
const router = useRouter()
const cartStore = useCartStore()
const { subtotal: cartSubtotal, itemCount: cartItemCount } = storeToRefs(cartStore)

const submitSearch = () => {
  router.push({ path: '/', query: searchQuery.value.trim() ? { q: searchQuery.value.trim() } : {} })
}
</script>

<template>
  <div class="app">
    <!-- Header Top -->
    <header class="header-top">
      <div class="container flex-between">
        <div class="logo-area">
          <AppIcon name="store" :size="22" />
          <span>Farmart</span>
        </div>

        <div class="search-bar">
          <div class="search-cat">All Categories <AppIcon name="chevron-down" :size="10" /></div>
          <input
            v-model="searchQuery"
            type="text"
            class="search-input"
            placeholder="I'm shopping for..."
            @keyup.enter="submitSearch"
          />
          <button class="search-btn" aria-label="Search" @click="submitSearch">
            <AppIcon name="magnifying-glass" />
          </button>
        </div>

        <div class="header-actions">
          <div class="contact-info">
            <div class="contact-phone">8 800 332 65-66</div>
            <div class="contact-text">Support 24/7</div>
          </div>
          <div class="icon-btn"><AppIcon name="user" :size="20" /></div>
          <button
            class="cart-info"
            type="button"
            aria-label="View cart"
            @click="router.push('/cart')"
          >
            <div class="icon-btn">
              <AppIcon name="cart" :size="20" />
              <span v-if="cartItemCount > 0" class="cart-badge" aria-hidden="true">{{
                cartItemCount
              }}</span>
            </div>
            <div class="cart-price">
              <div class="cart-label">Your Cart</div>
              ${{ cartSubtotal.toFixed(2) }}
            </div>
          </button>
        </div>
      </div>
    </header>

    <!-- Navigation -->
    <nav class="header-nav">
      <div class="container flex-between">
        <div class="nav-left">
          <div class="shop-cat-btn"><AppIcon name="bars" /> Shop By Category</div>
          <ul class="nav-links">
            <li class="deals"><AppIcon name="bolt" /> Deals Today</li>
            <li>Special Prices</li>
            <li>Fresh <AppIcon name="chevron-down" :size="10" /></li>
            <li>Frozen <AppIcon name="chevron-down" :size="10" /></li>
            <li>Demo <AppIcon name="chevron-down" :size="10" /></li>
            <li>Shop <AppIcon name="chevron-down" :size="10" /></li>
          </ul>
        </div>
        <div class="recently-viewed"><AppIcon name="clock" /> Recently Viewed</div>
      </div>
    </nav>

    <!-- Main Content -->
    <RouterView />
  </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');

:root {
  --primary: #f8b82a;
  --primary-hover: #e0a320;
  --text-main: #333333;
  --text-light: #777777;
  --bg-gray: #f9f9f9;
  --border-color: #eaeaea;
  --container-width: 1200px;
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  font-family: 'Inter', sans-serif;
  color: var(--text-main);
  background-color: #ffffff;
  font-size: 14px;
}

.app {
  min-height: 100vh;
}

a {
  text-decoration: none;
  color: inherit;
}
ul {
  list-style: none;
}

.container {
  max-width: var(--container-width);
  margin: 0 auto;
  padding: 0 15px;
}

.flex-center {
  display: flex;
  align-items: center;
  justify-content: center;
}
.flex-between {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.btn {
  background-color: var(--primary);
  color: #000;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.3s;
}
.btn:hover {
  background-color: var(--primary-hover);
}
.btn-outline {
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  padding: 6px 12px;
}

/* --- Header Top --- */
.header-top {
  padding: 20px 0;
  border-bottom: 1px solid var(--border-color);
}
.logo-area {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 24px;
  font-weight: 700;
}
.logo-area i {
  color: var(--primary);
}

.search-bar {
  display: flex;
  align-items: center;
  border: 2px solid var(--primary);
  border-radius: 4px;
  width: 500px;
  overflow: hidden;
}
.search-cat {
  padding: 10px;
  background: #fff;
  border-right: 1px solid #ddd;
  color: var(--text-light);
  white-space: nowrap;
}
.search-input {
  flex-grow: 1;
  padding: 10px;
  border: none;
  outline: none;
}
.search-btn {
  padding: 10px 15px;
  background: #fff;
  border: none;
  cursor: pointer;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 20px;
}
.contact-info {
  text-align: right;
  line-height: 1.2;
}
.contact-phone {
  font-weight: 700;
  font-size: 16px;
}
.contact-text {
  font-size: 12px;
  color: var(--text-light);
}
.icon-btn {
  font-size: 20px;
  position: relative;
  cursor: pointer;
}
.cart-info {
  display: flex;
  align-items: center;
  gap: 10px;
  border: 0;
  padding: 0;
  color: inherit;
  background: transparent;
  text-align: left;
  cursor: pointer;
}
.cart-price {
  font-weight: 700;
}
.cart-label {
  font-size: 11px;
  color: #777;
  font-weight: normal;
}
.cart-badge {
  position: absolute;
  top: -8px;
  right: -8px;
  min-width: 18px;
  height: 18px;
  padding: 0 4px;
  border-radius: 9px;
  background: var(--primary);
  color: #000;
  font-size: 11px;
  font-weight: 700;
  line-height: 18px;
  text-align: center;
}

/* --- Header Nav --- */
.header-nav {
  padding: 10px 0;
  border-bottom: 1px solid var(--border-color);
}
.nav-left {
  display: flex;
  gap: 30px;
  align-items: center;
}
.shop-cat-btn {
  background: var(--primary);
  padding: 10px 20px;
  border-radius: 4px;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
}
.nav-links {
  display: flex;
  gap: 20px;
  font-weight: 500;
}
.nav-links li {
  cursor: pointer;
}
.nav-links .deals {
  color: #e74c3c;
}
.recently-viewed {
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
}
</style>
