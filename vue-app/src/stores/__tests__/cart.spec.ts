import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createPinia, setActivePinia } from 'pinia'
import { useCartStore } from '../cart'
import type { Product } from '../../services/api'

const product: Product = {
  id: 1,
  name: 'Fresh apples',
  slug: 'fresh-apples',
  category_id: 1,
  category_name: 'Fruit',
  brand_id: 1,
  brand_name: 'Farmart',
  price: 3.5,
  compare_at_price: null,
  rating: 5,
  review_count: 2,
  stock: 10,
  image_url: '/apples.jpg',
  description: null,
}
const storage = new Map<string, string>()
const localStorageMock = {
  getItem: (key: string) => storage.get(key) ?? null,
  setItem: (key: string, value: string) => storage.set(key, value),
  removeItem: (key: string) => storage.delete(key),
  clear: () => storage.clear(),
}

describe('useCartStore', () => {
  beforeEach(() => {
    vi.stubGlobal('window', { localStorage: localStorageMock })
    localStorageMock.clear()
    setActivePinia(createPinia())
  })

  it('adds, merges and calculates cart totals', () => {
    const cart = useCartStore()
    cart.add(product, 2)
    cart.add(product, 1)

    expect(cart.items[0]?.quantity).toBe(3)
    expect(cart.itemCount).toBe(3)
    expect(cart.subtotal).toBe(10.5)
  })

  it('updates and removes items while persisting safely', () => {
    const cart = useCartStore()
    cart.add(product)
    cart.updateQuantity(product.id, 4)
    expect(JSON.parse(window.localStorage.getItem('farmart-cart') ?? '{}').items[0].quantity).toBe(
      4,
    )

    cart.remove(product.id)
    expect(cart.items).toHaveLength(0)
    expect(JSON.parse(window.localStorage.getItem('farmart-cart') ?? '{}').items).toEqual([])
  })

  it('expires after one week and caps merged quantity at stock', () => {
    const cart = useCartStore()
    cart.add(product, 9)
    cart.add(product, 9)

    expect(cart.items[0]?.quantity).toBe(10)

    const stored = JSON.parse(window.localStorage.getItem('farmart-cart') ?? '{}')
    stored.expiresAt = Date.now() - 1
    window.localStorage.setItem('farmart-cart', JSON.stringify(stored))
    setActivePinia(createPinia())

    expect(useCartStore().items).toEqual([])
  })

  it('blocks checkout when a persisted item is out of stock', () => {
    const cart = useCartStore()
    cart.add(product)
    cart.items[0]!.product.stock = 0

    expect(cart.canCheckout).toBe(false)
    expect(cart.isPurchasable(cart.items[0]!)).toBe(false)
  })
})
