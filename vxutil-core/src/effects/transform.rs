//! Transform effect (position, scale, rotation)

use serde::{Deserialize, Serialize};

/// Transform effect (position, scale, rotation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformEffect {
    pub position_x: f32,
    pub position_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32, // degrees
}

impl Default for TransformEffect {
    fn default() -> Self {
        Self {
            position_x: 0.0,
            position_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_default() {
        let effect = TransformEffect::default();
        assert_eq!(effect.scale_x, 1.0);
        assert_eq!(effect.scale_y, 1.0);
        assert_eq!(effect.rotation, 0.0);
    }
}
