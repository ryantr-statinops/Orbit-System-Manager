# Rebuild Architecture

**Planned — not implemented**

## Scope and boundaries

The rebuild targets Windows x64 only for the current product scope. Go owns collection, normalization, history, configuration, and application lifecycle. gopsutil v4 supplies CPU utilization, memory/swap, disk capacity and I/O, and network counters. Wails v3 connects Go to React/TypeScript. There is no remote monitoring service or public telemetry endpoint in the first release.

## Collection and hardware adapters

Collectors run outside the UI thread, with bounded waits and isolated errors. Sampling defaults to one second, selectable as one, two, or five seconds. Sensors are sampled no more frequently than every two seconds. Static inventory is refreshed at startup or device changes.

GPU and sensor metrics are exposed per device only when verified on Windows. NVIDIA uses the Windows NVML DLL adapter; a detected device does not imply every metric is available. ACPI readings must not be labeled as CPU package temperature. Linux DRM/sysfs/hwmon and other operating systems are outside the current support scope.

## Data and frontend boundary

Go keeps a bounded, in-memory 60-second history, emits new samples through Wails, and provides a snapshot when the interface reconnects. Each metric identifies timestamp, unit, source, nullable value, and availability/failure status. Missing readings remain missing; units use bytes and bytes per second internally.

Rates use elapsed monotonic time. Counter resets, interface changes, and sleep/resume restart the baseline instead of producing spikes. Visual interpolation never creates telemetry samples. Settings are versioned JSON in the user's configuration directory; metric history is not persisted.

## Permissions and lifecycle

The app and any helper use Go, with no C# helper or custom kernel driver. The app runs without elevation. Add an optional helper only if the Windows PoC proves a useful permission-dependent metric. Use authenticated local named-pipe communication with an allowlist of read operations.

Closing the window stops collectors and any helper. Logs are bounded and identify failing collectors without exporting telemetry. Only Windows/device/metric combinations with evidence may be advertised as supported.

## Validation gates

[Stage 1](plan/implementation/01-collector-poc.md) requires approval of measured Windows metric coverage. [Stage 4](plan/implementation/04-platform-hardening.md) covers permissions, device changes, and shutdown on Windows.
