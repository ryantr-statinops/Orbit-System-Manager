import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./e2e",
  reporter: "list",
  use: {
    baseURL: "http://127.0.0.1:9245",
    browserName: "chromium",
    headless: true,
  },
  webServer: {
    command: "npm run dev -- --host 127.0.0.1 --port 9245",
    url: "http://127.0.0.1:9245",
    reuseExistingServer: true,
    timeout: 30_000,
  },
});
