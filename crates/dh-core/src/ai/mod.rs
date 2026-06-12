pub mod ollama;
pub mod prompts;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait AiBackend: Send + Sync {
    async fn explain_process(&self, name: &str, description: Option<&str>, cpu: f32, memory_mb: f64) -> Result<String>;
    async fn analyze_findings(&self, findings_summary: &str) -> Result<String>;
    async fn suggest_fix(&self, finding_title: &str, context: &str) -> Result<String>;
    async fn is_available(&self) -> bool;
}
