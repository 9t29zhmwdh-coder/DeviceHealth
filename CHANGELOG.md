# Changelog, DeviceHealth

All notable changes to this project will be documented in this file.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

---

## [1.3.0] - 2026-08-05

### Changed

- `recharts` 2 to 3. This frontend uses three chart types from it: a pie chart and a radial bar chart on the dashboard, a line chart in the history view. All three were rendered in a real DOM under both versions with the same data and produced the same five paths, seven text elements and three SVG roots. Rendered in a DOM rather than server-side on purpose: version 3 builds the chart on the client and produces nothing server-side, which looks like a regression and is not.
- TypeScript 5.9.3 to 7. No source or configuration change was needed. The bundle dropped from 654 kB to 631 kB.
- `thiserror` 2.0.18 to 2.0.19. A patch release of the crate, but it moves `thiserror-impl` from `syn` 2.0 to `syn` 3.0, a major bump of a build-time proc-macro dependency arriving transitively.

---

## [1.2.2] - 2026-08-05

### Changed

- The `glib` advisory GHSA-wrw7-89jp-8q8g is now recorded in `SECURITY.md` and its unreachable versions ignored in `dependabot.yml`, matching the six sibling repositories. It had been dismissed as tolerable risk here, which left it invisible while Dependabot kept attempting an update that cannot succeed: `tauri` 2.11.5 requires `gtk ^0.18`, `gtk` 0.18.2 requires `glib ^0.18`, and cargo rejects 0.20.0 outright. The dismissal has been withdrawn so the entry and the Security tab agree.

---

## [1.2.1] - 2026-08-05

### Added

- A smoke test in CI: the application is built, started, and checked to still be running five seconds later. Until now the pipeline only ever established that the code compiles. A program that builds cleanly and dies on launch would have passed every check and been discovered by whoever downloaded it.
- It runs on Linux and macOS. The Linux job needs `xvfb`, since a GTK window closes immediately without an X server, and that would produce a failure the runner invents rather than one the code has.
- The test also fails on a panic in the output even when the process survives, because a background task that dies quietly leaves the window open and useless.

---

## [1.2.0] - 2026-08-03

### Changed

- `thiserror` 1 to 2. The error strings the frontend shows are unchanged, held by new tests that pass under both versions.
- `sysinfo` 0.33 to 0.39. The API surface used here compiles without edits, but that proves little for this crate, where the risk is a changed unit rather than a changed signature. Measured on the same machine before and after: 770 processes both times, 9662 MB against 9621 MB summed, the same health score and the same two findings.
- `sqlx` 0.8 to 0.9. All nine query sites use static SQL literals, so none runs into the guard 0.9 introduces for query strings that are not `&'static str`.
- React 18 to 19 with `react-dom` and both type packages together, since neither half resolves alone. Checked against what React 19 removes rather than assumed.
- `zustand` 4 to 5. All three stores already import the named `create`, which is the form version 5 expects.
- `vite` 8.1.5 to 8.2.0 and `@vitejs/plugin-react` 6.0.4 to 6.0.5.
- `github/codeql-action` 4.37.3 to 4.37.4 and `actions/attest` 4.2.0 to 4.2.1, merged separately and carried by this version.

### Added

- A test that holds the unit `sysinfo` reports memory in. The analyzer divides `proc.memory()` by 1024 twice and calls the result MB; if a future version switches to kilobytes, every value is off by a factor of 1024, the 500 MB threshold for memory-hungry processes stops firing, and the user simply sees no findings. Nothing about that would trouble a compiler.
- Tests that hold the `DhError` messages, including the `anyhow` conversion that goes through a hand-written `From`. `Serialize` hands those strings straight to the frontend.

### Removed

- `thiserror` from `dh-core` and `date-fns` from the frontend. Both were declared and never used. `date-fns` had been proposed for a 3 to 4 bump.

---

## [1.1.0] - 2026-08-03

### Changed

- Tailwind CSS 3 to 4. The config file is gone, the stylesheet imports tailwindcss directly, and PostCSS uses `@tailwindcss/postcss`. autoprefixer is no longer a dependency because version 4 prefixes on its own.
- Three utility names were rewritten across seven components: `rounded` to `rounded-sm` 8 times, `outline-none` to `outline-hidden` 6 times, `flex-shrink-0` to `shrink-0` 4 times. Only the last two change behaviour. Measured under 4.3.3, `rounded` is still 0.25rem and kept as an alias; the scale shifted under the name `rounded-sm`, which this code never used.

### Fixed

- The entry below was headed 1.0.10, but no 1.0.11 entry ever followed it and no 1.0.10 tag or release exists. What shipped as v1.0.11 is exactly what that entry describes, so the heading now says 1.0.11. Anyone comparing the release list against this file previously found a version with no entry and an entry with no version.

---

## [1.0.11] - 2026-07-31

### Fixed

- CI checked only macOS while the release builds for macOS, Linux and Windows. The AppImage and the Windows installer went out without ever having been compile-checked, so a fault appearing only on those platforms would have surfaced in a user's download rather than in a pull request. `check` now runs as a matrix over all three. The Linux runner installs the same GTK and WebKit packages the release workflow installs, since Tauri does not build without them.
- `db_path` bound `HOME` for every platform although the Windows branch uses `USERPROFILE`, so on Windows the binding was unused and the build failed under `-D warnings`. Each branch now reads its own variable.
- The first matrix run earned its keep. `cargo check` failed on Windows with an unused `mut` and a `classify_autostart_risk` reported as never used. Both are symptoms of the same gap: `get_autostart_entries` branches on macOS and Linux and has no Windows branch, so on Windows it returns an empty list and the autostart category silently finds nothing. The lints are now cfg-gated to say so, the READMEs state the limitation instead of implying full coverage, and implementing it is on the roadmap.
- The `solo-main-protection` ruleset now requires `Check (ubuntu-latest)`, `(macos-latest)` and `(windows-latest)` instead of the old single `Check`. Renaming a job without moving the required context leaves every later pull request permanently unmergeable while looking green.

---

## [1.0.9] - 2026-07-31

### Fixed

- The supported-versions table in `SECURITY.md` still listed `0.1.x`, a release line that no longer exists. Somebody reporting a vulnerability reads that table first, and it told them the current release was out of scope. It lists `1.0.x`.

---

## [1.0.8] - 2026-07-31

### Changed

- Both READMEs now open with the situation the tool is for, which is a process list full of names you cannot identify and therefore dare not touch, rather than with the analysis steps. A short paragraph says that anyone who already knows their process list is served faster by htop, Activity Monitor or Autoruns.

---

## [1.0.7] - 2026-07-29

### Security

- The release workflow no longer grants `contents: write` for its whole run. The permission moves to the one job that publishes the release, and everything else runs with `contents: read`. OpenSSF Scorecard scores the Token-Permissions check 0 out of 10 whenever any workflow holds a top-level write permission, regardless of how little of the run needs it, so this single line was what held the check at zero.
### Added

- `frontend/src/vite-env.d.ts`, referencing `vite/client`. Vite has always declared modules for `*.css` and the other asset types it handles, but nothing in this project pulled that declaration in. TypeScript 5 accepts the untyped side-effect import of `index.css` regardless, so the gap stayed invisible; TypeScript 7 rejects it with `TS2882`. The file belongs to Vite's own project scaffold and was simply missing, so this closes an existing hole rather than preparing for a specific upgrade.

---

## [1.0.6] - 2026-07-29

### Changed

Dependency and workflow updates merged since 1.0.5:

- chore(ci): bump the actions group across 1 directory with 3 updates
- chore(deps): bump the npm group across 1 directory with 3 updates
- chore(deps): bump the cargo group across 1 directory with 7 updates

---

## [1.0.5] - 2026-07-28

### Fixed

- The CodeQL job requested `packages: read`, `actions: read` and `contents: read` at job level, repeating grants the workflow level already provides. OpenSSF Scorecard counts that as excessive token permissions and scores `Token-Permissions` at 0 out of 10 for it. The job now requests only `security-events: write`, which is the one grant that genuinely exceeds the workflow default.

## [1.0.4] - 2026-07-28

### Changed

- CodeQL moved from GitHub's default setup to an advanced setup with a committed `.github/workflows/codeql.yml`. The default setup skips pull requests that touch no code of a given language, so a dependency pull request changing only a lock file reported `skipping` on the required `Analyze (...)` checks forever and could never be merged. The workflow runs on every pull request regardless of what changed. It also uses the `security-extended` query suite, which the default setup does not allow choosing. Required checks are unchanged: verified on `BugRadar` that all eight, the generic `CodeQL` check included, turn green under this setup.
- Dependabot now groups only minor and patch updates per ecosystem; majors arrive as individual pull requests. The previous grouping put React 18 to 19, Tailwind 3 to 4 and similar breaking changes into one pull request together with urgently needed security patches, which made the whole batch unreviewable and unmergeable. Actions stay grouped wholesale. Follows `engineering-standards` v0.11.0.

## [1.0.3] - 2026-07-28

### Security

- `postcss` updated to 8.5.24, closing a high-severity path traversal in the source map auto-loading via `sourceMappingURL` that affects all versions up to and including 8.5.17.

Applied as a normal pull request rather than by merging Dependabot's, because Dependabot pull requests cannot currently pass this repository's required checks: CodeQL runs through GitHub's default setup, which does not trigger on a pull request that only touches a lock file, so its checks report `skipping` and never turn green. Bypassing a required check is not an option per `standards/ci-cd.md` section 7, so the fix takes the route that runs the full pipeline.

## [1.0.2] - 2026-07-28

### Added

- `.github/dependabot.yml`, covering GitHub Actions, the Cargo workspace and the frontend npm packages, with grouped weekly updates. The file was missing, and without it there are no version updates at all: security alerts only fire for disclosed vulnerabilities. Follows `engineering-standards` v0.10.0.

### Fixed

- `frontend/package.json` carried version 0.2.6 while the workspace and `tauri.conf.json` were on 1.0.1, the tagged version. All manifests now agree, so the next bump can touch every file that carries a version, as `release-process.md` section 2 requires.
- `actions/checkout` was pinned to two different SHAs across the workflows. All now use v7.0.1 with the full version in the comment.

## [1.0.1] - 2026-07-20

### Changed

- OpenSSF Scorecard workflow and badge.
- `copilot-instructions.md` for consistent AI-assisted contributions.
- Coverage reporting in CI (cargo-tarpaulin, with the sqlx database prepared before compiling tests).
- Split the README's security/CI badges onto their own line, separate from the platform/tech/AI badges (they were rendering as a single merged line).

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
