import { computed, ref } from 'vue'
import { defineStore } from 'pinia'
import type { CartItem, Product } from '../services/api'

const STORAGE_KEY = 'farmart-cart'
const CART_TTL_MS = 7 * 24 * 60 * 60 * 1000

type StoredCart = { items: CartItem[]; expiresAt: number }

function canUseStorage() {
  return typeof window !== 'undefined' && typeof window.localStorage !== 'undefined'
}

function isCartItem(value: unknown): value is CartItem {
  if (!value || typeof value !== 'object') return false
  const item = value as Partial<CartItem>
  const product = item.product as Partial<Product> | undefined
  return Boolean(
    product &&
    typeof product.id === 'number' &&
    typeof product.price === 'number' &&
    typeof item.quantity === 'number' &&
    Number.isFinite(item.quantity) &&
    item.quantity >= 1,
  )
}

function readStorage(): StoredCart {
  if (!canUseStorage()) return { items: [], expiresAt: 0 }

  try {
    const value: unknown = JSON.parse(window.localStorage.getItem(STORAGE_KEY) ?? 'null')
    const stored = Array.isArray(value)
      ? { items: value, expiresAt: Date.now() + CART_TTL_MS }
      : (value as Partial<StoredCart> | null)
    if (
      !stored ||
      typeof stored.expiresAt !== 'number' ||
      stored.expiresAt <= Date.now() ||
      !Array.isArray(stored.items)
    ) {
      window.localStorage.removeItem(STORAGE_KEY)
      return { items: [], expiresAt: 0 }
    }
    return { items: stored.items.filter(isCartItem), expiresAt: stored.expiresAt }
  } catch {
    return { items: [], expiresAt: 0 }
  }
}

function persist(items: CartItem[], expiresAt: number) {
  if (!canUseStorage()) return

  try {
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify({ items, expiresAt }))
  } catch {
    // Storage can be unavailable in privacy mode or a restricted test environment.
  }
}

function normaliseQuantity(quantity: number) {
  return Number.isFinite(quantity) ? Math.max(1, Math.floor(quantity)) : 1
}

export const useCartStore = defineStore('cart', () => {
  const stored = readStorage()
  const items = ref<CartItem[]>(stored.items)
  const expiresAt = ref(stored.expiresAt)

  const subtotal = computed(() => items.value.reduce((sum, item) => sum + item.line_total, 0))
  const itemCount = computed(() => items.value.reduce((sum, item) => sum + item.quantity, 0))

  const sync = () => {
    if (!items.value.length) {
      expiresAt.value = 0
      persist([], 0)
      return
    }
    if (!expiresAt.value) expiresAt.value = Date.now() + CART_TTL_MS
    persist(items.value, expiresAt.value)
  }

  const isPurchasable = (item: CartItem) =>
    item.product.stock > 0 && item.quantity <= item.product.stock
  const canCheckout = computed(() => items.value.length > 0 && items.value.every(isPurchasable))

  const add = (product: Product, quantity = 1) => {
    if (product.stock < 1) return false
    const amount = normaliseQuantity(quantity)
    const existing = items.value.find((item) => item.product.id === product.id)

    if (existing) {
      existing.product = product
      existing.quantity = Math.min(existing.quantity + amount, product.stock)
      existing.line_total = existing.quantity * product.price
    } else {
      const safeQuantity = Math.min(amount, product.stock)
      items.value.push({
        product,
        quantity: safeQuantity,
        line_total: safeQuantity * product.price,
      })
    }

    sync()
    return true
  }

  const updateQuantity = (productId: number, quantity: number) => {
    const item = items.value.find((entry) => entry.product.id === productId)
    if (!item || item.product.stock < 1) return

    item.quantity = Math.min(normaliseQuantity(quantity), item.product.stock)
    item.line_total = item.quantity * item.product.price
    sync()
  }

  const remove = (productId: number) => {
    items.value = items.value.filter((item) => item.product.id !== productId)
    sync()
  }

  const clear = () => {
    items.value = []
    sync()
  }

  return {
    items,
    subtotal,
    itemCount,
    canCheckout,
    isPurchasable,
    add,
    updateQuantity,
    remove,
    clear,
  }
})

export const useCart = useCartStore
