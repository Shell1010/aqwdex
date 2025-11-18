use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("invalid class model: {0}")]
    InvalidClassModel(String),

    #[error("invalid weapon boost value: {0}")]
    InvalidWeaponBoost(String),

    #[error("parse error: {0}")]
    ParseError(String),
}
