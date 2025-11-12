//! VxUtil Core - Business logic and domain models
//!
//! This crate contains the core business logic for VxUtil video editor,
//! including project management, timeline models, media library, and effects system.
//! It has NO UI dependencies and NO media processing implementation.

pub mod types;
pub mod error;
pub mod project;
pub mod timeline;
pub mod media;
pub mod effects;

// Re-export commonly used types
pub use error::{Result, VxError};
pub use types::*;
pub use project::{Project, ProjectSettings};