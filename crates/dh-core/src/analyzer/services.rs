use crate::models::{
    finding::{Finding, FindingKind, Severity},
    process::{AutostartEntry, RiskLevel},
};

pub fn get_autostart_entries() -> Vec<AutostartEntry> {
    let mut entries = Vec::new();

    #[cfg(target_os = "macos")]
    {
        let dirs = vec![
            "/Library/LaunchAgents",
            "/Library/LaunchDaemons",
        ];
        if let Ok(home) = std::env::var("HOME") {
            entries.extend(scan_plist_dir(&format!("{}/Library/LaunchAgents", home)));
        }
        for dir in dirs {
            entries.extend(scan_plist_dir(dir));
        }
    }

    #[cfg(target_os = "linux")]
    {
        entries.extend(get_systemd_autostart());
    }

    entries
}

#[cfg(target_os = "macos")]
fn scan_plist_dir(dir: &str) -> Vec<AutostartEntry> {
    std::fs::read_dir(dir)
        .ok()
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map(|x| x == "plist").unwrap_or(false))
                .map(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    AutostartEntry {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: name.trim_end_matches(".plist").to_string(),
                        command: e.path().to_string_lossy().to_string(),
                        location: dir.to_string(),
                        risk: classify_autostart_risk(&name),
                        description: None,
                        can_disable: !dir.contains("LaunchDaemons"),
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(target_os = "linux")]
fn get_systemd_autostart() -> Vec<AutostartEntry> {
    let output = std::process::Command::new("systemctl")
        .args(["list-unit-files", "--state=enabled", "--no-pager", "--no-legend"])
        .output();

    output.ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| {
            s.lines()
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    parts.first().map(|name| AutostartEntry {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: name.trim_end_matches(".service").to_string(),
                        command: name.to_string(),
                        location: "systemd".to_string(),
                        risk: classify_autostart_risk(name),
                        description: None,
                        can_disable: !is_critical_service(name),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

pub fn detect_autostart_findings() -> Vec<Finding> {
    let entries = get_autostart_entries();
    let mut findings = Vec::new();

    let high_risk: Vec<&AutostartEntry> = entries.iter()
        .filter(|e| matches!(e.risk, RiskLevel::High | RiskLevel::Critical))
        .collect();

    if !high_risk.is_empty() {
        let names: Vec<&str> = high_risk.iter().map(|e| e.name.as_str()).collect();
        findings.push(Finding::new(
            FindingKind::AutostartExcess,
            Severity::Medium,
            &format!("{} verdächtige Autostart-Einträge", high_risk.len()),
            "Einige Autostart-Einträge sind unbekannt oder potenziell unerwünscht.",
            &names.join(", "),
            "Überprüfe die Autostart-Einträge und deaktiviere nicht benötigte.",
        ));
    }

    if entries.len() > 30 {
        findings.push(Finding::new(
            FindingKind::AutostartExcess,
            Severity::Low,
            &format!("{} Autostart-Einträge (viele)", entries.len()),
            "Sehr viele Autostart-Einträge können den Systemstart verlangsamen.",
            "Autostart",
            "Reduziere die Anzahl der Autostart-Programme auf das Notwendige.",
        ));
    }

    findings
}

fn classify_autostart_risk(name: &str) -> RiskLevel {
    let n = name.to_lowercase();
    if n.contains("miner") || n.contains("cryptonight") { return RiskLevel::Critical; }
    if n.contains("adware") || n.contains("spyware") { return RiskLevel::Critical; }
    if n.contains("google") || n.contains("apple") || n.contains("microsoft") { return RiskLevel::Low; }
    RiskLevel::Unknown
}

#[allow(dead_code)]
fn is_critical_service(name: &str) -> bool {
    let n = name.to_lowercase();
    matches!(n.as_str(),
        "network.service" | "systemd-resolved.service" | "dbus.service" |
        "sshd.service" | "cron.service" | "NetworkManager.service"
    )
}
