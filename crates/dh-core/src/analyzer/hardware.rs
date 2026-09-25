use sysinfo::{Disk, NetworkData, System};
use crate::models::{
    finding::{Finding, FindingKind, Severity},
    hardware::{DiskInfo, NetworkStat, SystemInfo, ThermalInfo},
    settings::AppSettings,
    Lang,
};

pub fn build_system_info(sys: &System) -> SystemInfo {
    let cpu_brand = sys.cpus().first()
        .map(|c| c.brand().to_string())
        .unwrap_or_default();
    let cpu_freq = sys.cpus().first()
        .map(|c| c.frequency())
        .unwrap_or(0);
    let cpu_per_core: Vec<f32> = sys.cpus().iter().map(|c| c.cpu_usage()).collect();

    SystemInfo {
        os_name: System::name().unwrap_or_else(|| "Unbekannt".to_string()),
        os_version: System::os_version().unwrap_or_default(),
        hostname: System::host_name().unwrap_or_default(),
        cpu_brand,
        cpu_cores: sys.cpus().len(),
        cpu_freq_mhz: cpu_freq,
        total_memory_bytes: sys.total_memory(),
        used_memory_bytes: sys.used_memory(),
        swap_total_bytes: sys.total_swap(),
        swap_used_bytes: sys.used_swap(),
        uptime_seconds: System::uptime(),
        boot_time: System::boot_time(),
        cpu_usage_global: sys.global_cpu_usage(),
        cpu_per_core,
    }
}

pub fn build_disk_info(disk: &Disk) -> DiskInfo {
    let kind = match disk.kind() {
        sysinfo::DiskKind::SSD     => "SSD",
        sysinfo::DiskKind::HDD     => "HDD",
        sysinfo::DiskKind::Unknown(_) => "Unbekannt",
    };
    DiskInfo {
        name: disk.name().to_string_lossy().to_string(),
        mount_point: disk.mount_point().to_string_lossy().to_string(),
        kind: kind.to_string(),
        total_bytes: disk.total_space(),
        available_bytes: disk.available_space(),
        file_system: disk.file_system().to_string_lossy().to_string(),
    }
}

pub fn build_thermal_info(comp: &sysinfo::Component) -> ThermalInfo {
    ThermalInfo {
        label: comp.label().to_string(),
        temperature_celsius: comp.temperature().unwrap_or(f32::NAN), // no reading: dropped by the plausibility filter, not shown as 0 °C
        critical_threshold: comp.critical(),
    }
}

pub fn build_network_stat(name: &str, data: &NetworkData) -> NetworkStat {
    NetworkStat {
        interface: name.to_string(),
        bytes_received: data.total_received(),
        bytes_transmitted: data.total_transmitted(),
        packets_received: data.total_packets_received(),
        packets_transmitted: data.total_packets_transmitted(),
        errors_in: data.total_errors_on_received(),
        errors_out: data.total_errors_on_transmitted(),
    }
}

pub fn detect_hardware_findings(
    sys: &SystemInfo,
    disks: &[DiskInfo],
    temps: &[ThermalInfo],
    settings: &AppSettings,
    lang: Lang,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    let mem_pct = sys.memory_used_pct();
    if mem_pct > settings.memory_high_threshold {
        let used = sys.used_memory_bytes as f64 / 1024.0 / 1024.0;
        let total = sys.total_memory_bytes as f64 / 1024.0 / 1024.0;
        findings.push(Finding::new(
            FindingKind::HighMemoryUsage,
            if mem_pct > 95.0 { Severity::Critical } else { Severity::High },
            &lang.pick(format!("Memory use: {mem_pct:.1}%"), format!("RAM-Auslastung: {mem_pct:.1}%")),
            &lang.pick(format!("{used:.0} MB of {total:.0} MB in use."), format!("{used:.0} MB von {total:.0} MB belegt.")),
            "RAM",
            &lang.pick(
                "Quit applications you do not need. A restart frees memory as well.",
                "Beende nicht benötigte Anwendungen. Ein Neustart gibt ebenfalls Arbeitsspeicher frei.",
            ),
        ));
    }

    for disk in disks {
        let pct = disk.used_pct();
        if pct > settings.disk_warning_threshold {
            findings.push(Finding::new(
                FindingKind::DiskNearlyFull,
                if pct > 95.0 { Severity::Critical } else if pct > 90.0 { Severity::High } else { Severity::Medium },
                &lang.pick(format!("Disk {} nearly full: {pct:.1}%", disk.name), format!("Datenträger {} fast voll: {pct:.1}%", disk.name)),
                &lang.pick(format!("'{}' is at {pct:.1}% of its capacity.", disk.name), format!("Datenträger '{}' bei {pct:.1}% Kapazität.", disk.name)),
                &disk.name,
                &lang.pick(
                    "Delete files you no longer need or move data to external storage.",
                    "Lösche nicht mehr benötigte Dateien oder verschiebe Daten auf externe Speicher.",
                ),
            ));
        }
    }

    for temp in temps.iter().filter(|t| t.is_hot(settings.temp_warning_celsius)) {
        findings.push(Finding::new(
            FindingKind::HighTemperature,
            if temp.temperature_celsius > settings.temp_warning_celsius + 10.0 { Severity::Critical } else { Severity::High },
            &format!("{}: {:.0}°C", temp.label, temp.temperature_celsius),
            &lang.pick(
                format!("'{}' is running unusually hot.", temp.label),
                format!("Komponente '{}' hat eine ungewöhnlich hohe Temperatur.", temp.label),
            ),
            &temp.label,
            &lang.pick(
                "Check the cooling: clean the fans and keep the vents free.",
                "Prüfe die Kühlung: Lüfter reinigen und Lüftungsschlitze freihalten.",
            ),
        ));
    }

    let uptime_days = sys.uptime_seconds / 86400;
    if uptime_days > 14 {
        findings.push(Finding::new(
            FindingKind::LongUptime,
            if uptime_days > 30 { Severity::Medium } else { Severity::Low },
            &lang.pick(format!("Running for {uptime_days} days without a restart"), format!("System läuft seit {uptime_days} Tagen ohne Neustart")),
            &lang.pick(
                "Long uptimes leave pending updates uninstalled and let memory fragment.",
                "Lange Laufzeiten lassen ausstehende Updates liegen und zerstückeln den Arbeitsspeicher.",
            ),
            &lang.pick("Uptime", "Betriebszeit"),
            &lang.pick("Plan a restart to install updates and free resources.", "Plane einen Neustart, um Updates einzuspielen und Ressourcen freizugeben."),
        ));
    }

    findings
}
