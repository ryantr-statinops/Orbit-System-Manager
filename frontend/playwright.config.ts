import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./e2e",
  reporter: "list",
  use: {
    baseURL: "http://127.0.0.1:9381",
    browserName: "chromium",
    headless: true,
  },
  webServer: {
    command: "npm run dev -- --host 127.0.0.1 --port 9381",
    url: "http://127.0.0.1:9381",
    reuseExistingServer: true,
    timeout: 30_000,
  },
});
