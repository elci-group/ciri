# Ciri Roadmap: State-of-the-Art Game Compatibility Assessment

## Executive Summary

This roadmap outlines the evolution of Ciri from a promising offline compatibility checker to a state-of-the-art, AI-powered game performance prediction platform. The vision maintains Ciri's core strengths (zero-dependency, offline-first, privacy-focused) while adding advanced capabilities through optional modules and community integration.

## Current State Analysis

### Strengths
- **Zero-dependency architecture**: Pure Rust with no external dependencies
- **Offline-first**: Works without network access, deterministic results
- **Fast and efficient**: Optimized Rust implementation with LTO
- **Privacy-focused**: No telemetry or data collection
- **Modular design**: Clean separation of concerns (assess, catalog, hardware, output)
- **Conservative assessment**: Errs on the side of caution to avoid false positives
- **Cross-platform foundation**: Linux-focused with extensible hardware detection

### Limitations
- **Small catalog**: Only 6 games, manual updates required
- **Basic scoring**: Manual tier mapping for CPU/GPU/RAM
- **Heuristic FPS**: Simple formulas, no real-world validation
- **Linux-only**: Hardware detection limited to Linux `/proc` and `lspci`
- **No ML/AI**: No machine learning or predictive capabilities
- **No real-time data**: No integration with live compatibility databases
- **Limited Steam integration**: No ProtonDB, Steam API, or automatic game detection
- **No web interface**: CLI-only, no GUI or web dashboard

## Strategic Pillars

### 1. **Expand Coverage & Accuracy**
### 2. **Add Intelligent Prediction** 
### 3. **Enable Community Integration**
### 4. **Multi-Platform Support**
### 5. **Developer & Power User Features**

---

## Phase 1: Foundation Expansion (v0.2 - v0.5)

### v0.2: Catalog Automation
**Goal**: Transition from manual to automated catalog management

#### Features
- **Automatic Game Discovery**
  - Steam API integration for game metadata
  - ProtonDB API integration for Linux compatibility ratings
  - SteamDB integration for system requirements
  - Crowdsourced game database (optional opt-in)

- **Dynamic Catalog Updates**
  - `ciri --update-catalog` command
  - Configurable update sources (Steam, ProtonDB, community)
  - Delta updates to minimize bandwidth
  - Catalog versioning and rollback capability

- **Enhanced Game Metadata**
  - Release dates and engine information
  - Known issues and workarounds
  - Proton/Steam Deck compatibility ratings
  - Graphics settings recommendations

#### Technical Implementation
```rust
// New modules
src/catalog/steam_api.rs
src/catalog/protondb.rs
src/catalog/steamdb.rs
src/catalog/community.rs
src/catalog/updater.rs
```

#### Dependencies (Optional)
- `reqwest` for HTTP requests
- `serde` for JSON parsing
- `tokio` for async operations

### v0.3: Enhanced Hardware Detection
**Goal**: Expand hardware detection across platforms and improve accuracy

#### Features
- **Windows Support**
  - WMI queries for CPU/GPU/RAM detection
  - DirectX/DirectML capability detection
  - GPU driver version checking

- **macOS Support**
  - IOKit framework integration
  - Metal capability detection
  - Apple Silicon detection and optimization

- **Advanced GPU Detection**
  - Vulkan physical device properties
  - DirectX feature levels
  - Ray tracing capability detection
  - VRAM and bandwidth measurement

- **Real Hardware Benchmarking**
  - Optional micro-benchmarks for accurate scoring
  - FLOPS estimation for CPU/GPU
  - Memory bandwidth measurement
  - Thermal throttling detection

#### Technical Implementation
```rust
// Enhanced modules
src/hardware/windows.rs
src/hardware/macos.rs
src/hardware/vulkan.rs
src/hardware/benchmark.rs
src/hardware/thermal.rs
```

### v0.4: Improved Assessment Engine
**Goal**: More sophisticated performance prediction logic

#### Features
- **Multi-factor Assessment**
  - CPU single-core vs multi-core consideration
  - GPU architecture-aware scoring (RDNA2, Ada Lovelace, etc.)
  - Storage speed considerations (NVMe vs SATA)
  - Display refresh rate optimization

- **Advanced FPS Estimation**
  - Resolution scaling curves
  - Quality preset multipliers
  - Upscaler impact (DLSS, FSR, XeSS)
  - Frame generation support detection

- **Proton/Steam Deck Specialization**
  - Proton version-specific compatibility
  - Steam Deck LCD vs OLED differences
  - Gamescope compositor considerations
  - Anti-cheat compatibility warnings

#### Technical Implementation
```rust
// Enhanced assessment
src/assess/multi_factor.rs
src/assess/fps_advanced.rs
src/assess/proton.rs
src/assess/steam_deck.rs
```

### v0.5: Configuration & Profiles
**Goal**: User customization and preset management

#### Features
- **User Configuration**
  - `~/.config/ciri/config.toml` for persistent settings
  - Custom quality presets
  - Hardware override capabilities
  - Assessment aggressiveness tuning

- **Hardware Profiles**
  - Save/load hardware configurations
  - "What-if" scenarios with different hardware
  - Profile sharing and import/export

- **Output Customization**
  - Custom output formats (CSV, Markdown, HTML)
  - Theme selection (light/dark/high-contrast)
  - Localization support (i18n)

#### Technical Implementation
```rust
// New modules
src/config.rs
src/profiles.rs
src/i18n.rs
src/output/custom.rs
```

---

## Phase 2: Intelligence & Prediction (v0.6 - v0.9)

### v0.6: Machine Learning Integration
**Goal**: AI-powered performance prediction using federated learning

#### Features
- **Local ML Models**
  - ONNX runtime integration for local inference
  - Pre-trained models for FPS prediction
  - Hardware-to-performance mapping models
  - No cloud dependency for core predictions

- **Federated Learning (Opt-in)**
  - Privacy-preserving model training
  - Local data stays on device
  - Only model gradients shared (aggregated)
  - Contributor recognition system

- **Model Architecture**
  - Ensemble of specialized models (per-game, per-genre)
  - Feature importance explanations
  - Uncertainty quantification
  - Continuous learning from user feedback

#### Technical Implementation
```rust
// ML modules
src/ml/models.rs
src/ml/onnx_runtime.rs
src/ml/federated.rs
src/ml/features.rs
src/ml/training.rs
```

#### Dependencies (Optional)
- `ort` (ONNX Runtime)
- `ndarray` for numerical operations
- `tract` for alternative ML inference

### v0.7: Real-time Telemetry Integration
**Goal**: Live compatibility data from community sources

#### Features
- **ProtonDB Integration**
  - Real-time compatibility ratings
  - Proton version recommendations
  - User-reported workarounds
  - Steam Deck verified status

- **Steam Integration**
  - Automatic game library detection
  - Playtime correlation with performance
  - Achievement-based difficulty assessment
  - Friend system for hardware comparisons

- **Community Database**
  - Crowdsourced FPS reports
  - Hardware configuration sharing
  - Custom settings repositories
  - Verified compatibility lists

#### Technical Implementation
```rust
// Integration modules
src/integration/protondb.rs
src/integration/steam.rs
src/integration/community.rs
src/integration/telemetry.rs
```

### v0.8: Advanced Diagnostics
**Goal**: Deep system analysis and troubleshooting

#### Features
- **System Health Check**
  - Driver version verification
  - System file integrity checks
  - Thermal performance analysis
  - Power delivery assessment

- **Game-Specific Diagnostics**
  - Known issue detection
  - Configuration file validation
  - Save data corruption checks
  - Mod compatibility assessment

- **Remediation Guidance**
  - Automated fix suggestions
  - Step-by-step troubleshooting guides
  - Configuration optimization
  - Community solution links

#### Technical Implementation
```rust
// Diagnostic modules
src/diagnostics/system.rs
src/diagnostics/game.rs
src/diagnostics/remediation.rs
src/diagnostics/guides.rs
```

### v0.9: Predictive Analytics
**Goal**: Future performance and upgrade planning

#### Features
- **Upgrade Recommendations**
  - GPU/CPU upgrade impact predictions
  - Cost-to-performance ratio analysis
  - Bottleneck identification
  - Upgrade path planning

- **Future Game Compatibility**
  - Upcoming game requirements analysis
  - Trend-based hardware predictions
  - Next-gen console comparisons
  - Technology adoption forecasting

- **Performance Trending**
  - Historical performance tracking
  - Driver update impact analysis
  - System degradation monitoring
  - Optimization effectiveness tracking

#### Technical Implementation
```rust
// Analytics modules
src/analytics/upgrades.rs
src/analytics/forecasting.rs
src/analytics/trending.rs
src/analytics/history.rs
```

---

## Phase 3: Platform & Ecosystem (v1.0 - v1.5)

### v1.0: Web Interface & API
**Goal**: GUI and programmatic access

#### Features
- **Web Dashboard**
  - Modern React-based UI
  - Real-time system monitoring
  - Game library management
  - Performance visualization

- **REST API**
  - Full CLI functionality via HTTP
  - Authentication and rate limiting
  - Webhook support for automation
  - Developer documentation

- **Desktop Integration**
  - System tray application
  - Steam overlay integration
  - Discord rich presence
  - Desktop notifications

#### Technical Implementation
```rust
// Web modules
src/web/server.rs
src/web/api.rs
src/web/dashboard.rs
src/web/auth.rs
```

#### Additional Stack
- `axum` or `actix-web` for web server
- `tokio` for async runtime
- `sqlx` for database (optional)
- React/TypeScript for frontend

### v1.1: Cloud Services (Optional)
**Goal**: Enhanced capabilities through cloud integration

#### Features
- **Cloud Catalog**
  - Centralized game database
  - Real-time compatibility updates
  - Community moderation
  - API access for third parties

- **Benchmark Sharing**
  - Anonymous performance data sharing
  - Regional performance comparisons
  - Hardware popularity statistics
  - Game optimization insights

- **Premium Features**
  - Priority catalog updates
  - Advanced ML models
  - Personalized recommendations
  - API rate limit increases

#### Technical Implementation
```rust
// Cloud modules
src/cloud/sync.rs
src/cloud/benchmarks.rs
src/cloud/analytics.rs
src/cloud/auth.rs
```

### v1.2: Mobile & Companion Apps
**Goal**: Cross-platform accessibility

#### Features
- **Mobile Apps**
  - iOS and Android applications
  - QR code hardware scanning
  - On-the-go compatibility checking
  - Push notifications for updates

- **Companion Tools**
  - Browser extension for store integration
  - Discord bot for server queries
  - Steam workshop integration
  - Mod manager compatibility

#### Technical Implementation
```rust
// Mobile support
src/mobile/api.rs
src/mobile/auth.rs
src/mobile/push.rs
```

### v1.3: Developer Tools
**Goal**: Support for game developers and modders

#### Features
- **Game Developer Portal**
  - System requirement testing
  - Performance profiling
  - Compatibility reporting
  - Beta testing coordination

- **Mod Compatibility**
  - Mod load order analysis
  - Performance impact assessment
  - Conflict detection
  - Community mod database

- **SDK & API**
  - Library integration for games
  - In-game compatibility overlay
  - Performance telemetry API
  - White-label solutions

#### Technical Implementation
```rust
// Developer modules
src/developer/portal.rs
src/developer/mods.rs
src/developer/sdk.rs
src/developer/telemetry.rs
```

### v1.4: Enterprise & Professional
**Goal**: B2B and professional use cases

#### Features
- **Enterprise Dashboard**
  - Multi-system management
  - Fleet compatibility reporting
  - Hardware procurement planning
  - IT support integration

- **Professional Tools**
  - Automated testing pipelines
  - CI/CD integration
  - Performance regression testing
  - SLA monitoring

- **Consulting Services**
  - Custom model training
  - Performance optimization
  - Hardware consulting
  - White-label deployment

#### Technical Implementation
```rust
// Enterprise modules
src/enterprise/dashboard.rs
src/enterprise/testing.rs
src/enterprise/cicd.rs
src/enterprise/consulting.rs
```

### v1.5: Ecosystem & Partnerships
**Goal**: Industry integration and standards

#### Features
- **Platform Integration**
  - Steam official integration
  - Epic Games Store support
  - GOG connectivity
  - Itch.io compatibility

- **Hardware Partnerships**
  - GPU vendor optimization guides
  - Pre-installed on systems
  - Hardware bundle partnerships
  - Retail integration

- **Industry Standards**
  - Open compatibility standard
  - Performance benchmarking methodology
  - System requirement guidelines
  - Certification program

---

## Technical Architecture Evolution

### Current Architecture
```
ciri/
├── src/
│   ├── main.rs (CLI entry point)
│   ├── assess.rs (Assessment logic)
│   ├── catalog.rs (Embedded game catalog)
│   ├── hardware.rs (Linux hardware detection)
│   └── output.rs (Output formatting)
```

### Target Architecture
```
ciri/
├── src/
│   ├── main.rs (CLI entry point)
│   ├── core/ (Core assessment engine)
│   │   ├── assess.rs
│   │   ├── scoring.rs
│   │   └── prediction.rs
│   ├── hardware/ (Platform-specific detection)
│   │   ├── mod.rs
│   │   ├── linux.rs
│   │   ├── windows.rs
│   │   ├── macos.rs
│   │   └── benchmark.rs
│   ├── catalog/ (Catalog management)
│   │   ├── mod.rs
│   │   ├── embedded.rs
│   │   ├── steam_api.rs
│   │   ├── protondb.rs
│   │   └── updater.rs
│   ├── ml/ (Machine learning)
│   │   ├── mod.rs
│   │   ├── models.rs
│   │   ├── onnx_runtime.rs
│   │   └── federated.rs
│   ├── integration/ (External services)
│   │   ├── mod.rs
│   │   ├── protondb.rs
│   │   ├── steam.rs
│   │   └── community.rs
│   ├── web/ (Web interface)
│   │   ├── mod.rs
│   │   ├── server.rs
│   │   ├── api.rs
│   │   └── dashboard.rs
│   ├── config/ (Configuration)
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── profiles.rs
│   └── output/ (Output formatting)
│       ├── mod.rs
│       ├── human.rs
│       ├── json.rs
│       └── custom.rs
├── models/ (ML models)
├── web/ (Frontend assets)
└── docs/ (Documentation)
```

---

## Dependency Strategy

### Core Dependencies (Zero-dependency philosophy maintained)
- **Always**: None (pure Rust standard library)

### Optional Dependencies (Feature flags)
```toml
[features]
default = []
steam = ["reqwest", "serde"]
protondb = ["reqwest", "serde"]
ml = ["ort", "ndarray"]
web = ["axum", "tokio"]
benchmark = ["criterion"]
windows = ["winapi", "wmi"]
macos = ["core-foundation", "objc"]
i18n = ["fluent"]
```

### External Services (Optional)
- Steam API (optional authentication)
- ProtonDB API (public, no auth required)
- Custom cloud services (opt-in, privacy-focused)

---

## Privacy & Security Philosophy

### Core Principles
1. **Offline-first**: Core functionality works without network
2. **Privacy by design**: No telemetry without explicit consent
3. **Local processing**: ML models run locally when possible
4. **Federated learning**: Only model gradients shared, never raw data
5. **Transparent**: Open source, auditable code
6. **User control**: Granular privacy controls and opt-in features

### Data Handling
- **Local data**: All hardware assessments stored locally
- **Anonymous sharing**: Only aggregate, anonymized data shared
- **Encryption**: All cloud communications encrypted
- **Compliance**: GDPR and privacy regulation compliant
- **Audit trail**: All data sharing logged and accessible

---

## Performance & Optimization

### Goals
- **Startup time**: <100ms for offline mode
- **Assessment time**: <500ms for single game
- **Memory usage**: <50MB baseline
- **Binary size**: <5MB for core CLI
- **Battery impact**: Minimal on laptops

### Optimization Strategies
- Lazy loading of optional features
- Cached hardware detection results
- Incremental catalog updates
- Efficient ML model quantization
- Database query optimization
- Asset compression and CDN usage

---

## Testing & Quality Assurance

### Testing Strategy
- **Unit tests**: 90%+ coverage for core modules
- **Integration tests**: API and service integration
- **Hardware tests**: Cross-platform hardware matrix
- **ML tests**: Model accuracy validation
- **Performance tests**: Benchmark regression detection
- **Security audits**: Regular penetration testing

### CI/CD Pipeline
- **Automated testing**: All platforms on every commit
- **Hardware matrix**: Test on real hardware configurations
- **ML model validation**: Accuracy regression testing
- **Security scanning**: Dependency vulnerability scanning
- **Performance monitoring**: Binary size and runtime metrics

---

## Community & Governance

### Open Source Development
- **GitHub**: Primary development repository
- **Contributors**: Open contribution policy
- **Roadmap**: Community-driven feature prioritization
- **Transparency**: Public development discussions

### Community Features
- **Game database**: Community-curated game compatibility
- **Hardware profiles**: User-shared hardware configurations
- **Modding support**: Community-created mods and plugins
- **Documentation**: Community-maintained guides

### Governance Model
- **Technical committee**: Core architectural decisions
- **Working groups**: Specialized feature teams
- **Community voting**: Feature prioritization
- **Code of conduct**: Inclusive community guidelines

---

## Business Model & Sustainability

### Freemium Model
- **Free tier**: Core CLI functionality (maintain current)
- **Pro tier**: Advanced features, cloud services, priority updates
- **Enterprise tier**: Custom solutions, SLA, dedicated support
- **Open source**: Core remains MIT-licensed

### Revenue Streams
- **Pro subscriptions**: Individual power users
- **Enterprise licenses**: B2B customers
- **Partnerships**: Hardware vendors, platforms
- ** consulting**: Custom development and optimization

### Cost Structure
- **Infrastructure**: Cloud services, CDN, databases
- **Development**: Core team, community contributors
- **ML training**: Compute resources for model training
- **Support**: Customer service and documentation

---

## Success Metrics

### Technical Metrics
- **Assessment accuracy**: >95% correlation with real performance
- **Catalog coverage**: >10,000 games
- **Platform support**: Linux, Windows, macOS
- **Response time**: <500ms average assessment
- **Uptime**: >99.9% for cloud services

### User Metrics
- **Active users**: >100,000 monthly active users
- **User satisfaction**: >4.5/5 rating
- **Community contributions**: >1,000 contributors
- **API usage**: >1M calls/month
- **Mobile apps**: >50,000 downloads

### Business Metrics
- **Free-to-paid conversion**: >5%
- **Enterprise customers**: >100 companies
- **Partnership deals**: >10 major partnerships
- **Revenue**: Sustainable growth trajectory
- **Market penetration**: Top 3 compatibility tools

---

## Competitive Analysis

### Current Competitors
- **SystemRequirementsLab**: Web-based, limited offline
- **GameDeb**: Crowdsourced, less technical
- **ProtonDB**: Linux-specific, no hardware detection
- **CanYouRunIt**: Outdated, Java-based

### Ciri Advantages
- **Offline-first**: Works without network
- **Privacy-focused**: No forced telemetry
- **Modern architecture**: Rust-based, fast and efficient
- **AI-powered**: Machine learning integration
- **Cross-platform**: Unified experience across OS
- **Developer-friendly**: API and SDK support

### Differentiation Strategy
- **Technical excellence**: Most accurate predictions
- **Privacy leadership**: Strongest privacy guarantees
- **Open source**: Community-driven development
- **Modular design**: Extensible and customizable
- **Performance**: Fastest and most efficient

---

## Risk Mitigation

### Technical Risks
- **ML accuracy**: Continuous validation and fallback to heuristics
- **Platform changes**: Abstraction layers and rapid adaptation
- **Dependency bloat**: Strict dependency management and feature flags
- **Performance regression**: Continuous benchmarking and optimization

### Business Risks
- **Competition**: Focus on unique value propositions
- **Monetization**: Balanced freemium model
- **Platform dependency**: Multi-platform strategy
- **Community fragmentation**: Inclusive governance model

### Legal Risks
- **Data privacy**: GDPR compliance and privacy-by-design
- **IP issues**: MIT license and clear contribution policies
- **Platform terms**: Careful API usage and terms compliance
- **Liability**: Clear disclaimers and user agreements

---

## Timeline & Milestones

### Year 1: Foundation
- **Q1**: v0.2-v0.3 (Catalog automation, enhanced hardware)
- **Q2**: v0.4-v0.5 (Improved assessment, configuration)
- **Q3**: v0.6-v0.7 (ML integration, telemetry)
- **Q4**: v0.8-v0.9 (Diagnostics, analytics)

### Year 2: Platform
- **Q1**: v1.0 (Web interface, API)
- **Q2**: v1.1 (Cloud services, mobile apps)
- **Q3**: v1.2-v1.3 (Companion tools, developer SDK)
- **Q4**: v1.4-v1.5 (Enterprise, partnerships)

### Year 3: Ecosystem
- **Year-long**: Industry adoption, standardization
- **Partnerships**: Major platform and vendor deals
- **Community**: Large-scale contributor base
- **Market**: Leadership position in compatibility space

---

## Conclusion

This roadmap transforms Ciri from a promising offline compatibility checker into a comprehensive, AI-powered game performance prediction platform while maintaining its core strengths of privacy, efficiency, and offline-first design. The phased approach ensures sustainable growth, technical excellence, and community-driven development.

The vision positions Ciri as the definitive tool for game compatibility assessment, serving everyone from casual gamers to enterprise customers, while establishing new standards for privacy, accuracy, and user experience in the gaming ecosystem.