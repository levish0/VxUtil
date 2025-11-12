//! Engine error types

use thiserror::Error;

pub type Result<T> = std::result::Result<T, EngineError>;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("FFmpeg error: {0}")]
    FFmpeg(String),

    #[error("Decoding error: {0}")]
    Decode(String),

    #[error("Encoding error: {0}")]
    Encode(String),

    #[error("GPU error: {0}")]
    Gpu(String),

    #[error("Playback error: {0}")]
    Playback(String),

    #[error("Rendering error: {0}")]
    Rendering(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Core error: {0}")]
    Core(#[from] vxutil_core::VxError),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
