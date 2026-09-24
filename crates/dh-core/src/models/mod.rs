pub mod finding;
pub mod hardware;
pub mod health;
pub mod process;
pub mod recommendation;
pub mod settings;

pub use finding::*;
pub use hardware::*;
pub use health::*;
pub use process::*;
pub use recommendation::*;
pub use settings::*;

use serde::{Deserialize, Serialize};

/// Language of the texts the analysis writes. The interface defaults to English;
/// findings used to be German only.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    #[default]
    En,
    De,
}

impl Lang {
    /// "de", "de-CH", "de_CH.UTF-8" → German; everything else English.
    pub fn from_code(code: &str) -> Self {
        if code.to_lowercase().starts_with("de") { Self::De } else { Self::En }
    }

    pub fn pick(self, en: impl Into<String>, de: impl Into<String>) -> String {
        match self {
            Self::En => en.into(),
            Self::De => de.into(),
        }
    }
}
