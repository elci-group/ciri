//! Predictive analytics for upgrade planning and performance forecasting
//! 
//! This module provides analytics for future compatibility and upgrade recommendations.

use crate::hardware::Hardware;
use crate::catalog::Game;
use serde::{Deserialize, Serialize};

/// Upgrade recommendation analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeRecommendation {
    pub component: ComponentType,
    pub current_score: u16,
    pub recommended_score: u16,
    pub performance_gain: f32,
    pub cost_benefit_ratio: f32,
    pub specific_recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentType {
    CPU,
    GPU,
    RAM,
    Storage,
}

/// Future game compatibility analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureCompatibility {
    pub game_id: String,
    pub expected_requirements: HardwareRequirements,
    pub current_readiness: ReadinessLevel,
    pub upgrade_needed: bool,
    pub estimated_upgrade_cost: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareRequirements {
    pub min_cpu_score: u16,
    pub min_gpu_score: u16,
    pub min_ram_gb: u16,
    pub min_vram_gb: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReadinessLevel {
    Ready,
    MinorUpgrade,
    ModerateUpgrade,
    MajorUpgrade,
    NotReady,
}

/// Performance trending analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrend {
    pub component: ComponentType,
    pub historical_performance: Vec<f32>,
    pub trend_direction: TrendDirection,
    pub degradation_rate: f32,
    pub predicted_lifetime_months: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Stable,
    Declining,
}

impl UpgradeRecommendation {
    pub fn analyze(hw: &Hardware, target_games: &[&Game]) -> Vec<UpgradeRecommendation> {
        let mut recommendations = Vec::new();
        
        // Analyze CPU
        let cpu_rec = Self::analyze_cpu(hw, target_games);
        recommendations.push(cpu_rec);
        
        // Analyze GPU
        let gpu_rec = Self::analyze_gpu(hw, target_games);
        recommendations.push(gpu_rec);
        
        // Analyze RAM
        let ram_rec = Self::analyze_ram(hw, target_games);
        recommendations.push(ram_rec);
        
        // Analyze Storage
        let storage_rec = Self::analyze_storage(hw, target_games);
        recommendations.push(storage_rec);
        
        recommendations
    }
    
    fn analyze_cpu(hw: &Hardware, games: &[&Game]) -> UpgradeRecommendation {
        let max_required_cpu = games.iter().map(|g| g.recommended.cpu).max().unwrap_or(50);
        let current_score = hw.cpu_score;
        let recommended_score = max_required_cpu;
        
        let performance_gain = if current_score < recommended_score {
            ((recommended_score - current_score) as f32 / current_score as f32) * 100.0
        } else {
            0.0
        };
        
        let cost_benefit_ratio = if performance_gain > 20.0 {
            0.8
        } else if performance_gain > 10.0 {
            0.6
        } else {
            0.4
        };
        
        let specific_recommendations = if current_score < recommended_score {
            vec![
                format!("Current CPU score {} below recommended {}", current_score, recommended_score),
                "Consider upgrading to a modern CPU with better single-core performance".to_string(),
                "Look for CPUs with higher clock speeds and more cores".to_string(),
            ]
        } else {
            vec!["CPU meets requirements for target games".to_string()]
        };
        
        UpgradeRecommendation {
            component: ComponentType::CPU,
            current_score,
            recommended_score,
            performance_gain,
            cost_benefit_ratio,
            specific_recommendations,
        }
    }
    
    fn analyze_gpu(hw: &Hardware, games: &[&Game]) -> UpgradeRecommendation {
        let max_required_gpu = games.iter().map(|g| g.recommended.gpu).max().unwrap_or(50);
        let current_score = hw.gpu_score;
        let recommended_score = max_required_gpu;
        
        let performance_gain = if current_score < recommended_score {
            ((recommended_score - current_score) as f32 / current_score.max(1) as f32) * 100.0
        } else {
            0.0
        };
        
        let cost_benefit_ratio = if performance_gain > 30.0 {
            0.9
        } else if performance_gain > 15.0 {
            0.7
        } else {
            0.5
        };
        
        let specific_recommendations = if current_score < recommended_score {
            vec![
                format!("Current GPU score {} below recommended {}", current_score, recommended_score),
                "Consider upgrading to a modern GPU with more VRAM".to_string(),
                "Look for GPUs with ray tracing support for future-proofing".to_string(),
            ]
        } else {
            vec!["GPU meets requirements for target games".to_string()]
        };
        
        UpgradeRecommendation {
            component: ComponentType::GPU,
            current_score,
            recommended_score,
            performance_gain,
            cost_benefit_ratio,
            specific_recommendations,
        }
    }
    
    fn analyze_ram(hw: &Hardware, games: &[&Game]) -> UpgradeRecommendation {
        let max_required_ram = games.iter().map(|g| g.recommended.ram_gb).max().unwrap_or(16);
        let current_gb = hw.ram_gb;
        let recommended_gb = max_required_ram;
        
        let performance_gain = if current_gb < recommended_gb {
            ((recommended_gb - current_gb) as f32 / current_gb as f32) * 50.0
        } else {
            0.0
        };
        
        let cost_benefit_ratio = if performance_gain > 25.0 {
            0.7
        } else if performance_gain > 10.0 {
            0.5
        } else {
            0.3
        };
        
        let specific_recommendations = if current_gb < recommended_gb {
            vec![
                format!("Current RAM {}GB below recommended {}GB", current_gb, recommended_gb),
                "Consider upgrading to 32GB for modern gaming".to_string(),
                "Ensure dual-channel configuration for best performance".to_string(),
            ]
        } else {
            vec!["RAM meets requirements for target games".to_string()]
        };
        
        UpgradeRecommendation {
            component: ComponentType::RAM,
            current_score: (current_gb / 32).min(100) as u16,
            recommended_score: (recommended_gb / 32).min(100) as u16,
            performance_gain,
            cost_benefit_ratio,
            specific_recommendations,
        }
    }
    
    fn analyze_storage(hw: &Hardware, games: &[&Game]) -> UpgradeRecommendation {
        let max_required_storage = games.iter().map(|g| g.storage_gb).max().unwrap_or(100);
        let current_gb = hw.storage_gb;
        let recommended_gb = max_required_storage + 50; // Add buffer
        
        let performance_gain = if current_gb < recommended_gb {
            ((recommended_gb - current_gb) as f32 / current_gb.max(1) as f32) * 20.0
        } else {
            0.0
        };
        
        let cost_benefit_ratio = if performance_gain > 15.0 {
            0.6
        } else if performance_gain > 5.0 {
            0.4
        } else {
            0.2
        };
        
        let specific_recommendations = if current_gb < recommended_gb {
            vec![
                format!("Current storage {}GB below recommended {}GB", current_gb, recommended_gb),
                "Consider adding additional storage or upgrading to SSD".to_string(),
                "SSD storage significantly improves game loading times".to_string(),
            ]
        } else {
            vec!["Storage meets requirements for target games".to_string()]
        };
        
        UpgradeRecommendation {
            component: ComponentType::Storage,
            current_score: (current_gb / 10).min(100) as u16,
            recommended_score: (recommended_gb / 10).min(100) as u16,
            performance_gain,
            cost_benefit_ratio,
            specific_recommendations,
        }
    }
}

impl FutureCompatibility {
    pub fn analyze(game: &Game, hw: &Hardware) -> Self {
        let expected_requirements = HardwareRequirements {
            min_cpu_score: game.minimum.cpu,
            min_gpu_score: game.minimum.gpu,
            min_ram_gb: game.minimum.ram_gb,
            min_vram_gb: game.minimum.vram_gb,
        };
        
        let current_readiness = Self::assess_readiness(hw, &expected_requirements);
        let upgrade_needed = !matches!(current_readiness, ReadinessLevel::Ready);
        
        let estimated_upgrade_cost = match current_readiness {
            ReadinessLevel::Ready => "$0".to_string(),
            ReadinessLevel::MinorUpgrade => "$100-300".to_string(),
            ReadinessLevel::ModerateUpgrade => "$300-800".to_string(),
            ReadinessLevel::MajorUpgrade => "$800-2000".to_string(),
            ReadinessLevel::NotReady => "$2000+".to_string(),
        };
        
        Self {
            game_id: game.id.to_string(),
            expected_requirements,
            current_readiness,
            upgrade_needed,
            estimated_upgrade_cost,
        }
    }
    
    fn assess_readiness(hw: &Hardware, req: &HardwareRequirements) -> ReadinessLevel {
        let cpu_ok = hw.cpu_score >= req.min_cpu_score;
        let gpu_ok = hw.gpu_score >= req.min_gpu_score;
        let ram_ok = hw.ram_gb >= req.min_ram_gb;
        let vram_ok = hw.vram_gb.unwrap_or(0) >= req.min_vram_gb;
        
        let failures = [!cpu_ok, !gpu_ok, !ram_ok, !vram_ok].iter().filter(|&&x| x).count();
        
        match failures {
            0 => ReadinessLevel::Ready,
            1 => ReadinessLevel::MinorUpgrade,
            2 => ReadinessLevel::ModerateUpgrade,
            3 => ReadinessLevel::MajorUpgrade,
            _ => ReadinessLevel::NotReady,
        }
    }
}

impl PerformanceTrend {
    pub fn analyze(hw: &Hardware, component: ComponentType) -> Self {
        // In a real implementation, this would analyze historical performance data
        // For now, provide simulated trends based on current hardware
        
        let (historical_performance, trend_direction, degradation_rate) = match component {
            ComponentType::CPU => {
                let score = hw.cpu_score as f32;
                let trend = if hw.is_laptop {
                    TrendDirection::Declining
                } else {
                    TrendDirection::Stable
                };
                let degradation = if hw.is_laptop { 0.02 } else { 0.0 };
                (vec![score, score * 0.98, score * 0.96], trend, degradation)
            }
            ComponentType::GPU => {
                let score = hw.gpu_score as f32;
                let trend = TrendDirection::Stable;
                let degradation = 0.01;
                (vec![score, score * 0.99, score * 0.98], trend, degradation)
            }
            ComponentType::RAM => {
                let gb = hw.ram_gb as f32;
                let trend = TrendDirection::Stable;
                let degradation = 0.0;
                (vec![gb, gb, gb], trend, degradation)
            }
            ComponentType::Storage => {
                let gb = hw.storage_gb as f32;
                let trend = TrendDirection::Declining;
                let degradation = 0.05;
                (vec![gb, gb * 0.95, gb * 0.9], trend, degradation)
            }
        };
        
        let predicted_lifetime_months = match trend_direction {
            TrendDirection::Declining => (24.0 / (degradation_rate * 100.0)).max(12.0) as u16,
            TrendDirection::Stable => 36,
            TrendDirection::Improving => 48,
        };
        
        PerformanceTrend {
            component,
            historical_performance,
            trend_direction,
            degradation_rate,
            predicted_lifetime_months,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upgrade_recommendations() {
        let hw = crate::hardware::Hardware {
            cpu_name: "Test CPU".into(),
            cpu_score: 40,
            logical_cores: 4,
            gpu_name: "Test GPU".into(),
            gpu_score: 30,
            vram_gb: Some(4),
            ram_gb: 8,
            storage_gb: 100,
            os: "Linux".into(),
            is_linux: true,
            is_laptop: false,
            vulkan: true,
        };
        
        let games = &crate::catalog::GAMES;
        let recommendations = UpgradeRecommendation::analyze(&hw, games);
        
        assert_eq!(recommendations.len(), 4);
        assert!(recommendations.iter().any(|r| matches!(r.component, ComponentType::CPU)));
    }

    #[test]
    fn test_future_compatibility() {
        let hw = crate::hardware::Hardware {
            cpu_name: "Test CPU".into(),
            cpu_score: 40,
            logical_cores: 4,
            gpu_name: "Test GPU".into(),
            gpu_score: 30,
            vram_gb: Some(4),
            ram_gb: 8,
            storage_gb: 100,
            os: "Linux".into(),
            is_linux: true,
            is_laptop: false,
            vulkan: true,
        };
        
        let game = &crate::catalog::GAMES[1]; // Cyberpunk 2077
        let compatibility = FutureCompatibility::analyze(game, &hw);
        
        assert!(compatibility.upgrade_needed);
        assert!(!compatibility.estimated_upgrade_cost.is_empty());
    }

    #[test]
    fn test_performance_trend() {
        let hw = crate::hardware::Hardware {
            cpu_name: "Test CPU".into(),
            cpu_score: 70,
            logical_cores: 8,
            gpu_name: "Test GPU".into(),
            gpu_score: 70,
            vram_gb: Some(8),
            ram_gb: 16,
            storage_gb: 500,
            os: "Linux".into(),
            is_linux: true,
            is_laptop: false,
            vulkan: true,
        };
        
        let trend = PerformanceTrend::analyze(&hw, ComponentType::CPU);
        assert!(!trend.historical_performance.is_empty());
        assert!(trend.predicted_lifetime_months > 0);
    }
}