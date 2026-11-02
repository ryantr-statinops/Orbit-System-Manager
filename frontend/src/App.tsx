function App() {
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
          <span className="status-dot" />
          Stage 2 UI preview
        </div>
      </header>

      <section className="welcome-panel" aria-labelledby="welcome-title">
        <p className="eyebrow">DETERMINISTIC PREVIEW</p>
        <h2 id="welcome-title">Dashboard surface is ready for review.</h2>
        <p className="welcome-copy">
          Chart fixtures and interaction controls will be added in the next
          Stage 2 tasks. This preview does not read or retain machine metrics.
        </p>
      </section>

      <footer className="page-footer">
        <span>Orbit System Manager</span>
        <span>PoC · no live telemetry</span>
      </footer>
    </main>
  );
}

export default App;
