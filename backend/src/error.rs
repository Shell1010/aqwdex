use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("invalid class model: {0}")]
    InvalidClassModel(String),

    #[error("invalid weapon boost value: {0}")]
    InvalidWeaponBoost(String),

    #[error("parse error: {0}")]
    ParseError(String),
    
    #[error("invalid damage source: {0}")]
    InvalidDamageSource(String),
    
    #[error("invalid damage type: {0}")]
    InvalidDamageType(String),

    #[error("invalid target type: {0}")]
    InvalidTargetType(String),

    #[error("invalid enhancement pattern: {0}")]
    InvalidEnhancementPattern(String),

    #[error("invalid gear slot: {0}")]
    InvalidGearSlot(String),
}

