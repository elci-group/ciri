//! Advanced diagnostics for system health and troubleshooting
//! 
//! This module provides comprehensive system diagnostics and remediation guidance.

use crate::hardware::Hardware;
use serde::{Deserialize, Serialize};

/// System health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub overall_status: HealthStatus,
    pub cpu_health: ComponentHealth,
    pub gpu_health: ComponentHealth,
    pub memory_health: ComponentHealth,
    pub storage_health: ComponentHealth,
    pub driver_status: DriverStatus,
    pub thermal_status: ThermalStatus,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub status: HealthStatus,
    pub score: u8,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverStatus {
    pub gpu_driver: Option<String>,
    pub vulkan_support: bool,
    pub directx_support: bool,
    pub outdated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalStatus {
    pub cpu_temp: Option<f32>,
    pub gpu_temp: Option<f32>,
    pub thermal_throttling: bool,
}

/// Game-specific diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDiagnostics {
    pub game_id: String,
    pub compatibility_issues: Vec<CompatibilityIssue>,
    pub known_fixes: Vec<String>,
    pub community_solutions: Vec<String>,
    pub configuration_recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityIssue {
    pub severity: IssueSeverity,
    pub description: String,
    pub category: IssueCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueCategory {
    Hardware,
    Software,
    Configuration,
    Network,
    AntiCheat,
}

/// Remediation guidance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationGuidance {
    pub issue: String,
    pub automated_fix: Option<AutomatedFix>,
    pub manual_steps: Vec<String>,
    pub community_links: Vec<String>,
    pub estimated_fix_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomatedFix {
    pub fix_type: FixType,
    pub description: String,
    pub safe: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FixType {
    ConfigChange,
    DriverUpdate,
    SystemSetting,
    FileModification,
}

impl SystemHealth {
    pub fn diagnose(hw: &Hardware) -> Self {
        let cpu_health = Self::check_cpu_health(hw);
        let gpu_health = Self::check_gpu_health(hw);
        let memory_health = Self::check_memory_health(hw);
        let storage_health = Self::check_storage_health(hw);
        let driver_status = Self::check_driver_status(hw);
        let thermal_status = Self::check_thermal_status();
        
        let overall_status = Self::calculate_overall_status(&[
            &cpu_health, &gpu_health, &memory_health, &storage_health
        ]);
        
        let recommendations = Self::generate_recommendations(&cpu_health, &gpu_health, &memory_health, &storage_health);
        
        Self {
            overall_status,
            cpu_health,
            gpu_health,
            memory_health,
            storage_health,
            driver_status,
            thermal_status,
            recommendations,
        }
    }
    
    fn check_cpu_health(hw: &Hardware) -> ComponentHealth {
        let status = if hw.cpu_score >= 50 {
            HealthStatus::Healthy
        } else if hw.cpu_score >= 30 {
            HealthStatus::Warning
        } else {
            HealthStatus::Critical
        };
        
        let score = hw.cpu_score as u8;
        let mut issues = Vec::new();
        
        if hw.cpu_score < 30 {
            issues.push("CPU performance below minimum requirements for modern games".to_string());
        }
        
        ComponentHealth { status, score, issues }
    }
    
    fn check_gpu_health(hw: &Hardware) -> ComponentHealth {
        let status = if hw.gpu_score >= 50 {
            HealthStatus::Healthy
        } else if hw.gpu_score >= 30 {
            HealthStatus::Warning
        } else {
            HealthStatus::Critical
        };
        
        let score = hw.gpu_score as u8;
        let mut issues = Vec::new();
        
        if hw.gpu_score == 0 {
            issues.push("GPU not detected or unsupported".to_string());
        } else if hw.gpu_score < 30 {
            issues.push("GPU performance below minimum requirements for modern games".to_string());
        }
        
        if hw.vram_gb.is_none() {
            issues.push("VRAM not detected - performance estimates may be inaccurate".to_string());
        }
        
        ComponentHealth { status, score, issues }
    }
    
    fn check_memory_health(hw: &Hardware) -> ComponentHealth {
        let status = if hw.ram_gb >= 16 {
            HealthStatus::Healthy
        } else if hw.ram_gb >= 8 {
            HealthStatus::Warning
        } else {
            HealthStatus::Critical
        };
        
        let score = (hw.ram_gb / 32).min(100) as u8;
        let mut issues = Vec::new();
        
        if hw.ram_gb < 8 {
            issues.push("RAM below minimum requirements for modern games".to_string());
        } else if hw.ram_gb < 16 {
            issues.push("RAM below recommended for optimal performance".to_string());
        }
        
        ComponentHealth { status, score, issues }
    }
    
    fn check_storage_health(hw: &Hardware) -> ComponentHealth {
        let status = if hw.storage_gb >= 100 {
            HealthStatus::Healthy
        } else if hw.storage_gb >= 50 {
            HealthStatus::Warning
        } else {
            HealthStatus::Critical
        };
        
        let score = (hw.storage_gb / 10).min(100) as u8;
        let mut issues = Vec::new();
        
        if hw.storage_gb < 50 {
            issues.push("Limited storage space - may not fit modern games".to_string());
        }
        
        ComponentHealth { status, score, issues }
    }
    
    fn check_driver_status(hw: &Hardware) -> DriverStatus {
        DriverStatus {
            gpu_driver: Some(hw.gpu_name.clone()),
            vulkan_support: hw.vulkan,
            directx_support: hw.is_linux, // DirectX not supported on Linux
            outdated: false, // Would check driver versions in real implementation
        }
    }
    
    fn check_thermal_status() -> ThermalStatus {
        // In a real implementation, this would read actual thermal sensors
        ThermalStatus {
            cpu_temp: None,
            gpu_temp: None,
            thermal_throttling: false,
        }
    }
    
    fn calculate_overall_status(components: &[&ComponentHealth]) -> HealthStatus {
        let critical_count = components.iter().filter(|c| c.status == HealthStatus::Critical).count();
        let warning_count = components.iter().filter(|c| c.status == HealthStatus::Warning).count();
        
        if critical_count > 0 {
            HealthStatus::Critical
        } else if warning_count >= 2 {
            HealthStatus::Warning
        } else if warning_count == 1 {
            HealthStatus::Warning
        } else {
            HealthStatus::Healthy
        }
    }
    
    fn generate_recommendations(
        cpu: &ComponentHealth,
        gpu: &ComponentHealth,
        memory: &ComponentHealth,
        storage: &ComponentHealth,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if cpu.status == HealthStatus::Critical {
            recommendations.push("Consider upgrading CPU for better performance".to_string());
        }
        
        if gpu.status == HealthStatus::Critical {
            recommendations.push("Consider upgrading GPU for better performance".to_string());
        }
        
        if memory.status == HealthStatus::Warning {
            recommendations.push("Consider upgrading to 16GB+ RAM for modern games".to_string());
        }
        
        if storage.status == HealthStatus::Warning {
            recommendations.push("Free up storage space or install games on larger drive".to_string());
        }
        
        if recommendations.is_empty() {
            recommendations.push("System appears healthy for gaming".to_string());
        }
        
        recommendations
    }
}

impl GameDiagnostics {
    pub fn diagnose_game(game_id: &str, hw: &Hardware) -> Self {
        let compatibility_issues = Self::check_compatibility(game_id, hw);
        let known_fixes = Self::get_known_fixes(game_id);
        let community_solutions = Self::get_community_solutions(game_id);
        let configuration_recommendations = Self::get_config_recommendations(game_id, hw);
        
        Self {
            game_id: game_id.to_string(),
            compatibility_issues,
            known_fixes,
            community_solutions,
            configuration_recommendations,
        }
    }
    
    fn check_compatibility(game_id: &str, hw: &Hardware) -> Vec<CompatibilityIssue> {
        let mut issues = Vec::new();
        
        // Check for common compatibility issues
        if hw.is_linux && (game_id == "grand-theft-auto-v" || game_id == "gta v") {
            issues.push(CompatibilityIssue {
                severity: IssueSeverity::Medium,
                description: "GTA Online anti-cheat may have issues on Linux".to_string(),
                category: IssueCategory::AntiCheat,
            });
        }
        
        if hw.gpu_score < 40 {
            issues.push(CompatibilityIssue {
                severity: IssueSeverity::High,
                description: "GPU may not meet minimum requirements".to_string(),
                category: IssueCategory::Hardware,
            });
        }
        
        if hw.ram_gb < 8 {
            issues.push(CompatibilityIssue {
                severity: IssueSeverity::High,
                description: "RAM below minimum requirements".to_string(),
                category: IssueCategory::Hardware,
            });
        }
        
        issues
    }
    
    fn get_known_fixes(game_id: &str) -> Vec<String> {
        // In a real implementation, this would fetch from a database
        match game_id {
            "cyberpunk-2077" => vec![
                "Disable ray tracing for better performance".to_string(),
                "Use DLSS if supported by GPU".to_string(),
                "Install on SSD for reduced loading times".to_string(),
            ],
            "elden-ring" => vec![
                "Enable game mode for better performance".to_string(),
                "Update GPU drivers for shader compilation fixes".to_string(),
            ],
            _ => vec!["Ensure latest GPU drivers are installed".to_string()],
        }
    }
    
    fn get_community_solutions(game_id: &str) -> Vec<String> {
        // In a real implementation, this would fetch from community forums
        vec![
            format!("Check ProtonDB for {} Linux compatibility", game_id),
            "Consult PCGamingWiki for community fixes".to_string(),
        ]
    }
    
    fn get_config_recommendations(game_id: &str, hw: &Hardware) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if hw.is_laptop {
            recommendations.push("Enable power saving mode may reduce performance".to_string());
        }
        
        if hw.gpu_score < 50 {
            recommendations.push("Consider lowering graphics settings".to_string());
        }
        
        recommendations.push("Start with Medium settings and adjust based on performance".to_string());
        
        recommendations
    }
}

impl RemediationGuidance {
    pub fn generate(issue: &str) -> Self {
        let automated_fix = Self::determine_automated_fix(issue);
        let manual_steps = Self::generate_manual_steps(issue);
        let community_links = Self::get_community_links(issue);
        let estimated_fix_time = Self::estimate_fix_time(issue);
        
        Self {
            issue: issue.to_string(),
            automated_fix,
            manual_steps,
            community_links,
            estimated_fix_time,
        }
    }
    
    fn determine_automated_fix(issue: &str) -> Option<AutomatedFix> {
        // Determine if an automated fix is available and safe
        if issue.contains("driver") {
            Some(AutomatedFix {
                fix_type: FixType::DriverUpdate,
                description: "Update GPU drivers to latest version".to_string(),
                safe: true,
            })
        } else if issue.contains("config") {
            Some(AutomatedFix {
                fix_type: FixType::ConfigChange,
                description: "Modify game configuration file".to_string(),
                safe: true,
            })
        } else {
            None
        }
    }
    
    fn generate_manual_steps(issue: &str) -> Vec<String> {
        vec![
            format!("Identify the specific issue: {}", issue),
            "Check for known solutions in game documentation".to_string(),
            "Verify system requirements are met".to_string(),
            "Test with different graphics settings".to_string(),
        ]
    }
    
    fn get_community_links(issue: &str) -> Vec<String> {
        vec![
            format!("Search for '{}' on Steam Community", issue),
            "Check Reddit for similar issues".to_string(),
            "Consult PCGamingWiki".to_string(),
        ]
    }
    
    fn estimate_fix_time(issue: &str) -> String {
        if issue.contains("driver") {
            "5-10 minutes".to_string()
        } else if issue.contains("config") {
            "2-5 minutes".to_string()
        } else {
            "10-30 minutes".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_health() {
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
        
        let health = SystemHealth::diagnose(&hw);
        assert!(matches!(health.overall_status, HealthStatus::Healthy));
    }

    #[test]
    fn test_game_diagnostics() {
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
        
        let diagnostics = GameDiagnostics::diagnose_game("cyberpunk-2077", &hw);
        assert_eq!(diagnostics.game_id, "cyberpunk-2077");
        assert!(!diagnostics.known_fixes.is_empty());
    }

    #[test]
    fn test_remediation_guidance() {
        let guidance = RemediationGuidance::generate("GPU driver outdated");
        assert!(guidance.automated_fix.is_some());
        assert!(!guidance.manual_steps.is_empty());
    }
}