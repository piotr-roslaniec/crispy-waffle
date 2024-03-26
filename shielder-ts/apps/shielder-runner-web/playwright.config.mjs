import { defineConfig, devices } from '@playwright/test';

const url = `http://127.0.0.1:3000/`;

export default defineConfig({
  timeout: 3000 * 1000, // TODO: Goood for tests, adjust for benchmarks
  fullyParallel: false, // Run tests sequentially
  forbidOnly: !!process.env.CI,
  reporter: process.env.CI ? 'github' : 'list', // TODO: Make sure it works on CI
  retries: process.env.CI ? 2 : 0, // Retry failed tests only on CI
  workers: 1, // Run projects sequentially
  webServer: {
    command: 'pnpm build && pnpm start:prod',
    url,
    reuseExistingServer: false,
  },
  use: {
    baseURL: url,
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
    },
    // Wasm doesn't seem to work in Playwright Webkit.
    // {
    //   name: 'webkit',
    //   use: { ...devices['Desktop Safari'] }
    // }
  ],
});
