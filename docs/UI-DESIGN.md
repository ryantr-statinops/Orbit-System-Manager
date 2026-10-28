# Dashboard UI design contract

## Reference

The approved target combines the V1 three-column dashboard structure with the dark visual direction of mockup 1. Keep the same panel order: CPU summary, logical processors, and CPU history on the left; CPU utilization surface in the center; memory, GPU, and network on the right.

Desktop review sizes are 1465x955 and 1280x800 logical pixels. The compact layout must remain usable at 1060x800 and 720x520. Desktop sidebars start at 290px with 5px splitters. Later layout refinement may use a 25% sidebar width clamped to 290–380px, provided the center remains at least 360px wide.

## Visual direction

Use charcoal surfaces, subtle borders, 8px card corners, 8px outer and inter-card spacing, and 12px panel padding. Use the platform sans-serif font, cyan as the main accent, green for network send, and a blue-to-cyan-to-green-to-yellow CPU surface palette. Keep chart labels, units, and secondary text readable at all review sizes.

## Preview behavior

The application opens in deterministic demo mode for this UI iteration. It starts with 61 timestamped samples spanning 60 seconds and advances all metrics once per second. Pause freezes the preview clock and all displayed values while camera interaction remains available. The header clearly labels demo data. Synthetic GPU metrics are allowed only in this preview mode.

The preview supports 0, 1, 4, 16, 32, and 64 logical processors and full, empty, single-sample, and unavailable scenarios. Missing values remain absent; charts do not invent connecting values. CPU summary statistics are calculated from the same processor sample shown in the dashboard.

## Rendering boundary

Slint owns component layout, text, controls, and 2D plots. The existing Rust offscreen renderer owns the projected CPU surface and supplies it as an image. Visual interpolation may smooth the surface but must not add telemetry samples.

## Out of scope

This UI iteration does not add new monitoring collectors, release tags, additional navigation tabs, or a new 3D graphics backend. Existing public telemetry types remain available for a later live-data integration.
