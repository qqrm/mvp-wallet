import { test, expect } from '@playwright/test';
import { setLocalSettings } from './helpers';

test.beforeEach(async ({ page }) => {
  await setLocalSettings(page);
});

test('wallet shows display name and full tx ids without copy button', async ({ page }) => {
  await page.goto('/wallet?as=u01');

  // User pill should include display name + id.
  await expect(page.locator('[data-testid="wallet-user-pill"]')).toContainText('Amina (u01)');

  // Recent transactions should render at least one tx id.
  const firstTxId = page.locator('.tx-id').first();
  await expect(firstTxId).toBeVisible();
  await expect(firstTxId).toHaveText(/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/i);

  // No copy buttons in the table.
  await expect(page.locator('.recent-txs button')).toHaveCount(0);
});
