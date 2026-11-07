import { mkdirSync } from "node:fs";
import { resolve } from "node:path";
import { expect, test } from "@playwright/test";

const viewports = [
  { width: 720, height: 520, mode: "compact" },
  { width: 1060, height: 800, mode: "full" },
  { width: 1280, height: 800, mode: "full" },
  { width: 1465, height: 955, mode: "full" },
] as const;

test("keeps the C dashboard inside supported viewports", async ({ page }) => {
  await page.goto("/");
  const artifactDirectory = resolve(process.cwd(), "../artifacts/ui-benchmark");
  mkdirSync(artifactDirectory, { recursive: true });

  for (const viewport of viewports) {
    await page.setViewportSize(viewport);
    await expect(page.getByRole("heading", { level: 1, name: "System overview" })).toBeVisible();
    await expect(page.getByRole("region", { name: "Resource metrics" })).toBeVisible();

    const layout = await page.evaluate(() => {
      const dashboard = document.querySelector<HTMLElement>(".dashboard-layout");
      const pageShell = document.querySelector<HTMLElement>(".page-shell");
      if (!dashboard || !pageShell) throw new Error("Dashboard layout is missing.");
      const dashboardBounds = dashboard.getBoundingClientRect();
      const shellBounds = pageShell.getBoundingClientRect();
      return {
        documentFits: document.documentElement.scrollHeight <= document.documentElement.clientHeight,
        dashboardFits: dashboardBounds.bottom <= shellBounds.bottom,
        dashboardHasNoInternalScroll: dashboard.scrollHeight <= dashboard.clientHeight + 1,
        railWidth: document.querySelector<HTMLElement>(".app-rail")?.getBoundingClientRect().width,
      };
    });

    expect(layout.documentFits, `${viewport.width}x${viewport.height}: document scroll`).toBe(true);
    expect(layout.dashboardFits, `${viewport.width}x${viewport.height}: dashboard clipped`).toBe(true);
    expect(layout.dashboardHasNoInternalScroll, `${viewport.width}x${viewport.height}: inner overflow`).toBe(true);
    expect(layout.railWidth).toBe(64);
    await page.screenshot({
      path: resolve(artifactDirectory, `compact-c-dashboard-${viewport.width}x${viewport.height}.png`),
      fullPage: false,
    });
  }

  await page.setViewportSize({ width: 720, height: 520 });
  await page.getByRole("button", { name: "Memory metric details" }).click();
  await expect(page.getByLabel("memory detail")).toBeVisible();
  await page.getByRole("button", { name: "← Overview" }).click();
  await expect(page.getByRole("region", { name: "Resource metrics" })).toBeVisible();
});

test("keeps deterministic fixture controls under Settings", async ({ page }) => {
  await page.setViewportSize({ width: 720, height: 520 });
  await page.goto("/");
  await page.getByRole("button", { name: "Settings" }).click();
  await expect(page.getByLabel("Logical cores")).toBeVisible();
  await page.getByLabel("Logical cores").selectOption("4");
  await expect(page.getByText("60 samples · 4 cores")).toBeVisible();
  await page.getByRole("button", { name: "Dashboard" }).click();
  await expect(page.getByRole("region", { name: "Resource metrics" })).toBeVisible();
});
