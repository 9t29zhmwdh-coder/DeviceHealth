use crate::models::{
    finding::{Finding, FindingKind, Severity},
    hardware::NetworkStat,
    process::ProcessEntry,
};

#[allow(dead_code)]
const KNOWN_TELEMETRY_PROCESSES: &[&str] = &[
    "CompatTelRunner", "DiagTrack", "dmwappushservice",
    "analyticspanetool", "coreduetd", "GoogleCrashHandler",
    "WerFault", "WerFaultSecure",
];

pub fn detect_network_findings(
    stats: &[NetworkStat],
    processes: &[ProcessEntry],
) -> Vec<Finding> {
    let mut findings = Vec::new();

    let telemetry_active: Vec<&ProcessEntry> = processes.iter()
        .filter(|p| p.is_telemetry && p.cpu_usage > 0.1)
        .collect();

    if !telemetry_active.is_empty() {
        let names: Vec<&str> = telemetry_active.iter().map(|p| p.name.as_str()).collect();
        findings.push(Finding::new(
            FindingKind::NetworkTelemetry,
            Severity::Low,
            &format!("{} aktive Telemetrie-Prozesse mit Netzwerkaktivität", telemetry_active.len()),
            "Einige Prozesse senden aktiv Daten an Drittanbieter-Server.",
            &names.join(", "),
            "Deaktiviere Telemetrie-Dienste in den System-Einstellungen oder blockiere sie per Firewall.",
        ));
    }

    let total_errors: u64 = stats.iter().map(|s| s.errors_in + s.errors_out).sum();
    if total_errors > 1000 {
        findings.push(Finding::new(
            FindingKind::NetworkTelemetry,
            Severity::Low,
            &format!("{} Netzwerkfehler erkannt", total_errors),
            "Ungewöhnlich viele Netzwerkfehler auf den Schnittstellen.",
            "Netzwerkinterface",
            "Prüfe die Netzwerkverbindung und Router-Konfiguration.",
        ));
    }

    findings
}

pub fn get_active_connections_text() -> Vec<String> {
    let cmd = if cfg!(target_os = "windows") {
        std::process::Command::new("netstat").args(["-n", "-o"]).output()
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("netstat").args(["-an"]).output()
    } else {
        std::process::Command::new("ss").args(["-tnp"]).output()
    };

    cmd.ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.lines().map(|l| l.to_string()).collect())
        .unwrap_or_default()
}
