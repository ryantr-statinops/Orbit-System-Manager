import { useMemo, useState } from "react";
import {
  CORE_COUNTS,
  createDashboardFixture,
  type FixtureScenario,
} from "./fixtures";
import CpuHistoryChart from "./CpuHistoryChart";
import CpuSurfaceChart from "./CpuSurfaceChart";

const scenarios: Array<{ value: FixtureScenario; label: string }> = [
  { value: "history", label: "60-second history" },
  { value: "empty", label: "Empty" },
  { value: "single", label: "Single sample" },
  { value: "unavailable", label: "Unavailable metrics" },
];

function App() {
  const [coreCount, setCoreCount] = useState<number>(16);
  const [scenario, setScenario] = useState<FixtureScenario>("history");
  const [activeView, setActiveView] = useState<"dashboard" | "settings">("dashboard");
  const fixture = useMemo(
    () => createDashboardFixture(coreCount, scenario),
    [coreCount, scenario],
  );
  const latest = fixture.samples[fixture.samples.length - 1];

  const fixtureControls = (
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
  );

  return (
    <div className="app-shell">
      <nav className="app-rail" aria-label="Primary navigation">
        <div className="rail-brand" aria-label="Orbit System Manager">O</div>
        <button
          className={`rail-button${activeView === "dashboard" ? " active" : ""}`}
          type="button"
          aria-label="Dashboard"
          aria-current={activeView === "dashboard" ? "page" : undefined}
          onClick={() => setActiveView("dashboard")}
        >
          <span aria-hidden="true">▦</span>
        </button>
        <button
          className={`rail-button${activeView === "settings" ? " active" : ""}`}
          type="button"
          aria-label="Settings"
          aria-current={activeView === "settings" ? "page" : undefined}
          onClick={() => setActiveView("settings")}
        >
          <span aria-hidden="true">⚙</span>
        </button>
      </nav>

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

      {activeView === "dashboard" ? (
      <section className="dashboard-preview" aria-labelledby="welcome-title">
        <div className="preview-heading">
          <div>
            <p className="eyebrow">STAGE 2 / UI PERFORMANCE POC</p>
            <h2 id="welcome-title">System overview</h2>
          </div>
          <p className="fixture-label">DETERMINISTIC · NO LIVE TELEMETRY</p>
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

        <section className="chart-panel" aria-label="CPU history fixture">
          <div className="chart-panel-heading">
            <div>
              <p className="eyebrow">CPU HISTORY</p>
              <p className="chart-description">Total logical processor usage</p>
            </div>
            <span className="chart-unit">Percent · 60 seconds</span>
          </div>
          <CpuHistoryChart samples={fixture.samples} />
        </section>

        <CpuSurfaceChart fixture={fixture} />
      </section>
      ) : (
        <section className="settings-view" aria-labelledby="settings-title">
          <div>
            <p className="eyebrow">PREVIEW CONTROLS</p>
            <h2 id="settings-title">Settings</h2>
            <p className="settings-description">
              Deterministic fixture controls for UI review. These values are not saved.
            </p>
          </div>
          {fixtureControls}
        </section>
      )}

      <footer className="page-footer">
        <span>Orbit System Manager</span>
        <span>PoC · in-memory fixture only</span>
      </footer>
      </main>
    </div>
  );
}

export default App;
