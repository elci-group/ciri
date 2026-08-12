# Ciri — Can I Run It?

Ciri is a fast, offline Rust CLI that assesses whether the current machine can run a game. It goes beyond a literal requirements checklist: Ciri identifies the likely bottleneck, accounts for Linux/Proton support and laptop power limits, estimates FPS at a chosen resolution, and returns one of three practical verdicts.

```console
$ ciri "Cyberpunk 2077" --target 1080p
Ciri — Can I Run It?
────────────────────────────────────────────────────────
Game: Cyberpunk 2077
...
VERDICT
  🟡 RUN WITH COMPROMISES
```

## Install

### Basic Installation (Default Features)

Build with a stable Rust toolchain:

```bash
cargo install --path .
```

### With Catalog Updates

Build with catalog update features to enable online game database integration:

```bash
cargo install --path . --features catalog-update
```

### Full Features

Build with all optional features:

```bash
cargo install --path . --features full
```

## Features

### Core Features (Default)
- **Offline-first**: Works without network access with embedded game catalog
- **Zero dependencies**: Pure Rust implementation with no external dependencies
- **Fast and efficient**: Optimized Rust with LTO and codegen optimizations
- **Privacy-focused**: No telemetry or data collection
- **Cross-platform**: Linux, Windows, and macOS support (with optional features)

### Optional Features

#### Catalog Automation (`catalog-update`)
- **Steam API integration**: Fetch game metadata from Steam
- **ProtonDB integration**: Get Linux compatibility ratings
- **Dynamic catalog updates**: `--update-catalog` command to refresh game database
- **Enhanced metadata**: Release dates, engine info, Proton ratings

#### Enhanced Hardware Detection (`windows-detection`, `macos-detection`)
- **Windows support**: WMI-based hardware detection on Windows
- **macOS support**: sysctl and system_profiler integration on macOS
- **Platform-specific optimizations**: Tailored detection for each OS

#### Advanced Assessment (`advanced-assessment`)
- **Multi-factor assessment**: CPU single-core vs multi-core consideration
- **GPU architecture awareness**: Ray tracing and VRAM pressure analysis
- **Game-specific optimizations**: CPU-intensive game detection

#### Configuration (`config`)
- **User configuration**: Persistent settings in `~/.config/ciri/config.toml`
- **Custom presets**: Save and load custom quality presets
- **Behavior tuning**: Adjust assessment aggressiveness and thresholds

## Use

### Basic Usage

```bash
ciri "Batman Arkham Knight"
ciri "Cyberpunk 2077" --target 720p
ciri bg3 --explain
ciri --list
```

### With Catalog Updates

```bash
# Update catalog from online sources
ciri --update-catalog

# Then use updated catalog
ciri "New Game"
```

### Supported Targets
- `720p`, `1080p` (default), `1440p`, `4k`

### Output Formats
- `--explain` / `--json`: Machine-readable JSON for scripts
- Exit code `2`: Invalid CLI usage
- Exit code `3`: No game matched

## How the assessment works

The bundled catalog records normalized minimum and recommended CPU/GPU capability tiers, RAM, VRAM, storage, graphics API, Linux support, and known caveats. Local hardware is scored conservatively against those tiers. Multiple hard failures produce `DON'T RUN`; a borderline component, unknown critical signal, or single shortfall produces `RUN WITH COMPROMISES`; meeting the recommended tier produces `RUN`.

FPS values are heuristic estimates, not benchmarks. They are derived from CPU and GPU headroom, VRAM pressure, target resolution, and quality preset. Driver versions, cooling, background load, game patches, upscalers, and individual graphics settings can materially change real performance.

The catalog is deliberately embedded so the command works without network access and produces deterministic answers. The first release includes Batman: Arkham Knight, Cyberpunk 2077, Elden Ring, Baldur's Gate 3, Grand Theft Auto V, and The Witcher 3.

## Architecture

Ciri is designed with a modular architecture that maintains zero-dependency principles while enabling optional features:

```
src/
├── assess/           # Assessment logic
├── catalog/          # Game catalog management
│   ├── embedded.rs   # Offline embedded catalog
│   ├── steam_api.rs  # Steam integration (optional)
│   ├── protondb.rs   # ProtonDB integration (optional)
│   └── updater.rs    # Catalog updates (optional)
├── hardware/         # Hardware detection
│   ├── linux.rs      # Linux detection (default)
│   ├── windows.rs    # Windows detection (optional)
│   └── macos.rs      # macOS detection (optional)
├── config.rs         # User configuration (optional)
└── output.rs         # Output formatting
```

## Develop

### Basic Development

```bash
cargo fmt --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --release
```

### With All Features

```bash
cargo test --all-targets --features full
cargo clippy --all-targets --features full -- -D warnings
cargo build --release --features full
```

### Feature Testing

Test specific feature combinations:

```bash
# Test catalog update features
cargo test --features catalog-update

# Test Windows detection
cargo test --features windows-detection

# Test full feature set
cargo test --features full
```

## Philosophy

Ciri maintains a strong commitment to:

1. **Zero-dependency core**: Core functionality works without external dependencies
2. **Offline-first design**: Primary use case doesn't require network access
3. **Privacy by default**: No telemetry or data collection without explicit consent
4. **Optional enhancements**: Advanced features available via feature flags
5. **Performance optimization**: Fast startup and low memory footprint
6. **Cross-platform support**: Works on Linux, Windows, and macOS

## License

MIT
