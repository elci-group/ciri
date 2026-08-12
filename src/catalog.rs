#[derive(Clone, Copy, Debug)]
pub struct Tier {
    pub cpu: u16,
    pub gpu: u16,
    pub ram_gb: u16,
    pub vram_gb: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct Game {
    pub id: &'static str,
    pub title: &'static str,
    pub aliases: &'static [&'static str],
    pub minimum: Tier,
    pub recommended: Tier,
    pub storage_gb: u16,
    pub api: &'static str,
    pub minimum_label: &'static str,
    pub recommended_label: &'static str,
    pub linux: LinuxSupport,
    pub issues: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinuxSupport {
    Native,
    ProtonGood,
    ProtonMixed,
}

pub static GAMES: &[Game] = &[
    Game {
        id: "batman-arkham-knight",
        title: "Batman: Arkham Knight",
        aliases: &["arkham knight", "batman ak", "batman arkham"],
        minimum: Tier {
            cpu: 38,
            gpu: 32,
            ram_gb: 6,
            vram_gb: 2,
        },
        recommended: Tier {
            cpu: 48,
            gpu: 45,
            ram_gb: 8,
            vram_gb: 3,
        },
        storage_gb: 45,
        api: "DirectX 11",
        minimum_label: "Core i5-750 / GTX 660 2GB",
        recommended_label: "Core i7-3770 / GTX 760 3GB",
        linux: LinuxSupport::ProtonGood,
        issues: &[
            "The Windows release had severe launch-era performance issues",
            "Proton may need shader compilation time",
        ],
    },
    Game {
        id: "cyberpunk-2077",
        title: "Cyberpunk 2077",
        aliases: &["cyberpunk", "cp2077", "cyberpunk2077"],
        minimum: Tier {
            cpu: 52,
            gpu: 55,
            ram_gb: 12,
            vram_gb: 6,
        },
        recommended: Tier {
            cpu: 68,
            gpu: 72,
            ram_gb: 16,
            vram_gb: 8,
        },
        storage_gb: 70,
        api: "DirectX 12 / Vulkan via Proton",
        minimum_label: "Core i7-6700 / GTX 1060 6GB",
        recommended_label: "Core i7-12700 / RTX 2060 Super",
        linux: LinuxSupport::ProtonGood,
        issues: &[
            "Ray tracing is excluded from Ciri's estimate",
            "An SSD is strongly recommended",
        ],
    },
    Game {
        id: "elden-ring",
        title: "Elden Ring",
        aliases: &["eldenring"],
        minimum: Tier {
            cpu: 52,
            gpu: 50,
            ram_gb: 12,
            vram_gb: 3,
        },
        recommended: Tier {
            cpu: 64,
            gpu: 65,
            ram_gb: 16,
            vram_gb: 8,
        },
        storage_gb: 60,
        api: "DirectX 12 / Vulkan via Proton",
        minimum_label: "Core i5-8400 / GTX 1060 3GB",
        recommended_label: "Core i7-8700K / GTX 1070 8GB",
        linux: LinuxSupport::ProtonGood,
        issues: &[
            "Shader compilation can cause traversal stutter",
            "The game is capped at 60 FPS by default",
        ],
    },
    Game {
        id: "baldurs-gate-3",
        title: "Baldur's Gate 3",
        aliases: &["baldurs gate 3", "bg3", "baldur's gate iii"],
        minimum: Tier {
            cpu: 42,
            gpu: 42,
            ram_gb: 8,
            vram_gb: 4,
        },
        recommended: Tier {
            cpu: 64,
            gpu: 65,
            ram_gb: 16,
            vram_gb: 8,
        },
        storage_gb: 150,
        api: "DirectX 11 / Vulkan",
        minimum_label: "Core i5-4690 / GTX 970",
        recommended_label: "Core i7-8700K / RTX 2060 Super",
        linux: LinuxSupport::ProtonGood,
        issues: &[
            "Dense late-game areas are unusually CPU-heavy",
            "An SSD is required",
        ],
    },
    Game {
        id: "grand-theft-auto-v",
        title: "Grand Theft Auto V",
        aliases: &["gta v", "gta 5", "gtav"],
        minimum: Tier {
            cpu: 28,
            gpu: 22,
            ram_gb: 4,
            vram_gb: 1,
        },
        recommended: Tier {
            cpu: 40,
            gpu: 35,
            ram_gb: 8,
            vram_gb: 2,
        },
        storage_gb: 120,
        api: "DirectX 11",
        minimum_label: "Core 2 Quad Q6600 / 9800 GT",
        recommended_label: "Core i5-3470 / GTX 660",
        linux: LinuxSupport::ProtonMixed,
        issues: &[
            "GTA Online anti-cheat support on Linux can change",
            "Enhanced-edition features are not included",
        ],
    },
    Game {
        id: "the-witcher-3",
        title: "The Witcher 3: Wild Hunt",
        aliases: &["witcher 3", "witcher iii", "wild hunt"],
        minimum: Tier {
            cpu: 38,
            gpu: 34,
            ram_gb: 6,
            vram_gb: 2,
        },
        recommended: Tier {
            cpu: 54,
            gpu: 58,
            ram_gb: 8,
            vram_gb: 6,
        },
        storage_gb: 50,
        api: "DirectX 11/12 / Vulkan via Proton",
        minimum_label: "Core i5-2500K / GTX 660",
        recommended_label: "Core i5-7400 / GTX 1070",
        linux: LinuxSupport::ProtonGood,
        issues: &[
            "The next-gen DX12 renderer is heavier than DX11",
            "HairWorks and ray tracing are excluded from estimates",
        ],
    },
];

pub fn resolve(query: &str) -> Option<(&'static Game, u8)> {
    let needle = normalize(query);
    if needle.is_empty() {
        return None;
    }

    let mut best: Option<(&Game, f32)> = None;
    for game in GAMES {
        for candidate in std::iter::once(game.title)
            .chain(std::iter::once(game.id))
            .chain(game.aliases.iter().copied())
        {
            let candidate = normalize(candidate);
            let score = similarity(&needle, &candidate);
            if best.is_none_or(|(_, current)| score > current) {
                best = Some((game, score));
            }
        }
    }

    best.filter(|(_, score)| *score >= 0.48)
        .map(|(game, score)| (game, (score * 100.0).round() as u8))
}

fn normalize(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn similarity(a: &str, b: &str) -> f32 {
    if a == b {
        return 1.0;
    }
    if b.contains(a) || a.contains(b) {
        return 0.88 + 0.12 * (a.len().min(b.len()) as f32 / a.len().max(b.len()) as f32);
    }
    let distance = levenshtein(a.as_bytes(), b.as_bytes());
    1.0 - distance as f32 / a.len().max(b.len()) as f32
}

fn levenshtein(a: &[u8], b: &[u8]) -> usize {
    let mut row: Vec<usize> = (0..=b.len()).collect();
    for (i, left) in a.iter().enumerate() {
        let mut diagonal = row[0];
        row[0] = i + 1;
        for (j, right) in b.iter().enumerate() {
            let above = row[j + 1];
            row[j + 1] = if left == right {
                diagonal
            } else {
                1 + diagonal.min(above).min(row[j])
            };
            diagonal = above;
        }
    }
    row[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_aliases_and_typos() {
        assert_eq!(resolve("BG3").unwrap().0.id, "baldurs-gate-3");
        assert_eq!(
            resolve("batman arkham nite").unwrap().0.id,
            "batman-arkham-knight"
        );
        assert!(resolve("spreadsheet deluxe").is_none());
    }
}
