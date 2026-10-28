# Dashboard preview workflow

The app starts in deterministic `DEMO DATA` mode. It shows a 60-second history (61 points at one-second intervals) and adds one sample per second. Use **Pause** to hold the values and charts while reviewing the layout. Restarting the app resets the fixture to its initial state.

## Run the interactive preview

```sh
cargo run --locked
```

The interactive preview supports Pause/Resume and camera rotation, panning, zooming, and reset. This command runs the app with the platform's normal Slint backend.

## Capture a fixed-size screenshot

Build with the optional screenshot feature. It runs the same Slint `AppWindow` on the software renderer, waits for layout and rendering, then writes a PNG. The default size is 1465×955 logical pixels.

```sh
cargo run --locked --features dashboard-screenshot -- \
  --capture-dashboard artifacts/dashboard-preview.png \
  --capture-size 1465x955 \
  --demo-state full \
  --demo-cores 16
```

Open `artifacts/dashboard-preview.png` with an image viewer. The capture contains the app's client area and does not include operating-system window chrome.

Supported capture sizes are positive `WIDTHxHEIGHT` values up to 4096×4096. For example:

```sh
cargo run --locked --features dashboard-screenshot -- --capture-dashboard artifacts/dashboard-720.png --capture-size 720x520 --demo-state full --demo-cores 16
cargo run --locked --features dashboard-screenshot -- --capture-dashboard artifacts/dashboard-1060.png --capture-size 1060x800 --demo-state full --demo-cores 16
```

The app also accepts `1280x800` and `1465x955` for the larger review layouts.

## Choose fixture data

Use `--demo-state` to capture a consistent availability case:

| Value | Preview |
|---|---|
| `full` | CPU, memory, GPU, and network fixture data |
| `empty` | No collected history yet |
| `single` | One sample for single-point chart states |
| `unavailable` | Metrics shown as unavailable |

Use `--demo-cores` with `0`, `1`, `4`, `16`, `32`, or `64` to check processor layout and terrain density. The default is `16`. For example, capture the 64-processor view with:

```sh
cargo run --locked --features dashboard-screenshot -- --capture-dashboard artifacts/dashboard-64-cores.png --capture-size 1465x955 --demo-state full --demo-cores 64
```

The preview fixture is deterministic: a given state and processor count produce the same values and history on each run. Surface interpolation only affects its presentation.

## Get the CI screenshot

The `Capture dashboard preview` job in `.github/workflows/ci.yml` runs on pushes to `main` and pull requests targeting `main`. It captures the `full` fixture at 1465×955 with 16 logical processors and uploads `dashboard-preview-<commit-sha>` as a GitHub Actions artifact for 14 days.

Open the repository's **Actions** page, select the CI run for the commit, open **Capture dashboard preview**, and download the artifact. A missing PNG fails the job.

## Record review results

Record the commit SHA, capture command, fixture state, processor count, size, and artifact location when reviewing a preview. Keep native runtime checks separate from the headless screenshot because the capture does not test OS scaling or real input devices.

| Commit | Capture size and fixture | CI build and screenshot artifact | Native runtime and scaling checks |
|---|---|---|---|
|  |  |  |  |

For native review, run the app at 1465×955, 1280×800, 1060×800, and 720×520. On Windows, check 100%, 150%, and 200% display scaling; resize repeatedly; drag, collapse, and restore both sidebars; scroll to the last processor; pause and resume; and rotate, pan, zoom, and reset the terrain. Record which operating systems and scaling factors were actually checked.

The screenshot feature still needs the Rust platform toolchain to compile. On Windows this includes the MSVC linker `link.exe`; the headless backend does not remove that build requirement. If it is unavailable locally, use the CI artifact and mark local native checks as blocked until a suitable toolchain is available.

### Implementation validation record

Recorded on 2026-10-29 for the local implementation checkout:

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | Passed |
| `cargo metadata --locked --offline --format-version 1 --no-deps` | Passed |
| `cargo check --locked` | Blocked before project compilation: `link.exe` is missing from the installed Windows MSVC target environment. |
| `cargo check --locked --features dashboard-screenshot` | Same `link.exe` blocker; screenshot feature has not been compiled locally. |
| CI screenshot artifact | Pending a CI run for a pushed commit or pull request. |
| Native Windows scaling and interaction review | Not run in this environment. |
