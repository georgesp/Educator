import { defineConfig, devices } from "@playwright/test";
import process from "node:process";

/**
 * End-to-end tests of the front in a browser, with the Tauri commands simulated in the page
 * (see `mockApi` in e2e/lot1.spec.ts). The Vite dev server is started on its own port.
 */

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
    {
      // The iPad of the tablet lot (ADR 0006): iPad Air 13 inches in landscape, touch screen (SUB023808, SUB023812).
      name: "ipad-landscape",
      testMatch: /lot1\.spec\.ts/,
      use: {
        ...devices["Desktop Chrome"],
        viewport: { width: 1366, height: 1024 },
        deviceScaleFactor: 2,
        hasTouch: true,
        isMobile: true,
        launchOptions: { executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE || undefined },
      },
    },
  ],
  webServer: {
    command: "npx vite --port 4173 --strictPort",
    url: "http://localhost:4173",
    reuseExistingServer: !process.env.CI,
  },
});
