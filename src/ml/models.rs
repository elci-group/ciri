//! ML models for performance prediction
//! 
//! This module provides the core ML model structures and prediction logic.

use crate::hardware::Hardware;
use crate::catalog::Game;
use serde::{Deserialize, Serialize};

/// Hardware features for ML prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareFeatures {
    pub cpu_score: f32,
    pub gpu_score: f32,
    pub ram_gb: f32,
    pub vram_gb: f32,
    pub target_resolution: String,
    pub quality_preset: String,
    pub is_laptop: bool,
    pub cpu_cores: u16,
}

impl HardwareFeatures {
    pub fn from_hardware(hw: &Hardware, target_resolution: &str, quality_preset: &str) -> Self {
        Self {
            cpu_score: hw.cpu_score as f32,
            gpu_score: hw.gpu_score as f32,
            ram_gb: hw.ram_gb as f32,
            vram_gb: hw.vram_gb.unwrap_or(0) as f32,
            target_resolution: target_resolution.to_string(),
            quality_preset: quality_preset.to_string(),
            is_laptop: hw.is_laptop,
            cpu_cores: hw.logical_cores,
        }
    }
    
    /// Convert to feature vector for ML model
    pub fn to_feature_vector(&self) -> Vec<f32> {
        let mut features = Vec::with_capacity(10);
        
        // Normalized hardware scores (0-1)
        features.push(self.cpu_score / 100.0);
        features.push(self.gpu_score / 100.0);
        features.push((self.ram_gb / 64.0).min(1.0)); // Normalize to 64GB max
        features.push((self.vram_gb / 24.0).min(1.0)); // Normalize to 24GB max
        
        // Resolution encoding (one-hot-like)
        match self.target_resolution.as_str() {
            "720p" => { features.extend(&[1.0, 0.0, 0.0, 0.0]); }
            "1080p" => { features.extend(&[0.0, 1.0, 0.0, 0.0]); }
            "1440p" => { features.extend(&[0.0, 0.0, 1.0, 0.0]); }
            "4k" => { features.extend(&[0.0, 0.0, 0.0, 1.0]); }
            _ => { features.extend(&[0.0, 1.0, 0.0, 0.0]); } // Default to 1080p
        }
        
        // Quality preset encoding
        match self.quality_preset.as_str() {
            "Low" => features.push(0.0),
            "Medium" => features.push(0.5),
            "High" => features.push(1.0),
            _ => features.push(0.5),
        }
        
        // Laptop penalty
        features.push(if self.is_laptop { 0.85 } else { 1.0 });
        
        // CPU cores normalized
        features.push((self.cpu_cores as f32 / 32.0).min(1.0));
        
        features
    }
}

/// Prediction result from ML model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResult {
    pub estimated_fps: f32,
    pub confidence: f32,
    pub bottleneck: String,
    pub frame_time_ms: f32,
    pub stability_score: f32,
}

impl PredictionResult {
    pub fn from_model_output(output: &[f32]) -> Self {
        // Assuming model outputs: [fps, confidence, bottleneck_score, frame_time, stability]
        let estimated_fps = output.get(0).copied().unwrap_or(60.0).clamp(5.0, 240.0);
        let confidence = output.get(1).copied().unwrap_or(0.8).clamp(0.0, 1.0);
        let bottleneck_score = output.get(2).copied().unwrap_or(0.5);
        let frame_time_ms = output.get(3).copied().unwrap_or(16.67); // ~60 FPS
        let stability_score = output.get(4).copied().unwrap_or(0.9).clamp(0.0, 1.0);
        
        let bottleneck = determine_bottleneck(bottleneck_score);
        
        Self {
            estimated_fps,
            confidence,
            bottleneck,
            frame_time_ms,
            stability_score,
        }
    }
}

fn determine_bottleneck(score: f32) -> String {
    if score < 0.3 {
        "CPU".to_string()
    } else if score < 0.6 {
        "GPU".to_string()
    } else if score < 0.8 {
        "VRAM".to_string()
    } else {
        "RAM".to_string()
    }
}

/// Performance predictor using ML models
pub struct PerformancePredictor {
    model_loaded: bool,
    use_heuristic_fallback: bool,
}

impl PerformancePredictor {
    pub fn new() -> Result<Self, super::MLError> {
        #[cfg(feature = "ml-local")]
        {
            Ok(Self {
                model_loaded: false, // Would load ONNX model here
                use_heuristic_fallback: true,
            })
        }
        
        #[cfg(not(feature = "ml-local"))]
        {
            Err(super::MLError::FeatureNotEnabled)
        }
    }
    
    pub fn with_heuristic_fallback() -> Self {
        Self {
            model_loaded: false,
            use_heuristic_fallback: true,
        }
    }
    
    pub fn predict(&self, features: &HardwareFeatures) -> Result<PredictionResult, super::MLError> {
        #[cfg(feature = "ml-local")]
        {
            if self.model_loaded {
                // Would use ONNX model here
                self.predict_with_model(features)
            } else if self.use_heuristic_fallback {
                Ok(self.heuristic_prediction(features))
            } else {
                Err(super::MLError::ModelNotLoaded("No model loaded".to_string()))
            }
        }
        
        #[cfg(not(feature = "ml-local"))]
        {
            Err(super::MLError::FeatureNotEnabled)
        }
    }
    
    #[cfg(feature = "ml-local")]
    fn predict_with_model(&self, features: &HardwareFeatures) -> Result<PredictionResult, super::MLError> {
        // In a real implementation, this would:
        // 1. Convert features to tensor
        // 2. Run ONNX inference
        // 3. Parse output tensor
        
        // For now, use heuristic as placeholder
        Ok(self.heuristic_prediction(features))
    }
    
    #[cfg(feature = "ml-local")]
    fn heuristic_prediction(&self, features: &HardwareFeatures) -> PredictionResult {
        // Enhanced heuristic prediction as fallback
        let cpu_factor = (features.cpu_score / 70.0).clamp(0.3, 1.5);
        let gpu_factor = (features.gpu_score / 70.0).clamp(0.25, 1.7);
        let vram_factor = (features.vram_gb / 8.0).clamp(0.55, 1.0);
        
        // Resolution scaling
        let resolution_scale = match features.target_resolution.as_str() {
            "720p" => 1.55,
            "1440p" => 0.68,
            "4k" => 0.38,
            _ => 1.0,
        };
        
        // Quality preset scaling
        let quality_scale = match features.quality_preset.as_str() {
            "Low" => 1.28,
            "Medium" => 1.0,
            "High" => 0.78,
            _ => 1.0,
        };
        
        // Laptop penalty
        let laptop_factor = if features.is_laptop { 0.85 } else { 1.0 };
        
        // Calculate estimated FPS
        let base_fps = 52.0;
        let estimated_fps = (base_fps * cpu_factor.min(gpu_factor * resolution_scale * quality_scale) * vram_factor * laptop_factor)
            .clamp(5.0, 240.0);
        
        // Calculate confidence based on how balanced the system is
        let balance_score = 1.0 - (cpu_factor - gpu_factor).abs().min(0.5);
        let confidence = (0.7 + balance_score * 0.2).min(0.95);
        
        // Determine bottleneck
        let bottleneck = if cpu_factor < gpu_factor * 0.8 {
            "CPU"
        } else if gpu_factor < cpu_factor * 0.8 {
            "GPU"
        } else if vram_factor < 0.8 {
            "VRAM"
        } else {
            "Balanced"
        }.to_string();
        
        // Frame time (inverse of FPS)
        let frame_time_ms = if estimated_fps > 0.0 {
            1000.0 / estimated_fps
        } else {
            100.0 // Very slow
        };
        
        // Stability score based on balance and laptop status
        let stability_score = (balance_score * if features.is_laptop { 0.8 } else { 0.95 }).min(1.0);
        
        PredictionResult {
            estimated_fps,
            confidence,
            bottleneck,
            frame_time_ms,
            stability_score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_features_creation() {
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
        
        let features = HardwareFeatures::from_hardware(&hw, "1080p", "High");
        assert_eq!(features.cpu_score, 70.0);
        assert_eq!(features.gpu_score, 70.0);
        assert_eq!(features.target_resolution, "1080p");
    }

    #[test]
    fn test_feature_vector() {
        let features = HardwareFeatures {
            cpu_score: 70.0,
            gpu_score: 70.0,
            ram_gb: 16.0,
            vram_gb: 8.0,
            target_resolution: "1080p".to_string(),
            quality_preset: "High".to_string(),
            is_laptop: false,
            cpu_cores: 8,
        };
        
        let vector = features.to_feature_vector();
        assert!(!vector.is_empty());
        assert!(vector.len() >= 10);
    }

    #[test]
    fn test_heuristic_prediction() {
        let predictor = PerformancePredictor::with_heuristic_fallback();
        let features = HardwareFeatures {
            cpu_score: 70.0,
            gpu_score: 70.0,
            ram_gb: 16.0,
            vram_gb: 8.0,
            target_resolution: "1080p".to_string(),
            quality_preset: "High".to_string(),
            is_laptop: false,
            cpu_cores: 8,
        };
        
        let result = predictor.predict(&features);
        assert!(result.is_ok());
        let prediction = result.unwrap();
        assert!(prediction.estimated_fps > 0.0);
        assert!(prediction.confidence > 0.0);
    }
}