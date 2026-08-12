//! Windows-specific hardware detection
//! 
//! This module provides hardware detection functionality for Windows systems.
//! Note: This requires the "windows-detection" feature to be enabled.

#[cfg(feature = "windows-detection")]
use std::process::Command;

#[cfg(feature = "windows-detection")]
pub fn detect_cpu_info() -> Option<(String, u16, u16)> {
    // Use wmic to get CPU information on Windows
    let output = Command::new("wmic")
        .args(&["cpu", "get", "name,numberofcores,numberoflogicalprocessors"])
        .output()
        .ok()?;
    
    let text = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = text.lines().collect();
    
    if lines.len() < 2 {
        return None;
    }
    
    // Parse the output (skip header line)
    let data_line = lines.get(1)?;
    let parts: Vec<&str> = data_line.split_whitespace().collect();
    
    if parts.len() < 3 {
        return None;
    }
    
    let cpu_name = parts.get(0)?.to_string();
    let cores: u16 = parts.get(1)?.parse().ok()?;
    let logical_processors: u16 = parts.get(2)?.parse().ok()?;
    
    Some((cpu_name, cores, logical_processors))
}

#[cfg(feature = "windows-detection")]
pub fn detect_gpu_info() -> Option<(String, u16, Option<u16>)> {
    // Use wmic to get GPU information on Windows
    let output = Command::new("wmic")
        .args(&["path", "win32_videocontroller", "get", "name,adapterram"])
        .output()
        .ok()?;
    
    let text = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = text.lines().collect();
    
    if lines.len() < 2 {
        return None;
    }
    
    // Parse the output (skip header line)
    let data_line = lines.get(1)?;
    let parts: Vec<&str> = data_line.split_whitespace().collect();
    
    if parts.is_empty() {
        return None;
    }
    
    let gpu_name = parts.get(0)?.to_string();
    
    // VRAM is in bytes, convert to GB
    let vram_bytes: Option<u64> = parts.get(1).and_then(|s| s.parse().ok());
    let vram_gb = vram_bytes.map(|bytes| (bytes / 1_073_741_824) as u16);
    
    // Score GPU based on name (simplified version)
    let gpu_score = score_gpu_from_name(&gpu_name);
    
    Some((gpu_name, gpu_score, vram_gb))
}

#[cfg(feature = "windows-detection")]
pub fn detect_memory_gb() -> Option<u16> {
    let output = Command::new("wmic")
        .args(&["computersystem", "get", "totalphysicalmemory"])
        .output()
        .ok()?;
    
    let text = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = text.lines().collect();
    
    if lines.len() < 2 {
        return None;
    }
    
    let memory_bytes: u64 = lines.get(1)?.trim().parse().ok()?;
    Some((memory_bytes / 1_073_741_824) as u16)
}

#[cfg(feature = "windows-detection")]
pub fn detect_storage_gb() -> Option<u16> {
    let output = Command::new("wmic")
        .args(&["logicaldisk", "get", "freespace"])
        .output()
        .ok()?;
    
    let text = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = text.lines().collect();
    
    if lines.len() < 2 {
        return None;
    }
    
    let free_space_bytes: u64 = lines.get(1)?.trim().parse().ok()?;
    Some((free_space_bytes / 1_073_741_824) as u16)
}

#[cfg(feature = "windows-detection")]
fn score_gpu_from_name(name: &str) -> u16 {
    let lower = name.to_lowercase();
    
    // Simplified GPU scoring for Windows
    let entries = [
        ("rtx 4090", 100),
        ("rtx 4080", 96),
        ("rtx 4070", 88),
        ("rtx 3090", 92),
        ("rtx 3080", 86),
        ("rtx 3070", 78),
        ("rtx 3060", 70),
        ("rtx 2080", 73),
        ("rtx 2070", 68),
        ("rtx 2060", 62),
        ("gtx 1080", 66),
        ("gtx 1070", 58),
        ("gtx 1060", 50),
        ("rx 7900", 96),
        ("rx 7800", 89),
        ("rx 6800", 84),
        ("rx 6700", 76),
        ("rx 6600", 65),
    ];
    
    entries
        .iter()
        .find(|(needle, _)| lower.contains(needle))
        .map(|(_, score)| *score)
        .unwrap_or(25)
}

#[cfg(not(feature = "windows-detection"))]
pub fn detect_cpu_info() -> Option<(String, u16, u16)> {
    None
}

#[cfg(not(feature = "windows-detection"))]
pub fn detect_gpu_info() -> Option<(String, u16, Option<u16>)> {
    None
}

#[cfg(not(feature = "windows-detection"))]
pub fn detect_memory_gb() -> Option<u16> {
    None
}

#[cfg(not(feature = "windows-detection"))]
pub fn detect_storage_gb() -> Option<u16> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "windows-detection")]
    #[test]
    fn test_gpu_scoring() {
        assert!(score_gpu_from_name("NVIDIA GeForce RTX 4090") > score_gpu_from_name("NVIDIA GeForce GTX 1060"));
    }
}