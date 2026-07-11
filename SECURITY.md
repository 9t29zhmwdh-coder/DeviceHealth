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

**Last updated: 2026-06-12**
