import { test, expect } from '@playwright/test';
import { setLocalSettings } from './helpers';

test.beforeEach(async ({ page }) => {
  await setLocalSettings(page);
});

const selectUzumOption = async (page: any, selector: string, optionText: string) => {
  const trigger = page.locator(selector);
  await expect(trigger).toBeVisible();
  await trigger.click();
  // Options are rendered in an overlay/portal; match by text.
  await page.getByText(optionText, { exact: true }).click();
};

test('admin create account and topup respect selected user', async ({ page }) => {
  await page.goto('/admin');

  // Wait for users to load.
  await expect(page.getByText('No users found.')).toHaveCount(0);
  await expect(page.getByText('Loading users...')).toHaveCount(0);
  await expect(page.getByText('Users')).toBeVisible();

  // Create account for Dilshod (u03) in EUR.
  await selectUzumOption(page, '[data-testid="admin-create-account-user"]', 'Dilshod (u03)');
  await selectUzumOption(page, '[data-testid="admin-create-account-currency"]', 'EUR');
  await page.locator('[data-testid="admin-create-account-label"]').fill('test');
  await page.getByTestId('admin-create-account').click();

  await expect(page.getByText(/Account\s+acc_\d+\s+created\s+for\s+Dilshod\s+\(u03\)\s+\(EUR\)\./)).toBeVisible();

  // Run topup for the same user.
  await selectUzumOption(page, '[data-testid="admin-topup-user"]', 'Dilshod (u03)');
  await selectUzumOption(page, '[data-testid="admin-topup-currency"]', 'RUB');
  await page.locator('[data-testid="admin-topup-amount"]').fill('1');
  await page.getByTestId('admin-topup').click();

  await expect(page.getByText(/Topup posted for Dilshod \(u03\):/)).toBeVisible();
});
