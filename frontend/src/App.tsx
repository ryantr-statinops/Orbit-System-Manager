import { useMemo, useState } from "react";
import {
  CORE_COUNTS,
  createDashboardFixture,
  type FixtureScenario,
} from "./fixtures";

const scenarios: Array<{ value: FixtureScenario; label: string }> = [
  { value: "history", label: "60-second history" },
  { value: "empty", label: "Empty" },
  { value: "single", label: "Single sample" },
  { value: "unavailable", label: "Unavailable metrics" },
];

function App() {
  const [coreCount, setCoreCount] = useState<number>(16);
  const [scenario, setScenario] = useState<FixtureScenario>("history");
  const fixture = useMemo(
    () => createDashboardFixture(coreCount, scenario),
    [coreCount, scenario],
  );
  const latest = fixture.samples[fixture.samples.length - 1];

  return (
    <main className="page-shell">
      <header className="top-bar">
        <div className="brand-lockup">
          <span className="brand-mark" aria-hidden="true">O</span>
          <div>
            <p className="eyebrow">ORBIT / SYSTEM MONITOR</p>
            <h1>System overview</h1>
          </div>
        </div>
        <div className="preview-status">
          <span className="status-dot" aria-hidden="true" />
          Synthetic fixture
        </div>
      </header>

      <section className="dashboard-preview" aria-labelledby="welcome-title">
        <div className="preview-heading">
          <div>
            <p className="eyebrow">STAGE 2 / UI PERFORMANCE POC</p>
            <h2 id="welcome-title">System overview</h2>
          </div>
          <p className="fixture-label">DETERMINISTIC · NO LIVE TELEMETRY</p>
        </div>

        <div className="fixture-controls" aria-label="Fixture controls">
          <label>
            Logical cores
            <select
              value={coreCount}
              onChange={(event) => setCoreCount(Number(event.target.value))}
            >
              {CORE_COUNTS.map((count) => (
                <option key={count} value={count}>
                  {count}
                </option>
              ))}
            </select>
          </label>
          <label>
            Fixture state
            <select
              value={scenario}
              onChange={(event) => setScenario(event.target.value as FixtureScenario)}
            >
              {scenarios.map((item) => (
                <option key={item.value} value={item.value}>
                  {item.label}
                </option>
              ))}
            </select>
          </label>
          <p className="sample-count" aria-live="polite">
            {fixture.samples.length} samples · {fixture.coreCount} cores
          </p>
        </div>

        <div className="metric-cards">
          <article className="metric-card">
            <p className="eyebrow">CPU TOTAL</p>
            <strong>
              {latest?.cpuTotalPercent == null
                ? "Unavailable"
                : `${latest.cpuTotalPercent.toFixed(1)}%`}
            </strong>
            <span>{fixture.coreCount} logical cores</span>
          </article>
          <article className="metric-card">
            <p className="eyebrow">MEMORY</p>
            <strong>
              {latest?.memoryPercent == null
                ? "Unavailable"
                : `${latest.memoryPercent.toFixed(1)}%`}
            </strong>
            <span>Fixture value</span>
          </article>
          <article className="metric-card">
            <p className="eyebrow">NETWORK RX</p>
            <strong>
              {latest?.networkRxMiBPerSecond == null
                ? "Unavailable"
                : `${latest.networkRxMiBPerSecond.toFixed(2)} MiB/s`}
            </strong>
            <span>Fixture value</span>
          </article>
        </div>

        <section className="chart-placeholder" aria-label="CPU history fixture">
          <div>
            <p className="eyebrow">CPU HISTORY</p>
            <p className="chart-description">
              {fixture.samples.length === 0
                ? "No samples in this fixture."
                : `${fixture.samples.length} deterministic samples are ready for chart rendering.`}
            </p>
          </div>
          <span className="chart-placeholder-mark" aria-hidden="true">↗</span>
        </section>
      </section>

      <footer className="page-footer">
        <span>Orbit System Manager</span>
        <span>PoC · in-memory fixture only</span>
      </footer>
    </main>
  );
}

export default App;
