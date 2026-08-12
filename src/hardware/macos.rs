//! macOS-specific hardware detection
//! 
//! This module provides hardware detection functionality for macOS systems.
//! Note: This requires the "macos-detection" feature to be enabled.

#[cfg(feature = "macos-detection")]
use std::process::Command;

#[cfg(feature = "macos-detection")]
pub fn detect_cpu_info() -> Option<(String, u16, u16)> {
    // Use sysctl to get CPU information on macOS
    let output = Command::new("sysctl")
        .args(&["-n", "machdep.cpu.brand_string"])
        .output()
        .ok()?;
    
    let cpu_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    // Get number of cores
    let cores_output = Command::new("sysctl")
        .args(&["-n", "hw.physicalcpu"])
        .output()
        .ok()?;
    let cores: u16 = String::from_utf8_lossy(&cores_output.stdout).trim().parse().ok()?;
    
    // Get number of logical processors
    let logical_output = Command::new("sysctl")
        .args(&["-n", "hw.logicalcpu"])
        .output()
        .ok()?;
    let logical_processors: u16 = String::from_utf8_lossy(&logical_output.stdout).trim().parse().ok()?;
    
    Some((cpu_name, cores, logical_processors))
}

#[cfg(feature = "macos-detection")]
pub fn detect_gpu_info() -> Option<(String, u16, Option<u16>)> {
    // Use system_profiler to get GPU information on macOS
    let output = Command::new("system_profiler")
        .args(&["SPDisplaysDataType", "-json"])
        .output()
        .ok()?;
    
    let text = String::from_utf8_lossy(&output.stdout);
    
    // Parse JSON output (simplified version)
    if let Some(gpu_name) = extract_gpu_name(&text) {
        let gpu_score = score_gpu_from_name(&gpu_name);
        let vram_gb = extract_vram(&text);
        Some((gpu_name, gpu_score, vram_gb))
    } else {
        None
    }
}

#[cfg(feature = "macos-detection")]
pub fn detect_memory_gb() -> Option<u16> {
    let output = Command::new("sysctl")
        .args(&["-n", "hw.memsize"])
        .output()
        .ok()?;
    
    let memory_bytes: u64 = String::from_utf8_lossy(&output.stdout).trim().parse().ok()?;
    Some((memory_bytes / 1_073_741_824) as u16)
}

#[cfg(feature = "macos-detection")]
pub fn detect_storage_gb() -> Option<u16> {
    let output = Command::new("df")
        .args(&["-h", "/"])
        .output()
        .ok()?;
    
    let text = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = text.lines().collect();
    
    if lines.len() < 2 {
        return None;
    }
    
    // Parse the output line (skip header)
    let data_line = lines.get(1)?;
    let parts: Vec<&str> = data_line.split_whitespace().collect();
    
    if parts.len() < 4 {
        return None;
    }
    
    // Extract available space (field 4)
    let available_str = parts.get(3)?;
    let available_gb = parse_size_string(available_str)?;
    
    Some(available_gb)
}

#[cfg(feature = "macos-detection")]
fn extract_gpu_name(text: &str) -> Option<String> {
    // Simple text extraction for GPU name
    // In a real implementation, this would parse the JSON properly
    let lines: Vec<&str> = text.lines().collect();
    for line in lines {
        if line.contains("Chipset Model") || line.contains("GPU") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line.rfind('"') {
                    return Some(line[start + 1..end].to_string());
                }
            }
        }
    }
    None
}

#[cfg(feature = "macos-detection")]
fn extract_vram(text: &str) -> Option<u16> {
    // Extract VRAM from system_profiler output
    let lines: Vec<&str> = text.lines().collect();
    for line in lines {
        if line.contains("VRAM") || line.contains("Memory") {
            if let Some(gb_str) = line.split_whitespace().find(|s| s.ends_with("GB")) {
                if let Some(num) = gb_str.strip_suffix("GB") {
                    return num.parse().ok();
                }
            }
        }
    }
    None
}

#[cfg(feature = "macos-detection")]
fn parse_size_string(size_str: &str) -> Option<u16> {
    let size_str = size_str.trim();
    let numeric_part: String = size_str.chars().filter(|c| c.is_digit(10) || *c == '.').collect();
    let value: f64 = numeric_part.parse().ok()?;
    
    if size_str.contains("G") {
        Some(value as u16)
    } else if size_str.contains("M") {
        Some((value / 1024.0) as u16)
    } else if size_str.contains("T") {
        Some((value * 1024.0) as u16)
    } else {
        None
    }
}

#[cfg(feature = "macos-detection")]
fn score_gpu_from_name(name: &str) -> u16 {
    let lower = name.to_lowercase();
    
    // Apple Silicon GPU scoring
    if lower.contains("m3") {
        if lower.contains("max") {
            return 85;
        } else if lower.contains("pro") {
            return 75;
        } else {
            return 66;
        }
    } else if lower.contains("m2") {
        if lower.contains("max") {
            return 72;
        } else if lower.contains("pro") {
            return 62;
        } else {
            return 55;
        }
    } else if lower.contains("m1") {
        if lower.contains("max") {
            return 66;
        } else if lower.contains("pro") {
            return 55;
        } else {
            return 48;
        }
    }
    
    // AMD GPUs in Macs
    let entries = [
        ("rx 6800", 84),
        ("rx 6700", 76),
        ("rx 580", 48),
        ("radeon pro 580", 50),
        ("radeon pro 560", 40),
    ];
    
    entries
        .iter()
        .find(|(needle, _)| lower.contains(needle))
        .map(|(_, score)| *score)
        .unwrap_or(30)
}

#[cfg(not(feature = "macos-detection"))]
pub fn detect_cpu_info() -> Option<(String, u16, u16)> {
    None
}

#[cfg(not(feature = "macos-detection"))]
pub fn detect_gpu_info() -> Option<(String, u16, Option<u16>)> {
    None
}

#[cfg(not(feature = "macos-detection"))]
pub fn detect_memory_gb() -> Option<u16> {
    None
}

#[cfg(not(feature = "macos-detection"))]
pub fn detect_storage_gb() -> Option<u16> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "macos-detection")]
    #[test]
    fn test_gpu_scoring() {
        assert!(score_gpu_from_name("Apple M3 Max") > score_gpu_from_name("Apple M1"));
    }

    #[cfg(feature = "macos-detection")]
    #[test]
    fn test_size_parsing() {
        assert_eq!(parse_size_string("500G"), Some(500));
        assert_eq!(parse_size_string("512M"), Some(0));
        assert_eq!(parse_size_string("1T"), Some(1024));
    }
}