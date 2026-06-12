use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DhError {
    #[error("DB error: {0}")]   Db(#[from] sqlx::Error),
    #[error("IO error: {0}")]   Io(#[from] std::io::Error),
    #[error("{0}")]             Other(String),
}

impl From<anyhow::Error> for DhError {
    fn from(e: anyhow::Error) -> Self { DhError::Other(e.to_string()) }
}

impl Serialize for DhError {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

pub type DhResult<T> = Result<T, DhError>;
