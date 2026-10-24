# Orbit System Manager

A native desktop dashboard for live CPU, memory, and network telemetry, built with Rust and Slint.

## Dashboard

- CPU utilization history displayed as a smoothed, shaded 3D surface. The surface interpolates only for presentation; reported samples remain the collected measurements.
- A responsive view of every logical processor.
- Memory usage and aggregate network receive/send rates with explicit units.
- GPU telemetry marked unavailable until GPU collection is implemented.
- Rotate with left drag, pan with right drag, zoom with the scroll wheel, and reset the camera from the chart controls.

CPU history retains up to 30 samples. System telemetry is collected once per second. Renderer frame rate is limited by the application redraw timer.

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

## Release artifacts

Pushing a version tag such as `v2.1.2` starts `.github/workflows/release.yml`. The workflow builds Windows x86_64, Linux x86_64, macOS Intel, and macOS Apple Silicon archives, then attaches them and `SHA256SUMS.txt` to the GitHub Release.
