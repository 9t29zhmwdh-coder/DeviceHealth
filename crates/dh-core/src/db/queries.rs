use anyhow::Result;
use chrono::Utc;
use sqlx::SqlitePool;

use crate::models::{
    finding::{Finding, FindingKind, Severity},
    health::{FindingCounts, HealthGrade, HealthSnapshot},
    settings::AppSettings,
};

pub async fn insert_snapshot(pool: &SqlitePool, s: &HealthSnapshot) -> Result<()> {
    let grade = format!("{:?}", s.grade);
    let ts = s.timestamp.timestamp();
    sqlx::query!(
        "INSERT OR REPLACE INTO health_snapshots(id, score, grade, cpu_usage, memory_used_pct,
         process_count, critical, high, medium, low, info, uptime_seconds, timestamp_ts)
         VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?)",
        s.id, s.score as i64, grade, s.cpu_usage, s.memory_used_pct,
        s.process_count as i64,
        s.finding_counts.critical as i64, s.finding_counts.high as i64,
        s.finding_counts.medium as i64, s.finding_counts.low as i64,
        s.finding_counts.info as i64, s.uptime_seconds as i64, ts
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_snapshots(pool: &SqlitePool, days: u32) -> Result<Vec<HealthSnapshot>> {
    let cutoff = Utc::now().timestamp() - (days as i64 * 86400);
    let rows = sqlx::query!(
        "SELECT id, score, grade, cpu_usage, memory_used_pct, process_count,
         critical, high, medium, low, info, uptime_seconds, timestamp_ts
         FROM health_snapshots WHERE timestamp_ts > ? ORDER BY timestamp_ts DESC",
        cutoff
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().filter_map(|r| {
        let grade = match r.grade.as_str() {
            "Excellent" => HealthGrade::Excellent,
            "Good"      => HealthGrade::Good,
            "Fair"      => HealthGrade::Fair,
            "Poor"      => HealthGrade::Poor,
            _           => HealthGrade::Critical,
        };
        Some(HealthSnapshot {
            id: r.id,
            score: r.score as u8,
            grade,
            cpu_usage: r.cpu_usage,
            memory_used_pct: r.memory_used_pct,
            process_count: r.process_count as u32,
            finding_counts: FindingCounts {
                critical: r.critical as u32,
                high: r.high as u32,
                medium: r.medium as u32,
                low: r.low as u32,
                info: r.info as u32,
            },
            uptime_seconds: r.uptime_seconds as u64,
            timestamp: chrono::DateTime::from_timestamp(r.timestamp_ts, 0)?,
        })
    }).collect())
}

pub async fn insert_findings(pool: &SqlitePool, findings: &[Finding], snapshot_id: &str) -> Result<()> {
    for f in findings {
        let kind = format!("{:?}", f.kind);
        let severity = format!("{:?}", f.severity);
        let ts = f.timestamp.timestamp();
        sqlx::query!(
            "INSERT OR IGNORE INTO findings(id, snapshot_id, kind, severity, title, description, affected_item, recommendation, timestamp_ts)
             VALUES(?,?,?,?,?,?,?,?,?)",
            f.id, snapshot_id, kind, severity, f.title, f.description, f.affected_item, f.recommendation, ts
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn get_findings_for_snapshot(pool: &SqlitePool, snapshot_id: &str) -> Result<Vec<Finding>> {
    let rows = sqlx::query!(
        "SELECT id, kind, severity, title, description, affected_item, recommendation, timestamp_ts
         FROM findings WHERE snapshot_id = ? ORDER BY timestamp_ts DESC",
        snapshot_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().filter_map(|r| {
        let severity = match r.severity.as_str() {
            "Critical" => Severity::Critical,
            "High"     => Severity::High,
            "Medium"   => Severity::Medium,
            "Low"      => Severity::Low,
            _          => Severity::Info,
        };
        Some(Finding {
            id: r.id,
            kind: FindingKind::SecurityRisk,
            severity,
            title: r.title,
            description: r.description,
            affected_item: r.affected_item,
            recommendation: r.recommendation,
            can_auto_fix: false,
            fix_action: None,
            timestamp: chrono::DateTime::from_timestamp(r.timestamp_ts, 0)?,
        })
    }).collect())
}

pub async fn load_settings(pool: &SqlitePool) -> Result<AppSettings> {
    let row = sqlx::query!("SELECT value FROM app_settings WHERE key = 'settings'")
        .fetch_optional(pool)
        .await?;
    Ok(row
        .and_then(|r| serde_json::from_str(&r.value).ok())
        .unwrap_or_default())
}

pub async fn save_settings(pool: &SqlitePool, s: &AppSettings) -> Result<()> {
    let json = serde_json::to_string(s)?;
    sqlx::query!(
        "INSERT OR REPLACE INTO app_settings(key, value) VALUES('settings', ?)",
        json
    )
    .execute(pool)
    .await?;
    Ok(())
}
