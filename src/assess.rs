use crate::catalog::{Game, LinuxSupport, Tier};
use crate::hardware::Hardware;

#[cfg(feature = "advanced-assessment")]
pub mod multi_factor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Run,
    Compromises,
    DontRun,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Pass,
    Borderline,
    Fail,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct Assessment {
    pub verdict: Verdict,
    pub confidence: u8,
    pub cpu: Status,
    pub gpu: Status,
    pub ram: Status,
    pub vram: Status,
    pub storage: Status,
    pub os: Status,
    pub api: Status,
    pub bottleneck: &'static str,
    pub fps: Vec<(String, u16)>,
    pub notes: Vec<String>,
}

pub fn assess(game: &Game, hw: &Hardware, target: &str, resolver_confidence: u8) -> Assessment {
    let cpu = compare(hw.cpu_score, game.minimum.cpu, game.recommended.cpu);
    let gpu = if hw.gpu_score == 0 {
        Status::Unknown
    } else {
        compare(hw.gpu_score, game.minimum.gpu, game.recommended.gpu)
    };
    let ram = compare(hw.ram_gb, game.minimum.ram_gb, game.recommended.ram_gb);
    let vram = hw.vram_gb.map_or(Status::Unknown, |v| {
        compare(v, game.minimum.vram_gb, game.recommended.vram_gb)
    });
    let storage = if hw.storage_gb == 0 {
        Status::Unknown
    } else if hw.storage_gb >= game.storage_gb {
        Status::Pass
    } else {
        Status::Fail
    };
    let os = os_status(game.linux, hw.is_linux);
    let api = if !hw.is_linux || !game.api.contains("Vulkan") || hw.vulkan {
        Status::Pass
    } else {
        Status::Unknown
    };

    let critical = [cpu, gpu, ram, vram, storage, os];
    let failures = critical
        .iter()
        .filter(|status| **status == Status::Fail)
        .count();
    let unknowns = critical
        .iter()
        .filter(|status| **status == Status::Unknown)
        .count();
    let borderline = critical
        .iter()
        .filter(|status| **status == Status::Borderline)
        .count();
    let verdict =
        if failures >= 2 || os == Status::Fail || gpu == Status::Fail && vram == Status::Fail {
            Verdict::DontRun
        } else if failures == 1 || borderline > 0 || unknowns > 0 || os == Status::Borderline {
            Verdict::Compromises
        } else {
            Verdict::Run
        };

    let bottleneck = bottleneck(game.minimum, hw);
    let known = 6_u8.saturating_sub(unknowns as u8);
    let confidence = (55 + known * 6 + resolver_confidence / 12).min(98);
    let fps = estimate_fps(game, hw, target);
    let mut notes = Vec::new();
    if hw.is_laptop {
        notes.push("Laptop thermals and power limits may reduce sustained performance".to_string());
    }
    if hw.is_linux && game.linux != LinuxSupport::Native {
        notes.push(
            match game.linux {
                LinuxSupport::ProtonGood => "Linux assessment assumes a current Proton release",
                LinuxSupport::ProtonMixed => {
                    "Linux/Proton support is mixed and can change after game updates"
                }
                LinuxSupport::Native => unreachable!(),
            }
            .to_string(),
        );
    }
    notes.extend(game.issues.iter().map(|s| (*s).to_string()));

    Assessment {
        verdict,
        confidence,
        cpu,
        gpu,
        ram,
        vram,
        storage,
        os,
        api,
        bottleneck,
        fps,
        notes,
    }
}

fn compare(value: u16, minimum: u16, recommended: u16) -> Status {
    if value < minimum {
        Status::Fail
    } else if value < recommended {
        Status::Borderline
    } else {
        Status::Pass
    }
}

fn os_status(support: LinuxSupport, linux: bool) -> Status {
    if !linux {
        return Status::Pass;
    }
    match support {
        LinuxSupport::Native | LinuxSupport::ProtonGood => Status::Pass,
        LinuxSupport::ProtonMixed => Status::Borderline,
    }
}

fn bottleneck(min: Tier, hw: &Hardware) -> &'static str {
    let mut ratios = vec![
        ("CPU", hw.cpu_score as f32 / min.cpu.max(1) as f32),
        ("GPU", hw.gpu_score as f32 / min.gpu.max(1) as f32),
        ("RAM", hw.ram_gb as f32 / min.ram_gb.max(1) as f32),
    ];
    if let Some(vram) = hw.vram_gb {
        ratios.push(("VRAM", vram as f32 / min.vram_gb.max(1) as f32));
    }
    ratios
        .into_iter()
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|v| v.0)
        .unwrap_or("Unknown")
}

fn estimate_fps(game: &Game, hw: &Hardware, target: &str) -> Vec<(String, u16)> {
    if hw.gpu_score == 0 {
        return Vec::new();
    }
    let cpu_factor = (hw.cpu_score as f32 / game.recommended.cpu as f32).clamp(0.35, 1.35);
    let gpu_factor = (hw.gpu_score as f32 / game.recommended.gpu as f32).clamp(0.25, 1.7);
    let vram_factor = hw
        .vram_gb
        .map(|v| (v as f32 / game.recommended.vram_gb as f32).clamp(0.55, 1.0))
        .unwrap_or(0.82);
    let target_scale = match target {
        "720p" => 1.55,
        "1440p" => 0.68,
        "4k" | "2160p" => 0.38,
        _ => 1.0,
    };
    [("Low", 1.28), ("Medium", 1.0), ("High", 0.78)]
        .into_iter()
        .map(|(preset, quality)| {
            let fps = (52.0_f32 * cpu_factor.min(gpu_factor * target_scale * quality) * vram_factor)
                .round()
                .clamp(5.0, 240.0) as u16;
            (format!("{} {preset}", canonical_target(target)), fps)
        })
        .collect()
}

pub fn canonical_target(target: &str) -> &'static str {
    match target.to_ascii_lowercase().as_str() {
        "720" | "720p" => "720p",
        "1440" | "1440p" | "2k" => "1440p",
        "2160" | "2160p" | "4k" => "4K",
        _ => "1080p",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::embedded::GAMES;

    fn hardware(cpu: u16, gpu: u16, ram: u16, vram: Option<u16>) -> Hardware {
        Hardware {
            cpu_name: "Test CPU".into(),
            cpu_score: cpu,
            logical_cores: 8,
            gpu_name: "Test GPU".into(),
            gpu_score: gpu,
            vram_gb: vram,
            ram_gb: ram,
            storage_gb: 500,
            os: "Linux".into(),
            is_linux: true,
            is_laptop: false,
            vulkan: true,
        }
    }

    #[test]
    fn produces_all_three_verdicts() {
        let game = &GAMES[1];
        assert_eq!(
            assess(game, &hardware(90, 90, 32, Some(12)), "1080p", 100).verdict,
            Verdict::Run
        );
        assert_eq!(
            assess(game, &hardware(60, 60, 16, Some(6)), "1080p", 100).verdict,
            Verdict::Compromises
        );
        assert_eq!(
            assess(game, &hardware(30, 25, 8, Some(2)), "1080p", 100).verdict,
            Verdict::DontRun
        );
    }
}
