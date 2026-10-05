import { test, expect } from '@playwright/test'

test.describe('product detail text regression', () => {
  test('keeps the product copy and add-to-cart flow intact', async ({ page }) => {
    await page.goto('/product-detail')

    await expect(page.getByRole('heading', { name: 'British Beef Mince (20% Fat)' })).toBeVisible()
    await expect(page.getByText('$18.50', { exact: true })).toBeVisible()
    await expect(
      page.getByText(
        'Rich, finely ground British beef with just the right amount of fat for juicy burgers, hearty sauces, and weeknight comfort food.',
        { exact: true },
      ),
    ).toBeVisible()
    await expect(page.getByText('Freshness guaranteed', { exact: true })).toBeVisible()
    await expect(page.getByRole('button', { name: 'Nutrition', exact: true })).toBeVisible()
    await expect(
      page.getByRole('button', { name: 'Shipping & Returns', exact: true }),
    ).toBeVisible()

    await page.getByRole('button', { name: 'Add To Cart', exact: true }).click()

    const dialog = page.getByRole('dialog')
    await expect(dialog).toBeVisible()
    await expect(dialog.getByRole('heading', { name: 'Added to cart' })).toBeVisible()
    await expect(
      dialog.getByText('1 x British Beef Mince (20% Fat)', { exact: true }),
    ).toBeVisible()
    await expect(dialog.getByText('Subtotal $18.50', { exact: true })).toBeVisible()

    await dialog.getByRole('button', { name: 'View cart', exact: true }).click()
    await expect(page).toHaveURL(/\/cart$/)
    await expect(page.getByRole('heading', { name: 'Shopping cart' })).toBeVisible()
    await expect(page.getByRole('heading', { name: 'British Beef Mince (20% Fat)' })).toBeVisible()
    const orderSummary = page.getByRole('complementary', { name: 'Order summary' })
    await expect(orderSummary.getByText('Grand total', { exact: true })).toBeVisible()
    await expect(orderSummary.getByText('$18.50', { exact: true }).last()).toBeVisible()
  })
})
