# Repository Guidelines

## Project Structure

The current application is a Rust desktop app built with Slint; Windows x64 is the sole supported target in the current project scope. Rust entry points and application logic live in `src/`: `telemetry/` collects metrics, `preview/` provides deterministic fixtures, and `renderer/` handles the 3D chart. The Slint interface is in `ui/`, with reusable components under `ui/components/`. Build-time Slint compilation is configured in `build.rs`.

## Migration Plans and Historical Documents

Start at the [documentation index](docs/README.md). The Go/Wails/React rebuild remains planned; the running application stays Rust/Slint until migration is approved. The [implementation stages](docs/plan/implementation/README.md) govern Windows-only checklists, progress, and evidence. Obtain user approval for Windows metric coverage after Stage 1 and performance thresholds after Stage 2, before Stage 3.

Use the [architecture](docs/ARCHITECTURE.md), [UI design](docs/UI-DESIGN.md), [development](docs/DEVELOPMENT.md), and [distribution](docs/DISTRIBUTION.md) guides for target requirements. Current Rust preview instructions live in `docs/current-rust/`. Keep historical validation records intact. The legacy archive is reference only; its Linux/macOS material is not current guidance. Repository skills belong in `.agents/skills/<skill-name>/SKILL.md`.

## Build, Test, and Development

Use Rust and Cargo 1.99.1 or later, plus Visual Studio Build Tools with the MSVC C++ workload.

- `cargo run --locked` launches the app with deterministic demo telemetry.
- `cargo fmt --all -- --check` checks Rust formatting.
- `cargo check --locked` checks compilation.
- `cargo test --locked` runs unit tests.
- `cargo clippy --locked --all-targets -- -D warnings` checks all targets.
- `cargo build --locked --release` creates an optimized build in `target/release/`.

For Windows UI review, follow `docs/current-rust/DASHBOARD-PREVIEW.md`. CI checks Rust and captures the dashboard on Windows only; Go/frontend checks remain pending Stage 2.

## Style and Testing

Format Rust with `rustfmt` (four-space indentation) and keep Clippy warning-free. Use `snake_case` for Rust identifiers and descriptive Slint component names. Tests use Rust's built-in framework in colocated `#[cfg(test)]` modules; name them for the behavior they verify. For dashboard changes, inspect representative fixture states and screenshots.

## Commits and Pull Requests

Use short Conventional Commit prefixes (`feat:`, `fix:`, `docs:`) and a concise imperative summary. A pull request should explain user-visible changes, list validation commands, link an issue when relevant, and include screenshots or a dashboard-preview artifact for visual changes. Do not claim Go/frontend validation until those checks are added and run.
