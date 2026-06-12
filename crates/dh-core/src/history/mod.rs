use anyhow::Result;
use sqlx::SqlitePool;
use crate::db::queries;
use crate::models::health::HealthSnapshot;

pub async fn save_snapshot(pool: &SqlitePool, snapshot: &HealthSnapshot) -> Result<()> {
    queries::insert_snapshot(pool, snapshot).await
}

pub async fn get_history(pool: &SqlitePool, days: u32) -> Result<Vec<HealthSnapshot>> {
    queries::get_snapshots(pool, days).await
}

pub async fn cleanup_old_snapshots(pool: &SqlitePool, keep_days: u32) -> Result<u32> {
    let cutoff = chrono::Utc::now().timestamp() - (keep_days as i64 * 86400);
    let rows = sqlx::query!(
        "DELETE FROM health_snapshots WHERE timestamp_ts < ?",
        cutoff
    )
    .execute(pool)
    .await?;
    Ok(rows.rows_affected() as u32)
}
