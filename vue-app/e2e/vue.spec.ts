import { test, expect } from '@playwright/test'

test('renders the Farmart application shell', async ({ page }) => {
  await page.goto('/')
  await expect(page.getByText('Farmart', { exact: true })).toBeVisible()
  await expect(page.getByPlaceholder("I'm shopping for...")).toBeVisible()
})
