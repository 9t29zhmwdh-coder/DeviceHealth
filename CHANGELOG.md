# Changelog — DeviceHealth

All notable changes to this project will be documented in this file.

Format: [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) — [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

---

## [0.1.0] — 2026-06-12

### Added

- Process analysis: running processes with per-process CPU and memory usage
- Hardware analysis: CPU, RAM, disk, and temperature metrics via sysinfo
- Security analysis: open port enumeration, suspicious process detection, basic firewall check
- Service analysis: system service and daemon status, startup item inventory
- HealthScore 0–100 with weighted per-category subscores (performance, security, stability, storage)
- AI-generated explanations and ranked optimization recommendations via Ollama (localhost:11434)
- `dh-core` Rust crate: `process/`, `hardware/`, `security/`, `service/`, `score/`, `ai/`
- `dh-cli` binary for headless and scripted diagnostic operation
- Tauri v2 desktop shell for macOS, Windows, and Linux
- React/TypeScript frontend with score dial, process table, and hardware panels

[0.1.0]: https://github.com/9t29zhmwdh-coder/DeviceHealth/releases/tag/v0.1.0
