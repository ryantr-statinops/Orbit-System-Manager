# Rebuild Development and Validation

**Planned — not implemented**

## Current application and target environment

The repository still builds Rust/Slint. Follow the root [README](../README.md) for existing Cargo commands and the [Rust preview guide](current-rust/DASHBOARD-PREVIEW.md) for screenshots. The Go application, frontend scripts, and replacement CI are not available yet; this document defines their required development workflow without inventing runnable commands.

The target stack is Go, gopsutil v4, Wails v3, React, TypeScript, Vite, ECharts, and Three.js. During the PoCs, pin compatible toolchain, framework, frontend runtime, and dependency versions. Record Windows and Ubuntu/Fedora x64 versions, CPU/GPU, drivers, and display scaling. Verify native WebView and GTK/WebKitGTK dependencies on each Linux distribution rather than assuming a successful build proves runtime compatibility.

## Development workflow

Provide a browser preview backed by deterministic fixtures, independent of collection or privileged access. Provide desktop development with the same frontend connected to real Go samples through Wails. Demo data must be clearly labeled and cannot count as evidence of hardware support.

Introduce documented entry points for preview, desktop development, checks, tests, and release builds when those commands actually exist. Until then, keep the Rust workflow usable. Do not remove the old implementation before the migration approval in [stage 6](plan/implementation/06-migration-release.md).

## PoC approvals

[Stage 1](plan/implementation/01-collector-poc.md) records inventory and an OS/device/metric/source/permission matrix. Compare measurements with system tools using equivalent intervals and definitions. The user approves metric coverage before progression.

[Stage 2](plan/implementation/02-ui-performance-poc.md) measures release builds: time to first UI and valid sample, memory across the full process tree, CPU normalized to the whole machine, GPU usage, and interaction frame-time p95. Run idle and interaction workloads for 30 minutes with reproducible fixture and display settings. The user approves the baseline and thresholds before the complete dashboard; no fixed performance limit has been accepted yet.

## Verification and evidence

Use Go tests for rates, counter resets, history bounds, availability, gaps, and shutdown. Use Vitest for frontend behavior and Playwright for fixture-driven UI workflows. Browser automation complements native checks; it does not establish WebView, scaling, or permission behavior on its own.

Native verification covers all three OS environments, resize/HiDPI, sleep/resume, changing interfaces/devices, multiple GPUs, denied permissions, and WebGL failure. Repeat performance measurements when relevant changes or unresolved issues justify them.

Record commands, versions, results, blockers, and evidence links directly in the corresponding stage file. Mark checklist items complete only after verification, without rewriting earlier validation records as current results.
