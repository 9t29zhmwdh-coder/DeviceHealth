use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use super::prompts;
use crate::models::Lang;

/// Small enough for 8 GB of memory and fluent in German and English.
pub const DEFAULT_MODEL: &str = "qwen3.5:4b";

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
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap_or_default(),
        }
    }

    async fn generate(&self, prompt: &str) -> Result<String> {
        #[derive(Serialize)]
        struct Options { num_ctx: u32, temperature: f32 }
        // `think: false` keeps reasoning models from writing their thoughts into
        // the answer; without `num_ctx` Ollama reserves the model's full window.
        #[derive(Serialize)]
        struct Req<'a> { model: &'a str, prompt: &'a str, stream: bool, think: bool, options: Options }
        #[derive(Deserialize)]
        struct Resp { response: String }

        let resp = self.client
            .post(format!("{}/api/generate", self.base_url))
            .json(&Req {
                model: &self.model,
                prompt,
                stream: false,
                think: false,
                options: Options { num_ctx: 4096, temperature: 0.2 },
            })
            .send()
            .await
            .context("Ollama is not reachable")?
            .error_for_status()
            .context("Ollama refused the request; is the model installed?")?
            .json::<Resp>()
            .await?;

        Ok(resp.response.trim().to_string())
    }

    pub async fn explain_process(&self, lang: Lang, name: &str, description: Option<&str>, cpu: f32, memory_mb: f64) -> Result<String> {
        let prompt = prompts::explain_process(lang, name, description.unwrap_or("-"), cpu, memory_mb);
        self.generate(&prompt).await
    }

    pub async fn is_available(&self) -> bool {
        self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }
}
