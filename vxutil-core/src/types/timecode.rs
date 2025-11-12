//! Timecode representation

use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::FrameRate;

/// Timecode representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timecode(pub Duration);

impl Timecode {
    pub fn from_seconds(seconds: f64) -> Self {
        Self(Duration::from_secs_f64(seconds))
    }

    pub fn from_frames(frames: u64, frame_rate: FrameRate) -> Self {
        let seconds = frames as f64 / frame_rate.as_f64();
        Self::from_seconds(seconds)
    }

    pub fn as_duration(&self) -> Duration {
        self.0
    }

    pub fn as_seconds(&self) -> f64 {
        self.0.as_secs_f64()
    }
}

/// Frame number
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FrameNumber(pub u64);
