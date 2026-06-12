use dh_core::{db::queries, models::settings::AppSettings};
use tauri::State;
use crate::{error::DhResult, state::AppState};

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> DhResult<AppSettings> {
    Ok(state.settings.read().await.clone())
}

#[tauri::command]
pub async fn save_settings(state: State<'_, AppState>, settings: AppSettings) -> DhResult<()> {
    queries::save_settings(&state.pool, &settings).await?;
    *state.settings.write().await = settings;
    Ok(())
}
