//! User configuration system
//! 
//! This module provides configuration management for Ciri.
//! Note: This requires the "config" feature to be enabled.

use std::fs;
use std::path::PathBuf;

#[cfg(feature = "config")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "config")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub default_target: String,
    pub show_notes: bool,
    pub confidence_threshold: u8,
    pub aggressive_mode: bool,
    pub output_format: OutputFormat,
}

#[cfg(feature = "config")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Human,
    Json,
    Csv,
}

#[cfg(feature = "config")]
impl Default for Config {
    fn default() -> Self {
        Self {
            default_target: "1080p".to_string(),
            show_notes: true,
            confidence_threshold: 70,
            aggressive_mode: false,
            output_format: OutputFormat::Human,
        }
    }
}

/// Get the config file path
#[cfg(feature = "config")]
fn get_config_path() -> Result<PathBuf, ConfigError> {
    use dirs::config_dir;
    
    let config_dir = config_dir()
        .ok_or_else(|| ConfigError::NoConfigDir)?;
    let ciri_dir = config_dir.join("ciri");
    fs::create_dir_all(&ciri_dir)
        .map_err(|e| ConfigError::IoError(e.to_string()))?;
    Ok(ciri_dir.join("config.toml"))
}

/// Load configuration from file
#[cfg(feature = "config")]
pub fn load_config() -> Result<Config, ConfigError> {
    let path = get_config_path()?;
    
    if !path.exists() {
        // Return default config if file doesn't exist
        return Ok(Config::default());
    }
    
    let content = fs::read_to_string(&path)
        .map_err(|e| ConfigError::IoError(e.to_string()))?;
    
    let config: Config = toml::from_str(&content)
        .map_err(|e| ConfigError::ParseError(e.to_string()))?;
    
    Ok(config)
}

/// Save configuration to file
#[cfg(feature = "config")]
pub fn save_config(config: &Config) -> Result<(), ConfigError> {
    let path = get_config_path()?;
    
    let content = toml::to_string_pretty(config)
        .map_err(|e| ConfigError::ParseError(e.to_string()))?;
    
    fs::write(&path, content)
        .map_err(|e| ConfigError::IoError(e.to_string()))?;
    
    Ok(())
}

/// Error type for configuration operations
#[derive(Debug)]
pub enum ConfigError {
    IoError(String),
    ParseError(String),
    NoConfigDir,
    FeatureNotEnabled,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::IoError(msg) => write!(f, "IO error: {}", msg),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ConfigError::NoConfigDir => write!(f, "Could not find config directory"),
            ConfigError::FeatureNotEnabled => write!(f, "Config feature not enabled. Build with --features config"),
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(not(feature = "config"))]
pub struct Config;

#[cfg(not(feature = "config"))]
pub fn load_config() -> Result<Config, ConfigError> {
    Err(ConfigError::FeatureNotEnabled)
}

#[cfg(not(feature = "config"))]
pub fn save_config(_config: &Config) -> Result<(), ConfigError> {
    Err(ConfigError::FeatureNotEnabled)
}

#[cfg(not(feature = "config"))]
#[derive(Debug)]
pub enum ConfigError {
    FeatureNotEnabled,
}

#[cfg(not(feature = "config"))]
impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::FeatureNotEnabled => write!(f, "Config feature not enabled. Build with --features config"),
        }
    }
}

#[cfg(not(feature = "config"))]
impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "config")]
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.default_target, "1080p");
        assert!(config.show_notes);
        assert_eq!(config.confidence_threshold, 70);
    }

    #[cfg(feature = "config")]
    #[test]
    fn test_config_roundtrip() {
        let config = Config::default();
        let result = save_config(&config);
        assert!(result.is_ok());
        
        let loaded = load_config();
        assert!(loaded.is_ok());
        let loaded_config = loaded.unwrap();
        assert_eq!(loaded_config.default_target, config.default_target);
    }
}