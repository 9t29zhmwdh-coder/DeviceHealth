use sysinfo::{Disk, NetworkData, System};
use crate::models::{
    finding::{Finding, FindingKind, Severity},
    hardware::{DiskInfo, NetworkStat, SystemInfo, ThermalInfo},
    settings::AppSettings,
};

pub fn build_system_info(sys: &System) -> SystemInfo {
    let cpu_brand = sys.cpus().first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unbekannt".to_string());
    let cpu_freq = sys.cpus().first()
        .map(|c| c.frequency())
        .unwrap_or(0);
    let cpu_per_core: Vec<f32> = sys.cpus().iter().map(|c| c.cpu_usage()).collect();

    SystemInfo {
        os_name: System::name().unwrap_or_else(|| "Unbekannt".to_string()),
        os_version: System::os_version().unwrap_or_else(|| "".to_string()),
        hostname: System::host_name().unwrap_or_else(|| "".to_string()),
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
        file_system: String::from_utf8_lossy(disk.file_system()).to_string(),
    }
}

pub fn build_thermal_info(comp: &sysinfo::Component) -> ThermalInfo {
    ThermalInfo {
        label: comp.label().to_string(),
        temperature_celsius: comp.temperature(),
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
) -> Vec<Finding> {
    let mut findings = Vec::new();

    let mem_pct = sys.memory_used_pct();
    if mem_pct > settings.memory_high_threshold {
        findings.push(Finding::new(
            FindingKind::HighMemoryUsage,
            if mem_pct > 95.0 { Severity::Critical } else { Severity::High },
            &format!("RAM-Auslastung: {:.1}%", mem_pct),
            &format!("{:.0} MB von {:.0} MB belegt.", sys.used_memory_bytes as f64 / 1024.0 / 1024.0, sys.total_memory_bytes as f64 / 1024.0 / 1024.0),
            "RAM",
            "Beende nicht benötigte Anwendungen. Ein Neustart kann helfen, Arbeitsspeicher freizugeben.",
        ));
    }

    for disk in disks {
        let pct = disk.used_pct();
        if pct > settings.disk_warning_threshold {
            findings.push(Finding::new(
                FindingKind::DiskNearlyFull,
                if pct > 95.0 { Severity::Critical } else if pct > 90.0 { Severity::High } else { Severity::Medium },
                &format!("Disk {} fast voll: {:.1}%", disk.name, pct),
                &format!("Datenträger '{}' bei {:.1}% Kapazität.", disk.name, pct),
                &disk.name,
                "Lösche nicht mehr benötigte Dateien oder verschiebe Daten auf externe Speicher.",
            ));
        }
    }

    for temp in temps.iter().filter(|t| t.is_hot()) {
        findings.push(Finding::new(
            FindingKind::HighTemperature,
            if temp.temperature_celsius > 90.0 { Severity::Critical } else { Severity::High },
            &format!("{}: {:.0}°C", temp.label, temp.temperature_celsius),
            &format!("Komponente '{}' hat eine ungewöhnlich hohe Temperatur.", temp.label),
            &temp.label,
            "Prüfe die Kühlung. Reinige Lüfter und stelle sicher, dass die Belüftung ausreicht.",
        ));
    }

    let uptime_days = sys.uptime_seconds / 86400;
    if uptime_days > 14 {
        findings.push(Finding::new(
            FindingKind::LongUptime,
            if uptime_days > 30 { Severity::Medium } else { Severity::Low },
            &format!("System läuft seit {} Tagen ohne Neustart", uptime_days),
            "Ein langer Betrieb ohne Neustart kann zu Speicherlecks, ausstehenden Updates und instabilem Verhalten führen.",
            "Betriebszeit",
            "Plane einen Neustart, um Systemressourcen freizugeben und Updates einzuspielen.",
        ));
    }

    findings
}
