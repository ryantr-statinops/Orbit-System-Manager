export const MAX_RENDER_PIXEL_RATIO = 1.5;

export function getRenderPixelRatio(devicePixelRatio: number): number {
  if (!Number.isFinite(devicePixelRatio) || devicePixelRatio <= 0) return 1;
  return Math.min(devicePixelRatio, MAX_RENDER_PIXEL_RATIO);
}
