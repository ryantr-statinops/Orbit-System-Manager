import { describe, expect, it } from "vitest";
import { CORE_COUNTS } from "./fixtures";
import { getCoreGridLayout } from "./coreGrid";

describe("getCoreGridLayout", () => {
  it("returns an empty layout for unavailable core data", () => {
    expect(getCoreGridLayout(0, 260, 420)).toEqual({ columns: 0, rows: 0 });
  });

  it("fits every supported count into a near-square adaptive grid", () => {
    for (const count of CORE_COUNTS.filter((value) => value > 0)) {
      const { columns, rows } = getCoreGridLayout(count, 300, 300);
      expect(columns * rows).toBeGreaterThanOrEqual(count);
      expect(columns).toBeLessThanOrEqual(count);
      expect(rows).toBeLessThanOrEqual(count);
      expect(Math.abs(columns - rows)).toBeLessThanOrEqual(1);
    }
  });

  it("adapts columns to a wide or tall panel", () => {
    expect(getCoreGridLayout(16, 600, 200).columns).toBeGreaterThan(
      getCoreGridLayout(16, 200, 600).columns,
    );
  });

  it("rejects invalid core counts", () => {
    expect(() => getCoreGridLayout(-1, 100, 100)).toThrow(RangeError);
    expect(() => getCoreGridLayout(1.5, 100, 100)).toThrow(RangeError);
  });
});
