# color-rs

CLI tool and library for color analysis, gradient generation, and color space conversions with LAB/LCH color distance calculations.

## Installation

### From Source
```bash
cargo install color-rs
```

### Pre-compiled Binaries

Pre-compiled binaries are available for download from [GitHub Releases](https://github.com/al-siv/color-rs/releases):

- **Windows (x86_64)**: `color-rs.exe` - Ready to run
- **Linux (x86_64)**: `color-rs-linux-x86_64` - May need `chmod +x`
- **Linux (ARM64)**: `color-rs-linux-aarch64` - May need `chmod +x`
- **macOS (x86_64 Intel)**: `color-rs-macos-x86_64` - See macOS setup below
- **macOS (ARM64 Apple Silicon)**: `color-rs-macos-aarch64` - See macOS setup below

**Important**: These cross-compiled binaries have not been tested on their target platforms. Please report any issues via GitHub.

### Running on macOS

After downloading the macOS binary, you need to make it executable and may need to bypass macOS security restrictions:

#### Step 1: Make the Binary Executable
```bash
# Navigate to your Downloads folder (or wherever you saved the file)
cd ~/Downloads

# Make the binary executable
chmod +x color-rs-macos-x86_64    # For Intel Macs
# OR
chmod +x color-rs-macos-aarch64   # For Apple Silicon Macs
```

#### Step 2: Handle macOS Security (if needed)
If macOS prevents execution with "cannot be opened because the developer cannot be verified":

**Option A: Use Terminal (Recommended)**
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine color-rs-macos-x86_64
# OR
xattr -d com.apple.quarantine color-rs-macos-aarch64
```

**Option B: System Preferences**
1. Try to run the binary first (it will fail)
2. Go to System Preferences → Security & Privacy → General
3. Click "Allow Anyway" next to the blocked application message
4. Try running again and click "Open" when prompted

#### Step 3: Run the Program
```bash
# Run directly from current directory
./color-rs-macos-x86_64 color red
# OR
./color-rs-macos-aarch64 color red

# Or move to a directory in your PATH for global access
sudo mv color-rs-macos-x86_64 /usr/local/bin/color-rs
# Then you can run from anywhere:
color-rs color red
```

#### Alternative: Run from Terminal
You can also run the program by dragging the binary file into Terminal:
1. Open Terminal
2. Type `chmod +x ` (with a space at the end)
3. Drag the downloaded binary file into the Terminal window
4. Press Enter to make it executable
5. Drag the binary into Terminal again to run it, then add your commands

## Core Functionality

### Color Analysis Command

The primary command for analyzing colors and generating color schemes:

```bash
color-rs color <COLOR> [OPTIONS]
```

### Gradient Generation Command

Generate color gradients with mathematical easing functions:

```bash
color-rs gradient <START_COLOR> <END_COLOR> [OPTIONS]
```

### Hue Analysis Command

Analyze and visualize entire color collections sorted by hue relationships:

```bash
color-rs hue <COLLECTION> [OPTIONS]
```

**Key Features:**
- **Selective Output Control**: Use `--func` to show only specific output blocks or fields
- **Multiple Output Formats**: YAML, TOML, SVG, PNG
- **Comprehensive Color Analysis**: WCAG compliance, color distance calculations, color scheme generation
- **RAL Color System Support**: Both Classic and Design System+ databases
- **Advanced Gradient Generation**: LAB color space with cubic-bezier easing
- **Color Collection Visualization**: Horizontal gradients and vertical palettes with filtering

**Input formats supported:**
- Named colors: `red`, `blue`, `forestgreen`
- Hex codes: `#FF0000`, `FF0000`
- RGB values: `rgb(255, 0, 0)`
- HSL values: `hsl(0, 100%, 50%)`
- LAB values: `lab(53.24, 80.092, 67.203)`
- RAL codes: `RAL 3020`, `RAL 050 50 78`

**Output formats:**
- YAML format (default)
- TOML format (with `--output toml`)
- File export (with `--file filename.yaml`)
- Selective output control (with `--func [filter_expression]`)
- Visual output (SVG/PNG for hue and gradient commands)

## Selective Output Control

The `--func` parameter allows you to control which parts of the analysis output are displayed, helping you focus on specific information or reduce output size.

### Basic Block Filtering
```bash
# Show only specific blocks
color-rs color red --func "[input]"          # Only input information
color-rs color red --func "[conversion]"     # Only color format conversions
color-rs color red --func "[contrast]"       # Only WCAG contrast data
color-rs color red --func "[grayscale]"      # Only grayscale variations
color-rs color red --func "[color_collections]"  # Only color database matches
color-rs color red --func "[color_schemes]"  # Only color harmonies

# Show multiple blocks
color-rs color red --func "[input,conversion,contrast]"
```

### Field-Level Filtering
```bash
# Show specific fields within blocks
color-rs color red --func "[contrast.wcag21_relative_luminance]"
color-rs color red --func "[grayscale.lch0,grayscale.lch0_hex]"
color-rs color red --func "[conversion.hex,conversion.rgb]"

# Mixed block and field selection
color-rs color red --func "[input,contrast.wcag21_relative_luminance]"
```

### Exclusion Filtering
```bash
# Show everything except specific blocks
color-rs color red --func "[all,!color_collections]"
color-rs color red --func "[all,!color_schemes,!grayscale]"

# Exclude specific fields
color-rs color red --func "[contrast,!contrast.brightness]"
```

## Current Output Structure

All commands output structured data in YAML format containing:

### Metadata Section
```yaml
metadata:
  program_name: color-rs
  version: '0.19.0'
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

# Selective output - show only input and conversion data
color-rs color red --func "[input,conversion]"

# Show only specific field from contrast analysis
color-rs color "#FF0000" --func "[contrast.wcag21_relative_luminance]"

# Show all data except color collections
color-rs color blue --func "[all,!color_collections]"

# Show only specific grayscale variants
color-rs color green --func "[grayscale.lch0,grayscale.lch0_hex]"
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

### Hue Analysis Examples

```bash
# Display entire CSS color collection
color-rs hue css

# Filter by hue range (warm colors: 0-60 degrees)
color-rs hue css -H"[0...60]"

# Multiple filters: warm, bright, saturated colors
color-rs hue css -H"[0...60]" -L"[50...80]" -C"[30...70]"

# RAL Classic collection filtered by blue hues
color-rs hue ralc -H"[200...260]"

# Large RAL Design collection with complex filtering
color-rs hue rald -H"[180...240]" -L"[40...70]" -C"[20...60]"

# Visual output: horizontal gradient
color-rs hue css -g -G gradient.svg

# Visual output: vertical palette with custom dimensions
color-rs hue css -p -G palette.svg -w 1200 -z 40

# Export both SVG and PNG
color-rs hue css -p -G palette.svg -P palette.png

# TOML export with filtering
color-rs hue css -H"[90...150]" --output toml --file green-spectrum

# Wraparound hue range (purple to red spectrum)
color-rs hue css -H"[300...30]" -p -G purple-red.svg
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
- LCH distance calculations (default)
- Delta E 2000
- Delta E 76
- Euclidean LAB

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
        distance_method: "lch".to_string(),
        scheme_strategy: "lab".to_string(),
        relative_luminance: None,
        luminance: None,
        output_format: Some(OutputFormat::Yaml),
        output_file: None,
        func_filter: Some("[input,conversion]".to_string()),
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

### Functional Programming Architecture (v0.15.4+)
- **Pure Functions**: Core color operations with immutable inputs and deterministic outputs
- **Function Composition**: Complex operations built from composing simple functions
- **Type-Driven Design**: Leveraging Rust's enum and struct systems for compile-time guarantees
- **Immutable Data**: Preference for immutable data structures and transformations
- **Error Handling**: Consistent use of Result<T, E> and Option<T> throughout

### Pattern Migration Status
- **Strategy Pattern** → **Functional Enum Selection**: Distance algorithms via enum + pattern matching
- **Builder Pattern** → **Immutable Configuration**: Gradient configuration with immutable structs  
- **Factory Pattern** → **Function Composition**: Color parsing via pure function composition
- **Template Method** → **Higher-Order Functions**: Color matching via function composition
- **Command Pattern** → **Function Pipelines**: CLI processing via Result pipelines

*See `docs/PATTERNS_FUNCTIONAL.md` for comprehensive functional programming patterns and `docs/PATTERNS.md` for migration guidance.*

## Testing

```bash
# Run all tests
cargo test

# Test with example colors
cargo run -- color "#FF5733"

# macOS users: use ./color-rs-macos-x86_64 or ./color-rs-macos-aarch64
# after making the binary executable (see Installation section)
```

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

## Documentation

Comprehensive documentation is available in the `docs/` directory:

### User Documentation
- **[UX.md](docs/UX.md)** - User experience reference with complete CLI interface documentation and HCI patterns
- **[EXAMPLES.md](docs/EXAMPLES.md)** - Practical usage examples with functional programming approach
- **[CONFIGURATION.md](docs/CONFIGURATION.md)** - Configuration options and functional design principles

### Developer Documentation  
- **[API.md](docs/API.md)** - Complete library API reference for developers
- **[MODULES.md](docs/MODULES.md)** - Detailed module APIs and functional programming interfaces
- **[ALGORITHMS.md](docs/ALGORITHMS.md)** - Mathematical foundations and algorithm implementations
- **[ARCHITECTURE.md](docs/ARCHITECTURE.md)** - System architecture with functional programming paradigm

### Technical References
- **[PATTERNS_FUNCTIONAL.md](docs/PATTERNS_FUNCTIONAL.md)** - PRIMARY functional programming patterns catalog
- **[PATTERNS.md](docs/PATTERNS.md)** - Migration guide from deprecated OOP patterns to functional alternatives
- **[TYPES.md](docs/TYPES.md)** - Type system reference and functional type design
- **[BUILD_RELEASE.md](docs/BUILD_RELEASE.md)** - Build system and release process documentation
- **[TESTING.md](docs/TESTING.md)** - Testing strategy with functional programming principles
- **[FEATURE_CATALOG.md](docs/FEATURE_CATALOG.md)** - Comprehensive feature catalog

All documentation follows **functional programming principles** and reflects the architectural migration from object-oriented patterns to modern functional approaches in Rust.

## License

MIT License - see LICENSE file

## Links

- [Repository](https://github.com/al-siv/color-rs)
- [Issues](https://github.com/al-siv/color-rs/issues)

