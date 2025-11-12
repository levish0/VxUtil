//! Track - a container for clips on the timeline

use serde::{Deserialize, Serialize};

use super::{Clip, ClipId};
use crate::types::{TimeRange, Timecode};

/// Unique identifier for a track
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackId(pub usize);

/// Type of track (video or audio)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackType {
    Video,
    Audio,
}

/// A track contains multiple clips arranged in timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: TrackId,
    pub name: String,
    pub track_type: TrackType,
    pub clips: Vec<Clip>,
    pub muted: bool,
    pub locked: bool,
}

impl Track {
    pub fn new(id: TrackId, name: String, track_type: TrackType) -> Self {
        Self {
            id,
            name,
            track_type,
            clips: Vec::new(),
            muted: false,
            locked: false,
        }
    }

    /// Add a clip to this track
    pub fn add_clip(&mut self, clip: Clip) {
        self.clips.push(clip);
        self.sort_clips();
    }

    /// Remove a clip by ID
    pub fn remove_clip(&mut self, clip_id: &ClipId) -> Option<Clip> {
        if let Some(index) = self.clips.iter().position(|c| &c.id == clip_id) {
            Some(self.clips.remove(index))
        } else {
            None
        }
    }

    /// Sort clips by timeline position
    fn sort_clips(&mut self) {
        self.clips
            .sort_by(|a, b| a.timeline_position.cmp(&b.timeline_position));
    }

    /// Find clip at given timeline position
    pub fn clip_at_time(&self, time: Timecode) -> Option<&Clip> {
        self.clips.iter().find(|clip| clip.contains_time(time))
    }

    /// Get all clips that overlap with given time range
    pub fn clips_in_range(&self, range: TimeRange) -> Vec<&Clip> {
        self.clips
            .iter()
            .filter(|clip| clip.overlaps_with(range))
            .collect()
    }
}
