//! Catalog update functionality for dynamic game database management
//! 
//! This module provides functionality to update the game catalog from various sources.
//! Note: This requires the "catalog-update" feature to be enabled.

use crate::catalog::embedded::{Game, Tier, LinuxSupport};
use crate::catalog::steam_api::{fetch_steam_game, search_steam_games, SteamGame};
use crate::catalog::protondb::{fetch_protondb_rating, ProtonDBRating};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogUpdateResult {
    pub timestamp: DateTime<Utc>,
    pub games_updated: usize,
    pub games_added: usize,
    pub games_failed: Vec<String>,
    pub sources_used: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    pub version: String,
    pub last_updated: DateTime<Utc>,
    pub games: Vec<SerializableGame>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableGame {
    pub id: String,
    pub title: String,
    pub aliases: Vec<String>,
    pub minimum: SerializableTier,
    pub recommended: SerializableTier,
    pub storage_gb: u16,
    pub api: String,
    pub minimum_label: String,
    pub recommended_label: String,
    pub linux: String,
    pub issues: Vec<String>,
    pub steam_app_id: Option<u32>,
    pub protondb_tier: Option<String>,
    pub release_date: Option<String>,
    pub engine: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableTier {
    pub cpu: u16,
    pub gpu: u16,
    pub ram_gb: u16,
    pub vram_gb: u16,
}

impl From<Tier> for SerializableTier {
    fn from(tier: Tier) -> Self {
        Self {
            cpu: tier.cpu,
            gpu: tier.gpu,
            ram_gb: tier.ram_gb,
            vram_gb: tier.vram_gb,
        }
    }
}

impl From<SerializableTier> for Tier {
    fn from(tier: SerializableTier) -> Self {
        Self {
            cpu: tier.cpu,
            gpu: tier.gpu,
            ram_gb: tier.ram_gb,
            vram_gb: tier.vram_gb,
        }
    }
}

impl From<LinuxSupport> for String {
    fn from(support: LinuxSupport) -> Self {
        match support {
            LinuxSupport::Native => "native".to_string(),
            LinuxSupport::ProtonGood => "proton_good".to_string(),
            LinuxSupport::ProtonMixed => "proton_mixed".to_string(),
        }
    }
}

impl From<String> for LinuxSupport {
    fn from(s: String) -> Self {
        match s.as_str() {
            "native" => LinuxSupport::Native,
            "proton_good" => LinuxSupport::ProtonGood,
            "proton_mixed" => LinuxSupport::ProtonMixed,
            _ => LinuxSupport::ProtonMixed,
        }
    }
}

/// Error type for catalog update operations
#[derive(Debug)]
pub enum CatalogUpdateError {
    IoError(String),
    ParseError(String),
    ApiError(String),
    FeatureNotEnabled,
}

impl std::fmt::Display for CatalogUpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CatalogUpdateError::IoError(msg) => write!(f, "IO error: {}", msg),
            CatalogUpdateError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CatalogUpdateError::ApiError(msg) => write!(f, "API error: {}", msg),
            CatalogUpdateError::FeatureNotEnabled => write!(f, "Catalog update feature not enabled. Build with --features catalog-update"),
        }
    }
}

impl std::error::Error for CatalogUpdateError {}

/// Get the catalog file path
#[cfg(feature = "catalog-update")]
fn get_catalog_path() -> Result<PathBuf, CatalogUpdateError> {
    #[cfg(feature = "config")]
    {
        use dirs::config_dir;
        let config_dir = config_dir()
            .ok_or_else(|| CatalogUpdateError::IoError("Could not find config directory".to_string()))?;
        let ciri_dir = config_dir.join("ciri");
        fs::create_dir_all(&ciri_dir)
            .map_err(|e| CatalogUpdateError::IoError(e.to_string()))?;
        Ok(ciri_dir.join("catalog.json"))
    }
    
    #[cfg(not(feature = "config"))]
    {
        Ok(PathBuf::from("catalog.json"))
    }
}

#[cfg(not(feature = "catalog-update"))]
fn get_catalog_path() -> Result<PathBuf, CatalogUpdateError> {
    Err(CatalogUpdateError::FeatureNotEnabled)
}

/// Load the catalog from disk
#[cfg(feature = "catalog-update")]
pub fn load_catalog() -> Result<Catalog, CatalogUpdateError> {
    let path = get_catalog_path()?;
    
    if !path.exists() {
        // Return empty catalog if file doesn't exist
        return Ok(Catalog {
            version: "0.2.0".to_string(),
            last_updated: Utc::now(),
            games: Vec::new(),
        });
    }
    
    let content = fs::read_to_string(&path)
        .map_err(|e| CatalogUpdateError::IoError(e.to_string()))?;
    
    let catalog: Catalog = serde_json::from_str(&content)
        .map_err(|e| CatalogUpdateError::ParseError(e.to_string()))?;
    
    Ok(catalog)
}

/// Save the catalog to disk
#[cfg(feature = "catalog-update")]
pub fn save_catalog(catalog: &Catalog) -> Result<(), CatalogUpdateError> {
    let path = get_catalog_path()?;
    
    let content = serde_json::to_string_pretty(catalog)
        .map_err(|e| CatalogUpdateError::ParseError(e.to_string()))?;
    
    fs::write(&path, content)
        .map_err(|e| CatalogUpdateError::IoError(e.to_string()))?;
    
    Ok(())
}

/// Update the catalog from online sources
#[cfg(feature = "catalog-update")]
pub async fn update_catalog() -> Result<CatalogUpdateResult, CatalogUpdateError> {
    let mut catalog = load_catalog()?;
    let mut games_updated = 0;
    let mut games_added = 0;
    let mut games_failed = Vec::new();
    let mut sources_used = Vec::new();
    
    // Update existing games from embedded catalog
    for game in crate::catalog::embedded::GAMES {
        let game_id = game.id;
        
        // Try to find Steam app ID for the game
        let steam_app_id = search_steam_games(game.title).await
            .map(|ids| ids.first().copied())
            .ok()
            .flatten();
        
        if let Some(app_id) = steam_app_id {
            sources_used.push("Steam".to_string());
            
            // Fetch Steam game details
            if let Ok(steam_game) = fetch_steam_game(app_id).await {
                sources_used.push("ProtonDB".to_string());
                
                // Fetch ProtonDB rating
                let protondb_tier = fetch_protondb_rating(app_id).await
                    .map(|rating| rating.tier)
                    .ok();
                
                // Create or update game in catalog
                let serializable_game = SerializableGame {
                    id: game_id.to_string(),
                    title: game.title.to_string(),
                    aliases: game.aliases.iter().map(|s| s.to_string()).collect(),
                    minimum: game.minimum.into(),
                    recommended: game.recommended.into(),
                    storage_gb: game.storage_gb,
                    api: game.api.to_string(),
                    minimum_label: game.minimum_label.to_string(),
                    recommended_label: game.recommended_label.to_string(),
                    linux: game.linux.into(),
                    issues: game.issues.iter().map(|s| s.to_string()).collect(),
                    steam_app_id: Some(app_id),
                    protondb_tier,
                    release_date: Some(steam_game.release_date.date),
                    engine: None, // Could be extracted from steam_game.genres
                };
                
                // Check if game already exists
                if let Some(existing) = catalog.games.iter().find(|g| g.id == game_id) {
                    if existing.steam_app_id != serializable_game.steam_app_id || 
                       existing.protondb_tier != serializable_game.protondb_tier {
                        *catalog.games.iter_mut().find(|g| g.id == game_id).unwrap() = serializable_game;
                        games_updated += 1;
                    }
                } else {
                    catalog.games.push(serializable_game);
                    games_added += 1;
                }
            } else {
                games_failed.push(game.title.to_string());
            }
        } else {
            games_failed.push(game.title.to_string());
        }
    }
    
    // Update catalog metadata
    catalog.last_updated = Utc::now();
    catalog.version = "0.2.0".to_string();
    
    // Save updated catalog
    save_catalog(&catalog)?;
    
    Ok(CatalogUpdateResult {
        timestamp: Utc::now(),
        games_updated,
        games_added,
        games_failed,
        sources_used: sources_used.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect(),
    })
}

#[cfg(not(feature = "catalog-update"))]
pub async fn update_catalog() -> Result<CatalogUpdateResult, CatalogUpdateError> {
    Err(CatalogUpdateError::FeatureNotEnabled)
}

#[cfg(not(feature = "catalog-update"))]
pub fn load_catalog() -> Result<Catalog, CatalogUpdateError> {
    Err(CatalogUpdateError::FeatureNotEnabled)
}

#[cfg(not(feature = "catalog-update"))]
pub fn save_catalog(_catalog: &Catalog) -> Result<(), CatalogUpdateError> {
    Err(CatalogUpdateError::FeatureNotEnabled)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serializable_tier_conversion() {
        let tier = Tier {
            cpu: 50,
            gpu: 60,
            ram_gb: 16,
            vram_gb: 8,
        };
        let serializable: SerializableTier = tier.into();
        assert_eq!(serializable.cpu, 50);
        assert_eq!(serializable.gpu, 60);
        
        let back: Tier = serializable.into();
        assert_eq!(back.cpu, 50);
        assert_eq!(back.gpu, 60);
    }

    #[test]
    fn test_linux_support_conversion() {
        let support = LinuxSupport::Native;
        let s: String = support.into();
        assert_eq!(s, "native");
        
        let back: LinuxSupport = s.into();
        assert_eq!(back, LinuxSupport::Native);
    }

    #[cfg(feature = "catalog-update")]
    #[tokio::test]
    async fn test_catalog_update() {
        let result = update_catalog().await;
        assert!(result.is_ok());
        let update_result = result.unwrap();
        println!("Updated {} games, added {} games", update_result.games_updated, update_result.games_added);
    }
}