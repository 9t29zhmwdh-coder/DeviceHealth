use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Safe,
    Low,
    Medium,
    High,
    Critical,
    Unknown,
}

impl RiskLevel {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Safe     => "Sicher",
            Self::Low      => "Niedrig",
            Self::Medium   => "Mittel",
            Self::High     => "Hoch",
            Self::Critical => "Kritisch",
            Self::Unknown  => "Unbekannt",
        }
    }
    pub fn color(&self) -> &'static str {
        match self {
            Self::Safe     => "#3fb950",
            Self::Low      => "#79c0ff",
            Self::Medium   => "#d29922",
            Self::High     => "#f0883e",
            Self::Critical => "#f85149",
            Self::Unknown  => "#8b949e",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProcessCategory {
    System,
    Security,
    Browser,
    Utility,
    Telemetry,
    Bloatware,
    Gaming,
    Development,
    Media,
    Network,
    Zombie,
    Unknown,
}

impl ProcessCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::System      => "System",
            Self::Security    => "Sicherheit",
            Self::Browser     => "Browser",
            Self::Utility     => "Dienstprogramm",
            Self::Telemetry   => "Telemetrie",
            Self::Bloatware   => "Bloatware",
            Self::Gaming      => "Gaming",
            Self::Development => "Entwicklung",
            Self::Media       => "Medien",
            Self::Network     => "Netzwerk",
            Self::Zombie      => "Zombie",
            Self::Unknown     => "Unbekannt",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessEntry {
    pub pid: u32,
    pub name: String,
    pub exe_path: Option<String>,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub status: String,
    pub is_zombie: bool,
    pub user: Option<String>,
    pub risk: RiskLevel,
    pub category: ProcessCategory,
    pub description: Option<String>,
    pub vendor: Option<String>,
    pub can_disable: bool,
    pub is_telemetry: bool,
    pub flags: Vec<String>,
}

impl ProcessEntry {
    pub fn memory_mb(&self) -> f64 {
        self.memory_bytes as f64 / 1024.0 / 1024.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutostartEntry {
    pub id: String,
    pub name: String,
    pub command: String,
    pub location: String,
    pub risk: RiskLevel,
    pub description: Option<String>,
    pub can_disable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnection {
    pub protocol: String,
    pub local_addr: String,
    pub remote_addr: String,
    pub state: String,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
    pub is_telemetry: bool,
    pub remote_host: Option<String>,
}
