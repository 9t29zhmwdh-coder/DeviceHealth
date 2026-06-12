use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use super::{AiBackend, prompts};

pub struct OllamaBackend {
    pub base_url: String,
    pub model: String,
    client: Client,
}

impl OllamaBackend {
    pub fn new(base_url: &str, model: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            model: model.to_string(),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(60))
                .build()
                .unwrap_or_default(),
        }
    }

    async fn generate(&self, prompt: &str) -> Result<String> {
        #[derive(Serialize)]
        struct Req<'a> { model: &'a str, prompt: &'a str, stream: bool }
        #[derive(Deserialize)]
        struct Resp { response: String }

        let resp = self.client
            .post(format!("{}/api/generate", self.base_url))
            .json(&Req { model: &self.model, prompt, stream: false })
            .send()
            .await
            .context("Ollama nicht erreichbar")?
            .json::<Resp>()
            .await?;

        Ok(resp.response.trim().to_string())
    }
}

#[async_trait]
impl AiBackend for OllamaBackend {
    async fn explain_process(&self, name: &str, description: Option<&str>, cpu: f32, memory_mb: f64) -> Result<String> {
        let prompt = prompts::EXPLAIN_PROCESS
            .replace("{name}", name)
            .replace("{description}", description.unwrap_or("Keine bekannte Beschreibung"))
            .replace("{cpu}", &format!("{:.1}", cpu))
            .replace("{memory_mb}", &format!("{:.0}", memory_mb));
        self.generate(&prompt).await
    }

    async fn analyze_findings(&self, findings_summary: &str) -> Result<String> {
        let prompt = prompts::ANALYZE_FINDINGS.replace("{findings}", findings_summary);
        self.generate(&prompt).await
    }

    async fn suggest_fix(&self, title: &str, context: &str) -> Result<String> {
        let prompt = prompts::SUGGEST_FIX
            .replace("{title}", title)
            .replace("{context}", context);
        self.generate(&prompt).await
    }

    async fn is_available(&self) -> bool {
        self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }
}
