//! Media item

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use super::{MediaId, MediaMetadata, MediaType};

/// A media item represents a source file in the project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub id: MediaId,
    pub name: String,
    pub path: PathBuf,
    pub media_type: MediaType,
    pub metadata: MediaMetadata,

    /// Thumbnail path (optional)
    pub thumbnail_path: Option<PathBuf>,

    /// Import timestamp
    pub imported_at: DateTime<Utc>,
}

impl MediaItem {
    pub fn new(path: PathBuf, media_type: MediaType) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled")
            .to_string();

        Self {
            id: MediaId::new(),
            name,
            path,
            media_type,
            metadata: MediaMetadata::default(),
            thumbnail_path: None,
            imported_at: Utc::now(),
        }
    }

    /// Check if the media file exists
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Get duration in seconds (if available)
    pub fn duration_seconds(&self) -> Option<f64> {
        self.metadata.duration
    }
}
