//! Machine learning integration for performance prediction
//! 
//! This module provides ML-based performance prediction capabilities.
//! Note: This requires the "ml-local" feature to be enabled.

#[cfg(feature = "ml-local")]
pub mod models;
#[cfg(feature = "ml-local")]
pub mod onnx_runtime;
#[cfg(feature = "ml-federated")]
pub mod federated;

#[cfg(feature = "ml-local")]
pub use models::{PerformancePredictor, HardwareFeatures, PredictionResult};
#[cfg(feature = "ml-federated")]
pub use federated::{FederatedLearningClient, ModelUpdate};

// Re-export for public API
#[cfg(feature = "ml-local")]
pub use onnx_runtime::ONNXModel;

/// Error type for ML operations
#[derive(Debug)]
pub enum MLError {
    ModelNotLoaded(String),
    InferenceFailed(String),
    FeatureExtractionFailed(String),
    FeatureNotEnabled,
}

impl std::fmt::Display for MLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MLError::ModelNotLoaded(msg) => write!(f, "Model not loaded: {}", msg),
            MLError::InferenceFailed(msg) => write!(f, "Inference failed: {}", msg),
            MLError::FeatureExtractionFailed(msg) => write!(f, "Feature extraction failed: {}", msg),
            MLError::FeatureNotEnabled => write!(f, "ML feature not enabled. Build with --features ml-local"),
        }
    }
}

impl std::error::Error for MLError {}

#[cfg(not(feature = "ml-local"))]
pub struct PerformancePredictor;

#[cfg(not(feature = "ml-local"))]
impl PerformancePredictor {
    pub fn new() -> Result<Self, MLError> {
        Err(MLError::FeatureNotEnabled)
    }
    
    pub fn predict(&self, _features: &HardwareFeatures) -> Result<PredictionResult, MLError> {
        Err(MLError::FeatureNotEnabled)
    }
}

#[cfg(not(feature = "ml-local"))]
#[derive(Debug, Clone)]
pub struct HardwareFeatures {
    pub cpu_score: f32,
    pub gpu_score: f32,
    pub ram_gb: f32,
    pub vram_gb: f32,
    pub target_resolution: String,
}

#[cfg(not(feature = "ml-local"))]
#[derive(Debug, Clone)]
pub struct PredictionResult {
    pub estimated_fps: f32,
    pub confidence: f32,
    pub bottleneck: String,
}

#[cfg(not(feature = "ml-federated"))]
pub struct FederatedLearningClient;

#[cfg(not(feature = "ml-federated"))]
pub struct ModelUpdate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ml_error_display() {
        let err = MLError::FeatureNotEnabled;
        assert!(err.to_string().contains("ML feature not enabled"));
    }

    #[cfg(feature = "ml-local")]
    #[test]
    fn test_predictor_creation() {
        let predictor = PerformancePredictor::new();
        assert!(predictor.is_ok());
    }
}