export const CORE_COUNTS = [0, 1, 4, 16, 32, 64] as const;
export const HISTORY_LENGTH_SECONDS = 60;

export type FixtureScenario = "history" | "empty" | "single" | "unavailable";

export interface FixtureSample {
  elapsedSeconds: number;
  cpuTotalPercent: number | null;
  cpuCorePercentages: Array<number | null>;
  memoryPercent: number | null;
  networkRxMiBPerSecond: number | null;
  diskReadMiBPerSecond: number | null;
}

export interface DashboardFixture {
  scenario: FixtureScenario;
  coreCount: number;
  samples: FixtureSample[];
}

function round(value: number): number {
  return Math.round(value * 10) / 10;
}

function makeSample(index: number, coreCount: number): FixtureSample {
  const cpuCorePercentages = Array.from({ length: coreCount }, (_, core) =>
    round(48 + 32 * Math.sin(index / 7 + core * 0.29)),
  );
  const cpuTotalPercent =
    cpuCorePercentages.length === 0
      ? null
      : round(
          cpuCorePercentages.reduce((total, value) => total + value, 0) /
            cpuCorePercentages.length,
        );

  return {
    elapsedSeconds: index,
    cpuTotalPercent,
    cpuCorePercentages,
    memoryPercent: round(54 + 4 * Math.sin(index / 13)),
    networkRxMiBPerSecond: round(1.2 + 0.8 * Math.sin(index / 5 + 0.4)),
    diskReadMiBPerSecond: round(3.5 + 2.1 * Math.sin(index / 8 + 1.2)),
  };
}

function unavailableSample(index: number, coreCount: number): FixtureSample {
  return {
    elapsedSeconds: index,
    cpuTotalPercent: null,
    cpuCorePercentages: Array.from({ length: coreCount }, () => null),
    memoryPercent: null,
    networkRxMiBPerSecond: null,
    diskReadMiBPerSecond: null,
  };
}

export function createDashboardFixture(
  coreCount: number,
  scenario: FixtureScenario,
): DashboardFixture {
  if (!CORE_COUNTS.includes(coreCount as (typeof CORE_COUNTS)[number])) {
    throw new RangeError(`Unsupported fixture core count: ${coreCount}`);
  }

  switch (scenario) {
    case "empty":
      return { scenario, coreCount, samples: [] };
    case "single":
      return { scenario, coreCount, samples: [makeSample(0, coreCount)] };
    case "unavailable":
      return {
        scenario,
        coreCount,
        samples: Array.from({ length: HISTORY_LENGTH_SECONDS }, (_, index) =>
          unavailableSample(index, coreCount),
        ),
      };
    case "history":
      return {
        scenario,
        coreCount,
        samples: Array.from({ length: HISTORY_LENGTH_SECONDS }, (_, index) =>
          makeSample(index, coreCount),
        ),
      };
  }
}
