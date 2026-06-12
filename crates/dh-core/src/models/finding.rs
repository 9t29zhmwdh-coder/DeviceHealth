use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Critical => "Kritisch",
            Self::High     => "Hoch",
            Self::Medium   => "Mittel",
            Self::Low      => "Niedrig",
            Self::Info     => "Info",
        }
    }
    pub fn color(&self) -> &'static str {
        match self {
            Self::Critical => "#f85149",
            Self::High     => "#f0883e",
            Self::Medium   => "#d29922",
            Self::Low      => "#58a6ff",
            Self::Info     => "#8b949e",
        }
    }
    pub fn score_penalty(&self) -> i32 {
        match self {
            Self::Critical => 15,
            Self::High     => 10,
            Self::Medium   => 5,
            Self::Low      => 2,
            Self::Info     => 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FindingKind {
    HighCpuUsage,
    HighMemoryUsage,
    ZombieProcess,
    SuspiciousProcess,
    Bloatware,
    Telemetry,
    AutostartExcess,
    DiskNearlyFull,
    HighTemperature,
    LongUptime,
    SecurityRisk,
    UnknownProcess,
    NetworkTelemetry,
    CrashDetected,
    DriverIssue,
    MissingUpdate,
    RamLeak,
    OpenPort,
    WeakSecurity,
}

impl FindingKind {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::HighCpuUsage     => "🔥",
            Self::HighMemoryUsage  => "🐏",
            Self::ZombieProcess    => "🧟",
            Self::SuspiciousProcess => "🔍",
            Self::Bloatware        => "📦",
            Self::Telemetry        => "📡",
            Self::AutostartExcess  => "🚀",
            Self::DiskNearlyFull   => "💾",
            Self::HighTemperature  => "🌡️",
            Self::LongUptime       => "⏰",
            Self::SecurityRisk     => "🔒",
            Self::UnknownProcess   => "❓",
            Self::NetworkTelemetry => "🌐",
            Self::CrashDetected    => "💥",
            Self::DriverIssue      => "⚙️",
            Self::MissingUpdate    => "📥",
            Self::RamLeak          => "🧠",
            Self::OpenPort         => "🔓",
            Self::WeakSecurity     => "⚠️",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub kind: FindingKind,
    pub severity: Severity,
    pub title: String,
    pub description: String,
    pub affected_item: String,
    pub recommendation: String,
    pub can_auto_fix: bool,
    pub fix_action: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl Finding {
    pub fn new(kind: FindingKind, severity: Severity, title: &str, description: &str, affected: &str, rec: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            kind,
            severity,
            title: title.to_string(),
            description: description.to_string(),
            affected_item: affected.to_string(),
            recommendation: rec.to_string(),
            can_auto_fix: false,
            fix_action: None,
            timestamp: Utc::now(),
        }
    }
}
