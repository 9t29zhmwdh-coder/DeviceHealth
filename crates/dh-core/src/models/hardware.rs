use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub hostname: String,
    pub cpu_brand: String,
    pub cpu_cores: usize,
    pub cpu_freq_mhz: u64,
    pub total_memory_bytes: u64,
    pub used_memory_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
    pub uptime_seconds: u64,
    pub boot_time: u64,
    pub cpu_usage_global: f32,
    pub cpu_per_core: Vec<f32>,
}

impl SystemInfo {
    pub fn memory_used_pct(&self) -> f32 {
        if self.total_memory_bytes == 0 { return 0.0; }
        (self.used_memory_bytes as f32 / self.total_memory_bytes as f32) * 100.0
    }
    pub fn uptime_human(&self) -> String {
        let s = self.uptime_seconds;
        let days = s / 86400;
        let hours = (s % 86400) / 3600;
        let mins = (s % 3600) / 60;
        if days > 0 { format!("{}d {}h {}m", days, hours, mins) }
        else if hours > 0 { format!("{}h {}m", hours, mins) }
        else { format!("{}m", mins) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub kind: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub file_system: String,
}

impl DiskInfo {
    pub fn used_bytes(&self) -> u64 {
        self.total_bytes.saturating_sub(self.available_bytes)
    }
    pub fn used_pct(&self) -> f32 {
        if self.total_bytes == 0 { return 0.0; }
        (self.used_bytes() as f32 / self.total_bytes as f32) * 100.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalInfo {
    pub label: String,
    pub temperature_celsius: f32,
    pub critical_threshold: Option<f32>,
}

impl ThermalInfo {
    pub fn is_hot(&self) -> bool {
        self.temperature_celsius > 80.0
            || self.critical_threshold.map(|t| self.temperature_celsius > t * 0.9).unwrap_or(false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStat {
    pub interface: String,
    pub bytes_received: u64,
    pub bytes_transmitted: u64,
    pub packets_received: u64,
    pub packets_transmitted: u64,
    pub errors_in: u64,
    pub errors_out: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareReport {
    pub system: SystemInfo,
    pub disks: Vec<DiskInfo>,
    pub temperatures: Vec<ThermalInfo>,
    pub network: Vec<NetworkStat>,
}
