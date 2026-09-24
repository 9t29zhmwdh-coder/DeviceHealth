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
    /// Belongs to an installed application; `vendor` holds the app's name.
    Application,
    Unknown,
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
