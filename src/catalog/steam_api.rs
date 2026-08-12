//! Steam API integration for fetching game metadata
//! 
//! This module provides functionality to fetch game information from Steam's public API.
//! Note: This requires the "steam" feature to be enabled.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamGame {
    pub steam_appid: u32,
    pub name: String,
    pub required_age: u16,
    pub is_free: bool,
    pub controller_support: String,
    pub dlcs: HashMap<u32, String>,
    pub detailed_description: String,
    pub about_the_game: String,
    pub short_description: String,
    pub supported_languages: String,
    pub header_image: String,
    pub website: String,
    pub pc_requirements: Option<Requirements>,
    pub mac_requirements: Option<Requirements>,
    pub linux_requirements: Option<Requirements>,
    pub legal_notice: String,
    pub developers: Vec<String>,
    pub publishers: Vec<String>,
    pub price_overview: Option<PriceOverview>,
    pub packages: Vec<u32>,
    pub package_groups: Vec<PackageGroup>,
    pub platforms: Platforms,
    pub metacritic: Option<Metacritic>,
    pub categories: Vec<Category>,
    pub genres: Vec<Genre>,
    pub screenshots: Vec<Screenshot>,
    pub movies: Vec<Movie>,
    pub recommendations: Recommendations,
    pub achievements: Option<Achievements>,
    pub release_date: ReleaseDate,
    pub background: String,
    pub content_descriptors: Vec<ContentDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirements {
    pub minimum: String,
    pub recommended: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceOverview {
    pub currency: String,
    pub initial: u32,
    pub r#final: u32,
    pub discount_percent: u16,
    pub initial_formatted: String,
    pub final_formatted: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageGroup {
    pub name: String,
    pub title: String,
    pub description: String,
    pub selection_text: String,
    pub save_text: String,
    pub display_type: u32,
    pub is_recurring_subscription: String,
    pub subs: Vec<Sub>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sub {
    pub packageid: u32,
    pub percent_savings_text: String,
    pub percent_savings: u32,
    pub option_text: String,
    pub option_description: String,
    pub can_get_free_license: String,
    pub price_in_cents_with_discount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Platforms {
    pub windows: bool,
    pub mac: bool,
    pub linux: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metacritic {
    pub score: u16,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: u32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    pub id: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Screenshot {
    pub id: u32,
    pub path_thumbnail: String,
    pub path_full: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub id: u32,
    pub name: String,
    pub thumbnail: String,
    pub mp4: Option<Mp4>,
    pub webm: Option<Webm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mp4 {
    pub max: String,
    pub min: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webm {
    pub max: String,
    pub min: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendations {
    pub total: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievements {
    pub total: u32,
    pub highlighted: Vec<Achievement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub name: String,
    pub percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseDate {
    pub coming_soon: bool,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentDescriptor {
    pub id: u32,
    pub description: String,
}

/// Error type for Steam API operations
#[derive(Debug)]
pub enum SteamApiError {
    RequestFailed(String),
    ParseError(String),
    GameNotFound(String),
    FeatureNotEnabled,
}

impl std::fmt::Display for SteamApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SteamApiError::RequestFailed(msg) => write!(f, "Steam API request failed: {}", msg),
            SteamApiError::ParseError(msg) => write!(f, "Failed to parse Steam API response: {}", msg),
            SteamApiError::GameNotFound(msg) => write!(f, "Game not found on Steam: {}", msg),
            SteamApiError::FeatureNotEnabled => write!(f, "Steam feature not enabled. Build with --features steam"),
        }
    }
}

impl std::error::Error for SteamApiError {}

/// Fetch game details from Steam API by app ID
/// 
/// This function uses Steam's public app details API which doesn't require authentication.
/// # Arguments
/// * `app_id` - The Steam app ID to fetch details for
/// 
/// # Returns
/// * `Ok(SteamGame)` - The game details
/// * `Err(SteamApiError)` - If the request fails or parsing fails
#[cfg(feature = "steam")]
pub async fn fetch_steam_game(app_id: u32) -> Result<SteamGame, SteamApiError> {
    use reqwest::Client;
    
    let url = format!(
        "https://store.steampowered.com/api/appdetails?appids={}&l=english",
        app_id
    );
    
    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Ciri/0.2.0")
        .send()
        .await
        .map_err(|e| SteamApiError::RequestFailed(e.to_string()))?;
    
    if !response.status().is_success() {
        return Err(SteamApiError::RequestFailed(format!(
            "HTTP {}",
            response.status()
        )));
    }
    
    let text = response
        .text()
        .await
        .map_err(|e| SteamApiError::RequestFailed(e.to_string()))?;
    
    let json: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| SteamApiError::ParseError(e.to_string()))?;
    
    let app_data = json
        .get(app_id.to_string())
        .and_then(|v| v.get("data"))
        .ok_or_else(|| SteamApiError::GameNotFound(app_id.to_string()))?;
    
    let game: SteamGame = serde_json::from_value(app_data.clone())
        .map_err(|e| SteamApiError::ParseError(e.to_string()))?;
    
    Ok(game)
}

/// Search for a game by name and return its Steam app ID
/// 
/// This function searches Steam's store for games matching the given name.
/// # Arguments
/// * `query` - The search query
/// 
/// # Returns
/// * `Ok(Vec<u32>)` - List of matching Steam app IDs
/// * `Err(SteamApiError)` - If the request fails or parsing fails
#[cfg(feature = "steam")]
pub async fn search_steam_games(query: &str) -> Result<Vec<u32>, SteamApiError> {
    use reqwest::Client;
    
    let url = format!(
        "https://store.steampowered.com/api/storesearch/?term={}&l=english&cc=US",
        urlencoding::encode(query)
    );
    
    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Ciri/0.2.0")
        .send()
        .await
        .map_err(|e| SteamApiError::RequestFailed(e.to_string()))?;
    
    if !response.status().is_success() {
        return Err(SteamApiError::RequestFailed(format!(
            "HTTP {}",
            response.status()
        )));
    }
    
    let text = response
        .text()
        .await
        .map_err(|e| SteamApiError::RequestFailed(e.to_string()))?;
    
    let json: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| SteamApiError::ParseError(e.to_string()))?;
    
    let items = json
        .get("items")
        .and_then(|v| v.as_array())
        .ok_or_else(|| SteamApiError::ParseError("No items in response".to_string()))?;
    
    let app_ids: Vec<u32> = items
        .iter()
        .filter_map(|item| item.get("id").and_then(|id| id.as_u64()).map(|id| id as u32))
        .collect();
    
    Ok(app_ids)
}

#[cfg(not(feature = "steam"))]
pub async fn fetch_steam_game(_app_id: u32) -> Result<SteamGame, SteamApiError> {
    Err(SteamApiError::FeatureNotEnabled)
}

#[cfg(not(feature = "steam"))]
pub async fn search_steam_games(_query: &str) -> Result<Vec<u32>, SteamApiError> {
    Err(SteamApiError::FeatureNotEnabled)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steam_api_error_display() {
        let err = SteamApiError::GameNotFound("12345".to_string());
        assert!(err.to_string().contains("Game not found"));
    }

    #[cfg(feature = "steam")]
    #[tokio::test]
    async fn test_fetch_steam_game() {
        // Test with a known game (Portal 2 has app ID 620)
        let result = fetch_steam_game(620).await;
        assert!(result.is_ok());
        let game = result.unwrap();
        assert_eq!(game.steam_appid, 620);
        assert!(game.name.contains("Portal"));
    }

    #[cfg(feature = "steam")]
    #[tokio::test]
    async fn test_search_steam_games() {
        let result = search_steam_games("Portal").await;
        assert!(result.is_ok());
        let app_ids = result.unwrap();
        assert!(!app_ids.is_empty());
        // Portal 2 should be in the results
        assert!(app_ids.contains(&620));
    }
}