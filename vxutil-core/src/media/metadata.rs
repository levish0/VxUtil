//! Media metadata

use serde::{Deserialize, Serialize};

use crate::types::{FrameRate, Resolution};

/// Metadata for a media file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMetadata {
    /// Duration in seconds (None for images)
    pub duration: Option<f64>,

    /// Resolution for video/image
    pub resolution: Option<Resolution>,

    /// Frame rate for video
    pub frame_rate: Option<FrameRate>,

    /// Codec name (e.g., "H.264", "AAC")
    pub codec: Option<String>,

    /// Audio sample rate (Hz)
    pub sample_rate: Option<u32>,

    /// Bitrate (bits per second)
    pub bitrate: Option<u64>,

    /// File size in bytes
    pub file_size: u64,
}

impl Default for MediaMetadata {
    fn default() -> Self {
        Self {
            duration: None,
            resolution: None,
            frame_rate: None,
            codec: None,
            sample_rate: None,
            bitrate: None,
            file_size: 0,
        }
    }
}
