# Windows Distribution and Updates

**Planned — not implemented**

## Packages and release source

The current Rust release workflow will produce Windows x64 archives only. The replacement is planned to ship a Windows x64 NSIS installer through GitHub Releases. Existing releases are historical and remain unchanged. Clean install, launch, uninstall, and upgrade must be verified before the installer is advertised as supported.

## Update behavior

Use one stable channel and a versioned manifest for Windows x64. Check after the UI is ready, then every 24 hours while the app remains open. Provide a manual check and a setting to disable automatic checking. Network failures must not delay startup or stop monitoring.

Show available version, release notes, download size, and progress. Ask before downloading and installing; do not silently terminate the application. Preserve Settings across upgrades and validate the full package before invoking its installer.

## Trust and installation

Sign update artifacts and pin the verification public key in the application. Keep the private key outside the repository in protected CI secrets. Reject missing or invalid signatures, checksum mismatches, and incompatible packages rather than allowing unverified installation.

No Authenticode certificate has been purchased. Windows installation instructions must explain that update signatures do not establish publisher identity or remove OS warnings. Launch the verified installer and request elevation only when needed.

## Recovery and acceptance

Cancellation or download/verification failure leaves the installed version unchanged. Installation failure must report its actual state and provide instructions to reinstall a known-good package. Automatic rollback is not promised. [Stage 5](plan/implementation/05-packaging-updates.md) verifies offline checks, interrupted/corrupt downloads, invalid signatures, denied elevation, and installer failure; [Stage 6](plan/implementation/06-migration-release.md) verifies a trial release and upgrade before stable.
