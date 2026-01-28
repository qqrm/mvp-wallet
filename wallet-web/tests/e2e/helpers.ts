import type { Page } from '@playwright/test';

export const setLocalSettings = async (page: Page) => {
  const apiBaseUrl = process.env.E2E_API_BASE_URL ?? 'http://127.0.0.1:3000';

  await page.addInitScript(({ apiBaseUrl }) => {
    const key = 'wallet-web.settings';
    const settings = {
      apiBaseUrl,
      themeMode: 'system',
    };
    window.localStorage.setItem(key, JSON.stringify(settings));
  }, { apiBaseUrl });
};
