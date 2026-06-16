pub mod hardware;
pub mod known_processes;
pub mod network;
pub mod processes;
pub mod security;
pub mod services;

use sysinfo::{Components, Disks, Networks, System};

use crate::models::{
    finding::Finding,
    hardware::{DiskInfo, HardwareReport, NetworkStat, ThermalInfo},
    health::{FindingCounts, HealthGrade, HealthSnapshot, calculate_health_score},
    process::ProcessEntry,
    recommendation::Recommendation,
    settings::AppSettings,
};

pub struct AnalysisResult {
    pub snapshot: HealthSnapshot,
    pub processes: Vec<ProcessEntry>,
    pub findings: Vec<Finding>,
    pub recommendations: Vec<Recommendation>,
    pub hardware: HardwareReport,
}

pub fn run_full_analysis(settings: &AppSettings) -> AnalysisResult {
    let mut sys = System::new_all();
    sys.refresh_all();

    let disks = Disks::new_with_refreshed_list();
    let components = Components::new_with_refreshed_list();
    let networks = Networks::new_with_refreshed_list();

    let hw_system = hardware::build_system_info(&sys);
    let hw_disks: Vec<DiskInfo> = disks.list().iter().map(hardware::build_disk_info).collect();
    let hw_temps: Vec<ThermalInfo> = components.list().iter().map(hardware::build_thermal_info).collect();
    let hw_network: Vec<NetworkStat> = networks.iter().map(|(name, data)| hardware::build_network_stat(name, data)).collect();

    let processes = processes::analyze_processes(&sys, settings);
    let mut findings = Vec::new();

    findings.extend(processes::detect_process_findings(&processes, settings));
    findings.extend(hardware::detect_hardware_findings(&hw_system, &hw_disks, &hw_temps, settings));
    findings.extend(network::detect_network_findings(&hw_network, &processes));
    findings.extend(security::detect_security_findings(&processes, &hw_system));
    findings.extend(services::detect_autostart_findings());

    findings.sort_by(|a, b| b.severity.score_penalty().cmp(&a.severity.score_penalty()));

    let cpu = hw_system.cpu_usage_global;
    let mem_pct = hw_system.memory_used_pct();
    let disk_max = hw_disks.iter().map(|d| d.used_pct()).fold(0.0f32, f32::max);
    let uptime = hw_system.uptime_seconds;

    let score = calculate_health_score(&findings, cpu, mem_pct, uptime, disk_max);
    let grade = HealthGrade::from_score(score);
    let finding_counts = FindingCounts::from_findings(&findings);

    let snapshot = HealthSnapshot {
        id: uuid::Uuid::new_v4().to_string(),
        score,
        grade,
        cpu_usage: cpu,
        memory_used_pct: mem_pct,
        process_count: processes.len() as u32,
        finding_counts,
        uptime_seconds: uptime,
        timestamp: chrono::Utc::now(),
    };

    let recommendations = build_recommendations(&findings, &processes);

    AnalysisResult {
        snapshot,
        processes,
        findings,
        recommendations,
        hardware: HardwareReport {
            system: hw_system,
            disks: hw_disks,
            temperatures: hw_temps,
            network: hw_network,
        },
    }
}

fn build_recommendations(findings: &[Finding], processes: &[ProcessEntry]) -> Vec<Recommendation> {
    let mut recs = Vec::new();

    for finding in findings.iter().filter(|f| f.can_auto_fix) {
        use crate::models::recommendation::{ActionKind, Recommendation};
        use crate::models::process::RiskLevel;
        recs.push(Recommendation::new(
            &format!("Behebe: {}", finding.title),
            &finding.recommendation,
            ActionKind::NoAction,
            &finding.affected_item,
            RiskLevel::Low,
        ));
    }

    for proc in processes.iter().filter(|p| p.is_telemetry && p.can_disable) {
        use crate::models::recommendation::{ActionKind, Recommendation};
        use crate::models::process::RiskLevel;
        recs.push(Recommendation::new(
            &format!("Telemetrie beenden: {}", proc.name),
            proc.description.as_deref().unwrap_or("Bekannter Telemetrie-Prozess"),
            ActionKind::KillProcess,
            &proc.pid.to_string(),
            RiskLevel::Low,
        ));
    }

    recs
}
