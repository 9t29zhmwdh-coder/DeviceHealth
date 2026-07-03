<div align="center">
  <img src="RayStudio.png" alt="RayStudio Logo" width="120"/>

  <h1>DeviceHealth</h1>
</div>

[🇬🇧 English Version](README.md)

**KI-gestützte Systemdiagnose und Gesundheitsüberwachung. Vollständig offline, entwickelt mit Rust und Tauri.**

DeviceHealth analysiert alle laufenden Prozesse, Dienste, Hardware-Komponenten und Netzwerkaktivitäten auf deinem Gerät, erkennt Probleme automatisch und liefert Klartext-KI-Erklärungen mit konkreten Optimierungsempfehlungen; 100% lokal, ohne jegliche Cloud-Verbindung.

[![CI](https://github.com/9t29zhmwdh-coder/DeviceHealth/actions/workflows/ci.yml/badge.svg)](https://github.com/9t29zhmwdh-coder/DeviceHealth/actions) ![Platform](https://img.shields.io/badge/Platform-macOS_%7C_Windows-lightgrey) ![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white) ![Tauri](https://img.shields.io/badge/Tauri-24C8D8?logo=tauri&logoColor=white) ![AI | Claude Code](https://img.shields.io/badge/AI-Claude_Code-black?logo=anthropic&logoColor=white) ![AI | Copilot](https://img.shields.io/badge/AI-Copilot-black?logo=github&logoColor=white) ![AI | Ollama](https://img.shields.io/badge/AI-Ollama-black?logo=ollama&logoColor=white)
![Plattform](https://img.shields.io/badge/Plattform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)
![Lizenz](https://img.shields.io/badge/Lizenz-MIT-green)

---

## Funktionen

| Funktion | Beschreibung |
|---|---|
| **Gesundheitsscore** | Numerische Bewertung 0 bis 100 mit Note (Ausgezeichnet → Kritisch) |
| **Prozess-Analyse** | Erkennt Bloatware, Telemetrie, Zombie-Prozesse, CPU-Spikes, RAM-Leaks |
| **Befunde** | Kategorisierte Probleme (Kritisch / Hoch / Mittel / Niedrig / Info) mit Lösungsempfehlungen |
| **Hardware-Monitor** | CPU, RAM, Festplattennutzung, Temperaturen, Netzwerk-I/O |
| **Autostart-Erkennung** | Scannt LaunchAgents (macOS) und systemd-Units (Linux) auf unnötige Einträge |
| **Sicherheitsbefunde** | Verdächtige Prozesse, offene Risiken, Treiberfehler, fehlende Updates |
| **KI-Erklärungen** | Ollama erklärt jeden Prozess in Klartext: Ergebnisse lokal gecacht |
| **Verlauf** | Score-Trend und System-Snapshots über die Zeit mit Diagrammen |
| **Empfehlungen** | Umsetzbare Vorschläge mit Risikolevel: Ein-Klick mit Bestätigung |

---

## Voraussetzungen

- [Rust](https://rustup.rs/) 1.77+
- [Node.js](https://nodejs.org/) 20+
- [Tauri CLI v2](https://tauri.app/): `cargo install tauri-cli`
- [Ollama](https://ollama.ai) (optional, für KI-Erklärungen)
- macOS / Windows / Linux

---

## Schnellstart

```bash
git clone https://github.com/9t29zhmwdh-coder/DeviceHealth
cd DeviceHealth

# Optional: KI-Modell für Prozess-Erklärungen herunterladen
ollama pull llama3

cd frontend && npm install && cd ..
cargo tauri dev
```

---

## Datenschutz

DeviceHealth verarbeitet alle Systemdaten **lokal auf deinem Gerät**. Es werden keine Daten an externe Server gesendet. Ollama läuft vollständig offline; KI-Erklärungen verlassen dein Gerät nie.

---

## Architektur

```
DeviceHealth/
├── crates/dh-core/      — Rust: Analyzer, Hardware, Prozess-Erkennung, DB
├── crates/dh-cli/       — CLI-Binary
├── src-tauri/           — Tauri v2 Backend + IPC-Commands
└── frontend/            — React + TypeScript + Tailwind + Recharts
```

### Analyse-Pipeline

```
sysinfo ──► spawn_blocking ──► ProcessAnalyzer + HardwareAnalyzer
                                       │
                               SecurityAnalyzer + ServiceAnalyzer
                                       │
                               HealthScore (0 bis 100) + Befunde
                                       │
                               SQLite (Snapshots, KI-Cache)
```

---

**Autor:** [Rafael Yilmaz](https://github.com/9t29zhmwdh-coder) · **Status:** Active · ![version](https://img.shields.io/github/v/release/9t29zhmwdh-coder/DeviceHealth?label=\&color=6b7280\&style=flat-square) · **Lizenz:** MIT
