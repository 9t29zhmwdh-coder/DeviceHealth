use serde::{Deserialize, Serialize};
use crate::models::process::RiskLevel;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionKind {
    KillProcess,
    DisableAutostart,
    RestartSystem,
    FreeMemory,
    ClearCache,
    CheckDisk,
    OpenSettings,
    NoAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub action_kind: ActionKind,
    pub target: String,
    pub risk_to_system: RiskLevel,
    pub effort: u8,
    pub impact: u8,
    pub confirmed: bool,
    pub finding_id: Option<String>,
}

impl Recommendation {
    pub fn new(
        title: &str,
        description: &str,
        action: ActionKind,
        target: &str,
        risk: RiskLevel,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            description: description.to_string(),
            action_kind: action,
            target: target.to_string(),
            risk_to_system: risk,
            effort: 1,
            impact: 5,
            confirmed: false,
            finding_id: None,
        }
    }
}
