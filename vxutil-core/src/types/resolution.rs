//! Video resolution

use serde::{Deserialize, Serialize};

/// Video resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub const HD: Self = Self {
        width: 1280,
        height: 720,
    };
    pub const FULL_HD: Self = Self {
        width: 1920,
        height: 1080,
    };
    pub const UHD_4K: Self = Self {
        width: 3840,
        height: 2160,
    };

    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }
}
