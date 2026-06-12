use crate::models::{
    finding::{Finding, FindingKind, Severity},
    hardware::SystemInfo,
    process::{ProcessEntry, ProcessCategory, RiskLevel},
};

pub fn detect_security_findings(processes: &[ProcessEntry], sys: &SystemInfo) -> Vec<Finding> {
    let mut findings = Vec::new();

    let high_risk: Vec<&ProcessEntry> = processes.iter()
        .filter(|p| matches!(p.risk, RiskLevel::Critical))
        .collect();

    if !high_risk.is_empty() {
        for proc in &high_risk {
            findings.push(Finding::new(
                FindingKind::SecurityRisk,
                Severity::Critical,
                &format!("Sicherheitsrisiko: {}", proc.name),
                &format!("Prozess '{}' wurde als kritisches Sicherheitsrisiko eingestuft.", proc.name),
                &proc.name,
                "Beende diesen Prozess sofort und führe einen Virenscan durch.",
            ));
        }
    }

    let unknown_high_cpu: Vec<&ProcessEntry> = processes.iter()
        .filter(|p| matches!(p.category, ProcessCategory::Unknown) && p.cpu_usage > 15.0)
        .collect();

    for proc in &unknown_high_cpu {
        findings.push(Finding::new(
            FindingKind::UnknownProcess,
            Severity::Medium,
            &format!("Unbekannter Prozess mit hoher CPU: {} ({:.1}%)", proc.name, proc.cpu_usage),
            &format!("Prozess '{}' ist unbekannt und verbraucht viele CPU-Ressourcen.", proc.name),
            &proc.name,
            "Recherchiere diesen Prozess. Bei Unklarheit: Task-Manager öffnen und Prozess beenden.",
        ));
    }

    let suspicious_names: Vec<&ProcessEntry> = processes.iter()
        .filter(|p| {
            let n = p.name.to_lowercase();
            n.contains("coinminer") || n.contains("miner") || n.contains("xmrig")
                || n.contains("cryptonight") || n.contains("ethminer")
        })
        .collect();

    for proc in &suspicious_names {
        findings.push(Finding::new(
            FindingKind::SecurityRisk,
            Severity::Critical,
            &format!("Möglicher Cryptominer: {}", proc.name),
            "Dieser Prozess entspricht bekannten Mustern von Kryptowährungs-Mining-Malware.",
            &proc.name,
            "Beende diesen Prozess SOFORT und führe einen vollständigen Virenscan durch.",
        ));
    }

    findings
}

pub fn get_open_ports() -> Vec<u16> {
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("netstat").args(["-an", "-p", "TCP"]).output()
    } else {
        std::process::Command::new("ss").args(["-tlnp"]).output()
    };

    let text = output.ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default();

    let re = once_cell::sync::Lazy::new(|| regex::Regex::new(r":(\d{2,5})\s").unwrap());
    re.find_iter(&text)
        .filter_map(|m| m.as_str().trim_matches(':').trim().parse::<u16>().ok())
        .collect()
}

mod once_cell {
    pub mod sync {
        pub use once_cell::sync::Lazy;
    }
}
mod regex {
    pub use regex::Regex;
}
