//! VxUtil Engine - Media processing and rendering
//!
//! This crate handles all media processing operations:
//! - Video/audio decoding and encoding (FFmpeg)
//! - Real-time playback engine
//! - Rendering and compositing pipeline
//! - GPU acceleration (WGPU)
//! - Frame caching

pub mod error;
pub mod ffmpeg;
pub mod playback;
pub mod rendering;
pub mod cache;

pub use error::{Result, EngineError};
