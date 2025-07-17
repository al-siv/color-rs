# Color-rs Build and Release

This document describes the compilation artifacts, cargo workspace layout, feature flags, build processes, and release procedures for color-rs.

## Table of Contents

- [Cargo Workspace Layout](#cargo-workspace-layout)
- [Feature Flags](#feature-flags)
- [Build Profiles](#build-profiles)
- [Target Platforms](#target-platforms)
- [Release Process](#release-process)
- [Reproducible Builds](#reproducible-builds)
- [Continuous Integration](#continuous-integration)

## Cargo Workspace Layout

### Current Structure

Color-rs is currently organized as a **single-crate project** with the following structure:

```
color-rs/
├── Cargo.toml              # Main package configuration
├── Cargo.lock              # Dependency lock file (committed)
├── src/                    # Source code
│   ├── lib.rs              # Library entry point
│   ├── main.rs             # Binary entry point
│   ├── cli.rs              # CLI argument parsing
│   ├── color/              # Color manipulation modules
│   ├── gradient/           # Gradient generation
│   └── ...                 # Other modules
├── color-table/            # Color collection CSV data
│   ├── css-colors.csv      # CSS named colors
│   ├── ral-classic.csv     # RAL Classic colors
│   └── ral-design.csv      # RAL Design System+ colors
├── docs/                   # Documentation
├── examples/               # Usage examples
├── target/                 # Build artifacts (gitignored)
└── tests/                  # Integration tests (planned)
```

### Package Configuration

```toml
[package]
name = "color-rs"
version = "0.11.1"
edition = "2024"            # Latest Rust edition
description = "A CLI tool and Rust library for color gradient calculations using LAB color space with cubic-bezier easing functions"
authors = ["al-siv <https://github.com/al-siv>"]
license = "MIT"
repository = "https://github.com/al-siv/color-rs"
homepage = "https://github.com/al-siv/color-rs"
documentation = "https://github.com/al-siv/color-rs#readme"
keywords = ["color", "gradient", "lab", "css", "cli", "library"]
categories = ["command-line-utilities", "graphics", "multimedia::images", "development-tools"]
readme = "README.md"
default-run = "color-rs"    # Default binary name
```

### Future Workspace Layout (Planned)

For future expansion, the project may evolve into a workspace structure:

```
color-rs-workspace/
├── Cargo.toml              # Workspace root
├── color-rs/               # Main library and CLI
│   ├── Cargo.toml
│   ├── src/
│   └── ...
├── color-rs-web/           # Web assembly bindings (planned)
│   ├── Cargo.toml
│   └── src/
├── color-rs-ffi/           # C FFI bindings (planned)
│   ├── Cargo.toml
│   └── src/
└── shared/                 # Shared utilities
    ├── color-types/        # Common color types
    └── color-data/         # Color collection data
```

## Feature Flags

### Current Feature Strategy

Currently, color-rs does **not use feature flags** to keep the dependency graph simple and ensure a consistent experience. All features are always available.

### Planned Feature Flags (Future Versions)

```toml
[features]
default = ["cli", "image-generation", "collections-bundled"]

# Core features
cli = ["clap"]                              # CLI interface
library-only = []                           # Library-only mode (no CLI deps)

# Image generation
image-generation = ["image", "tiny-skia", "usvg", "resvg"]
svg-only = ["usvg", "resvg"]               # SVG generation only
png-only = ["image", "tiny-skia"]          # PNG generation only

# Color collections
collections-bundled = []                    # Embed CSV data in binary
collections-runtime = ["csv"]              # Load CSV at runtime
collections-minimal = []                   # CSS colors only

# Advanced features
parallel = ["rayon"]                       # Parallel processing
caching = ["dashmap"]                      # Color collection caching
validation = ["regex"]                     # Enhanced input validation

# Development features
dev-tools = ["criterion", "proptest"]     # Development and testing tools
```

### Feature Combinations

```bash
# Minimal library (no CLI, no images)
cargo build --no-default-features --features library-only

# CLI only (no image generation)
cargo build --no-default-features --features cli,collections-bundled

# Full-featured build (default)
cargo build

# Web-optimized build (planned)
cargo build --target wasm32-unknown-unknown --features collections-minimal

# Performance-optimized build (planned)
cargo build --features parallel,caching --release
```

## Build Profiles

### Development Profile

```toml
[profile.dev]
opt-level = 0           # No optimization for fast compilation
debug = true            # Full debug info
debug-assertions = true # Debug assertions enabled
overflow-checks = true  # Integer overflow checks
lto = false            # No link-time optimization
panic = 'unwind'       # Unwind on panic for debugging
incremental = true     # Incremental compilation
codegen-units = 256    # Parallel codegen for fast compilation
```

**Use case**: Development, testing, debugging

```bash
cargo build                    # Development build
cargo run -- gradient red blue  # Run with development optimizations
```

### Release Profile

```toml
[profile.release]
opt-level = 3           # Full optimization
debug = false           # No debug info
debug-assertions = false # No debug assertions
overflow-checks = false # No overflow checks (for performance)
lto = true             # Link-time optimization
panic = 'abort'        # Abort on panic (smaller binaries)
incremental = false    # No incremental compilation
codegen-units = 1      # Single codegen unit for better optimization
strip = true           # Strip symbols for smaller binaries
```

**Use case**: Production builds, distribution

```bash
cargo build --release          # Optimized build
./target/release/color-rs gradient red blue
```

### Performance Testing Profile

```toml
[profile.bench]
opt-level = 3
debug = false
debug-assertions = false
overflow-checks = false
lto = true
panic = 'abort'
incremental = false
codegen-units = 1
```

**Use case**: Benchmarking and performance testing

```bash
cargo bench               # Run criterion benchmarks
```

### Size-Optimized Profile

```toml
[profile.release-small]
inherits = "release"
opt-level = 'z'          # Optimize for size
lto = true
panic = 'abort'
strip = true
codegen-units = 1
```

**Use case**: Embedded systems, distribution size constraints

```bash
cargo build --profile release-small
```

## Target Platforms

### Tier 1 Platforms (Fully Supported)

#### x86_64-unknown-linux-gnu
- **OS**: Linux (GNU libc)
- **Architecture**: x86_64
- **Support**: Full feature support
- **Testing**: Automated CI testing

```bash
cargo build --target x86_64-unknown-linux-gnu --release
```

#### x86_64-pc-windows-msvc
- **OS**: Windows (MSVC toolchain)  
- **Architecture**: x86_64
- **Support**: Full feature support
- **Testing**: Automated CI testing

```bash
cargo build --target x86_64-pc-windows-msvc --release
```

#### x86_64-apple-darwin
- **OS**: macOS
- **Architecture**: x86_64 Intel
- **Support**: Full feature support
- **Testing**: Automated CI testing

```bash
cargo build --target x86_64-apple-darwin --release
```

#### aarch64-apple-darwin
- **OS**: macOS
- **Architecture**: Apple Silicon (M1/M2)
- **Support**: Full feature support
- **Testing**: Automated CI testing

```bash
cargo build --target aarch64-apple-darwin --release
```

### Tier 2 Platforms (Best Effort Support)

#### aarch64-unknown-linux-gnu
- **OS**: Linux (ARM64)
- **Support**: Library and CLI, image generation may have limitations
- **Testing**: Manual testing

#### x86_64-unknown-linux-musl
- **OS**: Linux (musl libc, Alpine)
- **Support**: Static linking, smaller binaries
- **Use case**: Docker containers, embedded Linux

```bash
# Install target
rustup target add x86_64-unknown-linux-musl

# Build static binary
cargo build --target x86_64-unknown-linux-musl --release
```

#### wasm32-unknown-unknown (Planned)
- **Platform**: WebAssembly
- **Support**: Library only, no file I/O
- **Use case**: Web browsers, server-side WASM

```bash
# Planned build command
cargo build --target wasm32-unknown-unknown --no-default-features --features library-only
```

### Cross-Compilation Setup

#### Using cargo-cross

```bash
# Install cross-compilation tool
cargo install cross

# Cross-compile for different targets
cross build --target aarch64-unknown-linux-gnu --release
cross build --target x86_64-pc-windows-gnu --release
```

#### Manual Cross-Compilation

```bash
# Install target
rustup target add aarch64-unknown-linux-gnu

# Configure linker (in .cargo/config.toml)
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

# Build
cargo build --target aarch64-unknown-linux-gnu --release
```

## Release Process

### Version Numbering

Color-rs follows [Semantic Versioning (SemVer)](https://semver.org/):

- **MAJOR**: Incompatible API changes
- **MINOR**: Backward-compatible functionality additions
- **PATCH**: Backward-compatible bug fixes

#### Version Lifecycle

```
0.11.1 (current) → 0.11.2 (patch) → 0.12.0 (minor) → 1.0.0 (major)
```

### Release Checklist

#### Pre-Release (Development)

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md` with new changes
- [ ] Run full test suite: `cargo test --all-features`
- [ ] Run clippy: `cargo clippy --all-targets --all-features`
- [ ] Format code: `cargo fmt --all`
- [ ] Update documentation: `cargo doc --all-features`
- [ ] Test examples: `cargo run --example library_usage`

#### Release Build

- [ ] Create release builds for all target platforms
- [ ] Verify binary functionality on each platform
- [ ] Run performance benchmarks: `cargo bench`
- [ ] Test installation: `cargo install --path .`

#### Distribution

- [ ] Create Git tag: `git tag v0.11.1`
- [ ] Push tag: `git push origin v0.11.1`
- [ ] Publish to crates.io: `cargo publish`
- [ ] Create GitHub release with binaries
- [ ] Update documentation sites

#### Post-Release

- [ ] Verify crates.io publication
- [ ] Test installation from crates.io: `cargo install color-rs`
- [ ] Update README badges and links
- [ ] Announce release in relevant channels

### Automated Release Process

#### GitHub Actions Workflow

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build-and-release:
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
          - target: x86_64-pc-windows-msvc
            os: windows-latest
          - target: x86_64-apple-darwin
            os: macos-latest
          - target: aarch64-apple-darwin
            os: macos-latest
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          target: ${{ matrix.target }}
          
      - name: Build release
        run: cargo build --release --target ${{ matrix.target }}
        
      - name: Create release archive
        # Platform-specific archive creation
        
      - name: Upload to release
        # Upload binaries to GitHub release
```

## Reproducible Builds

### Cargo.lock Commitment

```bash
# Cargo.lock is committed to ensure reproducible builds
git add Cargo.lock
git commit -m "Update dependencies"
```

### Dependency Pinning

```toml
# Use exact versions for release builds
[dependencies]
clap = "=4.5.0"          # Exact version
palette = "=0.7.0"       # Exact version
anyhow = "=1.0.75"       # Exact version
```

### Build Environment

#### Container-based Builds

```dockerfile
# Dockerfile for reproducible builds
FROM rust:1.75-slim as builder

WORKDIR /usr/src/color-rs
COPY . .

# Use locked dependencies
RUN cargo build --release --locked

FROM debian:bookworm-slim
COPY --from=builder /usr/src/color-rs/target/release/color-rs /usr/local/bin/color-rs
CMD ["color-rs"]
```

#### Build Commands

```bash
# Reproducible build commands
cargo build --release --locked --target x86_64-unknown-linux-gnu
cargo vendor                    # Vendor dependencies for offline builds
cargo build --release --locked --offline
```

### Verification

```bash
# Verify build reproducibility
cargo auditable build --release
cargo audit                     # Security audit
shasum -a 256 target/release/color-rs  # Binary checksum
```

## Continuous Integration

### GitHub Actions Matrix

```yaml
# .github/workflows/ci.yml
strategy:
  matrix:
    os: [ubuntu-latest, windows-latest, macos-latest]
    rust: [stable, beta, nightly]
    features: [
      "--all-features",
      "--no-default-features --features cli",
      "--no-default-features --features library-only"
    ]
```

### CI Pipeline Stages

1. **Lint and Format**
   - `cargo fmt --all -- --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`

2. **Test**
   - `cargo test --all-features`
   - `cargo test --no-default-features`
   - `cargo doc --all-features`

3. **Build**
   - `cargo build --all-features`
   - `cargo build --release`

4. **Integration**
   - Example execution tests
   - CLI smoke tests
   - Performance regression tests

### Performance Monitoring

```yaml
# Criterion.rs benchmarks in CI
- name: Run benchmarks
  run: |
    cargo bench --all-features
    cargo bench --baseline=main
```

## Development Workflow

### Local Development

```bash
# Setup development environment
git clone https://github.com/al-siv/color-rs.git
cd color-rs
cargo build

# Run tests
cargo test --all-features
cargo test --doc

# Development cycle
cargo check                    # Fast syntax checking
cargo clippy                   # Linting
cargo fmt                      # Formatting
cargo run -- gradient red blue # Test CLI
```

### Contributing Builds

```bash
# Pre-commit checks
cargo test --all-features
cargo clippy --all-targets --all-features
cargo fmt --all
cargo doc --all-features

# Integration testing
cargo run --example library_usage
cargo install --path . --force
color-rs gradient red blue --svg
```

This comprehensive build and release documentation ensures consistent, reproducible builds across all supported platforms and provides clear guidelines for both development and production deployment.
