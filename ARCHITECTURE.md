# Architecture: DeviceHealth

## Overview

DeviceHealth is a Rust workspace with a Tauri v2 desktop shell and a React/TypeScript frontend. The core library (`dh-core`) performs all system analysis and is fully decoupled from the GUI. `dh-cli` allows headless operation. All AI-generated explanations and optimization recommendations are produced locally via Ollama; no data leaves the device.

```
DeviceHealth/
├── dh-core/              # Core library crate
│   └── src/
│       ├── process/      # ProcessAnalyzer: running processes, CPU/mem per process
│       ├── hardware/     # HardwareAnalyzer: CPU, RAM, disk, temps (sysinfo)
│       ├── security/     # SecurityAnalyzer: open ports, suspicious processes, firewall
│       ├── service/      # ServiceAnalyzer: system services / daemons, startup items
│       ├── score/        # HealthScore computation (0-100, weighted subscore model)
│       └── ai/           # Ollama client: generates explanations + recommendations
├── dh-cli/               # CLI binary crate (headless / scripted diagnostics)
├── src-tauri/            # Tauri v2 backend
│   └── src/
│       ├── main.rs
│       ├── error.rs
│       ├── state.rs
│       └── commands/     # IPC handlers (analyze, score, explain, export)
└── frontend/             # React + TypeScript + Vite
    └── src/
        ├── stores/       # Zustand state (analysis results, score history)
        └── components/   # UI components (ScoreDial, ProcessTable, HardwarePanel)
```

## Component Diagram

```
┌──────────────────────────────────────────────────────────────┐
│                    DeviceHealth Desktop                      │
│                                                              │
│  ┌──────────────┐    Tauri IPC    ┌───────────────────────┐  │
│  │   Frontend   │◄───────────────►│      src-tauri        │  │
│  │  React / TS  │                 │    commands/*.rs      │  │
│  └──────────────┘                 └──────────┬────────────┘  │
│                                              │               │
│                                   ┌──────────▼────────────┐  │
│                                   │       dh-core         │  │
│                                   │                       │  │
│                                   │  ProcessAnalyzer      │  │
│                                   │  HardwareAnalyzer ◄───┼──┼── sysinfo
│                                   │  SecurityAnalyzer     │  │
│                                   │  ServiceAnalyzer      │  │
│                                   │  HealthScore (0-100)  │  │
│                                   │  ai/ ─────────────────┼──┼──► Ollama
│                                   └───────────────────────┘  │    localhost:11434
│                                                              │
│  dh-cli ────────────────────────────────────► dh-core        │
└──────────────────────────────────────────────────────────────┘
```

## Data Flow

1. **Collect**: `HardwareAnalyzer` queries `sysinfo` for CPU, RAM, disk, and temperature data; `ProcessAnalyzer` enumerates running processes with resource usage; `ServiceAnalyzer` inspects system services; `SecurityAnalyzer` checks open ports and flags unusual processes.
2. **Score**: `HealthScore` aggregates all analyzer outputs into a weighted 0-100 score, with per-category subscores (performance, security, stability, storage).
3. **Explain**: the AI module sends a structured summary of detected issues to Ollama and returns human-readable explanations and ranked optimization recommendations.
4. **Display**: Tauri IPC pushes results to the React frontend, which renders the score dial, process table, hardware panels, and AI recommendations.
5. **Export**: users can export a full diagnostic report (JSON or plain text) for sharing or archiving.

## External Dependencies

| Dependency | Purpose | Network |
|------------|---------|---------|
| sysinfo | Cross-platform system metrics | none |
| Ollama (localhost:11434) | AI explanations + recommendations | localhost only |
| serde / serde_json | Serialization | none |
| Tauri v2 | Desktop shell + IPC | none |
| React + Vite | Frontend | none (build-time only) |
