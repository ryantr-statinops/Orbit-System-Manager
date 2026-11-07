import { useEffect, useMemo, useState } from "react";
import {
  CORE_COUNTS,
  createDashboardFixture,
  type FixtureScenario,
} from "./fixtures";
import CoreGrid from "./LogicalCoreGrid";
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
  const [compactViewport, setCompactViewport] = useState(false);
  const [detailView, setDetailView] = useState<string | null>(null);
  const fixture = useMemo(
    () => createDashboardFixture(coreCount, scenario),
    [coreCount, scenario],
  );
  const latest = fixture.samples[fixture.samples.length - 1];

  useEffect(() => {
    const updateViewport = () =>
      setCompactViewport(window.innerWidth < 1060 || window.innerHeight < 800);
    updateViewport();
    window.addEventListener("resize", updateViewport);
    return () => window.removeEventListener("resize", updateViewport);
  }, []);

  const openDetail = (name: string) => {
    if (compactViewport) setDetailView(name);
  };

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

        {compactViewport && detailView ? (
          <section className="compact-detail" aria-label={`${detailView} detail`}>
            <button className="detail-back" type="button" onClick={() => setDetailView(null)}>
              ← Overview
            </button>
            {detailView === "surface" ? (
              <CpuSurfaceChart fixture={fixture} />
            ) : detailView === "cores" ? (
              <section className="core-detail-view" aria-label="Logical processor details">
                <h3>Logical processors</h3>
                <CoreGrid values={latest?.cpuCorePercentages ?? []} />
              </section>
            ) : detailView === "history" ? (
              <section className="chart-panel detail-chart">
                <h3>CPU history</h3>
                <CpuHistoryChart samples={fixture.samples} />
              </section>
            ) : (
              <article className="detail-metric">
                <p className="eyebrow">{detailView.toUpperCase()}</p>
                <strong>{
                  detailView === "memory"
                    ? latest?.memoryPercent == null ? "Unavailable" : `${latest.memoryPercent.toFixed(1)}%`
                    : detailView === "disk"
                      ? latest?.diskReadMiBPerSecond == null ? "Unavailable" : `${latest.diskReadMiBPerSecond.toFixed(1)} MiB/s`
                      : detailView === "network"
                        ? latest?.networkRxMiBPerSecond == null ? "Unavailable" : `${latest.networkRxMiBPerSecond.toFixed(2)} MiB/s`
                        : "Not in fixture"
                }</strong>
                <span>{detailView === "gpu" ? "No GPU sample supplied in this fixture." : "Deterministic fixture value."}</span>
              </article>
            )}
          </section>
        ) : (
        <div className="dashboard-layout">
          <section className="cpu-column data-panel" aria-label="CPU overview">
            <div className="column-heading">
              <div>
                <p className="eyebrow">CPU TOTAL</p>
                <strong className="cpu-total-value">
                  {latest?.cpuTotalPercent == null
                    ? "Unavailable"
                    : `${latest.cpuTotalPercent.toFixed(1)}%`}
                </strong>
              </div>
              <span className="metric-badge">{fixture.coreCount} cores</span>
            </div>
            <div className="core-section-heading">
              <p className="eyebrow">LOGICAL PROCESSORS</p>
              <span>{fixture.coreCount} detected</span>
            </div>
            <CoreGrid values={latest?.cpuCorePercentages ?? []} />
            <button className="core-detail-link" type="button" onClick={() => openDetail("cores")}>
              View core details
            </button>
          </section>
          <div className="center-column">
            <div className="surface-detail-target" onClick={() => openDetail("surface")}>
              <CpuSurfaceChart fixture={fixture} />
              <button className="panel-detail-link" type="button" onClick={(event) => { event.stopPropagation(); openDetail("surface"); }}>
                View surface detail
              </button>
            </div>
            <section className="chart-panel" aria-label="CPU history fixture" onDoubleClick={() => openDetail("history")}>
              <div className="chart-panel-heading">
                <div>
                  <p className="eyebrow">CPU HISTORY</p>
                  <p className="chart-description">Total logical processor usage</p>
                </div>
                <button className="panel-detail-link" type="button" onClick={() => openDetail("history")}>
                  View detail
                </button>
              </div>
              <CpuHistoryChart samples={fixture.samples} />
            </section>
          </div>
          <section className="resource-column" aria-label="Resource metrics">
            <button className="metric-card resource-card" type="button" onClick={() => openDetail("memory")} aria-label="Memory metric details">
              <p className="eyebrow">MEMORY</p>
              <strong>
                {latest?.memoryPercent == null
                  ? "Unavailable"
                  : `${latest.memoryPercent.toFixed(1)}%`}
              </strong>
              <span>Fixture value</span>
            </button>
            <button className="metric-card resource-card unavailable-card" type="button" onClick={() => openDetail("gpu")} aria-label="GPU metric details">
              <p className="eyebrow">GPU</p>
              <strong>Not in fixture</strong>
              <span>No GPU sample supplied</span>
            </button>
            <button className="metric-card resource-card" type="button" onClick={() => openDetail("disk")} aria-label="Disk metric details">
              <p className="eyebrow">DISK READ</p>
              <strong>
                {latest?.diskReadMiBPerSecond == null
                  ? "Unavailable"
                  : `${latest.diskReadMiBPerSecond.toFixed(1)} MiB/s`}
              </strong>
              <span>Fixture value</span>
            </button>
            <button className="metric-card resource-card" type="button" onClick={() => openDetail("network")} aria-label="Network metric details">
              <p className="eyebrow">NETWORK RX</p>
              <strong>
                {latest?.networkRxMiBPerSecond == null
                  ? "Unavailable"
                  : `${latest.networkRxMiBPerSecond.toFixed(2)} MiB/s`}
              </strong>
              <span>Fixture value</span>
            </button>
          </section>
        </div>
        )}
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
