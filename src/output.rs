use crate::assess::{Assessment, Status, Verdict};
use crate::catalog::{Game, LinuxSupport};
use crate::hardware::Hardware;

pub fn human(game: &Game, hw: &Hardware, result: &Assessment) -> String {
    let mut out = String::new();
    out.push_str(
        "Ciri — Can I Run It?\n────────────────────────────────────────────────────────\n",
    );
    out.push_str(&format!("Game: {}\n\n", game.title));
    out.push_str("SYSTEM\n");
    row(
        &mut out,
        "CPU",
        &format!("{} ({} threads)", hw.cpu_name, hw.logical_cores),
        result.cpu,
    );
    row(&mut out, "GPU", &hw.gpu_name, result.gpu);
    row(
        &mut out,
        "VRAM",
        &hw.vram_gb
            .map_or_else(|| "Unknown".into(), |v| format!("{v} GB")),
        result.vram,
    );
    row(&mut out, "RAM", &format!("{} GB", hw.ram_gb), result.ram);
    row(
        &mut out,
        "Storage",
        &format!("{} GB available", hw.storage_gb),
        result.storage,
    );
    row(&mut out, "OS", &hw.os, result.os);
    row(&mut out, "Graphics API", game.api, result.api);
    out.push_str("\nREQUIREMENTS\n");
    out.push_str(&format!("  Minimum       {}\n", game.minimum_label));
    out.push_str(&format!("  Recommended   {}\n", game.recommended_label));
    out.push_str(&format!(
        "  RAM / storage {} GB / {} GB\n",
        game.minimum.ram_gb, game.storage_gb
    ));
    if !result.fps.is_empty() {
        out.push_str("\nESTIMATED PERFORMANCE\n");
        for (label, fps) in &result.fps {
            out.push_str(&format!("  {label:<20} ~{fps} FPS\n"));
        }
    }
    out.push_str("\nVERDICT\n");
    out.push_str(&format!("  {}\n", verdict_label(result.verdict)));
    out.push_str(&format!("  Confidence: {}%\n", result.confidence));
    out.push_str(&format!("  Primary bottleneck: {}\n", result.bottleneck));
    if !result.notes.is_empty() {
        out.push_str("\nNOTES\n");
        for note in &result.notes {
            out.push_str(&format!("  • {note}\n"));
        }
    }
    out
}

fn row(out: &mut String, label: &str, value: &str, status: Status) {
    out.push_str(&format!(
        "  {label:<12} {value:<32} {}\n",
        status_label(status)
    ));
}

fn status_label(status: Status) -> &'static str {
    match status {
        Status::Pass => "✓",
        Status::Borderline => "⚠",
        Status::Fail => "✗",
        Status::Unknown => "~",
    }
}

fn verdict_label(verdict: Verdict) -> &'static str {
    match verdict {
        Verdict::Run => "🟢 RUN",
        Verdict::Compromises => "🟡 RUN WITH COMPROMISES",
        Verdict::DontRun => "🔴 DON'T RUN",
    }
}

pub fn json(game: &Game, hw: &Hardware, result: &Assessment) -> String {
    let fps = result
        .fps
        .iter()
        .map(|(label, value)| format!("{{\"profile\":\"{}\",\"fps\":{value}}}", escape(label)))
        .collect::<Vec<_>>()
        .join(",");
    let notes = result
        .notes
        .iter()
        .map(|note| format!("\"{}\"", escape(note)))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        concat!(
            "{{\n  \"schema_version\": 1,\n  \"game\": {{\"id\": \"{}\", \"title\": \"{}\"}},\n",
            "  \"system\": {{\"cpu\": \"{}\", \"cpu_score\": {}, \"gpu\": \"{}\", \"gpu_score\": {}, \"ram_gb\": {}, \"vram_gb\": {}, \"storage_available_gb\": {}, \"os\": \"{}\"}},\n",
            "  \"assessment\": {{\"verdict\": \"{}\", \"confidence\": {}, \"bottleneck\": \"{}\", \"checks\": {{\"cpu\": \"{}\", \"gpu\": \"{}\", \"ram\": \"{}\", \"vram\": \"{}\", \"storage\": \"{}\", \"os\": \"{}\", \"api\": \"{}\"}}, \"estimated_performance\": [{}]}},\n",
            "  \"linux_support\": \"{}\",\n  \"notes\": [{}]\n}}"
        ),
        game.id, escape(game.title), escape(&hw.cpu_name), hw.cpu_score, escape(&hw.gpu_name), hw.gpu_score,
        hw.ram_gb, hw.vram_gb.map_or("null".to_string(), |v| v.to_string()), hw.storage_gb, escape(&hw.os),
        verdict_json(result.verdict), result.confidence, result.bottleneck,
        status_json(result.cpu), status_json(result.gpu), status_json(result.ram), status_json(result.vram), status_json(result.storage), status_json(result.os), status_json(result.api),
        fps, linux_json(game.linux), notes
    )
}

fn status_json(status: Status) -> &'static str {
    match status {
        Status::Pass => "pass",
        Status::Borderline => "borderline",
        Status::Fail => "fail",
        Status::Unknown => "unknown",
    }
}

fn verdict_json(verdict: Verdict) -> &'static str {
    match verdict {
        Verdict::Run => "run",
        Verdict::Compromises => "run_with_compromises",
        Verdict::DontRun => "dont_run",
    }
}

fn linux_json(support: LinuxSupport) -> &'static str {
    match support {
        LinuxSupport::Native => "native",
        LinuxSupport::ProtonGood => "proton_good",
        LinuxSupport::ProtonMixed => "proton_mixed",
    }
}

fn escape(value: &str) -> String {
    value
        .chars()
        .flat_map(|c| match c {
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\\' => "\\\\".chars().collect(),
            '\n' => "\\n".chars().collect(),
            c if c.is_control() => Vec::new(),
            c => vec![c],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assess::assess;
    use crate::catalog::GAMES;

    #[test]
    fn explanation_is_json_shaped_and_escaped() {
        let hw = Hardware {
            cpu_name: "A \"CPU\"".into(),
            cpu_score: 80,
            logical_cores: 8,
            gpu_name: "GPU".into(),
            gpu_score: 80,
            vram_gb: Some(8),
            ram_gb: 16,
            storage_gb: 200,
            os: "Linux".into(),
            is_linux: true,
            is_laptop: false,
            vulkan: true,
        };
        let result = assess(&GAMES[0], &hw, "1080p", 100);
        let value = json(&GAMES[0], &hw, &result);
        assert!(value.starts_with('{') && value.ends_with('}'));
        assert!(value.contains("A \\\"CPU\\\""));
    }
}
