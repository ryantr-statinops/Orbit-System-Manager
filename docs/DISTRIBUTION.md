# Rebuild Distribution and Updates

**Planned — not implemented**

## Packages and release source

The replacement will ship a Windows x64 NSIS installer, an Ubuntu x64 DEB, and a Fedora x64 RPM through the repository's GitHub Releases. Build and verify packages on their respective OS environments, declaring architecture and runtime dependencies. Exact supported versions and WebView/GTK/WebKitGTK combinations are established by the PoCs.

The current release workflow still builds Rust archives, including macOS. It does not implement these installers or the updater. Keep it operational until the migration is approved. The new release process must cover clean install, launch, uninstall, and upgrade before an installer is advertised as supported.

## Update behavior

Use one stable channel and a versioned manifest that identifies the appropriate OS, architecture, and package format. Check after the UI is ready, then every 24 hours while the app remains open. Provide a manual check and a setting to disable automatic checking. Network failures must not delay startup or stop monitoring.

Show the available version, release notes, download size, and progress. Ask before downloading and installing; do not silently terminate the application. Preserve Settings across an upgrade. Validate the complete package before handing it to an installer.

## Trust and installation

Sign update artifacts and pin the verification public key in the application. Keep the private key outside the repository in protected CI secrets. Reject missing or invalid signatures, checksum mismatches, and incompatible packages rather than falling back to unverified installation.

The first release has no purchased Windows Authenticode certificate. Explain this limitation in the Windows installation instructions; update signatures do not establish a Windows publisher identity or remove OS warnings.

On Windows, launch the verified installer and request installation privileges only when necessary. On Linux, install the verified DEB/RPM through the OS authentication and package-manager flow. Never directly replace an executable managed by a package manager. Read-only collector helpers are separate from update installation and must not become a general elevated command runner.

## Recovery and acceptance

Cancellation or download/verification failure leaves the installed version unchanged. Installation failure must report its actual state and provide instructions to reinstall a known-good package. Automatic rollback across platforms is not promised.

[Stage 5](plan/implementation/05-packaging-updates.md) verifies offline checks, interrupted downloads, corrupted or wrongly signed packages, wrong platforms, denied elevation, and installer failure. [Stage 6](plan/implementation/06-migration-release.md) requires a trial release and a tested upgrade from the preceding version before publishing stable. Record evidence and known issues in those stage files.
