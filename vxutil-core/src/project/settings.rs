//! Project settings

use serde::{Deserialize, Serialize};

use crate::types::{FrameRate, Resolution};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub frame_rate: FrameRate,
    pub resolution: Resolution,
    pub sample_rate: u32, // Audio sample rate (e.g., 48000)
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            frame_rate: FrameRate::FPS_30,
            resolution: Resolution::FULL_HD,
            sample_rate: 48000,
        }
    }
}
