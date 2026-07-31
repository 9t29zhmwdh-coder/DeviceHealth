# Roadmap: DeviceHealth

## v0.1.0, Initial Release (2026-06-12) ✅

- Process analysis: running processes with CPU and memory usage per process
- Hardware analysis: CPU, RAM, disk, temperatures via sysinfo
- Security analysis: open ports, suspicious processes, basic firewall check
- Service analysis: system services and daemon status, startup items
- HealthScore 0-100 with per-category subscores (performance, security, stability, storage)
- AI explanations and optimization recommendations via Ollama
- `dh-core` Rust crate with modular analyzer architecture
- `dh-cli` binary for headless diagnostics
- Tauri v2 desktop shell (macOS, Windows, Linux)
- React/TypeScript frontend with score dial and process/hardware panels

## v0.2.0, History & Alerts

- Autostart detection on Windows. `get_autostart_entries` branches on macOS and
  Linux only, so on Windows it returns an empty list and the category silently
  reports nothing. Needs the registry Run keys (`HKCU` and `HKLM`) and the
  Startup folder. Surfaced when CI began checking Windows: the risk classifier
  showed up as dead code there, which is what an unimplemented platform branch
  looks like from the compiler's side.

- Persistent scan history (SQLite) for trend tracking
- Configurable alert thresholds (CPU%, RAM%, disk space, temperature)
- Background monitoring mode with system tray integration
- Score history chart (last 7 / 30 days)
- Export diagnostic report (JSON, plain text, PDF)

## v0.3.0, Deep Diagnostics

- Startup impact analysis (boot-time contribution per service/app)
- Disk health via S.M.A.R.T. data (where OS supports it)
- Network activity per process (bandwidth usage)
- AI-generated prioritized action list ("fix these 3 things first")
- Scheduled periodic scans

## v1.0.0: Stable Release

- Stable public API for `dh-core` (semver)
- Full test coverage (unit + integration)
- Packaged installers (`.dmg`, `.msi`, `.AppImage`)
- Localization (EN + DE)
- Comprehensive documentation site

## Out of Scope

- Remote monitoring of other machines (network agents)
- Cloud-based diagnostics or telemetry upload
- Kernel-level drivers or system modifications
- Mobile platforms (iOS, Android)

## Dual-Licensing Readiness

Assessed 2026-07-11: Community-only, not a Dual-Licensing candidate. Endpoint health monitoring is a real enterprise category (NinjaOne, Datto and similar RMM tools), but DeviceHealth's own roadmap explicitly rules out remote monitoring of other machines and any telemetry upload by design, which forecloses the fleet-management angle that would justify an Enterprise tier. It is a single-machine, local-only diagnostics tool with no team dimension. Revisit only if the project's scope intentionally changes to support fleet-wide monitoring.
