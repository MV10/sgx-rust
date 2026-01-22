//! Settings service - handles application settings persistence

use std::path::PathBuf;

/// Application-wide settings (not playlist-specific)
#[derive(Debug, Clone)]
pub struct AppSettings {
    /// Last used directory for file dialogs
    pub last_directory: Option<PathBuf>,
    /// Recently opened playlists
    pub recent_playlists: Vec<PathBuf>,
    /// Maximum number of recent playlists to remember
    pub max_recent: usize,
    /// Window position and size
    pub window_geometry: Option<WindowGeometry>,
}

#[derive(Debug, Clone, Copy)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            last_directory: None,
            recent_playlists: Vec::new(),
            max_recent: 10,
            window_geometry: None,
        }
    }
}

impl AppSettings {
    /// Loads settings from the default config location
    pub fn load() -> Result<Self, SettingsError> {
        let path = Self::config_path()?;
        Self::load_from(&path)
    }

    /// Loads settings from a specific path
    pub fn load_from(_path: &PathBuf) -> Result<Self, SettingsError> {
        // TODO: Implement settings loading (JSON or TOML)
        Ok(Self::default())
    }

    /// Saves settings to the default config location
    pub fn save(&self) -> Result<(), SettingsError> {
        let path = Self::config_path()?;
        self.save_to(&path)
    }

    /// Saves settings to a specific path
    pub fn save_to(&self, _path: &PathBuf) -> Result<(), SettingsError> {
        // TODO: Implement settings saving
        Err(SettingsError::NotImplemented)
    }

    /// Returns the default config file path
    pub fn config_path() -> Result<PathBuf, SettingsError> {
        dirs::config_dir()
            .map(|p| p.join("slidegrid").join("settings.json"))
            .ok_or(SettingsError::NoConfigDir)
    }

    /// Adds a playlist to the recent list
    pub fn add_recent(&mut self, path: PathBuf) {
        // Remove if already present
        self.recent_playlists.retain(|p| p != &path);
        // Add to front
        self.recent_playlists.insert(0, path);
        // Trim to max
        self.recent_playlists.truncate(self.max_recent);
    }
}

#[derive(Debug)]
pub enum SettingsError {
    IoError(std::io::Error),
    ParseError(String),
    NoConfigDir,
    NotImplemented,
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsError::IoError(e) => write!(f, "I/O error: {}", e),
            SettingsError::ParseError(s) => write!(f, "Parse error: {}", s),
            SettingsError::NoConfigDir => write!(f, "Could not find config directory"),
            SettingsError::NotImplemented => write!(f, "Not implemented"),
        }
    }
}

impl std::error::Error for SettingsError {}
