import { defineConfig, devices } from "@playwright/test";
import process from "node:process";

/**
 * End-to-end tests of the front in a browser, with the UI API mocked by `page.route`
 * (no .NET backend needed). The Vite dev server is started on its own port.
 */
export const E2E_BACKEND_URL = "http://127.0.0.1:5999";
export const E2E_TOKEN = "e2e-token";

export default defineConfig({
  testDir: "./e2e",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  reporter: process.env.CI ? "github" : "list",
  use: {
    baseURL: "http://localhost:4173",
    trace: "retain-on-failure",
  },
  projects: [
    {
      name: "chromium",
      use: {
        ...devices["Desktop Chrome"],
        launchOptions: { executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE || undefined },
      },
    },
  ],
  webServer: {
    command: "npx vite --port 4173 --strictPort",
    url: "http://localhost:4173",
    reuseExistingServer: !process.env.CI,
    env: { VITE_EDUCATOR_BASE_URL: E2E_BACKEND_URL, VITE_EDUCATOR_TOKEN: E2E_TOKEN },
  },
});
