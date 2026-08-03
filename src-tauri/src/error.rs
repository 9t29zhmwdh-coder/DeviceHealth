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

#[cfg(test)]
mod tests {
    use super::*;

    /// Haelt die Fehlertexte fest. Sie sind nicht intern: `Serialize` reicht
    /// genau diesen String an das Frontend weiter, wo er im Fenster landet.
    /// Ein Versionssprung von `thiserror` darf die Formatierung deshalb nicht
    /// stillschweigend veraendern.
    #[test]
    fn fehlertexte_bleiben_wie_sie_sind() {
        let io = DhError::Io(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "kein Zugriff",
        ));
        assert_eq!(io.to_string(), "IO error: kein Zugriff");

        let db = DhError::Db(sqlx::Error::RowNotFound);
        assert_eq!(
            db.to_string(),
            "DB error: no rows returned by a query that expected to return at least one row"
        );

        let other = DhError::Other("nur der Text".into());
        assert_eq!(other.to_string(), "nur der Text");
    }

    /// anyhow kommt nicht ueber #[from], sondern ueber ein eigenes From, das
    /// die Meldung flach uebernimmt. Auch das gehoert festgehalten.
    #[test]
    fn anyhow_wird_flach_uebernommen() {
        let fehler: DhError = anyhow::anyhow!("etwas ging schief").into();
        assert_eq!(fehler.to_string(), "etwas ging schief");
    }

    #[test]
    fn serialisierung_liefert_denselben_text() {
        let fehler = DhError::Other("sichtbar im Fenster".into());
        assert_eq!(
            serde_json::to_string(&fehler).unwrap(),
            "\"sichtbar im Fenster\""
        );
    }
}
