import type { FixtureSample } from "./fixtures";

export type CpuHistoryPoint = [elapsedSeconds: number, percentage: number | null];

export function toCpuHistoryData(samples: FixtureSample[]): CpuHistoryPoint[] {
  return samples.map((sample) => [sample.elapsedSeconds, sample.cpuTotalPercent]);
}
