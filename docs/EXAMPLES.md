# Color-rs Usage Examples v0.14.1

Practical examples demonstrating color analysis, gradient generation, and YAML/TOML output formats.

## Basic Color Analysis

```bash
# Analyze any color format
color-rs color "#FF5733"
color-rs color "rgb(255, 87, 51)"
color-rs color "red"
color-rs color "hsl(11, 100%, 60%)"
```

### Example Output (YAML)
```yaml
metadata:
  program: "color-rs"
  version: "0.14.1"
  timestamp: "2024-01-15T10:30:45Z"
  analysis_type: "color"

input:
  value: "#FF5733"
  format: "hex"

conversion:
  rgb: [255, 87, 51]
  hsl: [11.0, 100.0, 60.0]
  hex: "#ff5733"
  lab: [60.18, 62.06, 54.34]
  lch: [60.18, 83.45, 41.15]
  xyz: [0.453, 0.283, 0.062]

contrast:
  wcag_relative_luminance: 0.283
  contrast_vs_white: 3.15
  contrast_vs_black: 6.66

grayscale:
  rgb: [153, 153, 153]
  hex: "#999999"
  lab_l_star: 60.18

color_collections:
  css_colors:
    - name: "tomato"
      hex: "#ff6347"
      distance: 2.84
    - name: "orangered"
      hex: "#ff4500"
      distance: 8.92
  ral_classic:
    - code: "RAL 2004"
      name: "pure orange"
      hex: "#f44611"
      distance: 9.25
  ral_design:
    - code: "RAL 040 60 60"
      name: "strong orange"
      hex: "#e55100"
      distance: 7.33

color_schemes:
  complementary: ["#ff5733", "#33c4ff"]
  triadic: ["#ff5733", "#33ff57", "#5733ff"]
  tetradic: ["#ff5733", "#57ff33", "#33c4ff", "#c433ff"]
```

## Selective Output Control

Use the `--func` parameter to display only specific blocks or fields from the analysis output.

### Basic Block Filtering

```bash
# Show only color conversion data
color-rs color red --func "[conversion]"

# Show only WCAG contrast information  
color-rs color "#FF5733" --func "[contrast]"

# Show multiple blocks
color-rs color blue --func "[input,conversion,contrast]"
```

#### Block Filter Output Example
```yaml
# color-rs color red --func "[conversion]"
metadata:
  program_name: color-rs
  version: '0.14.1'
  generated_at: '2025-07-23T13:00:00.000Z'
  distance_strategy: Delta E 2000

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

### Field-Level Filtering

```bash
# Show only WCAG relative luminance value
color-rs color "#FF0000" --func "[contrast.wcag21_relative_luminance]"

# Show specific grayscale variants
color-rs color green --func "[grayscale.lch0,grayscale.lch0_hex]"

# Show specific conversion formats
color-rs color blue --func "[conversion.hex,conversion.rgb,conversion.hsl]"
```

#### Field Filter Output Example
```yaml
# color-rs color "#FF0000" --func "[contrast.wcag21_relative_luminance]"
metadata:
  program_name: color-rs
  version: '0.14.1'
  generated_at: '2025-07-23T13:00:00.000Z'
  distance_strategy: Delta E 2000

contrast:
  wcag21_relative_luminance: 0.21267294883728027
```

### Exclusion Filtering

```bash
# Show all data except color collections
color-rs color red --func "[all,!color_collections]"

# Show contrast data but exclude brightness assessment
color-rs color "#FF5733" --func "[contrast,!contrast.brightness]"

# Exclude multiple blocks
color-rs color blue --func "[all,!color_collections,!color_schemes]"
```

#### Exclusion Filter Output Example
```yaml
# color-rs color red --func "[all,!color_collections]" 
# (Shows all blocks except color_collections)
metadata:
  program_name: color-rs
  version: '0.14.1'
  
input:
  input_color: red
  base_color: '#FF0000'
  
conversion:
  hex: '#FF0000'
  # ... full conversion data
  
contrast:
  wcag21_relative_luminance: 0.21267294883728027
  # ... full contrast data
  
grayscale:
  lch0_hex: '#7F7F7F'
  # ... full grayscale data
  
color_schemes:
  complementary:
    hex: '#00A2F3'
    # ... full scheme data
```

### Practical Filtering Examples

```bash
# Quick WCAG compliance check
color-rs color "#333333" --func "[contrast.wcag21_relative_luminance]"

# Extract only hex codes for web development
color-rs color "forestgreen" --func "[conversion.hex]"

# Get color scheme without collection matches (faster)
color-rs color "#FF5733" --func "[input,color_schemes]"

# Focus on specific color space conversions
color-rs color blue --func "[conversion.lab,conversion.lch,conversion.oklch]"

# Grayscale analysis only
color-rs color "#E74C3C" --func "[grayscale]"
```

## RAL Color System

```bash
# RAL Classic colors
color-rs color "RAL 3020"    # Traffic red
color-rs color "RAL1000"     # Green beige (no space)

# RAL Design System+ colors  
color-rs color "RAL 010 40 30"  # Deep red
color-rs color "RAL 270 30 40"  # Deep purple

# Search by RAL names
color-rs color "signal yellow"
color-rs color "traffic red"
color-rs color "luminous orange"
```

### RAL Analysis Output
```yaml
input:
  value: "RAL 3020" 
  format: "ral_classic"

conversion:
  rgb: [204, 6, 5]
  hex: "#cc0605"
  # ... complete conversion data

color_collections:
  ral_classic:
    - code: "RAL 3020"
      name: "traffic red"
      hex: "#cc0605"
      distance: 0.0    # Exact match
  # ... other collections with closest matches
```

## Basic Gradients

```bash
# Simple gradients
color-rs gradient red blue
color-rs gradient "#FF0000" "#0000FF"
color-rs gradient "rgb(255,0,0)" "hsl(240,100%,50%)"
```

### Gradient Output (YAML)
```yaml
metadata:
  program: "color-rs"
  version: "0.14.1"
  analysis_type: "gradient"
  parameters:
    start_position: 0
    end_position: 100
    ease_in: 0.65
    ease_out: 0.35
    stops: 5

start_color:
  value: "red"
  format: "name"
  conversion:
    rgb: [255, 0, 0]
    hex: "#ff0000"
    lab: [53.24, 80.09, 67.20]

end_color:
  value: "blue"
  format: "name"
  conversion:
    rgb: [0, 0, 255]
    hex: "#0000ff"
    lab: [32.30, 79.20, -107.86]

gradient_stops:
  - position: 0
    rgb: [255, 0, 0]
    hex: "#ff0000"
    wcag_luminance: 0.213
  - position: 25
    rgb: [191, 0, 152]
    hex: "#bf0098"
    wcag_luminance: 0.160
  - position: 50
    rgb: [132, 0, 213]
    hex: "#8400d5"
    wcag_luminance: 0.134
  - position: 75
    rgb: [66, 0, 240]
    hex: "#4200f0"
    wcag_luminance: 0.094
  - position: 100
    rgb: [0, 0, 255]
    hex: "#0000ff"
    wcag_luminance: 0.072

summary:
  total_stops: 5
  wcag_contrast_ratio: 2.96
  lab_distance: 157.42
```

## Advanced Gradient Control

```bash
# Custom positions
color-rs gradient red blue --start-position 20 --end-position 80

# Custom easing (ease-in-out)
color-rs gradient "#FF6B35" "#7209B7" --ease-in 0.42 --ease-out 0.58

# Different stop distributions
color-rs gradient red blue --step 10          # Every 10%
color-rs gradient red blue --stops 8          # 8 intelligent stops
color-rs gradient red blue --stops 6 --stops-simple  # 6 equal stops
```

## Image Generation

```bash
# Generate SVG gradient
color-rs gradient red blue --svg

# Generate PNG with custom dimensions
color-rs gradient red blue --png --width 1600

# Both formats without legend
color-rs gradient "#FF6B35" "#7209B7" --svg --png --no-legend

# Custom filenames
color-rs gradient red blue --svg --svg-name custom-gradient.svg
```

## Output Format Options

```bash
# YAML output (default)
color-rs color "#FF5733"

# TOML output
color-rs color "#FF5733" --output toml

# Save to file
color-rs color "#FF5733" --output yaml --file analysis
# Creates: analysis.yaml

color-rs gradient red blue --output toml --file gradient
# Creates: gradient.toml
```

### TOML Format Example
```toml
[metadata]
program = "color-rs"
version = "0.14.1"
timestamp = "2024-01-15T10:30:45Z"
analysis_type = "color"

[input]
value = "#FF5733"
format = "hex"

[conversion]
rgb = [255, 87, 51]
hsl = [11.0, 100.0, 60.0]
hex = "#ff5733"
lab = [60.18, 62.06, 54.34]

[contrast]
wcag_relative_luminance = 0.283
contrast_vs_white = 3.15
contrast_vs_black = 6.66

[[color_collections.css_colors]]
name = "tomato"
hex = "#ff6347"
distance = 2.84
```

## Distance Method Comparison

```bash
# Different distance calculation methods
color-rs color "#FF5733" --distance-method delta-e-2000  # Most accurate
color-rs color "#FF5733" --distance-method delta-e-76    # Faster
color-rs color "#FF5733" --distance-method euclidean-lab # Simple
color-rs color "#FF5733" --distance-method lch           # Cylindrical
```

## Color Scheme Strategies

```bash
# LAB-based schemes (default - perceptually uniform)
color-rs color "blue" --schemes lab

# HSL-based schemes (traditional)
color-rs color "blue" --schemes hsl
```

### Scheme Output Comparison
```yaml
# LAB schemes (perceptually uniform)
color_schemes:
  complementary: ["#0000ff", "#ffff00"]
  triadic: ["#0000ff", "#ff0000", "#00ff00"]

# HSL schemes (traditional color wheel)
color_schemes:
  complementary: ["#0000ff", "#ffff00"]
  triadic: ["#0000ff", "#ff8000", "#80ff00"]
```

## Luminance Replacement

```bash
# Replace with specific WCAG relative luminance
color-rs color "#FF5733" --relative-luminance 0.5

# Replace with specific Lab lightness
color-rs color "blue" --luminance 60
```

### Luminance Replacement Output
```yaml
input:
  value: "#FF5733"
  format: "hex"
  modifications:
    relative_luminance_target: 0.5
    
conversion:
  rgb: [182, 69, 38]    # Adjusted for target luminance
  hex: "#b64526"
  lab: [50.0, 45.2, 39.6]  # L* preserved from original

contrast:
  wcag_relative_luminance: 0.5  # Matches target
```

## Practical Workflows

### Web Development
```bash
# Check accessibility compliance
color-rs color "#FF5733" | grep "contrast_vs_white"
# 3.15 (doesn't meet WCAG AA 4.5:1 requirement)

# Generate CSS-ready gradient data
color-rs gradient red blue --stops 5 --output yaml
```

### Design Systems
```bash
# RAL color matching for brand compliance
color-rs color "#FF5733" 
# Shows closest RAL matches for material specification

# Corporate gradient with specific RAL colors
color-rs gradient "RAL 3020" "RAL 5005" --svg --width 2000
```

### Color Analysis Pipeline
```bash
# Batch process with file output
color-rs color "#FF5733" --output yaml --file red-analysis
color-rs color "#33C4FF" --output yaml --file blue-analysis

# Compare color schemes
color-rs color "red" --schemes lab --output toml --file red-lab-schemes
color-rs color "red" --schemes hsl --output toml --file red-hsl-schemes
```

## Technical Examples

### Perceptual Color Matching
```bash
# Use Delta E 2000 for most accurate matching (default)
color-rs color "#FF5733" --distance-method delta-e-2000

# Results show perceptually uniform distances
color_collections:
  css_colors:
    - name: "tomato"
      distance: 2.84    # Very close perceptually
    - name: "orangered"  
      distance: 8.92    # Moderate difference
```

### Color Space Conversions
```yaml
# Complete color space representation
conversion:
  rgb: [255, 87, 51]          # sRGB values
  hsl: [11.0, 100.0, 60.0]   # Hue, Saturation, Lightness
  hex: "#ff5733"              # Hexadecimal
  lab: [60.18, 62.06, 54.34] # CIE LAB (perceptually uniform)
  lch: [60.18, 83.45, 41.15] # CIE LCH (cylindrical LAB)
  xyz: [0.453, 0.283, 0.062] # CIE XYZ (device independent)
```
• Contrast vs Black: 6.66:1  (Meets WCAG AA requirements)
```

## CSS Integration

```bash
# Generate CSS-ready values with the new simple syntax
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

## Web Development Workflow

1. **Design your gradient**: `color-rs gradient primary-color secondary-color`
2. **Check accessibility**: `color-rs color-match your-color` (verify contrast ratios)
3. **Export for design**: Copy RGB values directly into your CSS
4. **Create assets**: Use `--svg` and `--png` flags for design mockups
5. **Validate compliance**: Ensure colors meet WCAG AA (4.5:1) or AAA (7:1) contrast requirements

## Real-World Examples

```bash
# Brand gradient for web design
color-rs gradient "#3B82F6" "#8B5CF6" --grad-stops 3 --svg

# Accessibility-focused color palette
color-rs color-match "#1F2937"  # Check if this dark color has good contrast
color-rs color-match "#F9FAFB"  # Check if this light color works as background

# Custom easing for smooth animations
color-rs gradient "#EF4444" "#10B981" --ease-in 0.42 --ease-out 0.58

# High-resolution export for print design
color-rs gradient coral navy --png --width 4000 --no-legend
```

## Color Distance Strategy Examples

### Strategy Pattern - Different Distance Methods

```bash
# Use different color distance calculation methods
color-rs color-match red --distance-method delta-e-76      # CIE Delta E 1976 (fast)
color-rs color-match red --distance-method delta-e-2000    # CIE Delta E 2000 (most accurate)
color-rs color-match red --distance-method euclidean-lab   # Euclidean distance in LAB space

# Compare results between different strategies
color-rs color-match "#FF6B35" --distance-method delta-e-76
color-rs color-match "#FF6B35" --distance-method delta-e-2000

# Use strategy in gradients for color selection
color-rs gradient red blue --distance-method delta-e-2000
```

**Distance Method Details:**
- **delta-e-76**: CIE Delta E 1976 - Fast, basic perceptual distance
- **delta-e-2000**: CIE Delta E 2000 - Most accurate perceptual distance, industry standard
- **euclidean-lab**: Euclidean distance in LAB color space - Mathematical distance

### Library Integration with Strategies

```rust
use color_rs::color_distance_strategies::{ColorDistanceStrategy, create_strategy};

// Create different strategies
let delta_e_76 = create_strategy("delta-e-76");
let delta_e_2000 = create_strategy("delta-e-2000");
let euclidean = create_strategy("euclidean-lab");

// Calculate distances with different methods
let lab1 = [50.0, 20.0, -30.0];
let lab2 = [60.0, 15.0, -20.0];

let distance_76 = delta_e_76.calculate_distance(lab1, lab2);
let distance_2000 = delta_e_2000.calculate_distance(lab1, lab2);
let distance_euclidean = euclidean.calculate_distance(lab1, lab2);

// Use strategies with UnifiedColorManager
let manager = UnifiedColorManager::new();
let results = manager.find_closest_with_strategy([255, 100, 50], &delta_e_2000, 5);
```

## Unified Color Collection System

### Advanced RAL Group Filtering

```bash
# Find red colors in RAL Classic groups
color-rs color-match red --ral-classic-groups "RAL 3000"

# Find colors by hue in RAL Design System+
color-rs color-match red --ral-design-hue "Red,Orange"

# Filter by lightness range in RAL Design System+
color-rs color-match "#808080" --ral-design-lightness 40-60

# Combine multiple filters
color-rs color-match "#FF5733" --ral-classic-groups "RAL 2000,RAL 3000" --max-results 5
```

### Search by Color Codes and Names

```bash
# Exact code lookup (works across all collections)
color-rs color-match "RAL 1000"          # RAL Classic: Green beige
color-rs color-match "H010L20C10"        # RAL Design System+: Wenge Black

# Pattern-based name search
color-rs find-by-name "red"              # All colors containing "red"
color-rs find-by-name "beige"            # All beige variations across collections
color-rs find-by-name "blue" --collection "CSS"    # Only CSS colors
```

## Design Patterns - Library Examples

### Builder Pattern for Advanced Gradient Configuration

```rust
use color_rs::GradientBuilder;

// Gradient with ease-in-out
let gradient = GradientBuilder::new()
    .start_color("#2C3E50")           // Dark blue-gray
    .end_color("#E74C3C")            // Red
    .ease_in_out()                   // Smooth acceleration/deceleration
    .intelligent_stops(8)            // Mathematically optimized stop placement
    .svg()                          // Export to SVG
    .svg_filename("gradient.svg")
    .width(1200)
    .build()?;

// Linear technical gradient
let technical = GradientBuilder::new()
    .start_color("hsl(200, 80%, 30%)")  // Dark blue
    .end_color("hsl(200, 80%, 70%)")    // Light blue
    .linear()                         // No easing - technical/data visualization
    .equal_stops(10)                  // Evenly distributed
    .steps(5)                         // 5% increments
    .png()                           // Export to PNG
    .build()?;

// Custom brand gradient
let brand = GradientBuilder::new()
    .start_color("#FF6B35")          // Brand orange
    .end_color("#004E89")            // Brand blue
    .ease_in(0.25)                   // Subtle ease-in
    .ease_out(0.85)                  // Strong ease-out
    .start_position(15)              // Offset start
    .end_position(85)                // Offset end
    .images()                        // Both SVG and PNG
    .no_legend()                     // Clean output
    .build()?;
```

### Factory Pattern for Specialized Color Parsing

```rust
use color_rs::{ColorParserFactory, ColorParserType, ColorParserConfig};

// Performance-optimized parser for web applications
let web_parser = ColorParserFactory::create_fast()?;
let (color, format) = web_parser.parse("#3498DB")?;

// Comprehensive parser for design tools
let design_parser = ColorParserFactory::create_comprehensive()?;
let closest_name = design_parser.get_color_name(52, 152, 219); // "Dodger Blue"

// Strict validation parser for data processing
let validator = ColorParserFactory::create_strict()?;
// Will fail on invalid input instead of fallback

// Custom configuration for specific use cases
let config = ColorParserConfig {
    parser_type: ColorParserType::Full,
    strict_validation: false,
    enable_fallback_naming: true,
    color_tolerance: 12.0,              // Slightly more permissive matching
};
let custom_parser = ColorParserFactory::create_with_config(config)?;

// Check parser capabilities for dynamic UI
let info = design_parser.get_info();
match info.parser_type {
    ColorParserType::Full => println!("Full feature parser with {} collections", info.collection_count),
    ColorParserType::Css => println!("Basic CSS parser"),
    _ => println!("Custom parser configuration"),
}
```

### Strategy Pattern for Color Science Applications

```rust
use color_rs::{create_strategy, available_strategies, ColorUtils};

// Scientific color analysis with different algorithms
let algorithms = available_strategies();
for algorithm in algorithms {
    let strategy = create_strategy(algorithm);
    println!("{}: {}", strategy.name(), strategy.description());
}

// Perceptual color matching (recommended for UI/design)
let perceptual = create_strategy("delta-e-2000");
let lab1 = ColorUtils::rgb_to_lab([255, 0, 0]);      // Red
let lab2 = ColorUtils::rgb_to_lab([255, 50, 50]);    // Light red
let perceptual_distance = perceptual.calculate_distance(lab1, lab2);

// Fast matching for real-time applications
let fast = create_strategy("euclidean-lab");
let fast_distance = fast.calculate_distance(lab1, lab2);

// Legacy compatibility
let legacy = create_strategy("delta-e-76");
let legacy_distance = legacy.calculate_distance(lab1, lab2);

println!("Perceptual (ΔE 2000): {:.2}", perceptual_distance);  // Most accurate
println!("Fast (Euclidean): {:.2}", fast_distance);           // Fastest
println!("Legacy (ΔE 76): {:.2}", legacy_distance);           // Compatibility

// Use in color palette generation
fn find_complementary_color(base_rgb: [u8; 3]) -> String {
    let strategy = create_strategy("delta-e-2000");
    let base_lab = ColorUtils::rgb_to_lab(base_rgb);
    
    // ... complement finding logic using strategy
    "Complementary color analysis result".to_string()
}
```

### Library Integration Examples

```rust
// Find colors across all collections
let manager = UnifiedColorManager::new();
let results = manager.find_closest_across_all([255, 0, 0], 2);

// RAL-specific filtering
let ral_groups = vec!["RAL 3000".to_string()];
let red_rals = manager.find_ral_classic_in_groups([255, 0, 0], &ral_groups, 5);

// Advanced search with multiple criteria
let filter = SearchFilter {
    groups: Some(vec!["Red".to_string()]),
    luminance_range: Some([0.3, 0.7]),
    max_distance: Some(25.0),
    ..Default::default()
};
let filtered = manager.search_with_filter([200, 50, 50], &filter, 10);
```
