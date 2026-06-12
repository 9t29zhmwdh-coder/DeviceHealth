use dh_core::{history, models::health::HealthSnapshot};
use tauri::State;
use crate::{error::DhResult, state::AppState};

#[tauri::command]
pub async fn get_history(state: State<'_, AppState>, days: Option<u32>) -> DhResult<Vec<HealthSnapshot>> {
    history::get_history(&state.pool, days.unwrap_or(30))
        .await
        .map_err(Into::into)
}

#[tauri::command]
pub async fn cleanup_history(state: State<'_, AppState>) -> DhResult<u32> {
    let keep = state.settings.read().await.keep_history_days;
    history::cleanup_old_snapshots(&state.pool, keep)
        .await
        .map_err(Into::into)
}
