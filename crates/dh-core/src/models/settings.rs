use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub ollama_url: String,
    pub text_model: String,
    pub auto_scan_on_startup: bool,
    pub scan_interval_minutes: u32,
    pub show_safe_processes: bool,
    pub keep_history_days: u32,
    pub cpu_spike_threshold: f32,
    pub memory_high_threshold: f32,
    pub disk_warning_threshold: f32,
    pub temp_warning_celsius: f32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            ollama_url: "http://localhost:11434".to_string(),
            text_model: "llama3".to_string(),
            auto_scan_on_startup: true,
            scan_interval_minutes: 60,
            show_safe_processes: false,
            keep_history_days: 30,
            cpu_spike_threshold: 50.0,
            memory_high_threshold: 80.0,
            disk_warning_threshold: 85.0,
            temp_warning_celsius: 80.0,
        }
    }
}
