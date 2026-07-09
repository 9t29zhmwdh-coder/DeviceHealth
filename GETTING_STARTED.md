# Getting Started with DeviceHealth

This guide walks you through setting up and running DeviceHealth from scratch, even if you have never used Rust, Node.js, or a terminal before. DeviceHealth is a native desktop app and runs on Windows, Linux, and macOS.

---

## Windows

### 1. Open a terminal

Right-click the Start button and choose **"Terminal"** (or **"Windows PowerShell"** on older versions of Windows).

### 2. Check prerequisites

Run each of these commands one by one:

```powershell
rustc --version
cargo --version
node --version
cargo tauri --version
```

If any command prints back a version number (e.g. `rustc 1.77.0`), you're good. If instead you see something like `'rustc' is not recognized as an internal or external command`, that tool is not installed yet:

- **Rust missing** → install it from [rustup.rs](https://rustup.rs) (also gives you `cargo`)
- **Node.js missing** → install it from [nodejs.org](https://nodejs.org) (LTS version recommended)
- **Tauri CLI missing** → after Rust is installed, run `cargo install tauri-cli`

Close and reopen your terminal after installing anything, so the new tools are recognized.

### 3. Get the code

**Easiest way (no git required):**
1. Go to the [DeviceHealth GitHub page](https://github.com/9t29zhmwdh-coder/DeviceHealth)
2. Click the green **"Code"** button → **"Download ZIP"**
3. Extract the ZIP file somewhere convenient, e.g. `C:\Projects\DeviceHealth`

**Alternative (if you have git):**
```powershell
git clone https://github.com/9t29zhmwdh-coder/DeviceHealth.git
```

### 4. Build and run

Open your terminal in the extracted/cloned folder (e.g. `cd C:\Projects\DeviceHealth`) and run:

```powershell
# Optional: pull AI model for process explanations
ollama pull llama3

cd frontend
npm install
cd ..
cargo tauri dev
```

<!-- TODO: Screenshot -->

### 5. What you should see

The first run downloads dependencies and compiles the Rust backend, which can take a few minutes. Once done, a native DeviceHealth window opens automatically. Click "Scan" to analyze your system and see a health score.

---

## Linux

### 1. Open a terminal

This depends on your desktop environment: try **Ctrl+Alt+T**, or look for "Terminal" in your application menu (GNOME, KDE, XFCE all have one).

### 2. Check prerequisites

```bash
rustc --version
cargo --version
node --version
cargo tauri --version
```

If you get a `command not found` error, that tool isn't installed:

- **Rust missing** → install via [rustup.rs](https://rustup.rs): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Node.js missing** → install from [nodejs.org](https://nodejs.org) or your distro's package manager
- **Tauri CLI missing** → `cargo install tauri-cli`

### 3. Get the code

**Easiest way (no git required):**
1. Go to the [DeviceHealth GitHub page](https://github.com/9t29zhmwdh-coder/DeviceHealth)
2. Click the green **"Code"** button → **"Download ZIP"**
3. Extract it, e.g. `unzip DeviceHealth-main.zip`

**Alternative (if you have git):**
```bash
git clone https://github.com/9t29zhmwdh-coder/DeviceHealth.git
```

### 4. Build and run

```bash
cd DeviceHealth

# Optional: pull AI model for process explanations
ollama pull llama3

cd frontend && npm install && cd ..
cargo tauri dev
```

### 5. What you should see

Tauri needs WebKitGTK and a few system libraries to build on Linux (see Troubleshooting below). Once the build finishes, the DeviceHealth window opens and you can trigger a scan.

---

## macOS

### 1. Open a terminal

Press **Cmd+Space** to open Spotlight, type "Terminal", and press Enter.

### 2. Check prerequisites

```bash
rustc --version
cargo --version
node --version
cargo tauri --version
```

If you see `command not found`:

- **Rust missing** → install via [rustup.rs](https://rustup.rs): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Node.js missing** → install from [nodejs.org](https://nodejs.org)
- **Tauri CLI missing** → `cargo install tauri-cli`

You will also need Xcode Command Line Tools: `xcode-select --install`

### 3. Get the code

**Easiest way (no git required):**
1. Go to the [DeviceHealth GitHub page](https://github.com/9t29zhmwdh-coder/DeviceHealth)
2. Click the green **"Code"** button → **"Download ZIP"**
3. Extract the ZIP (double-click it in Finder)

**Alternative (if you have git):**
```bash
git clone https://github.com/9t29zhmwdh-coder/DeviceHealth.git
```

### 4. Build and run

```bash
cd DeviceHealth

# Optional: pull AI model for process explanations
ollama pull llama3

cd frontend && npm install && cd ..
cargo tauri dev
```

### 5. What you should see

After the build completes, a native DeviceHealth window opens. You may need to allow the app in **System Settings → Privacy & Security** if macOS blocks it the first time.

---

## Troubleshooting

| Issue | Cause | Fix |
|---|---|---|
| `'rustc'`/`'cargo'` is not recognized / command not found | Rust not installed or not in PATH | Install via [rustup.rs](https://rustup.rs), then restart your terminal |
| `'node'`/`'npm'` is not recognized / command not found | Node.js not installed or not in PATH | Install via [nodejs.org](https://nodejs.org), then restart your terminal |
| PowerShell blocks `.ps1` scripts with an execution policy error | Windows execution policy defaults to "Restricted" | Run PowerShell as Administrator and execute `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned` |
| Rust build fails with linker errors on Windows | Missing C++ Build Tools | Install "Desktop development with C++" via the [Visual Studio Build Tools installer](https://visualstudio.microsoft.com/visual-cpp-build-tools/) |
| `cargo tauri dev` fails with missing `webkit2gtk` / glib errors on Linux | Missing WebKitGTK system dependencies | Install them via your package manager, e.g. `sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev` |
