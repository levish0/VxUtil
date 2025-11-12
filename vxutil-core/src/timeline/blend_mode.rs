//! Blend modes for compositing

use serde::{Deserialize, Serialize};

/// Blend mode for compositing clips
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Add,
    Subtract,
    Darken,
    Lighten,
}

impl Default for BlendMode {
    fn default() -> Self {
        BlendMode::Normal
    }
}
