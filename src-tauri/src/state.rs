use dh_core::models::settings::AppSettings;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub pool: SqlitePool,
    pub settings: Arc<RwLock<AppSettings>>,
    pub last_result: Arc<RwLock<Option<dh_core::analyzer::AnalysisResult>>>,
}

impl AppState {
    pub fn new(pool: SqlitePool, settings: AppSettings) -> Self {
        Self {
            pool,
            settings: Arc::new(RwLock::new(settings)),
            last_result: Arc::new(RwLock::new(None)),
        }
    }
}
