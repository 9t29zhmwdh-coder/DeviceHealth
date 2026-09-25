pub mod hardware;
pub mod known_processes;
pub mod network;
pub mod origin;
pub mod processes;
pub mod security;
pub mod services;

use sysinfo::{Components, Disks, Networks, System};

use crate::models::{
    finding::Finding,
    hardware::{DiskInfo, HardwareReport, NetworkStat, ThermalInfo},
    health::{FindingCounts, HealthGrade, HealthSnapshot, calculate_health_score},
    process::{AutostartEntry, ProcessCategory, ProcessEntry, RiskLevel},
    recommendation::{ActionKind, Recommendation},
    settings::AppSettings,
    Lang,
};

pub struct AnalysisResult {
    pub snapshot: HealthSnapshot,
    pub processes: Vec<ProcessEntry>,
    pub findings: Vec<Finding>,
    pub recommendations: Vec<Recommendation>,
    pub hardware: HardwareReport,
    pub autostart: Vec<AutostartEntry>,
}

/// CPU usage is a difference between two readings. A single refresh left every
/// process at 0 %, so no CPU spike was ever reported.
fn measured_system() -> System {
    let mut sys = System::new_all();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL.max(std::time::Duration::from_millis(500)));
    sys.refresh_all();
    sys
}

pub fn run_full_analysis(settings: &AppSettings, lang: Lang) -> AnalysisResult {
    let sys = measured_system();

    let disks = Disks::new_with_refreshed_list();
    let components = Components::new_with_refreshed_list();
    let networks = Networks::new_with_refreshed_list();

    let hw_system = hardware::build_system_info(&sys);
    let hw_disks: Vec<DiskInfo> = disks.list().iter().map(hardware::build_disk_info).collect();
    // Apple Silicon reports some sensors with nonsense such as -9201 °C.
    let hw_temps: Vec<ThermalInfo> = components.list().iter()
        .map(hardware::build_thermal_info)
        .filter(|t| (-40.0..=150.0).contains(&t.temperature_celsius))
        .collect();
    let hw_network: Vec<NetworkStat> = networks.iter().map(|(name, data)| hardware::build_network_stat(name, data)).collect();

    let processes = processes::analyze_processes(&sys, settings, lang);
    let autostart = services::get_autostart_entries();
    let mut findings = Vec::new();

    findings.extend(processes::detect_process_findings(&processes, settings, lang));
    findings.extend(hardware::detect_hardware_findings(&hw_system, &hw_disks, &hw_temps, settings, lang));
    findings.extend(network::detect_network_findings(&hw_network, &processes, lang));
    findings.extend(security::detect_security_findings(&processes, lang));
    findings.extend(services::detect_autostart_findings(&autostart, lang));

    findings.sort_by_key(|f| std::cmp::Reverse(f.severity.score_penalty()));

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

    let recommendations = build_recommendations(&processes, settings, uptime, lang);

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
        autostart,
    }
}

/// Processes worth quitting, largest first: telemetry, and apps that use a lot
/// of CPU or memory right now. System processes are never offered.
fn build_recommendations(processes: &[ProcessEntry], settings: &AppSettings, uptime_seconds: u64, lang: Lang) -> Vec<Recommendation> {
    let mut candidates: Vec<&ProcessEntry> = processes.iter()
        .filter(|p| p.can_disable && !matches!(p.category, ProcessCategory::System | ProcessCategory::Security))
        // A daemon of the operating system is restarted by it at once; offering
        // to quit it (coreduetd was offered) achieves nothing.
        .filter(|p| p.exe_path.as_deref().map(origin::origin_of) != Some(origin::Origin::OperatingSystem))
        .filter(|p| p.is_telemetry || p.cpu_usage > settings.cpu_spike_threshold || p.memory_mb() > 1024.0)
        .collect();
    candidates.sort_by_key(|p| std::cmp::Reverse(p.memory_bytes));

    let mut recs: Vec<Recommendation> = candidates.into_iter().take(10).map(|p| {
        let owner = p.vendor.as_deref().unwrap_or(&p.name);
        let why = if p.is_telemetry {
            lang.pick("sends usage data to its vendor", "sendet Nutzungsdaten an den Hersteller")
        } else {
            lang.pick(
                format!("uses {:.0}% CPU and {:.0} MB of memory", p.cpu_usage, p.memory_mb()),
                format!("braucht {:.0}% CPU und {:.0} MB Arbeitsspeicher", p.cpu_usage, p.memory_mb()),
            )
        };
        Recommendation::new(
            &lang.pick(format!("Quit {} ({owner})", p.name), format!("{} beenden ({owner})", p.name)),
            &lang.pick(
                format!("{} {why}. Unsaved work in it is lost; the app may start it again.", p.name),
                format!("{} {why}. Ungesicherte Arbeit darin geht verloren; die App kann ihn wieder starten.", p.name),
            ),
            ActionKind::KillProcess,
            &p.pid.to_string(),
            p.risk.clone(),
        )
    }).collect();

    if uptime_seconds > 14 * 86400 {
        recs.push(Recommendation::new(
            &lang.pick("Restart the computer", "Computer neu starten"),
            &lang.pick(
                "A restart installs pending updates and frees memory. The app does not do this for you.",
                "Ein Neustart spielt ausstehende Updates ein und gibt Speicher frei. Die App macht das nicht für dich.",
            ),
            ActionKind::NoAction,
            "",
            RiskLevel::Safe,
        ));
    }
    recs
}
