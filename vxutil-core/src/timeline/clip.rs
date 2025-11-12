//! Clip - a piece of media on the timeline

use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

use super::BlendMode;
use crate::types::{TimeRange, Timecode};
use crate::media::MediaId;
use crate::effects::EffectType;

/// Unique identifier for a clip
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClipId(Uuid);

impl ClipId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self::new()
    }
}

/// A clip represents a piece of media placed on the timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub id: ClipId,
    pub name: String,

    /// Reference to the source media
    pub source_media: MediaId,

    /// Position on the timeline where this clip starts
    pub timeline_position: Timecode,

    /// Range in the source media (in point, out point)
    pub source_in: Timecode,
    pub source_out: Timecode,

    /// Speed multiplier (1.0 = normal, 2.0 = 2x speed, 0.5 = half speed)
    pub speed: f64,

    /// Blend mode for compositing
    pub blend_mode: BlendMode,

    /// List of effects applied to this clip
    pub effects: Vec<EffectType>,
}

impl Clip {
    pub fn new(
        name: String,
        source_media: MediaId,
        timeline_position: Timecode,
        source_in: Timecode,
        source_out: Timecode,
    ) -> Self {
        Self {
            id: ClipId::new(),
            name,
            source_media,
            timeline_position,
            source_in,
            source_out,
            speed: 1.0,
            blend_mode: BlendMode::default(),
            effects: Vec::new(),
        }
    }

    /// Duration of this clip on the timeline (considering speed)
    pub fn timeline_duration(&self) -> Duration {
        let source_duration = self.source_out.as_duration() - self.source_in.as_duration();
        Duration::from_secs_f64(source_duration.as_secs_f64() / self.speed)
    }

    /// End position of this clip on the timeline
    pub fn timeline_end(&self) -> Timecode {
        Timecode(self.timeline_position.as_duration() + self.timeline_duration())
    }

    /// Check if this clip contains given timeline position
    pub fn contains_time(&self, time: Timecode) -> bool {
        time >= self.timeline_position && time < self.timeline_end()
    }

    /// Check if this clip overlaps with given time range
    pub fn overlaps_with(&self, range: TimeRange) -> bool {
        let clip_range = TimeRange::new(self.timeline_position, self.timeline_duration());
        // Overlap if either start or end is within the range
        clip_range.contains(range.start)
            || clip_range.contains(range.end())
            || range.contains(clip_range.start)
            || range.contains(clip_range.end())
    }

    /// Map timeline time to source media time
    pub fn timeline_to_source_time(&self, timeline_time: Timecode) -> Option<Timecode> {
        if !self.contains_time(timeline_time) {
            return None;
        }

        let offset_in_clip = timeline_time.as_duration() - self.timeline_position.as_duration();
        let source_offset = Duration::from_secs_f64(offset_in_clip.as_secs_f64() * self.speed);
        let source_time = self.source_in.as_duration() + source_offset;

        Some(Timecode(source_time))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::media::MediaId;

    #[test]
    fn test_clip_timeline_duration() {
        let clip = Clip::new(
            "test".to_string(),
            MediaId::new(),
            Timecode::from_seconds(0.0),
            Timecode::from_seconds(10.0),
            Timecode::from_seconds(20.0), // 10 second source
        );

        assert_eq!(clip.timeline_duration().as_secs(), 10);
    }

    #[test]
    fn test_clip_with_speed() {
        let mut clip = Clip::new(
            "test".to_string(),
            MediaId::new(),
            Timecode::from_seconds(0.0),
            Timecode::from_seconds(0.0),
            Timecode::from_seconds(10.0),
        );

        clip.speed = 2.0; // 2x speed

        // Source is 10 seconds, but at 2x speed = 5 seconds on timeline
        assert_eq!(clip.timeline_duration().as_secs(), 5);
    }

    #[test]
    fn test_clip_contains_time() {
        let clip = Clip::new(
            "test".to_string(),
            MediaId::new(),
            Timecode::from_seconds(10.0),
            Timecode::from_seconds(0.0),
            Timecode::from_seconds(5.0),
        );

        assert!(clip.contains_time(Timecode::from_seconds(12.0)));
        assert!(!clip.contains_time(Timecode::from_seconds(16.0)));
    }

    #[test]
    fn test_timeline_to_source_time() {
        let mut clip = Clip::new(
            "test".to_string(),
            MediaId::new(),
            Timecode::from_seconds(10.0), // Timeline position
            Timecode::from_seconds(20.0), // Source in
            Timecode::from_seconds(30.0), // Source out
        );
        clip.speed = 2.0; // 2x speed

        // Timeline 12 seconds -> 2 seconds into clip -> 4 seconds in source (2x) -> source 24 seconds
        let source_time = clip.timeline_to_source_time(Timecode::from_seconds(12.0));
        assert_eq!(source_time.unwrap().as_seconds(), 24.0);
    }
}
