//! Common types used throughout VxUtil

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Timecode representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timecode(Duration);

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

/// Frame rate
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FrameRate {
    pub numerator: u32,
    pub denominator: u32,
}

impl FrameRate {
    pub const FPS_24: Self = Self { numerator: 24, denominator: 1 };
    pub const FPS_25: Self = Self { numerator: 25, denominator: 1 };
    pub const FPS_30: Self = Self { numerator: 30, denominator: 1 };
    pub const FPS_60: Self = Self { numerator: 60, denominator: 1 };
    pub const FPS_23_976: Self = Self { numerator: 24000, denominator: 1001 };
    pub const FPS_29_97: Self = Self { numerator: 30000, denominator: 1001 };

    pub fn new(numerator: u32, denominator: u32) -> Self {
        Self { numerator, denominator }
    }

    pub fn as_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

/// Video resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub const HD: Self = Self { width: 1280, height: 720 };
    pub const FULL_HD: Self = Self { width: 1920, height: 1080 };
    pub const UHD_4K: Self = Self { width: 3840, height: 2160 };

    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }
}

/// Time range in the timeline
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: Timecode,
    pub duration: Duration,
}

impl TimeRange {
    pub fn new(start: Timecode, duration: Duration) -> Self {
        Self { start, duration }
    }

    pub fn end(&self) -> Timecode {
        Timecode(self.start.0 + self.duration)
    }

    pub fn contains(&self, time: Timecode) -> bool {
        time >= self.start && time < self.end()
    }
}