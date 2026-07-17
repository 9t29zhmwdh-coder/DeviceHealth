# Changelog, DeviceHealth

All notable changes to this project will be documented in this file.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

---

## [1.0.0] - 2026-07-17

First stable release: a real, packaged, installable distribution exists
for end users. Real macOS/Windows/Linux installers (DMG, NSIS, AppImage/deb/rpm).

## [0.2.8] - 2026-07-17

### Changed
- CI: added an explicit `permissions: contents: read` block to the workflow(s) that were missing one (CodeQL `actions/missing-workflow-permissions`), narrowing the default GITHUB_TOKEN scope.

## [0.2.7] - 2026-07-17
### Changed
- SECURITY.md: documented the accepted glib/gtk transitive RUSTSEC vulnerability (medium, no fix available without a Tauri major-version bump), matching the same entry already present in LifeSort/MailPilot.

## [0.2.6] - 2026-07-12

### Added

- Release workflow (`.github/workflows/release.yml`): builds and attaches macOS (DMG), Windows (NSIS installer), and Linux (AppImage) bundles to a GitHub Release on every tag push. Previously, no release ever had an installer attached.
- README/README.de.md: Download section linking to the latest release's installers.

### Security

- Bumped `vite` (v5 to v8) and `@vitejs/plugin-react` (v4 to v6) together to resolve a Dependabot-flagged advisory (esbuild dev-server request/response exposure). Dev-server only, does not affect the built application.

### Fixed

- All GitHub Actions in `ci.yml` pinned to a commit SHA, matching the portfolio's Action Pinning standard.

## [0.2.5] - 2026-07-11

### Fixed

- Removed an eszett and em-dashes across the repo (TEMPLATE_NOTES.md, ARCHITECTURE.md, ROADMAP.md, CONTRIBUTING.md, SKELETON.md, and two Rust source files). Swiss German orthography.

## [0.2.4] - 2026-07-11

### Fixed

- SemVer correction: v0.1.1 added a genuine new feature (full English/German UI translation, the app was previously German-only) but was versioned as a patch. Renumbered v0.1.1 through v0.1.4 to v0.2.0 through v0.2.3 (same commits, tags and releases recreated at identical SHAs), per the portfolio's SemVer discipline (patch = fix, minor = feature, major = finished product).

## [0.2.3] - 2026-07-11

### Added

- Documented Dual-Licensing assessment (Community-only) in ROADMAP.md.

### Fixed

- Removed em-dashes from ROADMAP.md and SECURITY.md headings.

## [0.2.2] - 2026-07-11

### Fixed

- Updated actions/setup-node to its latest major version in CI, since GitHub is deprecating the Node.js 20 runtime and the previous version was being forced onto Node 24 and crashing during post-run cleanup.

## [0.2.1] - 2026-07-10

### Fixed

- Removed a duplicate "New here? -> beginners guide" callout from README.md (was shown twice)

### Added

- Added the "New here?" beginner guide callout to README.de.md (was missing)

## [0.2.0] - 2026-07-08

### Fixed

- Missing `src-tauri/capabilities/` permissions were silently blocking the event system
- Missing `Emitter` trait import broke `app.emit()` calls at compile time
- Icons referenced in `tauri.conf.json` did not exist in the repo, breaking any CI build of the app crate
- Removed unused `tauri-plugin-shell` dependency and `protocol-asset` feature
- CI previously excluded the Tauri app crate from checks, hiding all of the above

### Added

- Full English/German UI translation (app was previously German-only)
- README onboarding sections: how it runs, screenshot, in practice, uninstall/cleanup
- Committed `Cargo.lock` and `frontend/package-lock.json` for reproducible builds

## [0.1.0] - 2026-06-12

### Added

- Process analysis: running processes with per-process CPU and memory usage
- Hardware analysis: CPU, RAM, disk, and temperature metrics via sysinfo
- Security analysis: open port enumeration, suspicious process detection, basic firewall check
- Service analysis: system service and daemon status, startup item inventory
- HealthScore 0 to 100 with weighted per-category subscores (performance, security, stability, storage)
- AI-generated explanations and ranked optimization recommendations via Ollama (localhost:11434)
- `dh-core` Rust crate: `process/`, `hardware/`, `security/`, `service/`, `score/`, `ai/`
- `dh-cli` binary for headless and scripted diagnostic operation
- Tauri v2 desktop shell for macOS, Windows, and Linux
- React/TypeScript frontend with score dial, process table, and hardware panels
