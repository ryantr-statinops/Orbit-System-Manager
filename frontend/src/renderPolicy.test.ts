import { describe, expect, it } from "vitest";
import { getRenderPixelRatio, MAX_RENDER_PIXEL_RATIO } from "./renderPolicy";

describe("getRenderPixelRatio", () => {
  it("preserves ordinary display scaling", () => {
    expect(getRenderPixelRatio(1)).toBe(1);
    expect(getRenderPixelRatio(1.25)).toBe(1.25);
  });

  it("caps high-density displays", () => {
    expect(getRenderPixelRatio(2)).toBe(MAX_RENDER_PIXEL_RATIO);
    expect(getRenderPixelRatio(3)).toBe(MAX_RENDER_PIXEL_RATIO);
  });

  it("uses a safe default for invalid values", () => {
    expect(getRenderPixelRatio(0)).toBe(1);
    expect(getRenderPixelRatio(Number.NaN)).toBe(1);
  });
});
