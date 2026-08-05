# Security Policy: DeviceHealth

## Supported Versions

| Version | Supported |
|---------|-----------|
| 1.0.x   | ✅        |
| < 1.0 | ❌        |

## Reporting a Vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Report via [GitHub Security Advisory](https://github.com/9t29zhmwdh-coder/DeviceHealth/security/advisories/new)
or contact the maintainer via the GitHub profile.

Include: description, steps to reproduce, potential impact, suggested fix.
Response within 7 days.

## Security Design

- No external network calls except localhost:11434 (Ollama)
- RAM-only processing during analysis
- All Tauri IPC commands explicitly allowlisted
- No third-party analytics SDKs

## Known Accepted Exceptions

- **glib (RUSTSEC, medium): unsoundness in `Iterator`/`DoubleEndedIterator` impls for `glib::VariantStrIter`**, present in `glib 0.18.5` (a transitive dependency of Tauri's Linux tray/menu integration via `gtk 0.18.2`, `atk 0.18.2`). `gtk 0.18.2` pins `glib` to `^0.18`; the fixed `glib 0.20.0` requires a `gtk`/Tauri major-version bump, not an isolated patch. This crate is only linked on Linux builds and the unsound pattern is not reachable from this application's own code. Accepted as of 2026-07-17; revisit when Tauri's own dependency tree moves past `gtk 0.18`.

**Last updated: 2026-07-17**

## Known unfixable advisories

One dependency advisory is open and cannot be closed from this repository. It
is recorded here rather than dismissed as an accepted risk, per section 10 of
the portfolio [security standard](https://github.com/9t29zhmwdh-coder/engineering-standards/blob/main/standards/security.md).

**[GHSA-wrw7-89jp-8q8g](https://github.com/advisories/GHSA-wrw7-89jp-8q8g)**, moderate: unsoundness in the `Iterator` and `DoubleEndedIterator` implementations for `glib::VariantStrIter`. Affects `glib` from 0.15.0 up to but excluding 0.20.0. This project resolves `glib` 0.18.5.

**Where it comes from.** Not a direct dependency. `tauri` 2.11.5 requires `gtk ^0.18`, and `gtk` 0.18.2 requires `glib ^0.18`.

**Why it cannot be upgraded.** There is no patched 0.18.x release. The fix lands in 0.20.0, a semver-incompatible bump that only the gtk-rs 0.20 stack uses. Cargo rejects the upgrade outright rather than resolving it:

```
$ cargo update -p glib --precise 0.20.0
error: failed to select a version for the requirement `glib = "^0.18"`
candidate versions found which didn't match: 0.20.0
required by package `gtk v0.18.2`
    ... which satisfies dependency `gtk = "^0.18"` of package `tauri v2.11.5`
```

**Exposure.** `glib` reaches this project only through Tauri's Linux GTK backend, so it is compiled into the Linux build and not into the macOS or Windows builds. No code in this repository calls `glib` directly or uses `VariantStrIter`. Whether the GTK and WebKit layers exercise the unsound iterator internally has not been established here, so this is not a claim that the defect is unreachable, only that nothing in this codebase reaches it.

**What would end this.** A Tauri 2.x release that moves to the gtk-rs 0.20 stack. Re-checked whenever Tauri publishes a minor release.

