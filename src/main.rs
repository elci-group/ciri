mod assess;
mod catalog;
mod hardware;
mod output;

use std::process::ExitCode;

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err((message, code)) => {
            eprintln!("ciri: {message}");
            ExitCode::from(code)
        }
    }
}

fn run(args: Vec<String>) -> Result<(), (String, u8)> {
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
        for game in catalog::GAMES {
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
    println!(concat!(
        "Ciri — Can I Run It?\n\n",
        "USAGE\n  ciri <GAME> [OPTIONS]\n\n",
        "OPTIONS\n",
        "  --target <RESOLUTION>  Performance target: 720p, 1080p, 1440p, or 4k\n",
        "  --explain, --json      Emit a machine-readable JSON diagnostic\n",
        "  --list                 List games in the bundled offline catalog\n",
        "  -h, --help             Show help\n",
        "  -V, --version          Show version\n\n",
        "EXAMPLES\n",
        "  ciri \"Batman Arkham Knight\"\n",
        "  ciri \"Cyberpunk 2077\" --target 720p\n",
        "  ciri bg3 --explain\n"
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_bad_target_and_unknown_game() {
        assert_eq!(
            run(vec!["Cyberpunk".into(), "--target".into(), "8k".into()])
                .unwrap_err()
                .1,
            2
        );
        assert_eq!(run(vec!["Spreadsheet Deluxe".into()]).unwrap_err().1, 3);
    }
}
