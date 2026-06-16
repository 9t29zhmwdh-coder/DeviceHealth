# DeviceHealth — Professional Repo Skeleton

**Generated:** 2026-06-16 | **Earliest commit:** 2026-06-12 | **Release:** v0.1.0

## Canonical File Tree

```
DeviceHealth/
├── .github/
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   └── PULL_REQUEST_TEMPLATE.md
├── dh-core/
│   └── src/
│       ├── process/
│       ├── hardware/
│       ├── security/
│       ├── service/
│       ├── score/
│       └── ai/
├── dh-cli/
├── src-tauri/
│   └── src/
│       ├── main.rs
│       ├── error.rs
│       ├── state.rs
│       └── commands/
├── frontend/
│   └── src/
│       ├── stores/
│       └── components/
├── ARCHITECTURE.md
├── CHANGELOG.md
├── CODE_OF_CONDUCT.md
├── CONTRIBUTING.md
├── LICENSE
├── PRIVACY.md
├── README.md
├── README.de.md
├── ROADMAP.md
├── SECURITY.md
└── SKELETON.md
```

## Migration Checklist

- ARCHITECTURE.md ✅
- PRIVACY.md ✅
- ROADMAP.md ✅
- CODE_OF_CONDUCT.md ✅
- SECURITY.md ✅
- CHANGELOG.md ✅
- .github/ISSUE_TEMPLATE/ ✅
- .github/PULL_REQUEST_TEMPLATE.md ✅
- .github/workflows/ci.yml ⚠️ — requires `workflows` OAuth scope (run: gh auth refresh -s workflows)
- GitHub Release v0.1.0 ✅

## CI Workflow (push manually after: gh auth refresh -s workflows)

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  check:
    name: Check & Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: cargo check
        run: cargo check --workspace
      - name: cargo test
        run: cargo test --workspace
      - name: cargo clippy
        run: cargo clippy --workspace -- -D warnings
      - name: cargo fmt
        run: cargo fmt --all -- --check

  build:
    name: Build (release)
    runs-on: ubuntu-latest
    needs: check
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: cargo build --release
        run: cargo build --workspace --release
```

## Reusable from this repo

- `dh-core/src/score/` — HealthScore (0–100, weighted subscore model) reusable as a diagnostic scoring primitive in other RayStudio monitoring tools
- `CODE_OF_CONDUCT.md` — identical across all RayStudio repos, copy as-is
- `PULL_REQUEST_TEMPLATE.md` — generic Rust/Tauri checklist, reusable for all Rust workspace repos

---

*DeviceHealth — RayStudio · Rafael Yilmaz · MIT License · 2026*
