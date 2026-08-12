use std::fs;
use std::process::Command;

#[cfg(feature = "windows-detection")]
pub mod windows;
#[cfg(feature = "macos-detection")]
pub mod macos;

#[derive(Clone, Debug)]
pub struct Hardware {
    pub cpu_name: String,
    pub cpu_score: u16,
    pub logical_cores: u16,
    pub gpu_name: String,
    pub gpu_score: u16,
    pub vram_gb: Option<u16>,
    pub ram_gb: u16,
    pub storage_gb: u16,
    pub os: String,
    pub is_linux: bool,
    pub is_laptop: bool,
    pub vulkan: bool,
}

impl Hardware {
    pub fn detect() -> Self {
        #[cfg(target_os = "linux")]
        {
            Self::detect_linux()
        }
        
        #[cfg(target_os = "windows")]
        {
            Self::detect_windows()
        }
        
        #[cfg(target_os = "macos")]
        {
            Self::detect_macos()
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Self::detect_fallback()
        }
    }
    
    #[cfg(target_os = "linux")]
    fn detect_linux() -> Self {
        let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
        let cpu_name = cpuinfo
            .lines()
            .find_map(|line| field(line, "model name"))
            .unwrap_or_else(|| std::env::consts::ARCH.to_string());
        let logical_cores = cpuinfo
            .lines()
            .filter(|line| line.starts_with("processor"))
            .count()
            .max(1) as u16;
        let cpu_score = cpu_score(&cpu_name, logical_cores);

        let gpu_name = detect_gpu().unwrap_or_else(|| "Unknown GPU".to_string());
        let (gpu_score, vram_gb) = gpu_score(&gpu_name);
        let ram_gb = memory_gb(&fs::read_to_string("/proc/meminfo").unwrap_or_default());
        let storage_gb = storage_gb();
        let os_release = fs::read_to_string("/etc/os-release").unwrap_or_default();
        let os = os_release
            .lines()
            .find_map(|line| {
                line.strip_prefix("PRETTY_NAME=")
                    .map(|v| v.trim_matches('"').to_string())
            })
            .unwrap_or_else(|| std::env::consts::OS.to_string());
        let is_laptop = fs::read_dir("/sys/class/power_supply")
            .map(|entries| {
                entries
                    .flatten()
                    .any(|entry| entry.file_name().to_string_lossy().starts_with("BAT"))
            })
            .unwrap_or(false);
        let vulkan = command_ok("vulkaninfo", &["--summary"]);

        Self {
            cpu_name,
            cpu_score,
            logical_cores,
            gpu_name,
            gpu_score,
            vram_gb,
            ram_gb,
            storage_gb,
            os,
            is_linux: true,
            is_laptop,
            vulkan,
        }
    }
    
    #[cfg(target_os = "windows")]
    fn detect_windows() -> Self {
        #[cfg(feature = "windows-detection")]
        {
            use windows as win;
            
            let (cpu_name, cores, logical_cores) = win::detect_cpu_info()
                .unwrap_or_else(|| ("Unknown CPU".to_string(), 4, 4));
            let cpu_score = cpu_score(&cpu_name, logical_cores);
            
            let (gpu_name, gpu_score, vram_gb) = win::detect_gpu_info()
                .unwrap_or_else(|| ("Unknown GPU".to_string(), 0, None));
            
            let ram_gb = win::detect_memory_gb().unwrap_or(8);
            let storage_gb = win::detect_storage_gb().unwrap_or(0);
            
            Self {
                cpu_name,
                cpu_score,
                logical_cores,
                gpu_name,
                gpu_score,
                vram_gb,
                ram_gb,
                storage_gb,
                os: "Windows".to_string(),
                is_linux: false,
                is_laptop: false, // Could detect via battery status
                vulkan: false, // Could detect via Vulkan loader
            }
        }
        
        #[cfg(not(feature = "windows-detection"))]
        {
            Self::detect_fallback()
        }
    }
    
    #[cfg(target_os = "macos")]
    fn detect_macos() -> Self {
        #[cfg(feature = "macos-detection")]
        {
            use macos as mac;
            
            let (cpu_name, cores, logical_cores) = mac::detect_cpu_info()
                .unwrap_or_else(|| ("Unknown CPU".to_string(), 4, 4));
            let cpu_score = cpu_score(&cpu_name, logical_cores);
            
            let (gpu_name, gpu_score, vram_gb) = mac::detect_gpu_info()
                .unwrap_or_else(|| ("Unknown GPU".to_string(), 0, None));
            
            let ram_gb = mac::detect_memory_gb().unwrap_or(8);
            let storage_gb = mac::detect_storage_gb().unwrap_or(0);
            
            Self {
                cpu_name,
                cpu_score,
                logical_cores,
                gpu_name,
                gpu_score,
                vram_gb,
                ram_gb,
                storage_gb,
                os: "macOS".to_string(),
                is_linux: false,
                is_laptop: true, // Most Macs are laptops
                vulkan: false, // Could detect via Vulkan loader
            }
        }
        
        #[cfg(not(feature = "macos-detection"))]
        {
            Self::detect_fallback()
        }
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    fn detect_fallback() -> Self {
        Self {
            cpu_name: std::env::consts::ARCH.to_string(),
            cpu_score: 50,
            logical_cores: 4,
            gpu_name: "Unknown GPU".to_string(),
            gpu_score: 0,
            vram_gb: None,
            ram_gb: 8,
            storage_gb: 0,
            os: std::env::consts::OS.to_string(),
            is_linux: false,
            is_laptop: false,
            vulkan: false,
        }
    }
}

fn field(line: &str, name: &str) -> Option<String> {
    let (key, value) = line.split_once(':')?;
    (key.trim() == name).then(|| value.trim().to_string())
}

fn memory_gb(meminfo: &str) -> u16 {
    meminfo
        .lines()
        .find_map(|line| field(line, "MemTotal"))
        .and_then(|value| value.split_whitespace().next()?.parse::<u64>().ok())
        .map(|kb| kb.div_ceil(1_048_576).min(u16::MAX as u64) as u16)
        .unwrap_or(0)
}

fn detect_gpu() -> Option<String> {
    let output = Command::new("lspci").arg("-mm").output().ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    parse_lspci_gpu(&text)
}

fn parse_lspci_gpu(text: &str) -> Option<String> {
    let line = text.lines().find(|line| {
        line.contains("VGA compatible controller") || line.contains("3D controller")
    })?;
    let fields: Vec<&str> = line.split('"').collect();
    match (fields.get(3), fields.get(5)) {
        (Some(vendor), Some(device)) if !device.trim().is_empty() => {
            Some(format!("{} {}", vendor.trim(), device.trim()))
        }
        _ => None,
    }
}

fn storage_gb() -> u16 {
    let output = Command::new("df").args(["-Pk", "."]).output();
    let text = output
        .ok()
        .map(|v| String::from_utf8_lossy(&v.stdout).into_owned())
        .unwrap_or_default();
    text.lines()
        .nth(1)
        .and_then(|line| line.split_whitespace().nth(3))
        .and_then(|value| value.parse::<u64>().ok())
        .map(|kb| (kb / 1_048_576).min(u16::MAX as u64) as u16)
        .unwrap_or(0)
}

fn command_ok(program: &str, args: &[&str]) -> bool {
    Command::new(program)
        .args(args)
        .output()
        .is_ok_and(|output| output.status.success())
}

fn cpu_score(name: &str, cores: u16) -> u16 {
    let lower = name.to_lowercase();
    let generation = intel_generation(&lower).unwrap_or(5);
    let family = if lower.contains("i9")
        || lower.contains("ryzen 9")
        || lower.contains("core(tm) 9")
    {
        24
    } else if lower.contains("i7") || lower.contains("ryzen 7") || lower.contains("core(tm) 7") {
        18
    } else if lower.contains("i5") || lower.contains("ryzen 5") || lower.contains("core(tm) 5") {
        13
    } else if lower.contains("i3") || lower.contains("ryzen 3") || lower.contains("core(tm) 3") {
        7
    } else {
        4
    };
    let mobile_penalty = if lower.contains('u') && !lower.contains("cpu") {
        8
    } else {
        0
    };
    (18 + family + generation.saturating_mul(3) + cores.min(16) / 2)
        .saturating_sub(mobile_penalty)
        .min(100)
}

fn intel_generation(name: &str) -> Option<u16> {
    if name.contains("core(tm) 5 1")
        || name.contains("core(tm) 7 1")
        || name.contains("core(tm) 9 1")
    {
        return Some(12);
    }
    let marker = ["i3-", "i5-", "i7-", "i9-"]
        .iter()
        .find_map(|m| name.find(m).map(|i| i + m.len()))?;
    let digits: String = name[marker..]
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();
    match digits.len() {
        4 => digits.get(0..1)?.parse().ok(),
        5 => digits.get(0..2)?.parse().ok(),
        _ => None,
    }
}

fn gpu_score(name: &str) -> (u16, Option<u16>) {
    let n = name.to_lowercase();
    let entries = [
        ("rtx 4090", 100, 24),
        ("rtx 4080", 96, 16),
        ("rtx 4070", 88, 12),
        ("rtx 3090", 92, 24),
        ("rtx 3080", 86, 10),
        ("rtx 3070", 78, 8),
        ("rtx 3060", 70, 12),
        ("rtx 2080", 73, 8),
        ("rtx 2070", 68, 8),
        ("rtx 2060", 62, 6),
        ("gtx 1080", 66, 8),
        ("gtx 1070", 58, 8),
        ("gtx 1060", 50, 6),
        ("gtx 1050 ti", 39, 4),
        ("gtx 1050", 34, 2),
        ("rx 7900", 96, 20),
        ("rx 7800", 89, 16),
        ("rx 6800", 84, 16),
        ("rx 6700", 76, 12),
        ("rx 6600", 65, 8),
        ("rx 580", 48, 8),
        ("radeon 780m", 43, 4),
        ("radeon 680m", 38, 4),
        ("iris xe", 29, 2),
        ("raptor lake-u", 30, 2),
        ("intel graphics", 27, 2),
        ("uhd graphics 620", 18, 1),
        ("hd graphics 620", 17, 1),
        ("apple m3", 66, 8),
        ("apple m2", 55, 8),
        ("apple m1", 48, 8),
    ];
    entries
        .iter()
        .find(|(needle, _, _)| n.contains(needle))
        .map(|(_, score, vram)| (*score, Some(*vram)))
        .unwrap_or_else(|| {
            if n == "unknown gpu" {
                (0, None)
            } else {
                (25, None)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_memory_and_scores_hardware() {
        assert_eq!(memory_gb("MemTotal:       16384000 kB\n"), 16);
        assert!(
            cpu_score("Intel(R) Core(TM) i7-12700K CPU", 20) > cpu_score("Intel Core i5-7300U", 4)
        );
        assert_eq!(gpu_score("NVIDIA GeForce GTX 1050 Ti").1, Some(4));
        assert_eq!(
            parse_lspci_gpu(
                "00:02.0 \"VGA compatible controller\" \"Intel Corporation\" \"Raptor Lake-U [Intel Graphics]\" -r04"
            )
            .as_deref(),
            Some("Intel Corporation Raptor Lake-U [Intel Graphics]")
        );
        assert!(cpu_score("Intel(R) Core(TM) 5 120U", 12) >= 60);
    }
}
