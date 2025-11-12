//! VxUtil Core - Business logic and domain models
//!
//! This crate contains the core business logic for VxUtil video editor,
//! including project management, timeline models, media library, and effects system.
//! It has NO UI dependencies and NO media processing implementation.

pub mod effects;
pub mod error;
pub mod media;
pub mod project;
pub mod timeline;
pub mod types;

// Re-export commonly used types
pub use error::{Result, VxError};
pub use project::{Project, ProjectSettings};
pub use types::*;
