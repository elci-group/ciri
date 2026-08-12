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

### Phase 2 Features (Intelligence & Prediction)

Build with machine learning and analytics features:

```bash
# ML and intelligence features
cargo install --path . --features ml-federated

# Diagnostics and analytics
cargo install --path . --features diagnostics

# Full Phase 2 feature set
cargo install --path . --features ml-federated,diagnostics,analytics
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

### Phase 1 Features (v0.2-v0.5)

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

### Phase 2 Features (v0.3-v0.9)

#### Machine Learning Integration (`ml-local`, `ml-federated`)
- **Local ML models**: Performance prediction using heuristic models
- **Hardware-to-performance mapping**: Advanced FPS estimation algorithms
- **Federated learning foundation**: Privacy-preserving model training framework
- **Feature extraction**: Comprehensive hardware feature vectors for ML models

#### Real-time Telemetry (`steam`, `diagnostics`)
- **Enhanced Steam integration**: Playtime correlation and library analysis
- **Community database**: Crowdsourced FPS reports and performance data
- **Live compatibility data**: Real-time updates from community sources

#### Advanced Diagnostics (`diagnostics`)
- **System health check**: Comprehensive component health analysis
- **Game-specific diagnostics**: Known issues, fixes, and configuration recommendations
- **Remediation guidance**: Automated fix suggestions and manual troubleshooting steps
- **Driver status**: GPU driver detection and Vulkan/DirectX support analysis

#### Predictive Analytics (`analytics`)
- **Upgrade recommendations**: Cost-benefit analysis for hardware upgrades
- **Future game compatibility**: Assessment of upcoming game requirements
- **Performance trending**: Historical performance analysis and degradation prediction
- **Upgrade path planning**: Strategic hardware upgrade recommendations

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

### ML-Powered Assessment (Phase 2)

```bash
# Build with ML features
cargo install --path . --features ml-federated

# ML-enhanced assessment
ciri "Cyberpunk 2077" --target 1080p
```

### Diagnostics (Phase 2)

```bash
# Build with diagnostics features
cargo install --path . --features diagnostics

# System health check
ciri --diagnose-system

# Game-specific diagnostics
ciri "Cyberpunk 2077" --diagnose-game
```

### Analytics (Phase 2)

```bash
# Build with analytics features
cargo install --path . --features analytics

# Upgrade recommendations
ciri --analyze-upgrades

# Future compatibility
ciri "Upcoming Game" --future-compat
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
│   └── multi_factor.rs  # Advanced assessment algorithms
├── catalog/          # Game catalog management
│   ├── embedded.rs   # Offline embedded catalog
│   ├── steam_api.rs  # Steam integration (optional)
│   ├── protondb.rs   # ProtonDB integration (optional)
│   └── updater.rs    # Catalog updates (optional)
├── hardware/         # Hardware detection
│   ├── linux.rs      # Linux detection (default)
│   ├── windows.rs    # Windows detection (optional)
│   └── macos.rs      # macOS detection (optional)
├── ml/               # Machine learning (Phase 2)
│   ├── models.rs     # ML model architectures
│   ├── onnx_runtime.rs  # ONNX Runtime integration
│   └── federated.rs  # Federated learning framework
├── telemetry/        # Real-time data (Phase 2)
│   └── mod.rs       # Steam and community integration
├── diagnostics/      # System diagnostics (Phase 2)
│   └── mod.rs       # Health checks and troubleshooting
├── analytics/        # Predictive analytics (Phase 2)
│   └── mod.rs       # Upgrade planning and forecasting
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

# Test ML features
cargo test --features ml-federated

# Test diagnostics
cargo test --features diagnostics

# Test analytics
cargo test --features analytics

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
7. **Privacy-preserving ML**: Federated learning for model training without sharing raw data
8. **Community-driven**: Crowdsourced data and collaborative improvement

## Roadmap

Ciri follows a comprehensive 3-phase roadmap:

- **Phase 1 (v0.2-v0.5)**: Foundation - Catalog automation, enhanced hardware detection, configuration
- **Phase 2 (v0.3-v0.9)**: Intelligence & Prediction - ML integration, real-time telemetry, diagnostics, analytics
- **Phase 3 (v1.0-v1.5)**: Platform & Ecosystem - Web interface, cloud services, mobile apps, developer tools

See [ROADMAP.md](ROADMAP.md) for detailed roadmap information.

## License

MIT
