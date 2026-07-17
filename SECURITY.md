# Security Policy: DeviceHealth

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅        |

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
