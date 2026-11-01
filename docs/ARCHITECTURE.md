# Rebuild Architecture

**Planned — not implemented**

## Scope and boundaries

The rebuild targets a personal desktop monitor on Windows and Ubuntu/Fedora x64. OS versions, drivers, and supported hardware must be recorded during the collector PoC. The executable currently in this repository remains Rust/Slint; these requirements describe the replacement.

Go owns collection, normalization, history, configuration, and application lifecycle. gopsutil v4 supplies CPU utilization, memory/swap, disk capacity and I/O, and network counters. Wails v3 connects this backend to a React/TypeScript interface. There is no remote monitoring service or public telemetry endpoint in the first release.

## Collection and hardware adapters

Collectors run outside the UI thread, with bounded waits and isolated errors. Sampling defaults to one second, selectable as one, two, or five seconds. Sensors are sampled no more frequently than every two seconds. Static inventory is refreshed at startup or device changes.

GPU and sensor adapters expose capabilities per device. Linux NVIDIA uses NVML bindings; Windows needs a separate DLL adapter because the official Go binding targets Linux. AMD/Intel coverage depends on available OS/driver counters, including Linux DRM/sysfs/hwmon. A discovered device does not imply every metric is available. ACPI thermal-zone readings must not be labeled as CPU package temperature.

## Data and frontend boundary

Go keeps a bounded, in-memory 60-second history, emits new samples through Wails, and provides a snapshot when the interface connects or resynchronizes. Each metric identifies its timestamp, unit, source, nullable value, and availability or failure status. Missing readings remain missing; units use bytes and bytes per second internally.

Rates use elapsed monotonic time. Counter resets, interface changes, and sleep/resume restart their baseline instead of producing spikes. Visual interpolation never creates telemetry samples. Settings are versioned JSON in the user's configuration directory; metric history is not persisted.

## Permissions and lifecycle

Backend and helper application code use Go, with no C# helper or custom kernel driver. The app runs without elevation. Introduce an optional elevated helper only when the PoC demonstrates a useful permission-dependent metric. Use authenticated local named-pipe or Unix-socket communication with an allowlist of read operations.

Closing the window stops collectors and any helper. Logs are bounded and identify failing collectors without exporting telemetry.

## Validation gates

[Stage 1](plan/implementation/01-collector-poc.md) requires user approval of measured metric coverage. [Stage 4](plan/implementation/04-platform-hardening.md) covers permissions, device changes, and shutdown. Only tested OS/device/metric combinations may be advertised as supported.
