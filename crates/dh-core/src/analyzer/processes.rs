use sysinfo::{ProcessStatus, System};
use crate::models::{
    finding::{Finding, FindingKind, Severity},
    process::{ProcessCategory, ProcessEntry, RiskLevel},
    settings::AppSettings,
    Lang,
};
use super::{known_processes, origin::{origin_of, tilde, Origin}};

/// What a process is, who it belongs to and whether it may be quit.
struct Classification {
    category: ProcessCategory,
    risk: RiskLevel,
    description: Option<String>,
    vendor: Option<String>,
    can_disable: bool,
    is_telemetry: bool,
}

pub fn analyze_processes(sys: &System, settings: &AppSettings, lang: Lang) -> Vec<ProcessEntry> {
    let own_user = sysinfo::get_current_pid().ok()
        .and_then(|pid| sys.process(pid))
        .and_then(|p| p.user_id().cloned());

    sys.processes()
        .iter()
        .map(|(pid, proc)| {
            let name = proc.name().to_string_lossy().to_string();
            let exe_path = proc.exe().map(|p| p.to_string_lossy().to_string()).filter(|p| !p.is_empty());
            let is_zombie = matches!(proc.status(), ProcessStatus::Zombie);
            let foreign = own_user.is_some() && proc.user_id() != own_user.as_ref();

            let mut c = classify(&name, exe_path.as_deref(), foreign, lang);
            if is_zombie {
                c.category = ProcessCategory::Zombie;
                c.risk = RiskLevel::Medium;
            } else if known_processes::is_suspicious_name(&name) {
                c.risk = RiskLevel::High;
            }

            let mut flags = Vec::new();
            if proc.cpu_usage() > settings.cpu_spike_threshold { flags.push("high-cpu".to_string()); }
            let mem_mb = proc.memory() as f64 / 1024.0 / 1024.0;
            if mem_mb > 500.0 { flags.push("high-memory".to_string()); }
            if is_zombie { flags.push("zombie".to_string()); }
            if c.is_telemetry { flags.push("telemetry".to_string()); }

            ProcessEntry {
                pid: pid.as_u32(),
                name,
                exe_path: exe_path.map(|p| tilde(&p)),
                cpu_usage: proc.cpu_usage(),
                memory_bytes: proc.memory(),
                status: format!("{:?}", proc.status()),
                is_zombie,
                user: proc.user_id().map(|u| u.to_string()),
                risk: c.risk,
                category: c.category,
                description: c.description,
                vendor: c.vendor,
                can_disable: c.can_disable,
                is_telemetry: c.is_telemetry,
                flags,
            }
        })
        .collect()
}

/// The name catalogue first (it knows telemetry and bloatware), then the path.
fn classify(name: &str, exe: Option<&str>, foreign_user: bool, lang: Lang) -> Classification {
    if let Some(k) = known_processes::lookup(name) {
        return Classification {
            category: k.category.clone(),
            risk: k.risk.clone(),
            description: Some(k.description(lang)),
            vendor: Some(k.vendor.to_string()),
            can_disable: k.can_disable,
            is_telemetry: k.is_telemetry,
        };
    }
    let (category, risk, description, vendor, can_disable) = match exe.map(origin_of) {
        Some(Origin::OperatingSystem) => (
            ProcessCategory::System,
            RiskLevel::Safe,
            lang.pick("Part of the operating system. Leave it running.", "Teil des Betriebssystems. Laufen lassen."),
            None,
            false,
        ),
        Some(Origin::App(app)) => (
            ProcessCategory::Application,
            RiskLevel::Low,
            lang.pick(
                format!("Belongs to {app}. Quitting it closes {app} or one of its helpers; it may start again with the app."),
                format!("Gehört zu {app}. Beenden schliesst {app} oder einen seiner Helfer; mit der App kann er wieder starten."),
            ),
            Some(app),
            true,
        ),
        Some(Origin::ThirdParty) => (
            ProcessCategory::Unknown,
            RiskLevel::Unknown,
            lang.pick(
                format!("Installed outside the system and outside any app: {}. Check what installed it before removing anything.", tilde(exe.unwrap_or_default())),
                format!("Ausserhalb des Systems und ausserhalb jeder App installiert: {}. Vor dem Entfernen prüfen, was ihn installiert hat.", tilde(exe.unwrap_or_default())),
            ),
            None,
            false,
        ),
        None if foreign_user => (
            ProcessCategory::Unknown,
            RiskLevel::Unknown,
            lang.pick(
                "Runs under another user account (usually the system). Its file is only visible with administrator rights.",
                "Läuft unter einem anderen Benutzerkonto (meist dem System). Die Datei ist nur mit Administratorrechten sichtbar.",
            ),
            None,
            false,
        ),
        None => (ProcessCategory::Unknown, RiskLevel::Unknown, String::new(), None, false),
    };
    Classification {
        category,
        risk,
        description: Some(description).filter(|d| !d.is_empty()),
        vendor,
        can_disable,
        is_telemetry: false,
    }
}

/// Asks the process to quit (SIGTERM on Unix, so it can save and clean up).
/// Checks the name first: the PID may meanwhile belong to another program.
pub fn quit(pid: u32, expected_name: &str) -> Result<(), String> {
    let pid = sysinfo::Pid::from_u32(pid);
    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true);
    let process = sys.process(pid).ok_or_else(|| format!("{expected_name} is no longer running"))?;
    if process.name().to_string_lossy() != expected_name {
        return Err(format!("PID {pid} now belongs to another program"));
    }
    let sent = process.kill_with(sysinfo::Signal::Term).unwrap_or_else(|| process.kill());
    if sent { Ok(()) } else { Err(format!("{expected_name} could not be quit")) }
}

fn names(list: &[&ProcessEntry]) -> String {
    list.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", ")
}

pub fn detect_process_findings(processes: &[ProcessEntry], settings: &AppSettings, lang: Lang) -> Vec<Finding> {
    let mut findings = Vec::new();

    let zombies: Vec<&ProcessEntry> = processes.iter().filter(|p| p.is_zombie).collect();
    if !zombies.is_empty() {
        findings.push(Finding::new(
            FindingKind::ZombieProcess,
            Severity::Medium,
            &lang.pick(format!("{} zombie process(es)", zombies.len()), format!("{} Zombie-Prozess(e) erkannt", zombies.len())),
            &lang.pick(
                "Zombie processes have finished but still hold an entry in the process list.",
                "Zombie-Prozesse haben ihren Ausführungszyklus beendet, belegen aber noch Einträge in der Prozessliste.",
            ),
            &names(&zombies),
            &lang.pick("Restart the system to clear them.", "Starte das System neu, um Zombie-Prozesse zu bereinigen."),
        ));
    }

    let high_cpu = processes.iter()
        .filter(|p| p.cpu_usage > settings.cpu_spike_threshold && !matches!(p.category, ProcessCategory::System));
    for proc in high_cpu {
        findings.push(Finding::new(
            FindingKind::HighCpuUsage,
            if proc.cpu_usage > 80.0 { Severity::High } else { Severity::Medium },
            &format!("{}: {:.1}% CPU", proc.name, proc.cpu_usage),
            &lang.pick(
                format!("'{}' is using a lot of CPU right now.", proc.name),
                format!("Prozess '{}' verbraucht gerade viel CPU-Leistung.", proc.name),
            ),
            &proc.name,
            &lang.pick(
                format!("Check whether '{}' is needed. Quit or restart it if not.", proc.name),
                format!("Prüfe, ob '{}' notwendig ist. Ggf. beenden oder neu starten.", proc.name),
            ),
        ));
    }

    let high_mem = processes.iter()
        .filter(|p| p.memory_mb() > 500.0 && matches!(p.category, ProcessCategory::Unknown | ProcessCategory::Bloatware));
    for proc in high_mem {
        findings.push(Finding::new(
            FindingKind::HighMemoryUsage,
            Severity::Medium,
            &format!("{}: {:.0} MB RAM", proc.name, proc.memory_mb()),
            &lang.pick(
                format!("'{}' uses an unusual amount of memory.", proc.name),
                format!("Prozess '{}' belegt ungewöhnlich viel Arbeitsspeicher.", proc.name),
            ),
            &proc.name,
            &lang.pick(
                "Restart the process, or uninstall it if you do not need it.",
                "Starte den Prozess neu oder deinstalliere ihn, wenn er nicht benötigt wird.",
            ),
        ));
    }

    let telemetry: Vec<&ProcessEntry> = processes.iter().filter(|p| p.is_telemetry).collect();
    if !telemetry.is_empty() {
        findings.push(Finding::new(
            FindingKind::Telemetry,
            Severity::Low,
            &lang.pick(format!("{} telemetry process(es) running", telemetry.len()), format!("{} Telemetrie-Prozess(e) aktiv", telemetry.len())),
            &lang.pick(
                "Some running processes send diagnostic and usage data to their vendor.",
                "Einige aktive Prozesse senden Diagnosedaten und Nutzungsstatistiken an den Hersteller.",
            ),
            &names(&telemetry),
            &lang.pick(
                "Turn off diagnostics you do not want in the system settings.",
                "Deaktiviere unnötige Diagnosedaten in den Systemeinstellungen.",
            ),
        ));
    }

    let suspicious = processes.iter()
        .filter(|p| matches!(p.risk, RiskLevel::High | RiskLevel::Critical) && !p.is_zombie);
    for proc in suspicious {
        findings.push(Finding::new(
            FindingKind::SuspiciousProcess,
            Severity::High,
            &lang.pick(format!("Suspicious process: {}", proc.name), format!("Verdächtiger Prozess: {}", proc.name)),
            &lang.pick(
                format!("'{}' has an unusual name and is not known to be safe.", proc.name),
                format!("Prozess '{}' hat einen ungewöhnlichen Namen und ist nicht als sicher bekannt.", proc.name),
            ),
            &proc.name,
            &lang.pick(
                "Check it by hand: where its file lives and what installed it.",
                "Prüfe ihn von Hand: wo seine Datei liegt und was ihn installiert hat.",
            ),
        ));
    }

    let bloatware: Vec<&ProcessEntry> = processes.iter()
        .filter(|p| matches!(p.category, ProcessCategory::Bloatware))
        .collect();
    if !bloatware.is_empty() {
        findings.push(Finding::new(
            FindingKind::Bloatware,
            Severity::Low,
            &lang.pick(format!("{} bloatware process(es) in the background", bloatware.len()), format!("{} Bloatware-Prozess(e) im Hintergrund", bloatware.len())),
            &lang.pick(
                "Known background software that uses resources without doing anything for you.",
                "Bekannte Hintergrundsoftware, die Ressourcen verbraucht, ohne etwas für dich zu tun.",
            ),
            &names(&bloatware),
            &lang.pick(
                "Uninstall software you do not need, or turn off its autostart.",
                "Deinstalliere nicht benötigte Software oder deaktiviere den Autostart.",
            ),
        ));
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Haelt die Einheit fest, in der `sysinfo` Speicher meldet.
    ///
    /// Der Code hier teilt `proc.memory()` durch 1024 * 1024 und nennt das
    /// Ergebnis MB. Wechselt eine neue `sysinfo`-Version auf Kilobyte, ist
    /// jeder Wert um Faktor 1024 zu klein, die Schwelle von 500 MB fuer
    /// speicherhungrige Prozesse loest nie mehr aus, und der Nutzer sieht
    /// einfach keine Befunde mehr. Nichts daran wuerde einen Compiler stoeren
    /// oder einen Fehler ausloesen.
    #[test]
    fn sysinfo_meldet_speicher_in_bytes() {
        let mut system = System::new_all();
        system.refresh_all();

        let gesamt = system.total_memory();
        assert!(
            gesamt > 1_000_000_000,
            "Gesamtspeicher {gesamt} ist zu klein fuer eine Byte-Angabe. \
             Bei Kilobyte laege der Wert etwa um Faktor 1024 darunter."
        );
        assert!(
            gesamt < 100_000_000_000_000,
            "Gesamtspeicher {gesamt} ist unplausibel gross"
        );
    }

    /// Der Analysepfad muss ueberhaupt Prozesse finden. Eine leere Liste waere
    /// kein Absturz, sondern eine leere Oberflaeche.
    #[test]
    fn die_prozessliste_ist_nicht_leer() {
        let mut system = System::new_all();
        system.refresh_all();
        assert!(!system.processes().is_empty());
    }
}
