import { describe, expect, it } from "vitest";
import { toCpuHistoryData } from "./chartData";
import { createDashboardFixture } from "./fixtures";

describe("toCpuHistoryData", () => {
  it("maps the deterministic history into elapsed-time chart points", () => {
    const samples = createDashboardFixture(4, "history").samples;
    const points = toCpuHistoryData(samples);

    expect(points).toHaveLength(60);
    expect(points[0][0]).toBe(0);
    expect(points[59][0]).toBe(59);
    expect(points[20][1]).toBe(samples[20].cpuTotalPercent);
  });

  it("preserves null CPU values for unavailable metrics", () => {
    const samples = createDashboardFixture(1, "unavailable").samples;

    expect(toCpuHistoryData(samples)[0]).toEqual([0, null]);
  });

  it("keeps empty input empty", () => {
    expect(toCpuHistoryData([])).toEqual([]);
  });
});
