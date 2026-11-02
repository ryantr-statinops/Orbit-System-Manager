import { expect, test } from "@playwright/test";

interface FrameProbe {
  active: boolean;
  lastFrame: number | null;
  intervals: number[];
}

declare global {
  interface Window {
    __orbitFrameProbe?: FrameProbe;
  }
}

test("records browser preview readiness and interaction frame times", async ({ page }) => {
  await page.addInitScript(() => {
    const probe: FrameProbe = { active: false, lastFrame: null, intervals: [] };
    window.__orbitFrameProbe = probe;
    const requestFrame = window.requestAnimationFrame.bind(window);

    window.requestAnimationFrame = (callback) =>
      requestFrame((timestamp) => {
        if (probe.active) {
          if (probe.lastFrame !== null) probe.intervals.push(timestamp - probe.lastFrame);
          probe.lastFrame = timestamp;
        }
        callback(timestamp);
      });
  });

  const navigationStarted = performance.now();
  await page.goto("/");
  await expect(page.getByRole("heading", { name: "System overview" })).toBeVisible();
  const browserUiReadyMs = performance.now() - navigationStarted;

  await page.getByLabel("Logical cores").selectOption("64");
  await page.getByLabel("Fixture state").selectOption("history");
  await expect(page.getByText("60 samples · 64 cores")).toBeVisible();
  await page.evaluate(() => {
    const probe = window.__orbitFrameProbe;
    if (!probe) return;
    probe.active = true;
    probe.lastFrame = null;
    probe.intervals = [];
  });

  const surfaceCanvas = page.locator(".surface-canvas canvas");
  const resetButton = page.getByRole("button", { name: "Reset camera" });
  if ((await surfaceCanvas.count()) && !(await resetButton.isDisabled())) {
    const bounds = await surfaceCanvas.boundingBox();
    if (bounds) {
      const centerY = bounds.y + bounds.height / 2;
      await page.mouse.move(bounds.x + bounds.width * 0.25, centerY);
      await page.mouse.down();
      await page.mouse.move(bounds.x + bounds.width * 0.75, centerY - 24, { steps: 36 });
      await page.mouse.up();
      await page.mouse.wheel(0, -160);
      await resetButton.click();
    }
  }

  const frameIntervals = await page.evaluate(() => {
    const probe = window.__orbitFrameProbe;
    if (!probe) return [];
    probe.active = false;
    return probe.intervals;
  });
  const sortedIntervals = [...frameIntervals].sort((left, right) => left - right);
  const p95Index = Math.max(0, Math.ceil(sortedIntervals.length * 0.95) - 1);
  const report = {
    browserUiReadyMs: Number(browserUiReadyMs.toFixed(1)),
    interactionFrameSamples: frameIntervals.length,
    interactionFrameP95Ms:
      sortedIntervals.length === 0 ? null : Number(sortedIntervals[p95Index].toFixed(2)),
    fixture: "64 cores / 60 deterministic samples",
    scope: "browser preview; not native Wails process-tree performance",
  };

  console.log(`ORBIT_UI_BENCHMARK ${JSON.stringify(report)}`);
  expect(frameIntervals.every((interval) => Number.isFinite(interval) && interval >= 0)).toBe(true);
});
