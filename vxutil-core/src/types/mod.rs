//! Common types used throughout VxUtil

mod framerate;
mod resolution;
mod timecode;
mod timerange;

pub use framerate::FrameRate;
pub use resolution::Resolution;
pub use timecode::{FrameNumber, Timecode};
pub use timerange::TimeRange;
