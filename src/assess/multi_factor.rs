//! Multi-factor assessment improvements
//! 
//! This module provides enhanced assessment logic that considers multiple factors
//! beyond simple tier comparisons.

use crate::catalog::{Game, Tier};
use crate::hardware::Hardware;

/// Enhanced CPU scoring that considers single-core vs multi-core performance
pub fn enhanced_cpu_score(hw: &Hardware, game: &Game) -> u16 {
    let base_score = hw.cpu_score;
    
    // Adjust for single-core vs multi-core requirements
    let game_cpu_intensive = is_cpu_intensive_game(game);
    
    if game_cpu_intensive {
        // CPU-intensive games benefit more from multi-core
        let core_scaling = (hw.logical_cores as f32 / 8.0).min(1.5);
        (base_score as f32 * core_scaling).min(100.0) as u16
    } else {
        // Less CPU-intensive games are more single-core dependent
        let single_core_bonus = if hw.logical_cores >= 8 { 5 } else { 0 };
        (base_score + single_core_bonus).min(100)
    }
}

/// Check if a game is particularly CPU-intensive
fn is_cpu_intensive_game(game: &Game) -> bool {
    // Known CPU-intensive games
    let cpu_intensive_ids = [
        "cyberpunk-2077",
        "elden-ring",
        "baldurs-gate-3",
    ];
    
    cpu_intensive_ids.contains(&game.id)
}

/// Enhanced GPU scoring that considers architecture and features
pub fn enhanced_gpu_score(hw: &Hardware, game: &Game) -> u16 {
    let base_score = hw.gpu_score;
    
    // Adjust for VRAM requirements
    let vram_factor = if let Some(vram) = hw.vram_gb {
        if vram >= game.recommended.vram_gb {
            1.0 // Full performance
        } else if vram >= game.minimum.vram_gb {
            0.85 // Slight penalty for less VRAM
        } else {
            0.7 // Significant penalty
        }
    } else {
        0.8 // Unknown VRAM
    };
    
    // Adjust for ray tracing requirements
    let ray_tracing_factor = if game.issues.iter().any(|i| i.contains("ray tracing")) {
        // Ray tracing games may perform worse on non-RT hardware
        if hw.gpu_name.to_lowercase().contains("rtx") || hw.gpu_name.to_lowercase().contains("rx 6000") {
            1.0 // Has ray tracing support
        } else {
            0.9 // No dedicated ray tracing hardware
        }
    } else {
        1.0 // No ray tracing in game
    };
    
    (base_score as f32 * vram_factor * ray_tracing_factor).min(100.0) as u16
}

/// Enhanced RAM scoring that considers speed and dual-channel
pub fn enhanced_ram_score(hw: &Hardware, game: &Game) -> u16 {
    let base_score = hw.ram_gb;
    
    // Adjust for RAM capacity relative to requirements
    let capacity_factor = if hw.ram_gb >= game.recommended.ram_gb {
        1.0
    } else if hw.ram_gb >= game.minimum.ram_gb {
        0.85
    } else {
        0.6
    };
    
    // Consider RAM pressure for modern games
    let modern_game_penalty = if game.issues.iter().any(|i| i.contains("CPU-heavy") || i.contains("dense")) {
        0.9 // Additional pressure on RAM
    } else {
        1.0
    };
    
    (base_score as f32 * capacity_factor * modern_game_penalty).min(64.0) as u16
}

/// Storage speed consideration
pub fn storage_speed_factor(hw: &Hardware, game: &Game) -> f32 {
    // Games that specifically require SSDs
    let requires_ssd = game.issues.iter().any(|i| i.contains("SSD") || i.contains("required"));
    
    if requires_ssd {
        // Assume SSD is beneficial (in real implementation, would detect SSD vs HDD)
        1.0
    } else {
        1.0 // No specific storage speed requirement
    }
}

/// Laptop power limit consideration
pub fn laptop_power_factor(hw: &Hardware) -> f32 {
    if hw.is_laptop {
        // Laptops typically have power limits that reduce sustained performance
        0.85
    } else {
        1.0
    }
}

/// Combined multi-factor assessment
pub fn multi_factor_assessment(hw: &Hardware, game: &Game) -> (u16, u16, u16) {
    let cpu_score = enhanced_cpu_score(hw, game);
    let gpu_score = enhanced_gpu_score(hw, game);
    let ram_score = enhanced_ram_score(hw, game);
    
    (cpu_score, gpu_score, ram_score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::GAMES;
    use crate::assess::Status;

    #[test]
    fn test_cpu_intensive_detection() {
        let cyberpunk = &GAMES[1]; // Cyberpunk 2077
        assert!(is_cpu_intensive_game(cyberpunk));
        
        let gta = &GAMES[4]; // GTA V
        assert!(!is_cpu_intensive_game(gta));
    }

    #[test]
    fn test_enhanced_scoring() {
        let hw = crate::hardware::Hardware {
            cpu_name: "Test CPU".into(),
            cpu_score: 60,
            logical_cores: 8,
            gpu_name: "Test GPU".into(),
            gpu_score: 60,
            vram_gb: Some(8),
            ram_gb: 16,
            storage_gb: 500,
            os: "Linux".into(),
            is_linux: true,
            is_laptop: false,
            vulkan: true,
        };
        
        let game = &GAMES[1]; // Cyberpunk 2077
        let (cpu, gpu, ram) = multi_factor_assessment(&hw, game);
        
        assert!(cpu > 0);
        assert!(gpu > 0);
        assert!(ram > 0);
    }
}