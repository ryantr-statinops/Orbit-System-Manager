# Windows Development and Validation

**Planned — not implemented**

## Current application and target environment

The running desktop application remains Rust/Slint. Follow the root [README](../README.md) for Cargo commands and the [Rust preview guide](current-rust/DASHBOARD-PREVIEW.md) for Windows screenshots. A Windows-only Go Stage 1A collector CLI is available; it does not replace the desktop app. Wails, React, browser preview, and replacement CI checks remain incomplete.

The planned stack is Go, gopsutil v4, Wails v3, React, TypeScript, Vite, ECharts, and Three.js. The collector pins Go 1.27.0 and gopsutil v4.26.10. Record Windows build, CPU/GPU, drivers, WebView2 version, and display scaling for each test machine.

## Windows collector commands

Run at repository root:

```powershell
go test ./...
go vet ./...
go build -o bin/telemetry-poc.exe ./cmd/telemetry-poc
go run ./cmd/telemetry-poc inventory
go run ./cmd/telemetry-poc sample --interval 1s --count 60
```

Sampling accepts `1s`, `2s`, or `5s`; `--count 0` streams until Ctrl+C. Inventory emits JSON and sampling emits JSONL to stdout; diagnostics go to stderr. Keep raw evidence under the ignored `artifacts/collector-poc/`. Null values are not zero. Avoid stress tests; compare normal workloads. These commands validate only the tested Windows machine and devices.

## Development workflow and gates

The browser preview uses deterministic fixtures independent of collection and privileged access. Desktop development connects the same frontend to Go samples through Wails and WebView2. Document preview, desktop development, checks, and release commands as they become available; keep Rust commands usable until [Stage 6](plan/implementation/06-migration-release.md).

[Stage 1](plan/implementation/01-collector-poc.md) records a Windows/device/metric/source/permission matrix and requires approval of measured metric coverage. [Stage 2](plan/implementation/02-ui-performance-poc.md) measures time to UI/sample, process-tree memory, whole-machine CPU, GPU use, and interaction frame-time p95 on Windows. Run idle and interaction workloads for 30 minutes with fixed fixtures and display settings; the user approves thresholds before Stage 3.

Use Go tests for rates, resets, history bounds, availability, gaps, and shutdown. Use Vitest and Playwright for fixture-driven frontend behavior. Native verification covers WebView2, resize/HiDPI, sleep/resume, device changes, permissions, multiple GPUs, and WebGL failure. Record commands, versions, results, blockers, and evidence in the stage file; historical validation is not current-checkout evidence.
