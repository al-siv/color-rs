# Color-rs Usage Examples

## Basic Usage - Simplified Interface

```bash
# Simple red to blue gradient - no more flags needed!
color-rs gradient red blue

# Using HEX colors directly
color-rs gradient FF0000 0000FF

# Mixed color formats
color-rs gradient "#FF6B35" "hsl(270, 100%, 50%)"

# Gradient with custom positions
color-rs gradient FF6B35 7209B7 --start-position 20 --end-position 80
```

## RAL Color System Examples

```bash
# RAL Classic colors (both formats supported)
color-rs color-match "RAL 1000"          # Green beige (with space)
color-rs color-match "RAL1000"           # Green beige (without space)
color-rs color-match "RAL 3020"          # Traffic red

# RAL Design System+ colors
color-rs color-match "H010L20C10"        # Wenge Black
color-rs color-match "H040L50C70"        # Pompeii Red

# Search by RAL color names
color-rs color-match "signal yellow"     # Finds RAL 1003
color-rs color-match "traffic red"       # Finds RAL 3020
color-rs color-match "pure blue"         # Finds RAL 5005

# Use RAL colors in gradients
color-rs gradient "RAL 1000" "RAL 5005"  # Green beige to pure blue
color-rs gradient "signal yellow" "traffic red" --svg --width 1600
```

**All RAL color inputs receive comprehensive analysis including:**
- Complete format conversions (RGB, HSL, LAB, XYZ, OKLCH)
- WCAG compliance data (relative luminance, contrast ratios)
- Accessibility information
- Closest RAL matches from both classifications

RAL color matching provides:
- **Separate Classifications**: 2 closest matches from RAL Classic and RAL Design System+ each
- **Precise Matching**: Delta E calculations for perceptually accurate closest color finding
- **Complete Information**: Code, name, hex value, and color distance for each match

## Advanced Features

```bash
# Mathematically distributed gradient stops with custom easing
color-rs gradient FF0000 0000FF --grad-stops 8 --ease-in 0.9 --ease-out 0.1

# Generate image files with simplified syntax
color-rs gradient FF6B35 7209B7 --svg --png --width 1600 --no-legend

# Perfect for design workflows
color-rs gradient "coral" "rebeccapurple" --svg --width 2000
```

## Color Analysis with WCAG Compliance

```bash
# Get comprehensive color analysis including accessibility data
color-rs color-match "#FF5733"

# Check any color format
color-rs color-match "rgb(255, 87, 51)"
color-rs color-match "tomato"
color-rs color-match "hsl(11, 100%, 60%)"
```

Example output shows WCAG compliance data:
```
• WCAG Relative Luminance: 0.283
• Contrast vs White: 3.15:1  (Doesn't meet WCAG AA 4.5:1 for normal text)
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
