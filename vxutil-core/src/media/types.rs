//! Media types and identifiers

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a media item
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MediaId(Uuid);

impl MediaId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MediaId {
    fn default() -> Self {
        Self::new()
    }
}

/// Type of media
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaType {
    Video,
    Audio,
    Image,
}
