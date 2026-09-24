use crate::models::{
    finding::{Finding, FindingKind, Severity},
    process::{ProcessEntry, ProcessCategory, RiskLevel},
    Lang,
};

/// Whole name parts only: "miner" inside "examiner" is not a cryptominer.
fn looks_like_miner(name: &str) -> bool {
    const MINERS: &[&str] = &["coinminer", "miner", "xmrig", "cryptonight", "ethminer"];
    name.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .any(|part| MINERS.contains(&part))
}

pub fn detect_security_findings(processes: &[ProcessEntry], lang: Lang) -> Vec<Finding> {
    let mut findings = Vec::new();

    for proc in processes.iter().filter(|p| matches!(p.risk, RiskLevel::Critical)) {
        findings.push(Finding::new(
            FindingKind::SecurityRisk,
            Severity::Critical,
            &lang.pick(format!("Security risk: {}", proc.name), format!("Sicherheitsrisiko: {}", proc.name)),
            &lang.pick(
                format!("'{}' is rated a critical security risk.", proc.name),
                format!("Prozess '{}' wurde als kritisches Sicherheitsrisiko eingestuft.", proc.name),
            ),
            &proc.name,
            &lang.pick("Quit it and run a malware scan.", "Beende ihn und führe einen Malware-Scan durch."),
        ));
    }

    let unknown_busy = processes.iter()
        .filter(|p| matches!(p.category, ProcessCategory::Unknown) && p.cpu_usage > 15.0);
    for proc in unknown_busy {
        findings.push(Finding::new(
            FindingKind::UnknownProcess,
            Severity::Medium,
            &lang.pick(
                format!("Unknown process with high CPU: {} ({:.1}%)", proc.name, proc.cpu_usage),
                format!("Unbekannter Prozess mit hoher CPU: {} ({:.1}%)", proc.name, proc.cpu_usage),
            ),
            &lang.pick(
                format!("'{}' belongs to no known app and uses a lot of CPU.", proc.name),
                format!("Prozess '{}' gehört zu keiner bekannten App und verbraucht viel CPU.", proc.name),
            ),
            &proc.name,
            &lang.pick(
                "Look up where its file lives. If you cannot place it, quit it and watch whether it returns.",
                "Prüfe, wo seine Datei liegt. Kannst du ihn nicht zuordnen, beende ihn und beobachte, ob er wiederkommt.",
            ),
        ));
    }

    for proc in processes.iter().filter(|p| looks_like_miner(&p.name)) {
        findings.push(Finding::new(
            FindingKind::SecurityRisk,
            Severity::Critical,
            &lang.pick(format!("Possible cryptominer: {}", proc.name), format!("Möglicher Cryptominer: {}", proc.name)),
            &lang.pick(
                "The name matches known cryptocurrency mining malware.",
                "Der Name entspricht bekannter Kryptowährungs-Mining-Malware.",
            ),
            &proc.name,
            &lang.pick("Quit it now and run a full malware scan.", "Beende ihn sofort und führe einen vollständigen Malware-Scan durch."),
        ));
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::looks_like_miner;

    #[test]
    fn miner_names_match_whole_parts_only() {
        assert!(looks_like_miner("xmrig"));
        assert!(looks_like_miner("coin-miner"));
        assert!(!looks_like_miner("QuickLookExaminer"));
        assert!(!looks_like_miner("determiner"));
    }
}
