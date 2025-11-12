//! FFmpeg wrapper for video/audio processing

use crate::{Result, EngineError};
use std::path::Path;
use vxutil_core::{FrameRate, Resolution};

pub struct VideoMetadata {
    pub duration_seconds: f64,
    pub frame_rate: FrameRate,
    pub resolution: Resolution,
    pub codec: String,
    pub has_audio: bool,
}

/// Get video metadata from file
pub fn get_video_metadata(path: &Path) -> Result<VideoMetadata> {
    // TODO: Implement using ffmpeg-next
    let _ = path;
    Err(EngineError::FFmpeg("Not yet implemented".to_string()))
}

pub struct VideoDecoder {
    // TODO: FFmpeg decoder state
}

impl VideoDecoder {
    pub fn new(_path: &Path) -> Result<Self> {
        // TODO: Initialize FFmpeg decoder
        todo!("VideoDecoder not yet implemented")
    }

    pub fn decode_frame(&mut self, _frame_number: u64) -> Result<DecodedFrame> {
        // TODO: Decode specific frame
        todo!("Frame decoding not yet implemented")
    }
}

pub struct DecodedFrame {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub timestamp: f64,
}
