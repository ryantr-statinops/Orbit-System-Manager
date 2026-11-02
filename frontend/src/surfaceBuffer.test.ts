import { describe, expect, it } from "vitest";
import { createDashboardFixture } from "./fixtures";
import { CpuSurfaceBuffer } from "./surfaceBuffer";

describe("CpuSurfaceBuffer", () => {
  it("reuses its vertex buffer when fixture values change", () => {
    const buffer = new CpuSurfaceBuffer(4, 60);
    const originalPositions = buffer.positions;
    const fixture = createDashboardFixture(4, "history");

    expect(buffer.update(fixture.samples)).toBe(true);
    expect(buffer.positions).toBe(originalPositions);
    expect(buffer.geometry.index?.count).toBe((4 - 1) * (60 - 1) * 6);
  });

  it("rejects unavailable values instead of treating them as zero", () => {
    const buffer = new CpuSurfaceBuffer(4, 60);
    const fixture = createDashboardFixture(4, "unavailable");

    expect(buffer.update(fixture.samples)).toBe(false);
  });

  it("requires enough dimensions to form a surface", () => {
    expect(() => new CpuSurfaceBuffer(1, 60)).toThrow(RangeError);
    expect(() => new CpuSurfaceBuffer(4, 1)).toThrow(RangeError);
  });
});
