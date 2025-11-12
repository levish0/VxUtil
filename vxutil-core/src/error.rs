//! Error types for VxUtil

use thiserror::Error;

pub type Result<T> = std::result::Result<T, VxError>;

#[derive(Debug, Error)]
pub enum VxError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Project error: {0}")]
    Project(String),

    #[error("Timeline error: {0}")]
    Timeline(String),

    #[error("Media error: {0}")]
    Media(String),

    #[error("Effect error: {0}")]
    Effect(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
