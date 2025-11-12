//! Sequence - a timeline containing multiple tracks

use serde::{Deserialize, Serialize};

use super::{Clip, Track, TrackId, TrackType};
use crate::types::{FrameRate, Resolution, Timecode};

/// A sequence is a timeline containing multiple tracks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sequence {
    pub name: String,
    pub frame_rate: FrameRate,
    pub resolution: Resolution,

    /// Video tracks (higher index = higher z-order)
    pub video_tracks: Vec<Track>,

    /// Audio tracks
    pub audio_tracks: Vec<Track>,

    /// Current playhead position
    pub playhead: Timecode,
}

impl Sequence {
    pub fn new(name: String, frame_rate: FrameRate, resolution: Resolution) -> Self {
        Self {
            name,
            frame_rate,
            resolution,
            video_tracks: Vec::new(),
            audio_tracks: Vec::new(),
            playhead: Timecode::from_seconds(0.0),
        }
    }

    /// Add a new track
    pub fn add_track(&mut self, track: Track) {
        match track.track_type {
            TrackType::Video => self.video_tracks.push(track),
            TrackType::Audio => self.audio_tracks.push(track),
        }
    }

    /// Get track by ID
    pub fn get_track(&self, track_id: TrackId) -> Option<&Track> {
        self.video_tracks
            .iter()
            .chain(self.audio_tracks.iter())
            .find(|t| t.id == track_id)
    }

    /// Get mutable track by ID
    pub fn get_track_mut(&mut self, track_id: TrackId) -> Option<&mut Track> {
        self.video_tracks
            .iter_mut()
            .chain(self.audio_tracks.iter_mut())
            .find(|t| t.id == track_id)
    }

    /// Calculate total duration of the sequence (longest clip end time)
    pub fn duration(&self) -> Timecode {
        let max_video = self
            .video_tracks
            .iter()
            .flat_map(|t| t.clips.iter())
            .map(|c| c.timeline_end())
            .max();

        let max_audio = self
            .audio_tracks
            .iter()
            .flat_map(|t| t.clips.iter())
            .map(|c| c.timeline_end())
            .max();

        max_video
            .into_iter()
            .chain(max_audio)
            .max()
            .unwrap_or(Timecode::from_seconds(0.0))
    }

    /// Get all video clips at given time (from all tracks, sorted by z-order)
    pub fn video_clips_at_time(&self, time: Timecode) -> Vec<&Clip> {
        self.video_tracks
            .iter()
            .filter_map(|track| track.clip_at_time(time))
            .collect()
    }

    /// Get all audio clips at given time
    pub fn audio_clips_at_time(&self, time: Timecode) -> Vec<&Clip> {
        self.audio_tracks
            .iter()
            .filter_map(|track| track.clip_at_time(time))
            .collect()
    }
}
