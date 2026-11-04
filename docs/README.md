# Documentation

## Project status

Windows x64 is the sole supported target in the current project scope. The running application remains Rust/Slint; the Go/Wails/React replacement is **planned, not implemented**. Requirements in rebuild guides do not prove that features or commands exist.

Use the root [README](../README.md) for the current application and [AGENTS.md](../AGENTS.md) for contributor guidance. The [implementation stages](plan/implementation/README.md) are the source of checklist status and evidence. Stage 1 still needs approval of Windows metric coverage; Stage 2 still needs Windows validation and baseline approval before Stage 3.

## Windows rebuild guides

| Guide | Purpose | Related stages |
|---|---|---|
| [Architecture](ARCHITECTURE.md) | Windows collector, adapters, data flow, permissions, lifecycle | 1, 4 |
| [Dashboard design](UI-DESIGN.md) | Layout, charts, interactions, fixtures and live behavior | 2, 3 |
| [Development and validation](DEVELOPMENT.md) | Windows workflow, preview, tests, baseline and approval gates | 1, 2, 4, 6 |
| [Distribution and updates](DISTRIBUTION.md) | Windows packaging, signatures, consent and recovery | 5, 6 |

## Current Rust/Slint application

- [UI design contract](current-rust/UI-DESIGN.md): current layout and rendering direction.
- [Dashboard preview workflow](current-rust/DASHBOARD-PREVIEW.md): Windows Cargo commands, fixtures, screenshots and existing validation records.

Historical records remain intact and do not establish support for untested devices or this checkout.

## Historical material

The [legacy document index](archive/legacy-agent/README.md) contains earlier proposals and checklists. It is historical context, not current platform guidance. Archived Linux/macOS references and prior release artifacts remain unchanged.
