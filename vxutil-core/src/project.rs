//! Project management

use crate::{Result, FrameRate, Resolution};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub settings: ProjectSettings,
    // TODO: sequences, media library
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub frame_rate: FrameRate,
    pub resolution: Resolution,
    pub sample_rate: u32, // Audio sample rate (e.g., 48000)
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            frame_rate: FrameRate::FPS_30,
            resolution: Resolution::FULL_HD,
            sample_rate: 48000,
        }
    }
}

impl Project {
    pub fn new(name: String, path: PathBuf, settings: ProjectSettings) -> Self {
        Self {
            name,
            path,
            settings,
        }
    }

    pub fn load(_path: &PathBuf) -> Result<Self> {
        // TODO: Implement project loading
        todo!("Project loading not yet implemented")
    }

    pub fn save(&self) -> Result<()> {
        // TODO: Implement project saving
        todo!("Project saving not yet implemented")
    }
}