//! Project management

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::ProjectSettings;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub settings: ProjectSettings,
    // TODO: sequences, media library
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
