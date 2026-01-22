//! Playback service - handles slideshow playback logic

use std::path::PathBuf;

/// Playback state
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
    #[default]
    Stopped,
    Playing,
    Paused,
}

/// Manages slideshow playback
#[derive(Default)]
pub struct PlaybackManager {
    state: PlaybackState,
    current_grid: Option<usize>,
    current_content_index: usize,
    highlights_only: bool,
}

impl PlaybackManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Starts playback
    pub fn play(&mut self) {
        self.state = PlaybackState::Playing;
        // TODO: Implement playback start
    }

    /// Stops playback
    pub fn stop(&mut self) {
        self.state = PlaybackState::Stopped;
        self.current_content_index = 0;
        // TODO: Implement playback stop
    }

    /// Pauses playback
    pub fn pause(&mut self) {
        if self.state == PlaybackState::Playing {
            self.state = PlaybackState::Paused;
        }
    }

    /// Resumes playback
    pub fn resume(&mut self) {
        if self.state == PlaybackState::Paused {
            self.state = PlaybackState::Playing;
        }
    }

    /// Toggles between play and pause
    pub fn toggle(&mut self) {
        match self.state {
            PlaybackState::Playing => self.pause(),
            PlaybackState::Paused => self.resume(),
            PlaybackState::Stopped => self.play(),
        }
    }

    /// Advances to the next image
    pub fn next(&mut self) {
        // TODO: Implement next image logic
        self.current_content_index += 1;
    }

    /// Goes back to the previous image
    pub fn previous(&mut self) {
        // TODO: Implement previous image logic
        if self.current_content_index > 0 {
            self.current_content_index -= 1;
        }
    }

    /// Sets the active grid (0-9, None for all)
    pub fn set_active_grid(&mut self, grid: Option<usize>) {
        self.current_grid = grid;
    }

    /// Toggles highlights-only mode
    pub fn toggle_highlights_only(&mut self) {
        self.highlights_only = !self.highlights_only;
    }

    /// Returns current playback state
    pub fn state(&self) -> PlaybackState {
        self.state
    }

    /// Returns whether in highlights-only mode
    pub fn is_highlights_only(&self) -> bool {
        self.highlights_only
    }
}

/// Loads an image from path for display
pub fn load_image(_path: &PathBuf) -> Result<Vec<u8>, ImageError> {
    // TODO: Implement image loading
    Err(ImageError::NotImplemented)
}

#[derive(Debug)]
pub enum ImageError {
    IoError(std::io::Error),
    DecodeError(String),
    NotImplemented,
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageError::IoError(e) => write!(f, "I/O error: {}", e),
            ImageError::DecodeError(s) => write!(f, "Decode error: {}", s),
            ImageError::NotImplemented => write!(f, "Not implemented"),
        }
    }
}

impl std::error::Error for ImageError {}
