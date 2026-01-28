import { test, expect } from '@playwright/test';
import { setLocalSettings } from './helpers';

test.beforeEach(async ({ page }) => {
  await setLocalSettings(page);
});

test('receipt lookup returns JSON for an existing tx', async ({ page }) => {
  await page.goto('/wallet?as=u01');
  const txId = await page.locator('.tx-id').first().textContent();
  expect(txId).toBeTruthy();

  await page.goto('/receipt');
  await page.locator('[data-testid="receipt-txid"]').fill((txId ?? '').trim());
  await page.getByTestId('receipt-fetch').click();

  const pre = page.locator('.receipt-json pre');
  await expect(pre).toBeVisible();
  await expect(pre).toContainText((txId ?? '').trim());
});
