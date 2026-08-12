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

Build with a stable Rust toolchain:

```bash
cargo install --path .
```

Ciri currently targets Linux hardware detection. It reads CPU and memory information from `/proc`, detects the display adapter with `lspci`, checks free disk space with `df`, and uses `vulkaninfo` when available. Missing signals are reported as unknown and reduce confidence rather than becoming false failures.

## Use

```bash
ciri "Batman Arkham Knight"
ciri "Cyberpunk 2077" --target 720p
ciri bg3 --explain
ciri --list
```

Supported targets are `720p`, `1080p` (default), `1440p`, and `4k`. `--explain` (also `--json`) emits versioned JSON suitable for scripts. Exit code `2` means invalid CLI usage and `3` means no game matched.

## How the assessment works

The bundled catalog records normalized minimum and recommended CPU/GPU capability tiers, RAM, VRAM, storage, graphics API, Linux support, and known caveats. Local hardware is scored conservatively against those tiers. Multiple hard failures produce `DON'T RUN`; a borderline component, unknown critical signal, or single shortfall produces `RUN WITH COMPROMISES`; meeting the recommended tier produces `RUN`.

FPS values are heuristic estimates, not benchmarks. They are derived from CPU and GPU headroom, VRAM pressure, target resolution, and quality preset. Driver versions, cooling, background load, game patches, upscalers, and individual graphics settings can materially change real performance.

The catalog is deliberately embedded so the command works without network access and produces deterministic answers. The first release includes Batman: Arkham Knight, Cyberpunk 2077, Elden Ring, Baldur's Gate 3, Grand Theft Auto V, and The Witcher 3.

## Develop

```bash
cargo fmt --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo build --release
```

The project intentionally has no third-party Rust dependencies.

## License

MIT
