//! Blur effect

use serde::{Deserialize, Serialize};

/// Blur effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlurEffect {
    pub radius: f32, // blur radius in pixels
}

impl Default for BlurEffect {
    fn default() -> Self {
        Self { radius: 5.0 }
    }
}
