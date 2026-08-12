mod assess;
mod catalog;
mod hardware;
mod output;

#[cfg(feature = "config")]
mod config;
#[cfg(any(feature = "ml-local", feature = "ml-federated"))]
mod ml;
#[cfg(any(feature = "steam", feature = "diagnostics"))]
mod telemetry;
#[cfg(feature = "diagnostics")]
mod diagnostics;
#[cfg(feature = "analytics")]
mod analytics;

use catalog::GAMES;

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    
    #[cfg(feature = "catalog-update")]
    {
        tokio::runtime::Runtime::new().unwrap().block_on(async_main(args))
    }
    
    #[cfg(not(feature = "catalog-update"))]
    {
        sync_main(args)
    }
}

#[cfg(feature = "catalog-update")]
async fn async_main(args: Vec<String>) -> ExitCode {
    match run_impl(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err((message, code)) => {
            eprintln!("ciri: {message}");
            ExitCode::from(code)
        }
    }
}

#[cfg(not(feature = "catalog-update"))]
fn sync_main(args: Vec<String>) -> ExitCode {
    match run_impl(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err((message, code)) => {
            eprintln!("ciri: {message}");
            ExitCode::from(code)
        }
    }
}

#[cfg(feature = "catalog-update")]
async fn run_impl(args: Vec<String>) -> Result<(), (String, u8)> {
    run_impl_core(args).await
}

#[cfg(not(feature = "catalog-update"))]
fn run_impl(args: Vec<String>) -> Result<(), (String, u8)> {
    run_impl_core(args)
}

#[cfg(feature = "catalog-update")]
async fn run_impl_core(args: Vec<String>) -> Result<(), (String, u8)> {
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return Ok(());
    }
    if args.iter().any(|arg| arg == "--version" || arg == "-V") {
        println!("ciri {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    if args.iter().any(|arg| arg == "--list") {
        println!("Games in the offline catalog:");
        for game in GAMES {
            println!("  {}", game.title);
        }
        return Ok(());
    }
    
    #[cfg(feature = "catalog-update")]
    if args.iter().any(|arg| arg == "--update-catalog") {
        use catalog::updater::update_catalog;
        println!("Updating catalog from online sources...");
        match update_catalog().await {
            Ok(result) => {
                println!("Catalog updated successfully!");
                println!("  Games updated: {}", result.games_updated);
                println!("  Games added: {}", result.games_added);
                if !result.games_failed.is_empty() {
                    println!("  Games failed: {}", result.games_failed.len());
                    for game in result.games_failed {
                        println!("    - {}", game);
                    }
                }
                println!("  Sources used: {}", result.sources_used.join(", "));
            }
            Err(e) => {
                eprintln!("Failed to update catalog: {}", e);
                return Err((e.to_string(), 1));
            }
        }
        return Ok(());
    }

    let mut explain = false;
    let mut target = "1080p".to_string();
    let mut title = Vec::new();
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--explain" | "--json" => explain = true,
            "--target" => {
                index += 1;
                target = args
                    .get(index)
                    .ok_or_else(|| ("--target requires 720p, 1080p, 1440p, or 4k".to_string(), 2))?
                    .to_ascii_lowercase();
                if !matches!(
                    target.as_str(),
                    "720"
                        | "720p"
                        | "1080"
                        | "1080p"
                        | "1440"
                        | "1440p"
                        | "2k"
                        | "2160"
                        | "2160p"
                        | "4k"
                ) {
                    return Err((format!("unsupported target '{target}'"), 2));
                }
            }
            value if value.starts_with('-') => {
                return Err((format!("unknown option '{value}'"), 2))
            }
            value => title.push(value),
        }
        index += 1;
    }
    if title.is_empty() {
        return Err(("provide a game title; try --list".to_string(), 2));
    }
    let query = title.join(" ");
    let (game, resolver_confidence) = catalog::resolve(&query).ok_or_else(|| {
        (
            format!("no close match for '{query}' in the offline catalog; try --list"),
            3,
        )
    })?;
    let hw = hardware::Hardware::detect();
    let result = assess::assess(
        game,
        &hw,
        assess::canonical_target(&target),
        resolver_confidence,
    );
    if explain {
        println!("{}", output::json(game, &hw, &result));
    } else {
        print!("{}", output::human(game, &hw, &result));
    }
    Ok(())
}

#[cfg(not(feature = "catalog-update"))]
fn run_impl_core(args: Vec<String>) -> Result<(), (String, u8)> {
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return Ok(());
    }
    if args.iter().any(|arg| arg == "--version" || arg == "-V") {
        println!("ciri {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    if args.iter().any(|arg| arg == "--list") {
        println!("Games in the offline catalog:");
        for game in GAMES {
            println!("  {}", game.title);
        }
        return Ok(());
    }

    let mut explain = false;
    let mut target = "1080p".to_string();
    let mut title = Vec::new();
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--explain" | "--json" => explain = true,
            "--target" => {
                index += 1;
                target = args
                    .get(index)
                    .ok_or_else(|| ("--target requires 720p, 1080p, 1440p, or 4k".to_string(), 2))?
                    .to_ascii_lowercase();
                if !matches!(
                    target.as_str(),
                    "720"
                        | "720p"
                        | "1080"
                        | "1080p"
                        | "1440"
                        | "1440p"
                        | "2k"
                        | "2160"
                        | "2160p"
                        | "4k"
                ) {
                    return Err((format!("unsupported target '{target}'"), 2));
                }
            }
            value if value.starts_with('-') => {
                return Err((format!("unknown option '{value}'"), 2))
            }
            value => title.push(value),
        }
        index += 1;
    }
    if title.is_empty() {
        return Err(("provide a game title; try --list".to_string(), 2));
    }
    let query = title.join(" ");
    let (game, resolver_confidence) = catalog::resolve(&query).ok_or_else(|| {
        (
            format!("no close match for '{query}' in the offline catalog; try --list"),
            3,
        )
    })?;
    let hw = hardware::Hardware::detect();
    let result = assess::assess(
        game,
        &hw,
        assess::canonical_target(&target),
        resolver_confidence,
    );
    if explain {
        println!("{}", output::json(game, &hw, &result));
    } else {
        print!("{}", output::human(game, &hw, &result));
    }
    Ok(())
}

fn print_help() {
    let mut help = String::from("Ciri — Can I Run It?\n\n");
    help.push_str("USAGE\n  ciri <GAME> [OPTIONS]\n\n");
    help.push_str("OPTIONS\n");
    help.push_str("  --target <RESOLUTION>  Performance target: 720p, 1080p, 1440p, or 4k\n");
    help.push_str("  --explain, --json      Emit a machine-readable JSON diagnostic\n");
    help.push_str("  --list                 List games in the bundled offline catalog\n");
    
    #[cfg(feature = "catalog-update")]
    help.push_str("  --update-catalog       Update catalog from online sources (requires catalog-update feature)\n");
    
    help.push_str("  -h, --help             Show help\n");
    help.push_str("  -V, --version          Show version\n\n");
    help.push_str("EXAMPLES\n");
    help.push_str("  ciri \"Batman Arkham Knight\"\n");
    help.push_str("  ciri \"Cyberpunk 2077\" --target 720p\n");
    help.push_str("  ciri bg3 --explain\n");
    
    println!("{}", help);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_bad_target_and_unknown_game() {
        #[cfg(not(feature = "catalog-update"))]
        {
            assert_eq!(
                run_impl(vec!["Cyberpunk".into(), "--target".into(), "8k".into()])
                    .unwrap_err()
                    .1,
                2
            );
            assert_eq!(run_impl(vec!["Spreadsheet Deluxe".into()]).unwrap_err().1, 3);
        }
    }
    
    #[cfg(feature = "ml-local")]
    #[test]
    fn test_ml_integration() {
        use ml::PerformancePredictor;
        let predictor = PerformancePredictor::new();
        assert!(predictor.is_ok());
    }
    
    #[cfg(feature = "diagnostics")]
    #[test]
    fn test_diagnostics() {
        use diagnostics::SystemHealth;
        let hw = hardware::Hardware {
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
        assert!(!health.recommendations.is_empty());
    }
    
    #[cfg(feature = "analytics")]
    #[test]
    fn test_analytics() {
        use analytics::UpgradeRecommendation;
        use catalog::GAMES;
        let hw = hardware::Hardware {
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
        let games = &GAMES;
        let recommendations = UpgradeRecommendation::analyze(&hw, games);
        assert_eq!(recommendations.len(), 4);
    }
}