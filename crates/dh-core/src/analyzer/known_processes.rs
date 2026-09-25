use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::models::{process::{ProcessCategory, RiskLevel}, Lang};

pub struct KnownProcess {
    pub description_en: &'static str,
    pub description_de: &'static str,
    pub vendor: &'static str,
    pub category: ProcessCategory,
    pub risk: RiskLevel,
    pub can_disable: bool,
    pub is_telemetry: bool,
}

static KNOWN: Lazy<HashMap<&'static str, KnownProcess>> = Lazy::new(|| {
    let mut m = HashMap::new();

    macro_rules! add {
        ($name:expr, $en:expr, $de:expr, $vendor:expr, $cat:expr, $risk:expr, $disable:expr, $tel:expr) => {
            m.insert($name, KnownProcess {
                description_en: $en, description_de: $de, vendor: $vendor, category: $cat,
                risk: $risk, can_disable: $disable, is_telemetry: $tel,
            });
        };
    }

    // ── Windows System ──────────────────────────────────────────────────────
    add!("svchost.exe", "Windows service host: runs several Windows system services", "Windows Dienst-Host: verwaltet mehrere Windows-Systemdienste", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("lsass.exe", "Windows Local Security Authority: sign-in and authentication", "Windows Local Security Authority: Authentifizierung und Anmeldung", "Microsoft", ProcessCategory::Security, RiskLevel::Safe, false, false);
    add!("csrss.exe", "Client/Server Runtime Subsystem: core Windows component", "Client/Server Runtime Subsystem: Windows-Kernkomponente", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("winlogon.exe", "Windows logon manager: handles sign-in and sign-out", "Windows Anmelde-Manager: Verwaltet Anmelde- und Abmeldeprozesse", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("smss.exe", "Session Manager Subsystem: starts Windows user sessions", "Session Manager Subsystem: Startet Windows-Benutzersitzungen", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("wininit.exe", "Windows start-up application: runs at system start", "Windows Initialisierungs-App: Startprozess beim Systemstart", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("explorer.exe", "Windows Explorer: file manager and desktop shell", "Windows Explorer: Datei-Manager und Desktop-Shell", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("dwm.exe", "Desktop Window Manager: draws the desktop and its effects", "Desktop Window Manager: Verwaltet visuelle Effekte des Desktops", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("taskhostw.exe", "Task Host Window: hosts Windows background tasks", "Task Host Window: Hostet Windows-Hintergrundaufgaben", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("spoolsv.exe", "Print Spooler: manages print jobs", "Print Spooler: Verwaltet Druckaufträge", "Microsoft", ProcessCategory::Utility, RiskLevel::Low, true, false);
    add!("MsMpEng.exe", "Windows Defender: real-time virus protection", "Windows Defender: Echtzeit-Virenschutz", "Microsoft", ProcessCategory::Security, RiskLevel::Safe, false, false);
    add!("SearchIndexer.exe", "Windows Search indexer: indexes files for fast search", "Windows Suchindexierung: Indexiert Dateien für schnelle Suche", "Microsoft", ProcessCategory::Utility, RiskLevel::Low, true, false);
    add!("SearchHost.exe", "Windows Search host", "Windows Suchhost", "Microsoft", ProcessCategory::Utility, RiskLevel::Low, true, false);
    add!("RuntimeBroker.exe", "Runtime Broker: manages permissions of Store apps", "Runtime Broker: Verwaltet Berechtigungen für Store-Apps", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("WmiPrvSE.exe", "WMI Provider Host: Windows Management Instrumentation", "WMI Provider Host: Windows Management Instrumentation", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("audiodg.exe", "Windows Audio Device Graph: processes audio", "Windows Audio Device Graph: Verarbeitet Audio", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("fontdrvhost.exe", "Usermode Font Driver Host: renders system fonts", "Usermode Font Driver Host: Verarbeitet Systemschriften", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("sihost.exe", "Shell Infrastructure Host: user interface components", "Shell Infrastructure Host: UI-Komponenten", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("TextInputHost.exe", "Text Input Application: touch keyboard", "Text Input Application: Touch-Tastatur", "Microsoft", ProcessCategory::System, RiskLevel::Safe, true, false);
    add!("ShellExperienceHost.exe", "Windows Shell Experience Host: Start menu and action centre", "Windows Shell Experience Host: Startmenü, Aktionszentrum", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("ctfmon.exe", "CTF Loader: text input and language framework", "CTF Loader: Text-Eingabe und Sprachverarbeitungs-Framework", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("conhost.exe", "Console Window Host: hosts console windows", "Console Window Host: Hostet Konsolenfenster", "Microsoft", ProcessCategory::System, RiskLevel::Safe, false, false);

    // ── Windows Telemetrie ──────────────────────────────────────────────────
    add!("CompatTelRunner.exe", "Windows telemetry: collects usage data and sends it to Microsoft", "Windows Telemetrie: Sammelt Nutzungsdaten und sendet sie an Microsoft", "Microsoft", ProcessCategory::Telemetry, RiskLevel::Medium, true, true);
    add!("DiagTrack", "Connected User Experiences: Microsoft diagnostic tracking service", "Connected User Experiences: Microsoft Diagnose-Tracking-Service", "Microsoft", ProcessCategory::Telemetry, RiskLevel::Medium, true, true);
    add!("dmwappushservice", "WAP Push Message Routing: forwards Microsoft telemetry", "WAP Push Message Routing: Microsoft Telemetrie-Weiterleitung", "Microsoft", ProcessCategory::Telemetry, RiskLevel::Medium, true, true);
    add!("WerFault.exe", "Windows Error Reporting: sends crash reports to Microsoft", "Windows Error Reporting: Sendet Absturzberichte an Microsoft", "Microsoft", ProcessCategory::Telemetry, RiskLevel::Low, true, true);
    add!("PerfHost.exe", "Performance Counter DLL Host: performance monitoring", "Performance Counter DLL Host: Leistungsüberwachung", "Microsoft", ProcessCategory::Telemetry, RiskLevel::Low, true, false);

    // ── Browser ──────────────────────────────────────────────────────────────
    add!("chrome.exe", "Google Chrome: web browser", "Google Chrome: Webbrowser", "Google", ProcessCategory::Browser, RiskLevel::Safe, true, false);
    add!("firefox.exe", "Mozilla Firefox: web browser", "Mozilla Firefox: Webbrowser", "Mozilla", ProcessCategory::Browser, RiskLevel::Safe, true, false);
    add!("msedge.exe", "Microsoft Edge: web browser with telemetry", "Microsoft Edge: Webbrowser mit Telemetrie", "Microsoft", ProcessCategory::Browser, RiskLevel::Low, true, true);
    add!("safari", "Safari: Apple web browser", "Safari: Apple Webbrowser", "Apple", ProcessCategory::Browser, RiskLevel::Safe, true, false);
    add!("brave", "Brave: privacy-focused web browser", "Brave: Datenschutzorientierter Webbrowser", "Brave Software", ProcessCategory::Browser, RiskLevel::Safe, true, false);

    // ── macOS System ─────────────────────────────────────────────────────────
    add!("launchd", "macOS init system: manages all system and user services", "macOS Init-System: Verwaltet alle System- und Benutzerdienste", "Apple", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("WindowServer", "macOS display server: manages all windows and graphics output", "macOS Display Server: Verwaltet alle Fenster und grafische Ausgabe", "Apple", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("kernel_task", "macOS kernel: the core of the operating system", "macOS Kernel: Kern des Betriebssystems", "Apple", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("Finder", "macOS Finder: file manager and desktop", "macOS Finder: Datei-Manager und Desktop", "Apple", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("Dock", "macOS Dock: app bar and app switcher", "macOS Dock: Aufgabenleiste und Programmwechsler", "Apple", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("loginwindow", "macOS login window: sign-in screen", "macOS Login Window: Anmeldebildschirm", "Apple", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("mds_stores", "macOS Spotlight indexing: file search index", "macOS Spotlight-Indexierung: Dateisuchindex", "Apple", ProcessCategory::Utility, RiskLevel::Safe, false, false);
    add!("mdworker", "macOS Spotlight worker: reads files for Spotlight", "macOS Spotlight Worker: Verarbeitet Dateien für Spotlight", "Apple", ProcessCategory::Utility, RiskLevel::Safe, false, false);
    add!("coreaudiod", "macOS Core Audio: audio system daemon", "macOS Core Audio: Audio-System-Daemon", "Apple", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("cloudd", "iCloud daemon: syncs files with iCloud", "iCloud-Daemon: Synchronisiert Dateien mit iCloud", "Apple", ProcessCategory::Utility, RiskLevel::Low, true, false);
    add!("bird", "iCloud Drive: iCloud file sync process", "iCloud Drive: iCloud Datei-Sync-Prozess", "Apple", ProcessCategory::Utility, RiskLevel::Low, true, false);
    add!("parentalcontrolsd", "Parental controls daemon", "Jugendschutz-Daemon", "Apple", ProcessCategory::System, RiskLevel::Safe, true, false);
    add!("coreduetd", "Core Duet: learns your usage patterns (telemetry)", "Core Duet: Lernt dein Nutzungsverhalten (Telemetrie)", "Apple", ProcessCategory::Telemetry, RiskLevel::Low, true, true);
    add!("analyticspanetool", "Apple Analytics: collects diagnostic data", "Apple Analytics: Sammelt Diagnosedaten", "Apple", ProcessCategory::Telemetry, RiskLevel::Low, true, true);
    add!("trustd", "Trust evaluation: verifies certificates and signatures", "Trust Evaluation: Verifiziert Zertifikate und Signaturen", "Apple", ProcessCategory::Security, RiskLevel::Safe, false, false);
    add!("syspolicyd", "System policy: Gatekeeper and code signature checks", "System Policy: Gatekeeper und Codesignatur-Prüfung", "Apple", ProcessCategory::Security, RiskLevel::Safe, false, false);
    add!("com.apple.WebKit.WebContent", "WebKit content process: renders web pages in Safari and apps", "WebKit Content Process: Web-Rendering in Safari/Apps", "Apple", ProcessCategory::Browser, RiskLevel::Safe, false, false);
    add!("distnoted", "Distributed notifications: system-wide notifications", "Distributed Notifications: Systemweite Benachrichtigungen", "Apple", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("cfprefsd", "Core Foundation preferences: settings daemon", "Core Foundation Preferences: Einstellungs-Daemon", "Apple", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("nsurlsessiond", "URL session daemon: background network transfers", "URL Session Daemon: Netzwerkverbindungen im Hintergrund", "Apple", ProcessCategory::System, RiskLevel::Safe, false, false);

    // ── Linux System ────────────────────────────────────────────────────────
    add!("systemd", "Linux init system and service manager", "Linux Init-System und Service-Manager", "systemd", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("NetworkManager", "NetworkManager: manages network connections", "NetworkManager: Verwaltet Netzwerkverbindungen", "GNOME", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("snapd", "Snap package daemon: manages Snap packages (heavy I/O)", "Snap Package Daemon: Verwaltet Snap-Pakete (hoher I/O)", "Canonical", ProcessCategory::Bloatware, RiskLevel::Low, true, false);
    add!("packagekitd", "PackageKit: package management daemon", "PackageKit: Paketmanagement-Daemon", "freedesktop", ProcessCategory::Utility, RiskLevel::Safe, true, false);
    add!("Xorg", "X.org display server: graphics system", "X.org Display Server: Grafik-System", "X.Org", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("pulseaudio", "PulseAudio: sound server", "PulseAudio: Audio-Server", "freedesktop", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("pipewire", "PipeWire: audio and video server (modern PulseAudio replacement)", "PipeWire: Audio/Video-Server (moderner Ersatz für PulseAudio)", "freedesktop", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("dockerd", "Docker daemon: container engine", "Docker Daemon: Container-Engine", "Docker", ProcessCategory::Development, RiskLevel::Safe, true, false);
    add!("sshd", "SSH daemon: allows remote connections over SSH", "SSH Daemon: Erlaubt Remote-Verbindungen per SSH", "OpenSSH", ProcessCategory::Network, RiskLevel::Low, true, false);
    add!("cron", "Cron: scheduled tasks", "Cron: Geplante Aufgaben", "GNU", ProcessCategory::System, RiskLevel::Safe, false, false);
    add!("avahi-daemon", "Avahi: mDNS/DNS-SD service (finds devices on the network)", "Avahi: mDNS/DNS-SD-Dienst (Netzwerk-Geräteerkennung)", "Avahi", ProcessCategory::Network, RiskLevel::Low, true, false);
    add!("cups", "CUPS: printing system", "CUPS: Drucksystem", "Apple", ProcessCategory::Utility, RiskLevel::Safe, true, false);

    // ── Bekannte Drittanbieter ───────────────────────────────────────────────
    add!("GoogleUpdate.exe", "Google Updater: updates Google software in the background", "Google Updater: Aktualisiert Google-Software im Hintergrund", "Google", ProcessCategory::Utility, RiskLevel::Low, true, false);
    add!("GoogleCrashHandler.exe", "Google Crash Handler: sends crash reports to Google", "Google Crash Handler: Sendet Absturzberichte an Google", "Google", ProcessCategory::Telemetry, RiskLevel::Low, true, true);
    add!("AdobeUpdateService", "Adobe Update Service: checks for Adobe updates", "Adobe Update Service: Sucht nach Adobe-Updates", "Adobe", ProcessCategory::Bloatware, RiskLevel::Low, true, false);
    add!("CCXProcess.exe", "Adobe Creative Cloud: Creative Cloud background process", "Adobe Creative Cloud: Hintergrundprozess der Creative Cloud", "Adobe", ProcessCategory::Bloatware, RiskLevel::Low, true, false);
    add!("AdobeIPCBroker.exe", "Adobe IPC Broker: inter-process communication for Adobe apps", "Adobe IPC Broker: Adobe Inter-Prozess-Kommunikation", "Adobe", ProcessCategory::Bloatware, RiskLevel::Low, true, false);
    add!("Teams.exe", "Microsoft Teams: communication platform", "Microsoft Teams: Kommunikationsplattform", "Microsoft", ProcessCategory::Utility, RiskLevel::Low, true, false);
    add!("slack", "Slack: team chat", "Slack: Team-Kommunikation", "Slack", ProcessCategory::Utility, RiskLevel::Safe, true, false);
    add!("zoom", "Zoom: video conferencing", "Zoom: Videokonferenz-Software", "Zoom", ProcessCategory::Utility, RiskLevel::Low, true, false);
    add!("zoom.us", "Zoom: video conferencing", "Zoom: Videokonferenz-Software", "Zoom", ProcessCategory::Utility, RiskLevel::Low, true, false);
    add!("Discord", "Discord: chat and voice (background overlay)", "Discord: Chat und Voice-Kommunikation (Hintergrund-Overlay)", "Discord", ProcessCategory::Utility, RiskLevel::Low, true, false);
    add!("Discord.exe", "Discord: chat and voice", "Discord: Chat und Voice-Kommunikation", "Discord", ProcessCategory::Utility, RiskLevel::Low, true, false);
    add!("Spotify", "Spotify: music streaming", "Spotify: Musikstreaming", "Spotify", ProcessCategory::Media, RiskLevel::Low, true, false);
    add!("Spotify.exe", "Spotify: music streaming", "Spotify: Musikstreaming", "Spotify", ProcessCategory::Media, RiskLevel::Low, true, false);
    add!("steam", "Steam: gaming platform and game library", "Steam: Gaming-Plattform und Spieleverwaltung", "Valve", ProcessCategory::Gaming, RiskLevel::Safe, true, false);
    add!("Steam.exe", "Steam: gaming platform", "Steam: Gaming-Plattform", "Valve", ProcessCategory::Gaming, RiskLevel::Safe, true, false);
    add!("EpicGamesLauncher.exe", "Epic Games Launcher: game store", "Epic Games Launcher: Gaming-Store", "Epic", ProcessCategory::Gaming, RiskLevel::Low, true, false);
    add!("OneDrive.exe", "Microsoft OneDrive: cloud sync (always running)", "Microsoft OneDrive: Cloud-Sync (ständig aktiv)", "Microsoft", ProcessCategory::Utility, RiskLevel::Low, true, false);
    add!("OneDrive", "Microsoft OneDrive: cloud sync", "Microsoft OneDrive: Cloud-Sync", "Microsoft", ProcessCategory::Utility, RiskLevel::Low, true, false);
    add!("Dropbox.exe", "Dropbox: cloud file sync", "Dropbox: Cloud-Datei-Sync", "Dropbox", ProcessCategory::Utility, RiskLevel::Low, true, false);
    add!("dropbox", "Dropbox: cloud file sync", "Dropbox: Cloud-Datei-Sync", "Dropbox", ProcessCategory::Utility, RiskLevel::Low, true, false);
    add!("code", "Visual Studio Code: code editor", "Visual Studio Code: Code-Editor", "Microsoft", ProcessCategory::Development, RiskLevel::Safe, true, false);
    add!("Code.exe", "Visual Studio Code: code editor", "Visual Studio Code: Code-Editor", "Microsoft", ProcessCategory::Development, RiskLevel::Safe, true, false);
    add!("node", "Node.js: JavaScript runtime", "Node.js: JavaScript-Laufzeitumgebung", "OpenJS", ProcessCategory::Development, RiskLevel::Safe, false, false);
    add!("python", "Python: script interpreter", "Python: Skriptsprachen-Interpreter", "Python", ProcessCategory::Development, RiskLevel::Safe, false, false);
    add!("python3", "Python 3: script interpreter", "Python 3: Skriptsprachen-Interpreter", "Python", ProcessCategory::Development, RiskLevel::Safe, false, false);
    add!("java", "Java runtime: runs Java applications", "Java Runtime: Java-Anwendungs-Laufzeitumgebung", "Oracle/OpenJDK", ProcessCategory::Development, RiskLevel::Safe, false, false);
    add!("git", "Git: version control", "Git: Versionskontrollsystem", "Git", ProcessCategory::Development, RiskLevel::Safe, false, false);
    add!("docker", "Docker CLI: container management", "Docker CLI: Container-Verwaltung", "Docker", ProcessCategory::Development, RiskLevel::Safe, false, false);
    add!("ollama", "Ollama: local AI model server", "Ollama: Lokaler KI-Modell-Server", "Ollama", ProcessCategory::Development, RiskLevel::Safe, true, false);
    add!("postgres", "PostgreSQL: database server", "PostgreSQL: Datenbank-Server", "PostgreSQL", ProcessCategory::Development, RiskLevel::Safe, true, false);
    add!("mysqld", "MySQL: database server", "MySQL: Datenbank-Server", "MySQL/Oracle", ProcessCategory::Development, RiskLevel::Safe, true, false);
    add!("nginx", "Nginx: web server and reverse proxy", "Nginx: Webserver / Reverse Proxy", "Nginx", ProcessCategory::Network, RiskLevel::Safe, true, false);
    add!("apache2", "Apache: web server", "Apache: Webserver", "Apache", ProcessCategory::Network, RiskLevel::Safe, true, false);
    add!("redis-server", "Redis: in-memory data store", "Redis: In-Memory-Datenstruktur-Store", "Redis", ProcessCategory::Development, RiskLevel::Safe, true, false);

    m
});

impl KnownProcess {
    pub fn description(&self, lang: Lang) -> String {
        lang.pick(self.description_en, self.description_de)
    }
}

pub fn lookup(name: &str) -> Option<&'static KnownProcess> {
    let lower = name.to_lowercase();
    KNOWN.get(lower.as_str()).or_else(|| KNOWN.get(name))
}


pub fn is_suspicious_name(name: &str) -> bool {
    let n = name.to_lowercase();
    let base = n.trim_end_matches(".exe").trim_end_matches(".bin");
    if base.len() < 4 { return false; }
    let alpha: usize = base.chars().filter(|c| c.is_alphabetic()).count();
    let total = base.len();
    let ratio = alpha as f32 / total as f32;
    ratio < 0.5 && total > 6
}
