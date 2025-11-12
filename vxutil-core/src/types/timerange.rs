//! Time range

use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::Timecode;

/// Time range in the timeline
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: Timecode,
    pub duration: Duration,
}

impl TimeRange {
    pub fn new(start: Timecode, duration: Duration) -> Self {
        Self { start, duration }
    }

    pub fn end(&self) -> Timecode {
        Timecode(self.start.0 + self.duration)
    }

    pub fn contains(&self, time: Timecode) -> bool {
        time >= self.start && time < self.end()
    }
}
