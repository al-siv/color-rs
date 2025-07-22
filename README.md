# color-rs

CLI tool and library for color analysis, gradient generation, and color space conversions with LAB/LCH color distance calculations.

## Installation

```bash
cargo install color-rs
```

## Core Functionality

### Color Analysis Command

The primary command for analyzing colors and generating color schemes:

```bash
color-rs color <COLOR> [OPTIONS]
```

**Input formats supported:**
- Named colors: `red`, `blue`, `forestgreen`
- Hex codes: `#FF0000`, `FF0000`
- RGB values: `rgb(255, 0, 0)`
- HSL values: `hsl(0, 100%, 50%)`
- LAB values: `lab(53.24, 80.092, 67.203)`
- RAL codes: `RAL 3020`, `RAL 050 50 78`

**Output format:**
- YAML format (default)
- TOML format (with `--output toml`)
- File export (with `--file filename.yaml`)

### Gradient Generation Command

Generate color gradients with mathematical easing functions:

```bash
color-rs gradient <START_COLOR> <END_COLOR> [OPTIONS]
```

## Current Output Structure

All commands output structured data in YAML format containing:

### Metadata Section
```yaml
metadata:
  program_name: color-rs
  version: '0.14.1'
  author: al-siv <https://github.com/al-siv>
  description: CLI tool and library for color analysis, gradient generation, and color space conversions with LAB/LCH color distance calculations
  generated_at: '2025-07-22T21:52:57.453462200+00:00'
  distance_strategy: Delta E 2000
```

### Input Processing
```yaml
input:
  input_color: red
  base_color: '#FF0000'
```

### Color Conversions
```yaml
conversion:
  hex: '#FF0000'
  rgb: rgb(255, 0, 0)
  hsl: hsl(0.0, 100.00%, 50.00%)
  hsb: hsv(0.0, 100.00%, 100.00%)
  lab: lab(53.24, 80.092, 67.203)
  lch: lch(53.24, 104.552, 40.0)
  cmyk: cmyk(0.00%, 100.00%, 100.00%, 0.00%)
  xyz: xyz(0.412, 0.213, 0.019)
  oklch: oklch(0.628, 0.258, 29.2)
```

### WCAG Accessibility Data
```yaml
contrast:
  wcag21_relative_luminance: 0.21267294883728027
  contrast_vs_white:
    ratio: 3.9973663243505535
    assessment: Low
  contrast_vs_black:
    ratio: 5.253458976745605
    assessment: Medium
  brightness:
    lab_assessment: Medium
    wcag_assessment: Light
```

### Grayscale Variants
```yaml
grayscale:
  lch0_hex: '#7F7F7F'
  lch0: lch(53.24, 0.000, 40.0)
  lch2_hex: '#837E7D'
  lch2: lch(53.24, 2.000, 40.0)
  # ... additional chroma levels
```

### Color Collection Matching

Nearest color matches from three comprehensive databases:

```yaml
color_collections:
  css_colors:
  - name: Red
    hex: '#FF0000'
    lch: lch(53.24, 104.552, 40.0)
    code: red
    distance: 0.0
    wcag21_relative_luminance: 0.21267294883728027
  ral_classic:
  - name: Luminous bright red
    hex: '#F71027'
    lch: lch(52.17, 93.407, 34.0)
    code: RAL 3026
    distance: 4.682887077331543
    wcag21_relative_luminance: 0.20297928154468536
  ral_design:
  - name: Pompeii red
    hex: '#D55845'
    lch: lch(53.56, 59.917, 36.7)
    code: RAL 040 50 70
    distance: 7.0684661865234375
    wcag21_relative_luminance: 0.2155948132276535
```

### Color Schemes

Complementary, split-complementary, triadic, and tetradic color schemes with nearest color matches:

```yaml
color_schemes:
  complementary:
    hex: '#00A2F3'
    hsl: hsl(200.1, 100.00%, 47.63%)
    lch: lch(53.24, 104.552, 220.0)
    css:
      name: Dodger Blue
      hex: '#1E90FF'
      distance: 6.035937786102295
      wcag_relative_luminance: 0.27438798523182434
    ral_classic:
      name: Pastel blue
      hex: '#6C8DAA'
      distance: 8.271005630493164
      wcag_relative_luminance: 0.25138992639889884
    ral_design:
      name: Structural blue
      hex: '#3A9FD3'
      distance: 4.235053062438965
      wcag_relative_luminance: 0.3039598609318176
```

## Command Reference

### Color Analysis Examples

```bash
# Basic color analysis
color-rs color red

# Analyze hex color with HSL schemes
color-rs color "#FF5733" --schemes hsl

# Export analysis to file
color-rs color "rgb(255, 87, 51)" --file analysis.yaml

# TOML format output
color-rs color blue --output toml

# Different color distance strategy
color-rs color "#A1D1E6" --schemes lab
```

### Gradient Generation Examples

```bash
# Basic gradient
color-rs gradient red blue

# Custom easing and file export
color-rs gradient "#FF0000" "#0000FF" --ease-in 0.25 --ease-out 0.75 --file gradient.yaml

# SVG gradient generation
color-rs gradient red blue --svg gradient.svg

# PNG image export
color-rs gradient forestgreen skyblue --png gradient.png --width 1200

# TOML output with custom stops
color-rs gradient "#FF5733" "#00AFF0" --stops 8 --output toml
```

## Technical Specifications

### Color Distance Calculations
- **CIE Delta E 2000**: Perceptually uniform color distance measurement
- **LAB Color Space**: All calculations performed in CIELAB color space
- **LCH Color Space**: Cylindrical representation of LAB for intuitive color relationships

### Color Collections
- **CSS Named Colors**: 140+ standard web colors
- **RAL Classic**: 213 industrial color standards
- **RAL Design System+**: 1825+ extended color palette

### Distance Strategies
- Delta E 76
- Delta E 2000 (default)
- Euclidean LAB
- LCH distance calculations

### Output Formats
- **YAML**: Default structured output format
- **TOML**: Alternative structured format
- **SVG**: Vector gradient files
- **PNG**: Raster image exports

## Library Usage

```toml
[dependencies]
color-rs = "0.14.1"
```

### Basic Library Integration

```rust
use color_rs::{ColorRs, cli::ColorArgs};

fn main() -> color_rs::Result<()> {
    let color_rs = ColorRs::new();
    
    // Color analysis
    let args = ColorArgs {
        color: "red".to_string(),
        schemes: Some("lab".to_string()),
        output: Some("yaml".to_string()),
        file: None,
    };
    
    color_rs.analyze_color(args)?;
    Ok(())
}
```

### Gradient Generation

```rust
use color_rs::{ColorRs, cli::GradientArgs};

fn main() -> color_rs::Result<()> {
    let color_rs = ColorRs::new();
    
    let args = GradientArgs {
        start_color: "#FF0000".to_string(),
        end_color: "#0000FF".to_string(),
        output: Some("yaml".to_string()),
        file: Some("gradient.yaml".to_string()),
        svg: None,
        png: None,
        width: Some(1000),
        ease_in: Some(0.25),
        ease_out: Some(0.75),
        stops: Some(5),
        // ... other fields with defaults
    };
    
    color_rs.generate_gradient(args)?;
    Ok(())
}
```

## Architecture

### Core Components
- **Color Analysis Engine**: LAB/LCH color space calculations
- **Distance Calculation**: Multiple perceptual distance strategies  
- **Color Collection Matching**: Comprehensive color database queries
- **Scheme Generation**: Mathematical color harmony calculations
- **Output Formatting**: Structured data serialization
- **File Export**: Multiple format support

### Design Patterns
- **Strategy Pattern**: Multiple color distance calculation methods
- **Builder Pattern**: Gradient configuration construction
- **Factory Pattern**: Color parser instantiation
- **Template Method**: Color matching algorithm structure
- **Command Pattern**: CLI command processing

## Testing

```bash
# Run all tests
cargo test

cargo run -- color "#FF5733"
```

## Installation

From source:
```bash
git clone https://github.com/al-siv/color-rs.git
cd color-rs
cargo build --release
```

As a Rust library:
```toml
[dependencies]
color-rs = "0.14.1"
```

## Architecture

Modular design with separated concerns:
- **Color operations**: RGB/HSL/LAB/LCH conversions with palette library
- **Format detection**: Automatic input format parsing (hex, rgb(), hsl(), names)
- **Color collections**: CSS colors, RAL Classic (213), RAL Design System+ (1825+)
- **Distance calculations**: CIE Delta E 2000 for perceptual accuracy
- **Output formats**: YAML (default), TOML, structured data

## Error Handling

Comprehensive error reporting for:
- Invalid color formats
- Unsupported conversions
- File operations
- Command validation

## Performance

- Optimized LAB calculations with palette
- Efficient collection lookups
- Memory-efficient operations
- Clean structured output

## Contributing

Standard Rust practices:
- Comprehensive error handling
- Modular architecture
- Unit test coverage
- rustfmt formatting
- clippy compliance

## License

MIT License - see LICENSE file

## Links

- [Repository](https://github.com/al-siv/color-rs)
- [Issues](https://github.com/al-siv/color-rs/issues)

