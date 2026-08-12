//! ProtonDB API integration for fetching Linux compatibility ratings
//! 
//! This module provides functionality to fetch game compatibility information from ProtonDB.
//! Note: This requires the "protondb" feature to be enabled.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtonDBRating {
    pub tier: String,
    pub confidence: String,
    pub trending_score: f32,
    pub reviews: Vec<Review>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: String,
    pub rating: String,
    pub fps: Option<f32>,
    pub cpu: Option<String>,
    pub gpu: Option<String>,
    pub ram: Option<String>,
    pub os: String,
    pub proton_version: String,
    pub timestamp: String,
}

/// Error type for ProtonDB API operations
#[derive(Debug)]
pub enum ProtonDBError {
    RequestFailed(String),
    ParseError(String),
    GameNotFound(String),
    FeatureNotEnabled,
}

impl std::fmt::Display for ProtonDBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtonDBError::RequestFailed(msg) => write!(f, "ProtonDB API request failed: {}", msg),
            ProtonDBError::ParseError(msg) => write!(f, "Failed to parse ProtonDB response: {}", msg),
            ProtonDBError::GameNotFound(msg) => write!(f, "Game not found on ProtonDB: {}", msg),
            ProtonDBError::FeatureNotEnabled => write!(f, "ProtonDB feature not enabled. Build with --features protondb"),
        }
    }
}

impl std::error::Error for ProtonDBError {}

/// Fetch ProtonDB rating for a game by its Steam app ID
/// 
/// # Arguments
/// * `app_id` - The Steam app ID
/// 
/// # Returns
/// * `Ok(ProtonDBRating)` - The ProtonDB compatibility rating
/// * `Err(ProtonDBError)` - If the request fails or parsing fails
#[cfg(feature = "protondb")]
pub async fn fetch_protondb_rating(app_id: u32) -> Result<ProtonDBRating, ProtonDBError> {
    use reqwest::Client;
    
    let url = format!("https://www.protondb.com/api/v1/reports/summaries/{}.json", app_id);
    
    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Ciri/0.2.0")
        .send()
        .await
        .map_err(|e| ProtonDBError::RequestFailed(e.to_string()))?;
    
    if !response.status().is_success() {
        return Err(ProtonDBError::GameNotFound(app_id.to_string()));
    }
    
    let text = response
        .text()
        .await
        .map_err(|e| ProtonDBError::RequestFailed(e.to_string()))?;
    
    let rating: ProtonDBRating = serde_json::from_str(&text)
        .map_err(|e| ProtonDBError::ParseError(e.to_string()))?;
    
    Ok(rating)
}

#[cfg(not(feature = "protondb"))]
pub async fn fetch_protondb_rating(_app_id: u32) -> Result<ProtonDBRating, ProtonDBError> {
    Err(ProtonDBError::FeatureNotEnabled)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protondb_error_display() {
        let err = ProtonDBError::GameNotFound("12345".to_string());
        assert!(err.to_string().contains("Game not found"));
    }

    #[cfg(feature = "protondb")]
    #[tokio::test]
    async fn test_fetch_protondb_rating() {
        // Test with a known game (Portal 2 has app ID 620)
        let result = fetch_protondb_rating(620).await;
        assert!(result.is_ok());
        let rating = result.unwrap();
        assert!(!rating.tier.is_empty());
    }
}