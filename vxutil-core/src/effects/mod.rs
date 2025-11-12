//! Effects system
//!
//! Basic effects that can be applied to clips on the timeline.
//! The actual processing is done in vxutil-engine.

use serde::{Deserialize, Serialize};

mod blur;
mod color;
mod opacity;
mod transform;

pub use blur::BlurEffect;
pub use color::ColorCorrectEffect;
pub use opacity::OpacityEffect;
pub use transform::TransformEffect;

/// Effect type that can be applied to clips
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    Transform(TransformEffect),
    Opacity(OpacityEffect),
    ColorCorrect(ColorCorrectEffect),
    Blur(BlurEffect),
}

impl EffectType {
    pub fn name(&self) -> &str {
        match self {
            EffectType::Transform(_) => "Transform",
            EffectType::Opacity(_) => "Opacity",
            EffectType::ColorCorrect(_) => "Color Correction",
            EffectType::Blur(_) => "Blur",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effect_serialization() {
        let effect = EffectType::Transform(TransformEffect::default());
        let json = serde_json::to_string(&effect).unwrap();
        let deserialized: EffectType = serde_json::from_str(&json).unwrap();
        assert_eq!(effect.name(), deserialized.name());
    }
}
