export interface Category {
  id: number
  name: string
  slug: string
  image_url: string | null
}
export interface Brand {
  id: number
  name: string
  description: string | null
  image_url: string | null
}
export interface Product {
  id: number
  name: string
  slug: string
  category_id: number
  category_name: string
  brand_id: number
  brand_name: string
  price: number
  compare_at_price: number | null
  rating: number
  review_count: number
  stock: number
  image_url: string
  description: string | null
}
export interface ProductList {
  items: Product[]
  page: number
  per_page: number
  total: number
}
export interface HomePayload {
  categories: Category[]
  brands: Brand[]
  top_savers: Product[]
  best_sellers: Product[]
  just_landed: Product[]
}
export interface AddCartRequest {
  product_id: number
  quantity: number
}
export interface CartItem {
  product: Product
  quantity: number
  line_total: number
}
export interface CartResponse {
  session_id: string
  items: CartItem[]
  subtotal: number
  item_count: number
}

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? 'http://127.0.0.1:8080/api/v1'

async function request<T>(path: string): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${path}`)
  if (!response.ok) throw new Error(`API request failed (${response.status})`)
  return response.json() as Promise<T>
}

async function requestJson<T>(path: string, body: unknown): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${path}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (!response.ok) throw new Error(`API request failed (${response.status})`)
  return response.json() as Promise<T>
}

export const getHome = () => request<HomePayload>('/home')
export const searchProducts = (query: string) =>
  request<ProductList>(`/products?q=${encodeURIComponent(query)}&per_page=50`)
export const addCartItem = (payload: AddCartRequest) =>
  requestJson<CartResponse>('/cart/items', payload)
