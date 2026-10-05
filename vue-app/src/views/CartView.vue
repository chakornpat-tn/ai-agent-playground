<script setup lang="ts">
import AppIcon from '../components/AppIcon.vue'
import DsButton from '../components/DsButton.vue'
import { useCartStore } from '../stores/cart'

const cart = useCartStore()
const money = (value: number) => `$${value.toFixed(2)}`
</script>

<template>
  <main class="cart-page">
    <div class="cart-container">
      <nav class="breadcrumbs" aria-label="Breadcrumb">
        <RouterLink to="/">Home</RouterLink><AppIcon name="chevron-right" :size="11" /><span
          >Shopping cart</span
        >
      </nav>

      <section class="cart-heading">
        <div>
          <p class="eyebrow">YOUR FRESH PICKS</p>
          <h1>Shopping cart</h1>
        </div>
        <span v-if="cart.itemCount" class="item-count"
          >{{ cart.itemCount }} {{ cart.itemCount === 1 ? 'item' : 'items' }}</span
        >
      </section>

      <section v-if="!cart.items.length" class="empty-cart" aria-live="polite">
        <div class="empty-icon"><AppIcon name="basket" :size="32" /></div>
        <p class="eyebrow">NOTHING HERE YET</p>
        <h2>Your cart is waiting for something good</h2>
        <p>Fill it with fresh groceries, pantry favourites, and everyday essentials.</p>
        <RouterLink to="/" class="button-link"
          ><DsButton>Continue shopping <AppIcon name="arrow-right" :size="15" /></DsButton
        ></RouterLink>
      </section>

      <div v-else class="cart-layout">
        <section class="cart-list" aria-label="Cart items">
          <article
            v-for="item in cart.items"
            :key="item.product.id"
            class="cart-row"
            :class="{ 'cart-row-disabled': !cart.isPurchasable(item) }"
          >
            <div class="product-image">
              <img :src="item.product.image_url" :alt="item.product.name" />
            </div>
            <div class="product-details">
              <span class="product-brand">{{ item.product.brand_name }}</span>
              <h2>{{ item.product.name }}</h2>
              <span class="unit-price">{{ money(item.product.price) }} each</span
              ><span v-if="!cart.isPurchasable(item)" class="disabled-note"
                >Unavailable — remove before checkout</span
              >
            </div>
            <div class="quantity-control" aria-label="Quantity">
              <button
                type="button"
                aria-label="Decrease quantity"
                :disabled="!cart.isPurchasable(item)"
                @click="cart.updateQuantity(item.product.id, item.quantity - 1)"
              >
                −
              </button>
              <input
                :value="item.quantity"
                type="number"
                min="1"
                :disabled="!cart.isPurchasable(item)"
                :aria-label="`Quantity for ${item.product.name}`"
                @change="
                  cart.updateQuantity(
                    item.product.id,
                    Number(($event.target as HTMLInputElement).value),
                  )
                "
              />
              <button
                type="button"
                aria-label="Increase quantity"
                :disabled="!cart.isPurchasable(item)"
                @click="cart.updateQuantity(item.product.id, item.quantity + 1)"
              >
                +
              </button>
            </div>
            <strong class="line-total">{{ money(item.line_total) }}</strong>
            <button
              class="remove-button"
              type="button"
              :aria-label="`Remove ${item.product.name}`"
              @click="cart.remove(item.product.id)"
            >
              <AppIcon name="xmark" :size="16" />
            </button>
          </article>
          <RouterLink to="/" class="continue-link"
            ><AppIcon name="arrow-left" :size="14" /> Continue shopping</RouterLink
          >
        </section>

        <aside class="summary-card" aria-label="Order summary">
          <p class="eyebrow">ORDER SUMMARY</p>
          <h2>Fresh, on its way</h2>
          <div class="summary-line">
            <span>Subtotal</span><strong>{{ money(cart.subtotal) }}</strong>
          </div>
          <div class="summary-line muted">
            <span>Delivery</span><span>Calculated at checkout</span>
          </div>
          <div class="summary-divider"></div>
          <div class="summary-line grand-total">
            <span>Grand total</span><strong>{{ money(cart.subtotal) }}</strong>
          </div>
          <DsButton block :disabled="!cart.canCheckout"
            >Checkout <AppIcon name="arrow-right" :size="15"
          /></DsButton>
          <p class="checkout-note">
            <AppIcon name="leaf" :size="16" />
            {{
              cart.canCheckout
                ? 'Secure checkout coming soon'
                : 'Remove unavailable items to continue'
            }}
          </p>
        </aside>
      </div>
    </div>
  </main>
</template>

<style scoped>
.cart-row-disabled {
  opacity: 0.55;
  filter: grayscale(0.8);
}
.disabled-note {
  display: block;
  margin-top: 6px;
  color: #c0392b;
  font-size: 11px;
}
.quantity-control button:disabled,
.quantity-control input:disabled {
  cursor: not-allowed;
  opacity: 0.5;
}
@media (max-width: 800px) {
  .cart-layout {
    grid-template-columns: 1fr;
    gap: 32px;
  }
  .summary-card {
    order: -1;
  }
  .cart-row {
    grid-template-columns: 72px minmax(0, 1fr) 28px;
    gap: 14px;
  }
  .product-image {
    width: 72px;
    height: 72px;
  }
  .quantity-control {
    grid-column: 2;
  }
  .line-total {
    grid-column: 2;
    text-align: left;
  }
  .remove-button {
    grid-column: 3;
    grid-row: 1;
  }
  .cart-heading h1 {
    font-size: 34px;
  }
}
</style>
