//! Playlist service - handles playlist file I/O and management

use std::path::PathBuf;

/// Represents a slidegrid playlist
#[derive(Default, Debug, Clone)]
pub struct Playlist {
    /// Path to the playlist file
    pub path: Option<PathBuf>,
    /// Grid configurations
    pub grids: Vec<GridConfig>,
    /// Content image paths
    pub content: Vec<PathBuf>,
    /// Highlight image paths
    pub highlights: Vec<PathBuf>,
    /// Playlist settings
    pub settings: PlaylistSettings,
}

/// Grid configuration entry
#[derive(Debug, Clone)]
pub struct GridConfig {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Playlist-specific settings
#[derive(Default, Debug, Clone)]
pub struct PlaylistSettings {
    pub advance_mode: AdvanceMode,
    pub resize_mode: ResizeMode,
    pub shuffle_time: f32,
    pub stagger_mode: StaggerMode,
    pub randomize_mode: RandomizeMode,
    pub sequencing_mode: SequencingMode,
    pub highlight_mode: HighlightMode,
    pub max_sequence_length: u32,
    pub highlight_frequency: u32,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum AdvanceMode {
    #[default]
    Automatic,
    Manual,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum ResizeMode {
    #[default]
    ByLargestDimension,
    ByWidth,
    ByHeight,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum StaggerMode {
    #[default]
    Staggered,
    Synchronized,
    None,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum RandomizeMode {
    #[default]
    Shuffle,
    Sequential,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum SequencingMode {
    #[default]
    ByFilename,
    ByTimestamp,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum HighlightMode {
    #[default]
    SameAsContent,
    Random,
    Sequential,
}

impl Playlist {
    /// Creates a new empty playlist
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads a playlist from a file
    pub fn load(_path: &PathBuf) -> Result<Self, PlaylistError> {
        // TODO: Implement playlist file loading
        Err(PlaylistError::NotImplemented)
    }

    /// Saves the playlist to a file
    pub fn save(&self, _path: &PathBuf) -> Result<(), PlaylistError> {
        // TODO: Implement playlist file saving
        Err(PlaylistError::NotImplemented)
    }

    /// Saves the playlist to its current path
    pub fn save_current(&self) -> Result<(), PlaylistError> {
        match &self.path {
            Some(path) => self.save(path),
            None => Err(PlaylistError::NoPath),
        }
    }
}

#[derive(Debug)]
pub enum PlaylistError {
    IoError(std::io::Error),
    ParseError(String),
    NoPath,
    NotImplemented,
}

impl std::fmt::Display for PlaylistError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaylistError::IoError(e) => write!(f, "I/O error: {}", e),
            PlaylistError::ParseError(s) => write!(f, "Parse error: {}", s),
            PlaylistError::NoPath => write!(f, "No playlist path set"),
            PlaylistError::NotImplemented => write!(f, "Not implemented"),
        }
    }
}

impl std::error::Error for PlaylistError {}
