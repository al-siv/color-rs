# Build and Release Guide v0.15.4

Comprehensive guide for building, testing, and releasing color-rs, including compilation artifacts, workspace layout, feature flags, and release procedures.

## Quick Reference

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Cross-platform builds
cross build --target aarch64-apple-darwin --release
cross build --target x86_64-unknown-linux-gnu --release
```

## Cargo Workspace Layout

### Project Structure

Color-rs is organized as a **single-crate project** with the following structure:

```
color-rs/
├── Cargo.toml              # Main package configuration
├── Cargo.lock              # Dependency lock file (committed)
├── Cross.toml              # Cross-compilation configuration
├── src/                    # Source code
│   ├── lib.rs              # Library entry point
│   ├── main.rs             # Binary entry point
│   ├── cli.rs              # CLI argument parsing
│   ├── precision_utils.rs  # Floating-point formatting (v0.15.4)
│   ├── output_filter/      # Output filtering system
│   ├── color_parser/       # Color parsing subsystem
│   ├── color_utils/        # Utility functions
│   ├── gradient/           # Gradient generation
│   └── ...                 # Other modules
├── color-table/            # Color collection CSV data
│   ├── css-colors.csv      # CSS named colors (140+)
│   ├── ral-classic.csv     # RAL Classic colors (213)
│   └── ral-design.csv      # RAL Design System+ colors (1825+)
├── docs/                   # Documentation
├── examples/               # Usage examples
├── phases/                 # Project management files
├── target/                 # Build artifacts (gitignored)
└── tests/                  # Integration tests
```

### Package Configuration

```toml
[package]
name = "color-rs"
version = "0.15.4"
edition = "2021"
description = "A CLI tool and Rust library for color analysis and gradient generation using LAB color space with cubic-bezier easing functions"
license = "MIT OR Apache-2.0"
repository = "https://github.com/username/color-rs"
documentation = "https://docs.rs/color-rs"
readme = "README.md"
keywords = ["color", "gradient", "lab", "accessibility", "wcag"]
categories = ["command-line-utilities", "graphics", "accessibility"]

[dependencies]
clap = { version = "4.4", features = ["derive"] }
palette = "0.7"
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
toml = "0.8"
csv = "1.3"
thiserror = "1.0"

[dev-dependencies]
criterion = "0.5"
proptest = "1.4"

[[bin]]
name = "color-rs"
path = "src/main.rs"

[lib]
name = "color_rs"
path = "src/lib.rs"
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
output-filtering = []                      # Output filtering system (v0.15.4+)

# Development features
dev-tools = ["criterion", "proptest"]     # Development and testing tools
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
cargo run -- --filter "[input,conversion]" red  # Test filtering features (v0.15.4+)
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
./target/release/color-rs --filter "[conversion]" red  # Test filtering (v0.15.4+)
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

### Cross-Compilation Setup

#### Using cargo-cross

```bash
# Install cross-compilation tool
cargo install cross

# Cross-compile for different targets
cross build --target aarch64-unknown-linux-gnu --release
cross build --target x86_64-pc-windows-gnu --release
```

## Release Process

### Version Numbering

Color-rs follows [Semantic Versioning (SemVer)](https://semver.org/):

- **MAJOR**: Incompatible API changes
- **MINOR**: Backward-compatible functionality additions
- **PATCH**: Backward-compatible bug fixes

#### Version Lifecycle

```
0.15.4 (current) → 0.15.5 (patch) → 0.16.0 (minor) → 1.0.0 (major)
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
- [ ] Test precision formatting: verify 3 decimal places in YAML/TOML output

#### Release Build

- [ ] Create release builds for all target platforms
- [ ] Verify binary functionality on each platform
- [ ] Run performance benchmarks: `cargo bench`
- [ ] Test installation: `cargo install --path .`
- [ ] Verify precision formatting across all output formats

#### Distribution

- [ ] Create Git tag: `git tag v0.15.4`
- [ ] Push tag: `git push origin v0.15.4`
- [ ] Publish to crates.io: `cargo publish`
- [ ] Create GitHub release with binaries
- [ ] Update documentation sites

## Version 0.15.4 Changes

### Precision Formatting System
- **Enhanced formatting**: New `precision_utils.rs` module for consistent floating-point formatting
- **Serde integration**: Custom serialization functions for YAML/TOML output
- **Cross-format consistency**: Unified precision across terminal, file, and serialized outputs

### Build Improvements
- **Test coverage**: 115 unit tests with enhanced precision validation
- **Cross-platform**: Verified builds on Windows, macOS, and Linux
- **Performance**: Optimized formatting functions for minimal overhead

### Dependencies Update
- **Precision handling**: Enhanced with custom formatting utilities
- **Serialization**: Improved YAML/TOML output formatting
- **Testing**: Expanded test coverage for precision requirements

## Development Workflow

### Local Development

```bash
# Setup development environment
git clone https://github.com/username/color-rs.git
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
color-rs --filter "[input,conversion]" red  # Test filtering (v0.15.4+)
```
