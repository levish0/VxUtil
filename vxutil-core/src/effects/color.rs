//! Color correction effect

use serde::{Deserialize, Serialize};

/// Color correction effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorCorrectEffect {
    pub brightness: f32, // -1.0 to 1.0
    pub contrast: f32,   // -1.0 to 1.0
    pub saturation: f32, // 0.0 to 2.0
    pub hue: f32,        // -180.0 to 180.0 degrees
}

impl Default for ColorCorrectEffect {
    fn default() -> Self {
        Self {
            brightness: 0.0,
            contrast: 0.0,
            saturation: 1.0,
            hue: 0.0,
        }
    }
}
