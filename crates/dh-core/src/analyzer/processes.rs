use sysinfo::{ProcessStatus, System};
use crate::models::{
    finding::{Finding, FindingKind, Severity},
    process::{ProcessCategory, ProcessEntry, RiskLevel},
    settings::AppSettings,
};
use super::known_processes;

pub fn analyze_processes(sys: &System, settings: &AppSettings) -> Vec<ProcessEntry> {
    sys.processes()
        .iter()
        .map(|(pid, proc)| {
            let name = proc.name().to_string_lossy().to_string();
            let is_zombie = matches!(proc.status(), ProcessStatus::Zombie);

            let known = known_processes::lookup(&name);
            let risk = if is_zombie {
                RiskLevel::Medium
            } else if known_processes::is_suspicious_name(&name) {
                RiskLevel::High
            } else {
                known.map(|k| k.risk.clone()).unwrap_or(RiskLevel::Unknown)
            };

            let category = if is_zombie {
                ProcessCategory::Zombie
            } else {
                known.map(|k| k.category.clone()).unwrap_or(ProcessCategory::Unknown)
            };

            let is_telemetry = known.map(|k| k.is_telemetry).unwrap_or(false);
            let can_disable = known.map(|k| k.can_disable).unwrap_or(false);

            let mut flags = Vec::new();
            if proc.cpu_usage() > settings.cpu_spike_threshold { flags.push("high-cpu".to_string()); }
            let mem_mb = proc.memory() as f64 / 1024.0 / 1024.0;
            if mem_mb > 500.0 { flags.push("high-memory".to_string()); }
            if is_zombie { flags.push("zombie".to_string()); }
            if is_telemetry { flags.push("telemetry".to_string()); }

            ProcessEntry {
                pid: pid.as_u32(),
                name: name.clone(),
                exe_path: proc.exe().map(|p| p.to_string_lossy().to_string()),
                cpu_usage: proc.cpu_usage(),
                memory_bytes: proc.memory(),
                status: format!("{:?}", proc.status()),
                is_zombie,
                user: proc.user_id().map(|u| u.to_string()),
                risk,
                category,
                description: known.map(|k| k.description.to_string()),
                vendor: known.map(|k| k.vendor.to_string()),
                can_disable,
                is_telemetry,
                flags,
            }
        })
        .collect()
}

pub fn detect_process_findings(processes: &[ProcessEntry], settings: &AppSettings) -> Vec<Finding> {
    let mut findings = Vec::new();

    let zombies: Vec<&ProcessEntry> = processes.iter().filter(|p| p.is_zombie).collect();
    if !zombies.is_empty() {
        findings.push(Finding::new(
            FindingKind::ZombieProcess,
            Severity::Medium,
            &format!("{} Zombie-Prozess(e) erkannt", zombies.len()),
            "Zombie-Prozesse haben ihren Ausführungszyklus beendet, belegen aber noch Einträge in der Prozessliste.",
            &zombies.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", "),
            "Starte das System neu, um Zombie-Prozesse zu bereinigen.",
        ));
    }

    let high_cpu: Vec<&ProcessEntry> = processes.iter()
        .filter(|p| p.cpu_usage > settings.cpu_spike_threshold && !matches!(p.category, ProcessCategory::System))
        .collect();
    for proc in &high_cpu {
        findings.push(Finding::new(
            FindingKind::HighCpuUsage,
            if proc.cpu_usage > 80.0 { Severity::High } else { Severity::Medium },
            &format!("{}: {:.1}% CPU", proc.name, proc.cpu_usage),
            &format!("Prozess '{}' verbraucht dauerhaft viel CPU-Leistung.", proc.name),
            &proc.name,
            &format!("Prüfe, ob '{}' notwendig ist. Ggf. beenden oder neu starten.", proc.name),
        ));
    }

    let high_mem: Vec<&ProcessEntry> = processes.iter()
        .filter(|p| p.memory_mb() > 500.0 && matches!(p.category, ProcessCategory::Unknown | ProcessCategory::Bloatware))
        .collect();
    for proc in &high_mem {
        findings.push(Finding::new(
            FindingKind::HighMemoryUsage,
            Severity::Medium,
            &format!("{}: {:.0} MB RAM", proc.name, proc.memory_mb()),
            &format!("Prozess '{}' belegt ungewöhnlich viel Arbeitsspeicher.", proc.name),
            &proc.name,
            "Starte den Prozess neu oder deinstalliere ihn, wenn er nicht benötigt wird.",
        ));
    }

    let telemetry: Vec<&ProcessEntry> = processes.iter().filter(|p| p.is_telemetry).collect();
    if !telemetry.is_empty() {
        let names: Vec<&str> = telemetry.iter().map(|p| p.name.as_str()).collect();
        findings.push(Finding::new(
            FindingKind::Telemetry,
            Severity::Low,
            &format!("{} Telemetrie-Prozess(e) aktiv", telemetry.len()),
            "Einige aktive Prozesse senden Diagnosedaten und Nutzungsstatistiken an Dritte.",
            &names.join(", "),
            "Deaktiviere unnötige Telemetrie-Dienste in den System-Einstellungen.",
        ));
    }

    let suspicious: Vec<&ProcessEntry> = processes.iter()
        .filter(|p| matches!(p.risk, RiskLevel::High | RiskLevel::Critical) && !p.is_zombie)
        .collect();
    for proc in &suspicious {
        findings.push(Finding::new(
            FindingKind::SuspiciousProcess,
            Severity::High,
            &format!("Verdächtiger Prozess: {}", proc.name),
            &format!("Prozess '{}' hat einen ungewöhnlichen Namen oder ist nicht als sicher bekannt.", proc.name),
            &proc.name,
            "Überprüfe diesen Prozess manuell. Handelt es sich um Malware, beende ihn sofort.",
        ));
    }

    let bloatware: Vec<&ProcessEntry> = processes.iter()
        .filter(|p| matches!(p.category, ProcessCategory::Bloatware))
        .collect();
    if !bloatware.is_empty() {
        let names: Vec<&str> = bloatware.iter().map(|p| p.name.as_str()).collect();
        findings.push(Finding::new(
            FindingKind::Bloatware,
            Severity::Low,
            &format!("{} Bloatware-Prozess(e) im Hintergrund", bloatware.len()),
            "Einige aktive Prozesse sind bekannte Bloatware, die Ressourcen verbrauchen ohne Mehrwert.",
            &names.join(", "),
            "Deinstalliere nicht benötigte Software oder deaktiviere den Autostart.",
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
