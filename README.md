# color-rs

A professional CLI tool and Rust library for color gradient calculations using perceptually uniform LAB color space with CSS cubic-bezier easing functions.

## Features

- **Simplified Interface**: Direct color arguments without flags - just `gradient red blue`
- **Universal Color Parsing**: Supports HEX, RGB, HSL, and 148+ named colors across all commands
- **RAL Color System**: Complete support for RAL Classic (213 colors) and RAL Design System+ (1825 colors) with precise color matching
- **Perceptually Accurate Processing**: LAB color space for smooth gradients and ImprovedCiede2000 Delta E for precise color distance calculations
- **WCAG Compliance**: Official WCAG 2.1 implementation for relative luminance and contrast ratio calculations using palette library
- **Comprehensive Color Analysis**: Detailed output with RGB, HEX, HSL, LAB, XYZ, OKLCH, WCAG luminance, and contrast ratios
- **Modular Architecture**: Clean separation of concerns with dedicated modules for color utilities, formatting, and parsing
- **Library & CLI**: Use as a command-line tool or integrate as a Rust library
- **Cargo-Style Output**: Professional terminal formatting matching Rust toolchain aesthetics
- **CSS Cubic-Bezier Timing**: Professional easing functions matching web standards
- **Intelligent Stop Placement**: Automatically places gradient stops where colors change most rapidly
- **Multiple Output Formats**: 
    - Beautiful terminal tables with color information and right-aligned numeric columns
    - SVG gradients with optional legends
    - High-quality PNG exports
- **Proportional Design**: All dimensions scale with width (1:5 aspect ratio)
- **Integer Percentages**: CSS-friendly percentage values for practical use
- **Rich Color Information**: RGB, HSL, LAB, XYZ, OKLCH values with color name recognition
- **Type Safety**: Custom error types and comprehensive error handling
- **Well Tested**: Comprehensive unit test suite with 37+ tests

## Library Usage

Color-rs can be used as a Rust library in your projects:

```toml
[dependencies]
color-rs = "0.10.0"
```

### Basic Library Usage

```rust
use color_rs::{ColorRs, cli::GradientArgs};

fn main() -> color_rs::Result<()> {
    let color_rs = ColorRs::new();
    
    // Generate gradient with named colors
    let args = GradientArgs {
        start_color: "red".to_string(),           // Named color
        end_color: "blue".to_string(),           // Named color
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
    
    // Analyze a color with WCAG compliance data
    let analysis = color_rs.color_match("rgb(255, 87, 51)")?;
    println!("{}", analysis);
    
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

```bash
# Using HEX colors - simplified syntax
color-rs gradient FF0000 0000FF

# Using named colors
color-rs gradient red blue

# Using RGB/HSL colors
color-rs gradient "rgb(255,0,0)" "hsl(240,100%,50%)"
```

### Custom Easing

```bash
color-rs gradient FF6B35 7209B7 --ease-in 0.25 --ease-out 0.75
```

### Generate Images

```bash
# SVG with legend
color-rs gradient FF0000 0000FF --svg --svg-name my-gradient.svg

# PNG without legend
color-rs gradient FF0000 0000FF --png --no-legend --png-name clean-gradient.png

# Both formats with custom size
color-rs gradient FF0000 0000FF --svg --png --width 1600
```

### Intelligent Stop Placement

```bash
# 8 intelligently placed stops
color-rs gradient FF0000 0000FF --grad-stops 8 --ease-in 0.9 --ease-out 0.1

# 10 equally spaced stops
color-rs gradient FF0000 0000FF --grad-stops-simple 10
```

### Partial Gradients

```bash
color-rs gradient FF0000 0000FF --start-position 20 --end-position 80
```

### Color Matching and Analysis

```bash
# Color analysis with WCAG compliance data
color-rs color-match "#FF5733"
color-rs color-match "rgb(255, 87, 51)"
color-rs color-match "red"
color-rs color-match "hsl(240, 100%, 50%)"
```

## Installation

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
color-rs = "0.8.3"
```

### As a CLI Tool

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

### Universal Color Format Support

```bash
# Use any color format for gradients - now with simplified syntax
color-rs gradient red blue
color-rs gradient "#FF0000" "rgb(0, 0, 255)"
color-rs gradient "hsl(0, 100%, 50%)" "hsl(240, 100%, 50%)"
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

### Color-Match Enhanced Output with WCAG Compliance
```
Color Analysis for: #FF5733
──────────────────────────────────────────────────
Input: #FF5733
Name: Tomato

Format Conversions:
• RGB:    rgb(255, 87, 51)
• Hex:    #ff5733
• HSL:    hsl(11, 100.0%, 60.0%)
• LAB:    lab(60.18, 62.06, 54.34)
• XYZ:    xyz(0.453, 0.283, 0.062)
• OKLCH:  oklch(0.680, 0.210, 33.7°)

Additional Information:
• Grayscale: rgb(153, 153, 153) #999999 (LAB L* = 60.2)
• WCAG Relative Luminance: 0.283
• Contrast vs White: 3.15:1
• Contrast vs Black: 6.66:1
• Brightness: Light
```

## Command Line Options

### Gradient Command
```
color-rs gradient [OPTIONS] <START_COLOR> <END_COLOR>

ARGUMENTS:
    <START_COLOR>    Starting color (HEX, RGB, HSL, or named color, e.g., #FF0000, rgb(255,0,0), red)
    <END_COLOR>      Ending color (HEX, RGB, HSL, or named color, e.g., #0000FF, rgb(0,0,255), blue)

OPTIONS:
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

### Color-Match Command
```
color-rs color-match <COLOR>

ARGUMENTS:
    <COLOR>    Input color value (any format: hex, rgb(), rgba(), hsl(), hsla(), or color name)

The color-match command automatically detects the input format and provides comprehensive color analysis with all color format conversions, WCAG-compliant relative luminance calculations, contrast ratios, and accessibility information.

#### RAL Color System Support
The color-match command now includes comprehensive RAL color system support with:
- **RAL Classic**: 213 standardized colors (e.g., "RAL 1000", "RAL1000")
- **RAL Design System+**: 1825 colors in hue/lightness/chromaticity format (e.g., "H010L20C10")
- **Name-based lookup**: Search by RAL color names (e.g., "signal yellow", "traffic red")
- **Closest matches**: Shows 2 closest colors from each classification separately
- **Complete Analysis**: RAL colors receive the same comprehensive analysis as all other color inputs

Examples:
```bash
color-rs color-match "RAL 1000"          # RAL Classic with space
color-rs color-match "RAL1000"           # RAL Classic without space  
color-rs color-match "H010L20C10"        # RAL Design System+
color-rs color-match "signal yellow"     # RAL name search
```

All color inputs (including RAL colors) receive comprehensive analysis including:
- **Input Format Auto-Detection**: Supports hex (#FF5733), RGB/RGBA (rgb(255,87,51)), HSL/HSLA (hsl(11,100%,60%)), and named colors (red, blue, etc.)
- **Comprehensive Conversion**: Outputs RGB, Hex, HSL, LAB, XYZ, and OKLCH formats
- **Color Name Recognition**: Uses a database of 148+ named colors to find the closest match
- **Grayscale Conversion**: Calculates grayscale equivalent using LAB L* component for perceptually accurate results
- **WCAG Compliance**: Provides proper relative luminance with gamma correction (not simple weighted averages)
- **Contrast Analysis**: Shows contrast ratios against white and black backgrounds
- **Accessibility Testing**: Helps determine if colors meet WCAG AA/AAA contrast requirements

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
# Generate CSS-ready gradient with simplified syntax
color-rs gradient "FF6B35" "7209B7" --grad-stops 5 --ease-in 0.25 --ease-out 0.75
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
color-rs gradient "FF6B35" "7209B7" --svg --png --width 3000 --no-legend
```

### Color Analysis for Accessibility
```bash
# Check WCAG compliance
color-rs color-match "#FF5733"
# Output includes contrast ratios: 3.15:1 vs white, 6.66:1 vs black
# Helps determine if color meets WCAG AA (4.5:1) requirements
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

### v0.8.4 - Color Accuracy Improvements (2025-07-15)
- **ImprovedCiede2000 Delta E**: Replaces simple Euclidean distance with perceptually uniform color difference calculations. Red vs Blue ΔE ≈ 23 (vs ≈ 175 with old method)
- **Official WCAG 2.1 Implementation**: Uses palette's `Wcag21RelativeContrast` for standards-compliant accessibility calculations
- **Professional Color Interpolation**: Leverages palette's `Mix` trait for accurate color blending
- **Dual HSL Conversion Paths**: Offers both direct HSL→RGB and HSL→XYZ→LAB→RGB conversion with typically <1 RGB unit difference

These improvements ensure color calculations match professional color management standards.

### Architecture
- **Unified Color Collection System**: Advanced architecture for managing multiple color standards
  - **Extensible Design**: Support for CSS Named Colors, RAL Classic, RAL Design System+, and future collections (Pantone, etc.)
  - **Advanced Filtering**: Group-based filtering for RAL collections (by color families, lightness, chromaticity)
  - **Multiple Search Methods**: Closest match, exact name/code, luminance-based, and pattern-based searching
  - **Perceptually Accurate Matching**: LAB color space for consistent color distance calculations across all collections
  - **Library-First Design**: Clean APIs suitable for integration into other Rust projects

### Advanced Color Collection System

```rust
use color_rs::color_parser::{
    UnifiedColorManager, UniversalColor, SearchFilter
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create unified manager with all color collections
    let manager = UnifiedColorManager::new();
    
    // Find closest colors across all collections (CSS, RAL Classic, RAL Design System+)
    let red_rgb = [255, 0, 0];
    let results = manager.find_closest_across_all(red_rgb, 2);
    
    for (collection_name, matches) in results {
        println!("{} Collection:", collection_name);
        for color_match in matches {
            println!("  {} - Distance: {:.2}", 
                color_match.entry.metadata.name,
                color_match.distance
            );
        }
    }
    
    // Advanced RAL filtering by groups
    let ral_red_groups = vec!["RAL 3000".to_string()]; // Red group
    let ral_reds = manager.find_ral_classic_in_groups(red_rgb, &ral_red_groups, 3);
    
    // Filter RAL Design System+ by hue
    let red_hue_groups = vec!["Red".to_string()];
    let design_reds = manager.find_ral_design_in_hue_groups(red_rgb, &red_hue_groups, 3);
    
    // Search by exact codes
    if let Some((collection, entry)) = manager.find_by_code("RAL 1000") {
        println!("Found {} in {}", entry.metadata.name, collection);
    }
    
    // Advanced filtering with SearchFilter
    let filter = SearchFilter {
        luminance_range: Some([0.3, 0.8]), // Medium to high luminance
        groups: Some(vec!["RAL 3000".to_string()]), // Red group only
        ..Default::default()
    };
    
    let filtered_results = manager.search_with_filter([200, 50, 50], &filter, 5);
    
    Ok(())
}
```

