# Release and distribution

Current target version: `v2.2.0`.

Orbit System Manager releases are built and published by `.github/workflows/release.yml`. A pushed Git tag matching `v*` starts the four-platform build matrix:

| Platform | Rust target | Archive |
| --- | --- | --- |
| Windows x86_64 | `x86_64-pc-windows-msvc` | `orbit-system-manager-windows-x86_64.zip` |
| Linux x86_64 | `x86_64-unknown-linux-gnu` | `orbit-system-manager-linux-x86_64.tar.gz` |
| macOS Intel | `x86_64-apple-darwin` | `orbit-system-manager-macos-x86_64.zip` |
| macOS Apple Silicon | `aarch64-apple-darwin` | `orbit-system-manager-macos-aarch64.zip` |

Each build uses the committed lockfile. The publish job waits for all four builds, generates `SHA256SUMS.txt`, and attaches the archives and checksum file to the GitHub Release.

## Release procedure

1. Update the package version and lockfile, then run formatting, check, tests, and Clippy.
2. Confirm the cross-platform CI and release build results.
3. Create and push the version tag, for example `v2.2.0`.
4. Confirm the Release page contains all four archives and `SHA256SUMS.txt`.

The repository does not claim binary size, startup time, or memory benchmarks without recorded measurements. The release workflow builds and uploads artifacts; it does not perform platform-specific GUI runtime validation.
