use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::finding::{Finding, Severity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSnapshot {
    pub id: String,
    pub score: u8,
    pub grade: HealthGrade,
    pub cpu_usage: f32,
    pub memory_used_pct: f32,
    pub process_count: u32,
    pub finding_counts: FindingCounts,
    pub uptime_seconds: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthGrade {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

impl HealthGrade {
    pub fn from_score(score: u8) -> Self {
        match score {
            85..=100 => Self::Excellent,
            70..=84  => Self::Good,
            50..=69  => Self::Fair,
            30..=49  => Self::Poor,
            _        => Self::Critical,
        }
    }
    pub fn label(&self) -> &'static str {
        match self {
            Self::Excellent => "Ausgezeichnet",
            Self::Good      => "Gut",
            Self::Fair      => "Akzeptabel",
            Self::Poor      => "Schlecht",
            Self::Critical  => "Kritisch",
        }
    }
    pub fn color(&self) -> &'static str {
        match self {
            Self::Excellent => "#3fb950",
            Self::Good      => "#79c0ff",
            Self::Fair      => "#d29922",
            Self::Poor      => "#f0883e",
            Self::Critical  => "#f85149",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FindingCounts {
    pub critical: u32,
    pub high: u32,
    pub medium: u32,
    pub low: u32,
    pub info: u32,
}

impl FindingCounts {
    pub fn from_findings(findings: &[Finding]) -> Self {
        let mut c = Self::default();
        for f in findings {
            match f.severity {
                Severity::Critical => c.critical += 1,
                Severity::High     => c.high += 1,
                Severity::Medium   => c.medium += 1,
                Severity::Low      => c.low += 1,
                Severity::Info     => c.info += 1,
            }
        }
        c
    }
    pub fn total(&self) -> u32 {
        self.critical + self.high + self.medium + self.low + self.info
    }
}

pub fn calculate_health_score(
    findings: &[Finding],
    cpu_usage: f32,
    memory_used_pct: f32,
    uptime_seconds: u64,
    disk_max_pct: f32,
) -> u8 {
    let mut score = 100i32;

    for f in findings {
        score -= f.severity.score_penalty();
    }

    if cpu_usage > 90.0 { score -= 12; }
    else if cpu_usage > 75.0 { score -= 6; }

    if memory_used_pct > 92.0 { score -= 12; }
    else if memory_used_pct > 80.0 { score -= 6; }

    if disk_max_pct > 95.0 { score -= 15; }
    else if disk_max_pct > 85.0 { score -= 6; }

    let uptime_days = uptime_seconds / 86400;
    if uptime_days > 30 { score -= 10; }
    else if uptime_days > 14 { score -= 6; }
    else if uptime_days > 7  { score -= 3; }

    score.clamp(0, 100) as u8
}
