use crate::models::{
    finding::{Finding, FindingKind, Severity},
    hardware::NetworkStat,
    process::ProcessEntry,
    Lang,
};

pub fn detect_network_findings(stats: &[NetworkStat], processes: &[ProcessEntry], lang: Lang) -> Vec<Finding> {
    let mut findings = Vec::new();

    let telemetry_active: Vec<&ProcessEntry> = processes.iter()
        .filter(|p| p.is_telemetry && p.cpu_usage > 0.1)
        .collect();
    if !telemetry_active.is_empty() {
        let names: Vec<&str> = telemetry_active.iter().map(|p| p.name.as_str()).collect();
        findings.push(Finding::new(
            FindingKind::NetworkTelemetry,
            Severity::Low,
            &lang.pick(
                format!("{} telemetry process(es) currently active", telemetry_active.len()),
                format!("{} Telemetrie-Prozess(e) gerade aktiv", telemetry_active.len()),
            ),
            &lang.pick(
                "Known telemetry processes are working right now and may be sending data to their vendor.",
                "Bekannte Telemetrie-Prozesse arbeiten gerade und senden möglicherweise Daten an den Hersteller.",
            ),
            &names.join(", "),
            &lang.pick(
                "Turn off diagnostics in the system settings or block them with a firewall.",
                "Deaktiviere Diagnosedaten in den Systemeinstellungen oder blockiere sie per Firewall.",
            ),
        ));
    }

    let total_errors: u64 = stats.iter().map(|s| s.errors_in + s.errors_out).sum();
    if total_errors > 1000 {
        findings.push(Finding::new(
            FindingKind::NetworkTelemetry,
            Severity::Low,
            &lang.pick(format!("{total_errors} network errors"), format!("{total_errors} Netzwerkfehler erkannt")),
            &lang.pick(
                "Unusually many errors on the network interfaces.",
                "Ungewöhnlich viele Fehler auf den Netzwerkschnittstellen.",
            ),
            &lang.pick("Network interface", "Netzwerkschnittstelle"),
            &lang.pick("Check the cable or Wi-Fi and the router.", "Prüfe Kabel oder WLAN und den Router."),
        ));
    }

    findings
}
