# Color-rs CLI Reference v0.14.1

Command-line reference for color-rs: color analysis, gradient generation, and color space conversions with LAB/LCH color distance calculations.

## Global Commands

```bash
color-rs <COMMAND> [OPTIONS]
```

**Commands:**
- `gradient` - Generate color gradients using LAB color space with cubic-bezier timing
- `color` - Analyze and convert colors between different color spaces
- `help` - Print help information

**Global Options:**
- `-h, --help` - Print help
- `-V, --version` - Print version

## Color Command

Analyze and convert colors between different color spaces. Outputs comprehensive YAML/TOML data with metadata, conversions, contrast analysis, color collections, and color schemes.

### Syntax
```bash
color-rs color [OPTIONS] <COLOR>
```

### Arguments
- `<COLOR>` - Input color value (any format: hex, rgb(), rgba(), hsl(), hsla(), or color name)

### Options
- `--distance-method <METHOD>` - Distance calculation method [default: delta-e-2000]
  - `delta-e-76` - CIE Delta E 1976 (faster)
  - `delta-e-2000` - CIE Delta E 2000 (perceptually accurate)
  - `euclidean-lab` - Euclidean distance in LAB space
  - `lch` - LCH-based calculation

- `--schemes <STRATEGY>` - Color scheme strategy [default: lab]
  - `hsl` - HSL color space schemes
  - `lab` - LAB color space schemes (perceptually uniform)

- `-r, --relative-luminance <LUM_VALUE>` - Replace color with specified WCAG relative luminance (0.0-1.0)

- `-l, --luminance <LUM_VALUE>` - Replace color with specified Lab luminance value

- `-o, --output <OUTPUT_FORMAT>` - Output format [default: yaml]
  - `yaml` - YAML format output
  - `toml` - TOML format output

- `-f, --file <FILENAME>` - Output filename (extension added automatically based on format)

### Output Structure
The color command outputs structured data containing:
- **metadata** - Program version, timestamp, analysis info
- **input** - Original input value and detected format
- **conversion** - All color space conversions (RGB, HSL, HEX, LAB, LCH, XYZ)
- **contrast** - WCAG luminance, contrast ratios vs white/black
- **grayscale** - Perceptually accurate grayscale conversion using LAB L*
- **color_collections** - Closest matches from CSS colors, RAL Classic, RAL Design System+
- **color_schemes** - Generated color harmonies (complementary, triadic, tetradic)

### Examples
```bash
# Basic color analysis
color-rs color "#FF5733"
color-rs color "rgb(255, 87, 51)"
color-rs color "red"

# RAL color system
color-rs color "RAL 3020"
color-rs color "RAL 010 40 30"

# Different distance methods
color-rs color "#FF5733" --distance-method delta-e-76
color-rs color "#FF5733" --distance-method euclidean-lab

# HSL-based color schemes
color-rs color "blue" --schemes hsl

# Output to file
color-rs color "#FF5733" --output toml --file analysis
# Creates: analysis.toml

# Luminance replacement
color-rs color "#FF5733" --relative-luminance 0.5
color-rs color "blue" --luminance 60
```

## Gradient Command

Generate color gradients using LAB color space with cubic-bezier timing functions. Outputs structured data with gradient stops and metadata.

### Syntax
```bash
color-rs gradient [OPTIONS] <START_COLOR> <END_COLOR>
```

### Arguments
- `<START_COLOR>` - Starting color (HEX, RGB, HSL, or named color)
- `<END_COLOR>` - Ending color (HEX, RGB, HSL, or named color)

### Position Options
- `-s, --start-position <PERCENT>` - Starting position as percentage [default: 0]
- `-e, --end-position <PERCENT>` - Ending position as percentage [default: 100]

### Easing Options
- `--ease-in <EASE_IN>` - Ease-in control point for cubic-bezier (0.0-1.0) [default: 0.65]
- `--ease-out <EASE_OUT>` - Ease-out control point for cubic-bezier (0.0-1.0) [default: 0.35]

### Gradient Control
- `-t, --step <STEP>` - Output gradient values every X percent
- `-g, --stops <STOPS>` - Number of gradient stops using curve derivatives [default: 5]
- `--stops-simple` - Use equally spaced gradient stops instead of intelligent placement

### Image Generation
- `--svg` - Generate SVG image of the gradient
- `--png` - Generate PNG image of the gradient
- `--no-legend` - Disable legend/caption on gradient images (only valid with --svg or --png)
- `--width <WIDTH>` - Width of the image in pixels [default: 1000]
- `-v, --svg-name <SVG_NAME>` - Output filename for SVG image [default: gradient.svg]
- `-p, --png-name <PNG_NAME>` - Output filename for PNG image [default: gradient.png]

### Output Options
- `-o, --output <OUTPUT_FORMAT>` - Output format [default: yaml]
  - `yaml` - YAML format output
  - `toml` - TOML format output
- `-f, --file <FILENAME>` - Output filename (extension added automatically based on format)

### Output Structure
The gradient command outputs structured data containing:
- **metadata** - Program version, timestamp, gradient parameters
- **start_color** - Complete analysis of starting color
- **end_color** - Complete analysis of ending color
- **gradient_stops** - Array of gradient stops with position, colors, and luminance
- **summary** - Contrast ratios and overall gradient statistics

### Examples
```bash
# Basic gradients
color-rs gradient red blue
color-rs gradient "#FF0000" "#0000FF"
color-rs gradient "rgb(255,0,0)" "hsl(240,100%,50%)"

# Partial gradients
color-rs gradient red blue --start-position 20 --end-position 80

# Custom easing (ease-in-out)
color-rs gradient red blue --ease-in 0.42 --ease-out 0.58

# Different stop distributions
color-rs gradient red blue --step 10          # Every 10%
color-rs gradient red blue --stops 8          # 8 intelligent stops
color-rs gradient red blue --stops 6 --stops-simple  # 6 equal stops

# Image generation
color-rs gradient red blue --svg
color-rs gradient red blue --png --width 1600
color-rs gradient red blue --svg --png --no-legend

# Custom filenames
color-rs gradient red blue --svg --svg-name custom-gradient.svg
color-rs gradient red blue --output toml --file my-gradient
# Creates: my-gradient.toml

# RAL colors
color-rs gradient "RAL 3020" "RAL 5005"
color-rs gradient "RAL 010 40 30" "RAL 270 30 40"
```

## Color Format Support

Both commands support multiple input formats:

### Standard Formats
- **HEX**: `#FF0000`, `#ff0000`, `FF0000`
- **RGB**: `rgb(255,0,0)`, `rgba(255,0,0,1.0)`
- **HSL**: `hsl(0,100%,50%)`, `hsla(0,100%,50%,1.0)`
- **Named Colors**: `red`, `blue`, `forestgreen`, etc.

### RAL Color System
- **RAL Classic**: `RAL 3020`, `RAL1000` (213 colors)
- **RAL Design System+**: `RAL 010 40 30` (1825+ colors)
- **RAL Names**: `traffic red`, `signal yellow`, etc.

## Output Formats

### YAML (Default)
```yaml
metadata:
  program: "color-rs"
  version: "0.14.1"
  timestamp: "2024-01-15T10:30:45Z"
  
input:
  value: "#FF5733"
  format: "hex"
  
conversion:
  rgb: [255, 87, 51]
  hsl: [11.0, 100.0, 60.0]
  # ... additional conversions
```

### TOML
```toml
[metadata]
program = "color-rs"
version = "0.14.1"
timestamp = "2024-01-15T10:30:45Z"

[input]
value = "#FF5733"
format = "hex"

[conversion]
rgb = [255, 87, 51]
hsl = [11.0, 100.0, 60.0]
# ... additional conversions
```

## Technical Details

### Color Spaces
- **LAB**: Perceptually uniform color space for accurate calculations
- **LCH**: Cylindrical representation of LAB (Lightness, Chroma, Hue)
- **sRGB**: Standard RGB color space for display
- **HSL**: Hue, Saturation, Lightness for intuitive color manipulation

### Distance Methods
- **Delta E 2000**: Most perceptually accurate (recommended)
- **Delta E 76**: Faster computation, less accurate
- **Euclidean LAB**: Simple geometric distance in LAB space
- **LCH**: Distance in cylindrical LAB coordinates

### Cubic-Bezier Easing
The gradient command uses cubic-bezier timing functions:
- `cubic-bezier(ease-in, 0, ease-out, 1)`
- Standard CSS timing function compatibility
- Intelligent stop placement based on curve derivatives

### Performance
- Optimized LAB color space calculations using palette library
- Efficient color collection lookups with indexed data structures
- Memory-efficient gradient generation

```bash
# RAL Classic colors
color-rs gradient "RAL 1000" "RAL 5005"
```

```bash
# RAL Design System+ colors  
color-rs gradient "H010L20C10" "H040L50C70" --svg --width 800
```

```bash
# RAL named colors
color-rs gradient "signal yellow" "pure blue" --grad-stops 6
```

---

## Color Command

Analyze colors with comprehensive format conversion and WCAG compliance data.

### Basic Syntax

```bash
color-rs color <COLOR> [OPTIONS]
```

### Positional Arguments

- `<COLOR>` - Input color value (required)
  - **Formats**: HEX, RGB, RGBA, HSL, HSLA, LAB, named colors, RAL colors
  - **Examples**: #FF5722, 457FB3, rgb(255,87,34), hsl(14,100%,57%), lab(60.18, 62.06, 54.34), "deep orange", "RAL 3020", "luminous orange"

### Options

- `--distance-method <METHOD>` - Distance calculation method for color matching (default: delta-e-2000)
  - **Available methods**:
    - `delta-e-76` - Fast CIE Delta E 1976 formula
    - `delta-e-2000` - Industry-standard CIE Delta E 2000 (perceptually accurate)
    - `euclidean-lab` - Simple Euclidean distance in LAB space

### Usage Examples

#### Basic Color Analysis

```bash
# Analyze a HEX color
color-rs color "#FF5722"
```

Output includes comprehensive color format conversions (RGB, HEX, HSL, HSB, CMYK, LAB, XYZ, OKLCH), WCAG compliance data, closest matches from CSS/RAL collections, and color harmony schemes calculated automatically in both HSL and Lab color space strategies.

```bash
# Analyze RGB color with comprehensive output
color-rs color "rgb(255, 87, 34)"
```

```bash
# Named color analysis
color-rs color "deep orange"
```

#### RAL Color Matching

```bash
# RAL Classic color analysis
color-rs color "RAL 3020"
```

```bash
# RAL Design System+ color
color-rs color "RAL 010 40 30"  
```

```bash
# Search by RAL color name
color-rs color "luminous orange"
```

#### Advanced Distance Calculations

```bash
# Use fast Delta E 76 algorithm
color-rs color "#3498DB" --distance-method delta-e-76
```

```bash
# Use Euclidean distance for rapid matching
color-rs color "rgb(52, 152, 219)" --distance-method euclidean-lab
```

#### Enhanced Color Format Support

```bash
# Hex color without # symbol
color-rs color 457FB3
```

```bash
# LAB color format
color-rs color "lab(60.18, 62.06, 54.34)"
```

```bash
# Case-insensitive named colors
color-rs color "Light Blue"
```

#### Complex Color Formats

```bash
# HSL color with alpha
color-rs color "hsla(207, 74%, 53%, 0.8)"
```

```bash
# RGBA format
color-rs color "rgba(52, 152, 219, 0.9)"
```

---

## Environment Variables

Currently, color-rs does not use environment variables for configuration. All settings are controlled via command-line arguments.

Future versions may support:

- `COLOR_RS_CONFIG_PATH` - Path to configuration file
- `COLOR_RS_CACHE_DIR` - Directory for caching color collection data  
- `COLOR_RS_DEFAULT_WIDTH` - Default image width
- `COLOR_RS_DEFAULT_DISTANCE_METHOD` - Default color distance calculation method

---

## Output Formats

### Terminal Table Output

All commands output terminal tables with:

- Color information with RGB, HEX, HSL, HSB, CMYK, LAB, XYZ, OKLCH values
- WCAG compliance data (relative luminance, contrast ratios)
- RAL color matches with distance calculations
- Color schemes calculated in both HSL and Lab color space strategies
- Right-aligned numeric columns for easy reading

### Image Outputs

#### SVG Format
- Scalable vector graphics with optional legends
- CSS-compatible color values
- Typography using system fonts

#### PNG Format  
- High-quality raster images
- Configurable dimensions (1:5 aspect ratio maintained)
- Embedded metadata with color information

---

## Error Handling

The CLI provides helpful error messages for common issues:

### Invalid Color Formats
```bash
$ color-rs gradient "invalid-color" blue
Error: Unable to parse color: invalid-color
```

### Invalid Arguments
```bash
$ color-rs gradient red blue --start-position 80 --end-position 20
Error: Start position must be less than end position
```

### Invalid Bezier Values
```bash
$ color-rs gradient red blue --ease-in 1.5
Error: Ease-in value must be between 0.0 and 1.0
```

### File Writing Issues
```bash
$ color-rs gradient red blue --svg --svg-name "/readonly/path.svg"
Error: Unable to write SVG file: Permission denied
```

---

## Performance Notes

- **Color Collection Loading**: Initial load of RAL collections (~2000 colors) may take 100-200ms
- **LAB Conversions**: Perceptually accurate but computationally intensive
- **Image Generation**: PNG rendering for large images (>2000px) may take 1-2 seconds
- **Gradient Calculations**: Optimized for typical gradients (5-20 stops), scales linearly

---

## Exit Codes

- `0` - Success
- `1` - General error (invalid arguments, parsing failures)
- `2` - File I/O error (unable to write output files)
- `64` - Usage error (invalid command-line usage)

---

## Compatibility

- **Rust Version**: Requires Rust 2024 edition (1.75+)
- **Operating Systems**: Windows, macOS, Linux
- **Terminal**: Supports ANSI color codes for enhanced output
- **Image Formats**: SVG (all browsers), PNG (universal support)
