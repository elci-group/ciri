pub mod embedded;
#[cfg(feature = "steam")]
pub mod steam_api;
#[cfg(feature = "protondb")]
pub mod protondb;
#[cfg(feature = "catalog-update")]
pub mod updater;

#[cfg(feature = "steam")]
pub use steam_api::{SteamGame, fetch_steam_game};
#[cfg(feature = "protondb")]
pub use protondb::{ProtonDBRating, fetch_protondb_rating};
#[cfg(feature = "catalog-update")]
pub use updater::{update_catalog, CatalogUpdateResult};

// Re-export existing catalog items
pub use embedded::{Game, Tier, LinuxSupport, GAMES, resolve};

// Enhanced game metadata structure
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GameMetadata {
    pub base: Game,
    pub release_date: Option<String>,
    pub engine: Option<String>,
    #[cfg(feature = "protondb")]
    pub protondb_rating: Option<ProtonDBRating>,
    #[cfg(not(feature = "protondb"))]
    pub protondb_rating: Option<String>,
    pub steam_deck_verified: bool,
    pub steam_app_id: Option<u32>,
    pub known_issues: Vec<String>,
    pub workarounds: Vec<String>,
}

impl GameMetadata {
    #[allow(dead_code)]
    pub fn from_game(game: &Game) -> Self {
        Self {
            base: game.clone(),
            release_date: None,
            engine: None,
            protondb_rating: None,
            steam_deck_verified: false,
            steam_app_id: None,
            known_issues: game.issues.iter().map(|s| s.to_string()).collect(),
            workarounds: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_metadata_creation() {
        let game = &GAMES[0];
        let metadata = GameMetadata::from_game(game);
        assert_eq!(metadata.base.id, game.id);
        assert!(!metadata.known_issues.is_empty());
    }
}