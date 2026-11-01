# Documentation

## Project status

The current application is a Rust/Slint desktop dashboard with deterministic demo telemetry. The Go/Wails/React replacement is **planned, not implemented**. Its documentation describes requirements, not working commands, measured hardware coverage, or completed features.

Use the root [README](../README.md) to run the current application and [AGENTS.md](../AGENTS.md) for contributor guidance.

## Rebuild reference guides

Read these guides alongside the [implementation stages](plan/implementation/README.md):

| Guide | Purpose | Related stages |
|---|---|---|
| [Architecture](ARCHITECTURE.md) | Collector, adapters, data, bridge, permissions, lifecycle | 1, 4 |
| [Web dashboard](UI-DESIGN.md) | Layout, charts, surface, live/fixture behavior, Settings | 2, 3 |
| [Development and validation](DEVELOPMENT.md) | Environments, preview, tests, baseline and approvals | 1, 2, 4, 6 |
| [Distribution and updates](DISTRIBUTION.md) | Packages, signatures, update consent, installation and recovery | 5, 6 |

The stage files remain the source of implementation checklists, progress, results, and blockers. Stage 1 requires user approval of metric coverage; stage 2 requires approval of performance thresholds before stage 3. Neither approval is implied by publishing these guides. Preserve the Rust application until the migration is accepted.

## Current Rust/Slint application

- [UI design contract](current-rust/UI-DESIGN.md): current layout, visual direction, and native rendering boundary.
- [Dashboard preview workflow](current-rust/DASHBOARD-PREVIEW.md): Cargo commands, deterministic fixtures, screenshots, and dated validation records.

These documents describe the existing application. In particular, its demo startup and pause behavior differ from the planned live dashboard. Historical check results are not evidence that checks passed on the present checkout.

## Historical material

The [legacy document index](archive/legacy-agent/README.md) contains earlier Python, Tauri, and Rust/Slint proposals and checklists. Consult it for context, not current implementation requirements. Existing stage checklists and archived material retain their original language; the new reference guides are written in English.
