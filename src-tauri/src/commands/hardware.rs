use dh_core::models::hardware::HardwareReport;
use tauri::State;
use crate::{error::DhResult, state::AppState};

#[tauri::command]
pub async fn get_hardware(state: State<'_, AppState>) -> DhResult<Option<HardwareReport>> {
    let result = state.last_result.read().await;
    Ok(result.as_ref().map(|r| r.hardware.clone()))
}
