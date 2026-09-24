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
