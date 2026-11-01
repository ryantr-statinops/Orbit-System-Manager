# Web Dashboard Design

**Planned — not implemented**

## Layout and visual direction

Retain the dark, three-column dashboard: CPU summary, logical processors, and history on the left; CPU surface in the center; memory, GPU, and network on the right. Add a collapsible disk card. Keep explicit units, readable labels, and a clear distinction between measured values and availability messages.

Use charcoal surfaces, subtle borders, cyan accents, green network-send indicators, and the existing CPU surface color direction. The [current Rust design](current-rust/UI-DESIGN.md) provides visual reference, not the rendering architecture for the replacement. Review layouts at 720×520, 1060×800, 1280×800, and 1465×955 logical pixels, including native scaling and resizing.

## Rendering and interaction

React and TypeScript own layout, controls, and Settings. ECharts supplies 2D charts through selective module imports. Three.js directly owns the surface, camera, and render loop. Keep per-frame camera and geometry updates outside React state and independent of collector intervals.

The surface displays per-core CPU utilization over the latest 60 seconds. Reuse mesh buffers, bound pixel ratio, and reduce rendering while hidden. Interpolation smooths presentation without adding observations. Rotation, panning, zoom, reset, core selection, and tooltips must remain responsive while samples arrive.

If WebGL initialization or context recovery fails, show an explanation and a 2D chart alternative. Release scene resources when the view is destroyed.

## Live data, fixtures, and pause

Desktop mode defaults to live measurements. On startup, show only collected samples; never prefill real history with synthetic values. Missing, stale, unsupported, permission-denied, warming-up, and failed readings need distinguishable messages. Do not turn an unavailable value into zero or connect a chart across a data gap.

Browser preview uses clearly labeled deterministic fixtures, including full, empty, single, and unavailable states with 0, 1, 4, 16, 32, and 64 logical processors. Fixtures may start with history so layouts can be reviewed immediately.

Pause freezes the displayed snapshot while collection continues in its bounded history. Camera controls remain active. Resume returns to current data rather than replaying missed frames.

## Settings and acceptance

Settings select the sampling interval, GPU, disk, and network interface, plus the optional helper when justified by the collector PoC. Show devices individually; do not sum GPU utilization across adapters.

[Stage 2](plan/implementation/02-ui-performance-poc.md) establishes the approved performance baseline. [Stage 3](plan/implementation/03-live-dashboard.md) implements this dashboard against real data. Layout acceptance includes availability states, units, resizing, and interaction at the approved performance thresholds.
