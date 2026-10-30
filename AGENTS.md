# Repository Guidelines

## Project Structure

This is a Rust desktop application built with Slint. Rust entry points and application logic live in `src/`: `telemetry/` collects and models metrics, `preview/` provides deterministic dashboard fixtures, and `renderer/` handles the 3D chart. The Slint interface is in `ui/`, with reusable components under `ui/components/` and icons under `ui/assets/`. Image and app icon assets are in `assets/`; design and preview workflows are documented in `docs/`. Build-time Slint compilation is configured in `build.rs`.

## Migration Plans and Historical Documents

The Go/Wails/React rebuild is planned in [docs/plan/implementation/README.md](docs/plan/implementation/README.md); the current application remains Rust/Slint until migration is approved. Follow its stage gates: obtain approval for metric coverage after stage 1 and performance thresholds after stage 2. Historical documents from the former `agent/` directory are indexed in [docs/archive/legacy-agent/README.md](docs/archive/legacy-agent/README.md); they are reference material, not current implementation instructions. Repository skills, when added, belong in `.agents/skills/<skill-name>/SKILL.md`.

## Build, Test, and Development

Use Rust and Cargo 1.99.1 or later, plus the platform dependencies required by Slint (Windows requires the MSVC C++ workload).

- `cargo run --locked` launches the app with deterministic demo telemetry.
- `cargo fmt --all -- --check` checks Rust formatting.
- `cargo check --locked` checks compilation.
- `cargo test --locked` runs the unit tests.
- `cargo clippy --locked --all-targets -- -D warnings` checks all targets for lint issues.
- `cargo build --locked --release` creates an optimized build in `target/release/`.

For UI review, follow `docs/DASHBOARD-PREVIEW.md` to capture screenshots with `--features dashboard-screenshot` and fixed fixture values.

## Style and Naming

Format Rust with `rustfmt` (the default four-space indentation) and keep Clippy warning-free. Use `snake_case` for Rust modules, files, functions, and variables; use descriptive component names in Slint and keep shared UI elements in `ui/components/`. Prefer small, focused changes that preserve deterministic preview data and explicit metric units.

## Testing

Tests use Rust's built-in test framework and are colocated with implementation in `#[cfg(test)]` modules. Name tests for the behavior they verify, such as `parses_capture_size` or `keeps_history_bounded`. Run the full CI checks listed above before submitting changes. For dashboard changes, also capture and inspect representative layouts and fixture states; see the preview guide for supported sizes and states.

## Commits and Pull Requests

Recent commits use short Conventional Commit prefixes, for example `fix: ...`; use `feat:`, `fix:`, or `docs:` as appropriate, followed by a concise imperative summary. A pull request should explain the user-visible change, list relevant validation commands, and link a related issue when one exists. Include before/after screenshots or a CI dashboard-preview artifact for visual changes. CI checks formatting, compilation, tests, Clippy, and a deterministic dashboard capture across the configured platforms.
