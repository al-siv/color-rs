# color-rs

A professional CLI tool and Rust library for color gradient calculations using perceptually uniform LAB color space with CSS cubic-bezier easing functions.

## Features

- **Modular Architecture**: Clean separation of CLI and library functionality
- **Library & CLI**: Use as a command-line tool or integrate as a Rust library
- **Cargo-Style Output**: Professional terminal formatting matching Rust toolchain aesthetics
- **Perceptually Uniform Gradients**: Uses LAB color space for visually smooth color transitions
- **CSS Cubic-Bezier Timing**: Professional easing functions matching web standards
- **Intelligent Stop Placement**: Automatically places gradient stops where colors change most rapidly
- **Multiple Output Formats**: 
    - Beautiful terminal tables with color information and right-aligned numeric columns
    - SVG gradients with optional legends
    - High-quality PNG exports
- **Proportional Design**: All dimensions scale with width (1:5 aspect ratio)
- **Integer Percentages**: CSS-friendly percentage values for practical use
- **Rich Color Information**: RGB, HSL, and LAB values for both start and end colors
- **Type Safety**: Custom error types and comprehensive error handling
- **Well Tested**: Comprehensive unit test suite with 16+ tests

## Library Usage

Color-rs can be used as a Rust library in your projects:

```toml
[dependencies]
color-rs = "0.8.0"
```

### Basic Library Usage

```rust
use color_rs::{ColorRs, cli::GradientArgs};

fn main() -> color_rs::Result<()> {
    let color_rs = ColorRs::new();
    
    let args = GradientArgs {
        start_color: "FF0000".to_string(),
        end_color: "0000FF".to_string(),
        start_position: 0,
        end_position: 100,
        ease_in: 0.25,
        ease_out: 0.75,
        svg: true,
        png: false,
        no_legend: false,
        width: 1000,
        svg_name: "my-gradient.svg".to_string(),
        png_name: "gradient.png".to_string(),
        grad_step: 5,
        grad_stops: None,
        grad_stops_simple: None,
    };
    
    color_rs.generate_gradient(args)?;
    Ok(())
}
```

### Using Individual Modules

```rust
use color_rs::{
    color::ColorProcessor,
    gradient::GradientCalculator,
    image::ImageGenerator,
};

// Parse colors
let start_lab = ColorProcessor::parse_hex_color("#FF0000")?;
let end_lab = ColorProcessor::parse_hex_color("#0000FF")?;

// Generate gradient with cubic-bezier easing
let smooth_t = GradientCalculator::cubic_bezier_ease(0.5, 0.25, 0.75);
let mid_color = ColorProcessor::interpolate_lab(start_lab, end_lab, smooth_t);
let hex_color = ColorProcessor::lab_to_hex(mid_color);

println!("Mid-point color: {}", hex_color);
```

## CLI Usage

#### Windows Users (Recommended)

Download the pre-compiled executable from the [latest release](https://github.com/al-siv/color-rs/releases/latest):

1. Download `color-rs.exe` from the release assets
2. Place it in a folder that's in your PATH or use it directly
3. Run `color-rs.exe gradient --help` to get started

#### From Source

```bash
git clone https://github.com/al-siv/color-rs.git
cd color-rs
cargo build --release
```

The binary will be available at `target/release/color-rs` (or `target/release/color-rs.exe` on Windows).

#### Requirements

- For Windows users: No additional requirements with the pre-compiled executable
- For building from source: Rust 1.70+ and Cargo

### Basic Gradient

## Installation

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
color-rs = "0.8.0"
```

### As a CLI Tool

```bash
color-rs gradient --start-color FF0000 --end-color 0000FF
```

### Custom Easing

```bash
color-rs gradient --start-color FF6B35 --end-color 7209B7 \
        --ease-in 0.25 --ease-out 0.75
```

### Generate Images

```bash
# SVG with legend
color-rs gradient --start-color FF0000 --end-color 0000FF \
        --svg --svg-name my-gradient.svg

# PNG without legend
color-rs gradient --start-color FF0000 --end-color 0000FF \
        --png --no-legend --png-name clean-gradient.png

# Both formats with custom size
color-rs gradient --start-color FF0000 --end-color 0000FF \
        --svg --png --width 1600
```

### Intelligent Stop Placement

```bash
# 8 intelligently placed stops
color-rs gradient --start-color FF0000 --end-color 0000FF \
        --grad-stops 8 --ease-in 0.9 --ease-out 0.1

# 10 equally spaced stops
color-rs gradient --start-color FF0000 --end-color 0000FF \
        --grad-stops-simple 10
```

### Partial Gradients

```bash
color-rs gradient --start-color FF0000 --end-color 0000FF \
        --start-position 20 --end-position 80
```

## Output Examples

### Application Information (Cargo-Style)
```
 Application: Color-rs v0.7.2
             About: A CLI tool for color gradient calculations using LAB color space with cubic-bezier easing functions
            Author: https://github.com/al-siv
```

### Color Information Table
```
COLOR INFORMATION:
╭─────────────┬─────────┬────────────────┬────────────────────────────┬─────────────────────────╮
│ Color       │ Hex     │ RGB            │ HSL                        │ Lab                     │
├─────────────┼─────────┼────────────────┼────────────────────────────┼─────────────────────────┤
│ Start Color │ #FF0000 │ RGB(255, 0, 0) │ HSL(0.0°, 100.0%, 50.0%)   │ Lab(53.2, 80.1, 67.2)   │
│ End Color   │ #0000FF │ RGB(0, 0, 255) │ HSL(240.0°, 100.0%, 50.0%) │ Lab(32.3, 79.2, -107.9) │
╰─────────────┴─────────┴────────────────┴────────────────────────────┴─────────────────────────╯
```

### Gradient Values Table
```
GRADIENT VALUES:
╭──────────┬─────────┬──────────────────╮
│ Position │ Hex     │ RGB              │
├──────────┼─────────┼──────────────────┤
│       0% │ #FF0000 │ rgb(255, 0, 0)   │
│      24% │ #F0003D │ rgb(240, 0, 61)  │
│      35% │ #E2005C │ rgb(226, 0, 92)  │
│      45% │ #D30079 │ rgb(211, 0, 121) │
│      55% │ #BF0098 │ rgb(191, 0, 152) │
│      65% │ #A700B6 │ rgb(167, 0, 182) │
│      76% │ #8400D5 │ rgb(132, 0, 213) │
│     100% │ #0000FF │ rgb(0, 0, 255)   │
╰──────────┴─────────┴──────────────────╯
```

## Command Line Options

```
color-rs gradient [OPTIONS] --start-color <HEX> --end-color <HEX>

OPTIONS:
        --start-color <HEX>              Starting color (e.g., #FF0000 or FF0000)
        --end-color <HEX>                Ending color (e.g., #0000FF or 0000FF)
        --start-position <PERCENT>       Starting position [default: 0]
        --end-position <PERCENT>         Ending position [default: 100]
        --ease-in <EASE_IN>              Ease-in control point [default: 0.65]
        --ease-out <EASE_OUT>            Ease-out control point [default: 0.35]
        --svg                            Generate SVG image
        --png                            Generate PNG image
        --no-legend                      Disable legend (only with --svg or --png)
        --width <WIDTH>                  Image width in pixels [default: 1000]
        --svg-name <SVG_NAME>            SVG filename [default: gradient.svg]
        --png-name <PNG_NAME>            PNG filename [default: gradient.png]
        --grad-step <GRAD_STEP>          Output every X percent [default: 5]
        --grad-stops <GRAD_STOPS>        Number of intelligent stops
        --grad-stops-simple <GRAD_STOPS> Number of equal stops
```

## Color Spaces

### LAB Color Space
- **Perceptually uniform**: Equal numerical differences appear as equal visual differences
- **Device independent**: Consistent across different displays and printers
- **Wide gamut**: Encompasses all colors visible to the human eye

### RGB → LAB → RGB Pipeline
1. Input colors parsed as sRGB hex values
2. Converted to LAB for perceptually uniform interpolation
3. Converted back to sRGB for output

## Cubic-Bezier Easing

The tool uses industry-standard cubic-bezier curves matching CSS timing functions:

- `cubic-bezier(ease-in, 0, ease-out, 1)`
- **Linear**: `--ease-in 0 --ease-out 1`
- **Ease**: `--ease-in 0.25 --ease-out 1` (default-ish)
- **Ease-in**: `--ease-in 0.42 --ease-out 1`
- **Ease-out**: `--ease-in 0 --ease-out 0.58`
- **Ease-in-out**: `--ease-in 0.42 --ease-out 0.58`

## Intelligent Stop Placement

The `--grad-stops` option uses curve derivatives to automatically place gradient stops where colors change most rapidly:

- Analyzes the cubic-bezier curve's rate of change
- Places more stops in areas of rapid color transition
- Results in smoother gradients with fewer visible bands
- Always uses integer percentages for CSS compatibility

## Image Generation

### SVG Features
- Scalable vector format
- Optional typography-rich legends
- Professional font stacks
- Text automatically converted to paths for PNG export

### PNG Features
- High-quality rasterization via resvg
- System font loading for text rendering
- Consistent output across platforms
- Optional legend control

### Proportional Design
- Gradient height = width × 0.2 (1:5 aspect ratio)
- Legend height = gradient height × 0.2 (when enabled)
- Font size = legend height × 0.6
- All dimensions scale proportionally

## Technical Details

### Architecture

Color-rs follows a modular architecture with clear separation of concerns:

- **`lib.rs`**: Main library entry point with public API
- **`cli.rs`**: Command-line interface and argument parsing
- **`color.rs`**: Color operations, conversions, and LAB color space handling
- **`gradient.rs`**: Gradient calculations and cubic-bezier easing functions
- **`image.rs`**: SVG and PNG image generation
- **`error.rs`**: Custom error types and error handling
- **`config.rs`**: Configuration constants and default values
- **`utils.rs`**: Utility functions and validation
- **`main.rs`**: CLI entry point (minimal, delegates to library)

### Error Handling

Custom error types provide clear error messages and proper error propagation:

```rust
pub enum ColorError {
    InvalidColor(String),
    InvalidGradient(String),
    ImageError(String),
    IoError(std::io::Error),
    SvgError(String),
    InvalidArguments(String),
    General(String),
}
```

### Dependencies
- **kurbo**: Industry-standard 2D curve operations
- **palette**: Professional color space conversions
- **usvg/resvg**: SVG parsing and PNG rendering
- **clap**: Modern CLI argument parsing
- **tabled**: Beautiful terminal table formatting
- **colored**: Rich terminal output formatting

### Performance
- Optimized curve calculations with binary search
- High-resolution sampling (10,000 points) for intelligent stops
- Efficient LAB color space interpolation
- Minimal memory allocation

### Development

```bash
# Clone the repository
git clone https://github.com/al-siv/color-rs.git
cd color-rs

# Run tests
cargo test

# Run with debug output
cargo run -- gradient --start-color FF0000 --end-color 0000FF

# Build optimized release
cargo build --release

# Run benchmarks (if available)
cargo bench

# Generate documentation
cargo doc --open

# Check code formatting
cargo fmt --check

# Run linter
cargo clippy
```

### Project Structure

```
src/
├── lib.rs          # Library entry point
├── main.rs         # CLI entry point
├── cli.rs          # CLI argument parsing
├── color.rs        # Color operations
├── gradient.rs     # Gradient calculations
├── image.rs        # SVG/PNG generation
├── error.rs        # Error handling
├── config.rs       # Configuration
└── utils.rs        # Utilities
```

### Testing

The project includes comprehensive unit tests:

```bash
cargo test
```

Tests cover:
- Color parsing and conversion
- Gradient calculations
- Cubic-bezier easing functions
- Image generation validation
- Error handling
- Utility functions

## Examples

### Web Development
```bash
# Generate CSS-ready gradient
color-rs gradient --start-color "FF6B35" --end-color "7209B7" \
        --grad-stops 5 --ease-in 0.25 --ease-out 0.75
```

Output for CSS:
```css
background: linear-gradient(
        to right,
        rgb(255, 107, 53) 0%,
        rgb(226, 78, 99) 35%,
        rgb(189, 53, 132) 55%,
        rgb(151, 28, 161) 75%,
        rgb(114, 9, 183) 100%
);
```

### Design Assets
```bash
# High-resolution design asset
color-rs gradient --start-color "FF6B35" --end-color "7209B7" \
        --svg --png --width 3000 --no-legend
```

### Color Analysis
```bash
# Analyze color relationships
color-rs gradient --start-color "FF6B35" --end-color "7209B7" \
        --grad-step 10
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- **kurbo**: Rust graphics ecosystem for curve mathematics
- **palette**: Comprehensive color science library
- **usvg/resvg**: SVG processing pipeline
- **tabled**: Professional terminal table formatting
- **LAB color space**: Perceptually uniform color representation

## Links

- [Repository](https://github.com/al-siv/color-rs)
- [Issues](https://github.com/al-siv/color-rs/issues)
- [CSS cubic-bezier reference](https://developer.mozilla.org/en-US/docs/Web/CSS/easing-function)
- [LAB color space](https://en.wikipedia.org/wiki/CIELAB_color_space)

---

**color-rs** - Professional color gradients for modern workflows

## Changelog

### v0.8.0 - Major Refactoring (2025-07-15)
- **Breaking Changes**: Restructured codebase into modular library architecture
- **Library API**: Color-rs can now be used as a Rust library with clean public API
- **Enhanced Error Handling**: Custom error types replace generic anyhow errors
- **Comprehensive Testing**: Added 16+ unit tests covering all modules
- **Improved Documentation**: Inline documentation and better code organization
- **Type Safety**: Better type safety with custom Result type and validation
- **Modular Design**: Separated CLI from library functionality for better reusability

### v0.7.2 - First Full Release (2025-07-14)
- Professional Table Formatting: Cargo-style output with right-aligned numeric columns
- Enhanced Visual Design: Improved terminal output with beautiful ASCII tables
- Integer Percentages: CSS-compatible integer percentage calculations
- Solid PNG Backgrounds: Fixed PNG rendering with proper solid backgrounds
- Comprehensive Documentation: Complete README with examples and usage guides
- Intelligent Stop Placement: Advanced derivative-based gradient stop calculation
- CSS Integration: Ready-to-use output for web development workflows
- Production Ready: Stable API and comprehensive error handling

### Features Summary
- LAB color space for perceptually uniform gradients
- CSS cubic-bezier timing functions
- SVG and PNG export capabilities  
- Multiple output formats (step-based, intelligent, equal spacing)
- Professional terminal interface matching Rust toolchain aesthetics
- Windows executable distribution for easy installation

