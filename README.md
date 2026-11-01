# Orbit System Manager

[![Release](https://img.shields.io/badge/Release-v2.2.0-blue?style=for-the-badge&logo=rust)](https://github.com/ryantr-statinops/Orbit-System-Manager/releases)

A native desktop dashboard built with Rust and Slint. It starts with deterministic demo telemetry so the full dashboard can be reviewed without depending on the host's hardware sensors.

## Rebuild status

The Go + gopsutil + Wails rebuild, with React/TypeScript, ECharts, and Three.js, is **planned — not implemented**. It targets Windows and Ubuntu/Fedora x64, live metrics, and a 60-second in-memory history. GPU and sensor support will depend on measured hardware capabilities.

The [implementation stages](docs/plan/implementation/README.md) track progress and evidence. Metric coverage requires approval after the collector PoC; performance thresholds require approval after the UI PoC. The commands and dashboard below describe the current Rust/Slint application, which remains in place until migration is approved.

## Dashboard

- CPU utilization history displayed as a smoothed, shaded 3D surface. Surface interpolation affects presentation only; stored preview measurements stay unchanged.
- A responsive view of every logical processor.
- Memory usage and aggregate network receive/send rates with explicit units.
- CPU, memory, GPU, and network use deterministic demo data by default; the `unavailable` scenario previews N/A states.
- Rotate with left drag, pan with right drag, zoom with the scroll wheel, and reset the camera from the chart controls.

The default preview starts with 60 seconds of history (61 one-second samples), then adds one sample per second. Pause holds the displayed values and histories. The 3D surface interpolation only affects presentation; the stored samples remain unchanged.

## Requirements

- Rust and Cargo 1.99.1 or later.
- Platform build dependencies required by Slint. On Windows, install Visual Studio Build Tools with the MSVC C++ workload.

## Development

Run the application:

```sh
cargo run --locked
```

Check and test:

```sh
cargo fmt --all -- --check
cargo check --locked
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
```

Build an optimized binary:

```sh
cargo build --locked --release
```

The binary is written to `target/release/`.

### Preview and screenshot

Run the interactive demo with `cargo run --locked`. To capture a deterministic PNG at a chosen size and scenario, build with the optional screenshot feature. See [the dashboard preview workflow](docs/current-rust/DASHBOARD-PREVIEW.md) for commands, supported fixture values, CI artifacts, and review steps.

## Documentation

- [Documentation index](docs/README.md): current application, rebuild guides, and historical references.
- [Rebuild architecture](docs/ARCHITECTURE.md): collector, adapters, data flow, and permissions.
- [Planned web dashboard](docs/UI-DESIGN.md): layout, interactions, and live/fixture behavior.
- [Development and validation](docs/DEVELOPMENT.md): target workflow, baseline measurements, and approval gates.
- [Distribution and updates](docs/DISTRIBUTION.md): planned installers and signed updates; these are not provided by the current release workflow.

## Current Rust release artifacts

Pushing a version tag such as `v2.2.0` starts `.github/workflows/release.yml`. The workflow builds Windows x86_64, Linux x86_64, macOS Intel, and macOS Apple Silicon archives, then attaches them and `SHA256SUMS.txt` to the GitHub Release.
