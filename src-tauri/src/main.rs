mod commands;
mod error;
mod state;

use dh_core::db;
use state::AppState;

#[tokio::main]
async fn main() {
    let db_path = db_path();
    let pool = db::open_db(&db_path).await.expect("DB konnte nicht geöffnet werden");
    let settings = dh_core::db::queries::load_settings(&pool)
        .await
        .unwrap_or_default();

    tauri::Builder::default()
        .manage(AppState::new(pool, settings))
        .invoke_handler(tauri::generate_handler![
            commands::analysis::run_analysis,
            commands::analysis::get_processes,
            commands::analysis::get_findings,
            commands::analysis::get_recommendations,
            commands::analysis::explain_process,
            commands::analysis::check_ollama,
            commands::analysis::get_last_snapshot,
            commands::hardware::get_hardware,
            commands::history::get_history,
            commands::history::cleanup_history,
            commands::settings::get_settings,
            commands::settings::save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("DeviceHealth konnte nicht gestartet werden");
}

fn db_path() -> String {
    // Jeder Zweig holt sich seine eigene Variable. Vorher stand `home` oben
    // fuer alle da, obwohl Windows `USERPROFILE` verwendet: unter Windows war
    // die Bindung damit ungenutzt und der Build scheiterte an -D warnings.
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    #[cfg(target_os = "macos")]
    return format!("{}/Library/Application Support/com.raystudio.devicehealth/devicehealth.db", home);
    #[cfg(target_os = "linux")]
    return format!("{}/.local/share/devicehealth/devicehealth.db", home);
    #[cfg(target_os = "windows")]
    return format!("{}\\AppData\\Local\\RayStudio\\DeviceHealth\\devicehealth.db",
        std::env::var("USERPROFILE").unwrap_or_else(|_| ".".to_string()));
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    return "devicehealth.db".to_string();
}
