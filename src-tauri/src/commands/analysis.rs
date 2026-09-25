use dh_core::{
    ai::ollama::OllamaBackend,
    analyzer::run_full_analysis,
    db::queries,
    history,
    models::{
        finding::Finding,
        health::HealthSnapshot,
        process::{AutostartEntry, ProcessEntry},
        recommendation::Recommendation,
        Lang,
    },
};
use tauri::{Emitter, State};
use crate::{error::DhResult, state::AppState};

#[tauri::command]
pub async fn run_analysis(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    lang: Option<String>,
) -> DhResult<HealthSnapshot> {
    let settings = state.settings.read().await.clone();
    let lang = lang.as_deref().map(Lang::from_code).unwrap_or_default();
    let pool = state.pool.clone();

    let _ = app.emit("analysis://started", ());

    let keep_days = settings.keep_history_days;
    let result = tokio::task::spawn_blocking(move || run_full_analysis(&settings, lang))
        .await
        .map_err(|e| crate::error::DhError::Other(e.to_string()))?;

    let snapshot = result.snapshot.clone();
    queries::insert_snapshot(&pool, &snapshot).await?;
    queries::insert_findings(&pool, &result.findings, &snapshot.id).await?;
    // "Keep history for N days" was a setting nothing acted on.
    history::cleanup_old_snapshots(&pool, keep_days).await?;

    *state.last_result.write().await = Some(result);

    let _ = app.emit("analysis://done", &snapshot);
    Ok(snapshot)
}

#[tauri::command]
pub async fn get_processes(state: State<'_, AppState>, show_safe: bool) -> DhResult<Vec<ProcessEntry>> {
    let result = state.last_result.read().await;
    let procs = result.as_ref().map(|r| {
        if show_safe {
            r.processes.clone()
        } else {
            r.processes.iter()
                // "Safe" covers catalogued safe processes too (trustd, Spotlight),
                // not only those classified as system.
                .filter(|p| !matches!(p.category, dh_core::models::process::ProcessCategory::System)
                    && !matches!(p.risk, dh_core::models::process::RiskLevel::Safe))
                .cloned()
                .collect()
        }
    }).unwrap_or_default();
    Ok(procs)
}

#[tauri::command]
pub async fn get_findings(state: State<'_, AppState>) -> DhResult<Vec<Finding>> {
    let result = state.last_result.read().await;
    Ok(result.as_ref().map(|r| r.findings.clone()).unwrap_or_default())
}

#[tauri::command]
pub async fn get_recommendations(state: State<'_, AppState>) -> DhResult<Vec<Recommendation>> {
    let result = state.last_result.read().await;
    Ok(result.as_ref().map(|r| r.recommendations.clone()).unwrap_or_default())
}

#[tauri::command]
pub async fn get_autostart(state: State<'_, AppState>) -> DhResult<Vec<AutostartEntry>> {
    let result = state.last_result.read().await;
    Ok(result.as_ref().map(|r| r.autostart.clone()).unwrap_or_default())
}

/// Quits a process the analysis offered for quitting. The PID must still belong
/// to the same program (PIDs are reused) and the process must not be part of
/// the system; anything else is refused.
#[tauri::command]
pub async fn quit_process(state: State<'_, AppState>, pid: u32, name: String) -> DhResult<()> {
    let allowed = state.last_result.read().await.as_ref()
        .map(|r| r.processes.iter().any(|p| p.pid == pid && p.name == name && p.can_disable))
        .unwrap_or(false);
    if !allowed {
        return Err(crate::error::DhError::Other(format!("{name} ({pid}) may not be quit")));
    }
    tokio::task::spawn_blocking(move || dh_core::analyzer::processes::quit(pid, &name))
        .await
        .map_err(|e| crate::error::DhError::Other(e.to_string()))?
        .map_err(crate::error::DhError::Other)
}

#[tauri::command]
pub async fn explain_process(
    state: State<'_, AppState>,
    name: String,
    description: Option<String>,
    cpu: f32,
    memory_mb: f64,
    lang: Option<String>,
) -> DhResult<String> {
    let settings = state.settings.read().await.clone();
    let pool = state.pool.clone();
    let lang = lang.as_deref().map(Lang::from_code).unwrap_or_default();
    // One cached answer per language and model; a German answer in the English
    // interface, or one from a model since replaced, was served forever.
    let cache_key = format!("{lang:?}:{name}");

    let cached = sqlx::query!(
        "SELECT explanation FROM ai_explanations WHERE process_name = ? AND model = ?", cache_key, settings.text_model
    )
    .fetch_optional(&pool)
    .await?;

    if let Some(row) = cached {
        return Ok(row.explanation);
    }

    let backend = OllamaBackend::new(&settings.ollama_url, &settings.text_model);
    if !backend.is_available().await {
        return Err(crate::error::DhError::Other(lang.pick(
            "Ollama is not running. Start it with: ollama serve",
            "Ollama läuft nicht. Starten mit: ollama serve",
        )));
    }

    let explanation = backend
        .explain_process(lang, &name, description.as_deref(), cpu, memory_mb)
        .await
        .map_err(|e| crate::error::DhError::Other(e.to_string()))?;

    let ts = chrono::Utc::now().timestamp();
    sqlx::query!(
        "INSERT OR REPLACE INTO ai_explanations(process_name, explanation, model, created_ts) VALUES(?,?,?,?)",
        cache_key, explanation, settings.text_model, ts
    )
    .execute(&pool)
    .await?;

    Ok(explanation)
}

#[tauri::command]
pub async fn check_ollama(state: State<'_, AppState>) -> DhResult<bool> {
    let settings = state.settings.read().await.clone();
    let backend = OllamaBackend::new(&settings.ollama_url, &settings.text_model);
    Ok(backend.is_available().await)
}

#[tauri::command]
pub async fn get_last_snapshot(state: State<'_, AppState>) -> DhResult<Option<HealthSnapshot>> {
    let result = state.last_result.read().await;
    Ok(result.as_ref().map(|r| r.snapshot.clone()))
}
