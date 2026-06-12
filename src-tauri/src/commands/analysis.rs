use dh_core::{
    ai::ollama::OllamaBackend,
    ai::AiBackend,
    analyzer::{run_full_analysis, AnalysisResult},
    db::queries,
    history,
    models::{
        finding::Finding,
        health::HealthSnapshot,
        process::ProcessEntry,
        recommendation::Recommendation,
    },
};
use tauri::State;
use crate::{error::DhResult, state::AppState};

#[tauri::command]
pub async fn run_analysis(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> DhResult<HealthSnapshot> {
    let settings = state.settings.read().await.clone();
    let pool = state.pool.clone();

    let _ = app.emit("analysis://started", ());

    let result = tokio::task::spawn_blocking(move || run_full_analysis(&settings))
        .await
        .map_err(|e| crate::error::DhError::Other(e.to_string()))?;

    let snapshot = result.snapshot.clone();
    queries::insert_snapshot(&pool, &snapshot).await?;
    queries::insert_findings(&pool, &result.findings, &snapshot.id).await?;

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
                .filter(|p| !matches!(p.category,
                    dh_core::models::process::ProcessCategory::System))
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
pub async fn explain_process(
    state: State<'_, AppState>,
    name: String,
    description: Option<String>,
    cpu: f32,
    memory_mb: f64,
) -> DhResult<String> {
    let settings = state.settings.read().await.clone();
    let pool = state.pool.clone();

    let cached = sqlx::query!(
        "SELECT explanation FROM ai_explanations WHERE process_name = ?", name
    )
    .fetch_optional(&pool)
    .await?;

    if let Some(row) = cached {
        return Ok(row.explanation);
    }

    let backend = OllamaBackend::new(&settings.ollama_url, &settings.text_model);
    if !backend.is_available().await {
        return Ok(description.unwrap_or_else(|| "Keine KI verfügbar. Ollama starten: ollama serve".to_string()));
    }

    let explanation = backend
        .explain_process(&name, description.as_deref(), cpu, memory_mb)
        .await
        .map_err(|e| crate::error::DhError::Other(e.to_string()))?;

    let ts = chrono::Utc::now().timestamp();
    sqlx::query!(
        "INSERT OR REPLACE INTO ai_explanations(process_name, explanation, model, created_ts) VALUES(?,?,?,?)",
        name, explanation, settings.text_model, ts
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
