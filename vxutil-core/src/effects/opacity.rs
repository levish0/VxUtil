//! Opacity effect

use serde::{Deserialize, Serialize};

/// Opacity effect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpacityEffect {
    pub opacity: f32, // 0.0 to 1.0
}

impl Default for OpacityEffect {
    fn default() -> Self {
        Self { opacity: 1.0 }
    }
}
