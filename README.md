<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>DeviceHealth</h1>
</div>

[🇩🇪 Deutsche Version](README.de.md)

**AI-powered system diagnostics and health monitoring. Fully offline, built with Rust and Tauri.**

DeviceHealth analyzes all running processes, services, hardware components, and network activity on your machine, detects problems automatically, and provides plain-language AI explanations with concrete optimization recommendations; 100% locally, without any cloud connection.

[![CI](https://github.com/9t29zhmwdh-coder/DeviceHealth/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/DeviceHealth/actions/workflows/ci.yml)
![Rust](https://img.shields.io/badge/Rust-1.77+-orange?logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-v2-blue?logo=tauri)
![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)
![License](https://img.shields.io/badge/License-MIT-green)

---

## Features

| Feature | Description |
|---|---|
| **Health Score** | Numerical 0 to 100 rating with grade (Excellent → Critical) |
| **Process Analysis** | Detects bloatware, telemetry, zombie processes, CPU spikes, RAM leaks |
| **Findings** | Categorized issues (Critical / High / Medium / Low / Info) with fix recommendations |
| **Hardware Monitor** | CPU, RAM, disk usage, temperatures, network I/O |
| **Autostart Detection** | Scans LaunchAgents (macOS) and systemd units (Linux) for unnecessary entries |
| **Security Findings** | Suspicious processes, open risks, driver errors, missing updates |
| **AI Explanations** | Ollama explains every process in plain language: results cached locally |
| **History** | Score trend and system snapshots over time with charts |
| **Smart Recommendations** | Actionable suggestions with risk level: one-click with confirmation |

---

## Requirements

- [Rust](https://rustup.rs/) 1.77+
- [Node.js](https://nodejs.org/) 20+
- [Tauri CLI v2](https://tauri.app/): `cargo install tauri-cli`
- [Ollama](https://ollama.ai) (optional, for AI explanations)
- macOS / Windows / Linux

---

## Quick Start

```bash
git clone https://github.com/9t29zhmwdh-coder/DeviceHealth
cd DeviceHealth

# Optional: pull AI model for process explanations
ollama pull llama3

cd frontend && npm install && cd ..
cargo tauri dev
```

---

## Privacy

DeviceHealth processes all system data **locally on your machine**. No data is sent to any external server. Ollama runs entirely offline; AI explanations never leave your device.

---

## Architecture

```
DeviceHealth/
├── crates/dh-core/      — Rust: analyzer, hardware, process detection, DB
├── crates/dh-cli/       — CLI binary
├── src-tauri/           — Tauri v2 backend + IPC commands
└── frontend/            — React + TypeScript + Tailwind + Recharts
```

### Analysis Pipeline

```
sysinfo ──► spawn_blocking ──► ProcessAnalyzer + HardwareAnalyzer
                                       │
                               SecurityAnalyzer + ServiceAnalyzer
                                       │
                               HealthScore (0 to 100) + Findings
                                       │
                               SQLite (snapshots, AI cache)
```

---

**Author:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Framework Preview · **Last Updated:** Juni 2026
