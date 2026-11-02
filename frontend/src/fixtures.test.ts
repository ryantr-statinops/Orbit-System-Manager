import { describe, expect, it } from "vitest";
import {
  CORE_COUNTS,
  HISTORY_LENGTH_SECONDS,
  createDashboardFixture,
} from "./fixtures";

describe("createDashboardFixture", () => {
  it("provides deterministic 60-second histories for supported core counts", () => {
    for (const coreCount of CORE_COUNTS) {
      const first = createDashboardFixture(coreCount, "history");
      const second = createDashboardFixture(coreCount, "history");

      expect(first.samples).toHaveLength(HISTORY_LENGTH_SECONDS);
      expect(first).toEqual(second);
      expect(first.samples.every((sample) => sample.cpuCorePercentages.length === coreCount)).toBe(true);
    }
  });

  it("preserves empty and single-sample states", () => {
    expect(createDashboardFixture(4, "empty").samples).toEqual([]);
    expect(createDashboardFixture(1, "single").samples).toHaveLength(1);
  });

  it("keeps unavailable values null instead of presenting zeros", () => {
    const fixture = createDashboardFixture(4, "unavailable");

    expect(fixture.samples).toHaveLength(HISTORY_LENGTH_SECONDS);
    expect(fixture.samples[0]).toMatchObject({
      cpuTotalPercent: null,
      cpuCorePercentages: [null, null, null, null],
      memoryPercent: null,
      networkRxMiBPerSecond: null,
      diskReadMiBPerSecond: null,
    });
  });

  it("keeps CPU unavailable when the fixture has zero cores", () => {
    const sample = createDashboardFixture(0, "history").samples[0];

    expect(sample.cpuTotalPercent).toBeNull();
    expect(sample.cpuCorePercentages).toEqual([]);
    expect(sample.memoryPercent).not.toBeNull();
  });

  it("rejects unlisted core counts", () => {
    expect(() => createDashboardFixture(8, "history")).toThrow(RangeError);
  });
});
