//! Media library

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{MediaId, MediaItem, MediaType};

/// Media library manages all imported media items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaLibrary {
    items: HashMap<MediaId, MediaItem>,
}

impl Default for MediaLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaLibrary {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Add a media item to the library
    pub fn add_item(&mut self, item: MediaItem) -> MediaId {
        let id = item.id.clone();
        self.items.insert(id.clone(), item);
        id
    }

    /// Remove a media item by ID
    pub fn remove_item(&mut self, id: &MediaId) -> Option<MediaItem> {
        self.items.remove(id)
    }

    /// Get a media item by ID
    pub fn get_item(&self, id: &MediaId) -> Option<&MediaItem> {
        self.items.get(id)
    }

    /// Get a mutable reference to a media item
    pub fn get_item_mut(&mut self, id: &MediaId) -> Option<&mut MediaItem> {
        self.items.get_mut(id)
    }

    /// Get all media items
    pub fn items(&self) -> impl Iterator<Item = &MediaItem> {
        self.items.values()
    }

    /// Get all video items
    pub fn video_items(&self) -> impl Iterator<Item = &MediaItem> {
        self.items
            .values()
            .filter(|item| item.media_type == MediaType::Video)
    }

    /// Get all audio items
    pub fn audio_items(&self) -> impl Iterator<Item = &MediaItem> {
        self.items
            .values()
            .filter(|item| item.media_type == MediaType::Audio)
    }

    /// Get all image items
    pub fn image_items(&self) -> impl Iterator<Item = &MediaItem> {
        self.items
            .values()
            .filter(|item| item.media_type == MediaType::Image)
    }

    /// Search media items by name (case-insensitive)
    pub fn search(&self, query: &str) -> Vec<&MediaItem> {
        let query_lower = query.to_lowercase();
        self.items
            .values()
            .filter(|item| item.name.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Count total items
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Verify all media files exist
    pub fn verify_files(&self) -> Vec<MediaId> {
        self.items
            .iter()
            .filter(|(_, item)| !item.exists())
            .map(|(id, _)| id.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_media_library_add_remove() {
        let mut library = MediaLibrary::new();

        let item = MediaItem::new(PathBuf::from("video.mp4"), MediaType::Video);
        let id = library.add_item(item.clone());

        assert_eq!(library.count(), 1);
        assert!(library.get_item(&id).is_some());

        library.remove_item(&id);
        assert_eq!(library.count(), 0);
    }

    #[test]
    fn test_media_library_filter_by_type() {
        let mut library = MediaLibrary::new();

        library.add_item(MediaItem::new(
            PathBuf::from("video1.mp4"),
            MediaType::Video,
        ));
        library.add_item(MediaItem::new(
            PathBuf::from("audio1.mp3"),
            MediaType::Audio,
        ));
        library.add_item(MediaItem::new(
            PathBuf::from("video2.mov"),
            MediaType::Video,
        ));
        library.add_item(MediaItem::new(
            PathBuf::from("image1.png"),
            MediaType::Image,
        ));

        assert_eq!(library.video_items().count(), 2);
        assert_eq!(library.audio_items().count(), 1);
        assert_eq!(library.image_items().count(), 1);
    }

    #[test]
    fn test_media_library_search() {
        let mut library = MediaLibrary::new();

        library.add_item(MediaItem::new(
            PathBuf::from("vacation_2024.mp4"),
            MediaType::Video,
        ));
        library.add_item(MediaItem::new(
            PathBuf::from("interview.mp4"),
            MediaType::Video,
        ));
        library.add_item(MediaItem::new(
            PathBuf::from("vacation_audio.mp3"),
            MediaType::Audio,
        ));

        let results = library.search("vacation");
        assert_eq!(results.len(), 2);
    }
}
