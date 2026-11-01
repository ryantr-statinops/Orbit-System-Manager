# Rebuild Development and Validation

**Planned — not implemented**

## Current application and target environment

The desktop application remains Rust/Slint. Follow the root [README](../README.md) for Cargo commands and the [Rust preview guide](current-rust/DASHBOARD-PREVIEW.md) for screenshots. A Windows-only Go Stage 1A collector CLI is available; it does not replace the desktop app. Wails, React, browser preview, and replacement CI remain unimplemented.

The planned stack is Go, gopsutil v4, Wails v3, React, TypeScript, Vite, ECharts, and Three.js. The collector module targets Go 1.27.0 and pins gopsutil v4.26.10 in `go.mod`/`go.sum`. Record OS versions, CPU/GPU, drivers, and display scaling for each test machine. Verify native WebView and GTK/WebKitGTK dependencies on Ubuntu/Fedora rather than assuming a successful build proves runtime compatibility.

## Windows collector commands

Run these at repository root:

```powershell
go test ./...
go vet ./...
go build -o bin/telemetry-poc.exe ./cmd/telemetry-poc
go run ./cmd/telemetry-poc inventory
go run ./cmd/telemetry-poc sample --interval 1s --count 60
```

The sample command accepts `1s`, `2s`, or `5s`; `--count 0` streams until Ctrl+C. Inventory emits one JSON document and sampling emits JSONL to stdout; diagnostics go to stderr. To retain raw evidence, create `artifacts/collector-poc/` and redirect stdout and stderr there. This directory is ignored by Git. Review samples for `warming_up`, permission, unsupported, and error states; null values are not zero. Avoid stress tests; compare normal workloads. These commands currently validate Windows only; they do not establish Linux support or availability on untested devices.

## Development workflow

The future browser preview must use deterministic fixtures independent of collection and privileged access. Desktop development should connect the same frontend to real Go samples through Wails. Demo data must be clearly labeled and cannot count as evidence of hardware support.

Add documented entry points for preview, desktop development, checks, and release builds when they exist. Keep the Rust workflow usable until migration approval in [stage 6](plan/implementation/06-migration-release.md).

## PoC approvals

[Stage 1](plan/implementation/01-collector-poc.md) records inventory and an OS/device/metric/source/permission matrix. Compare measurements with system tools using equivalent intervals and definitions. The user approves metric coverage before progression.

[Stage 2](plan/implementation/02-ui-performance-poc.md) measures release builds: time to first UI and valid sample, memory across the full process tree, CPU normalized to the whole machine, GPU usage, and interaction frame-time p95. Run idle and interaction workloads for 30 minutes with reproducible fixture and display settings. The user approves the baseline and thresholds before the complete dashboard; no fixed performance limit has been accepted yet.

## Verification and evidence

Use Go tests for rates, counter resets, history bounds, availability, gaps, and shutdown. Use Vitest for frontend behavior and Playwright for fixture-driven UI workflows. Browser automation complements native checks; it does not establish WebView, scaling, or permission behavior on its own.

Native verification covers all three OS environments, resize/HiDPI, sleep/resume, changing interfaces/devices, multiple GPUs, denied permissions, and WebGL failure. Repeat performance measurements when relevant changes or unresolved issues justify them.

Record commands, versions, results, blockers, and evidence links directly in the corresponding stage file. Mark checklist items complete only after verification, without rewriting earlier validation records as current results.
