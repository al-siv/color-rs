# Color-rs API Reference v0.15.4

Comprehensive API documentation for the color-rs library, providing color analysis, gradient generation, and color space conversions with enhanced precision formatting and distance calculation consistency.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
color-rs = "0.15.4"
```

## Quick Start

```rust
use color_rs::{ColorOperationsFacade, ColorUtils, GradientCalculator, Result};
use color_rs::gradient::easing::CubicBezierEasing;

fn main() -> Result<()> {
    // Color analysis using ColorOperationsFacade
    let facade = ColorOperationsFacade::new();
    let analysis = facade.analyze_color("#FF5733")?;
    
    println!("Hex: {}", analysis.hex);
    println!("RGB: {:?}", analysis.srgb);
    println!("LAB: {:?}", analysis.lab);
    println!("HSL: {:?}", analysis.hsl);
    println!("Luminance: {}", analysis.luminance);
    
    // Gradient generation with GradientCalculator
    let calculator = GradientCalculator::with_intelligent_stops(0.65, 0.35);
    let easing = CubicBezierEasing::new(0.65, 0.35);
    
    let start_lab = ColorUtils::parse_hex_color("#FF5733")?;
    let end_lab = ColorUtils::parse_hex_color("#3366FF")?;
    
    let gradient_values = calculator.generate_gradient_values(
        start_lab,
        end_lab,
        5,          // number of stops
        0,          // start position
        100,        // end position
        &easing,    // easing strategy
    )?;
    
    for (i, value) in gradient_values.iter().enumerate() {
        println!("Stop {}: {} at position {}", i, value.hex, value.position);
    }
    
    Ok(())
}
```

## Core API Modules

### Color Analysis Module

Primary color analysis functionality with precision formatting (3 decimal places):

```rust
use color_rs::color;

// Analyze a color with full output
let result = color::analyze_color(
    color_input: &str,          // Color value (any format)
    distance_method: &str,      // "delta-e-2000", "delta-e-76", etc.
    scheme_strategy: &str,      // "lab" or "hsl"
    relative_luminance: Option<f64>,  // WCAG luminance replacement
    lab_luminance: Option<f64>        // LAB L* replacement
)?;

// Result contains structured color data:
// - metadata (program info, timestamp)
// - input (original value, detected format) 
// - conversion (all color space conversions)
// - contrast (WCAG compliance data)
// - grayscale (perceptually accurate conversion)
// - color_collections (closest matches from CSS/RAL)
// - color_schemes (complementary, triadic, tetradic)
```

### Gradient Module

Gradient generation with LAB interpolation and enhanced precision formatting:

```rust
use color_rs::gradient;

// Generate gradient with intelligent stops
let result = gradient::generate_gradient(
    start_color: &str,      // Starting color (any format)
    end_color: &str,        // Ending color (any format)
    start_position: u8,     // Start position (0-100)
    end_position: u8,       // End position (0-100)
    ease_in: f64,           // Cubic-bezier ease-in (0.0-1.0)
    ease_out: f64,          // Cubic-bezier ease-out (0.0-1.0)
    stops: usize,           // Number of gradient stops
    simple_stops: bool,     // Use equal spacing vs curve derivatives
    step_override: Option<u8>,  // Override with fixed step size
    generate_images: bool   // Generate SVG/PNG files
)?;

// Result contains structured gradient data:
// - metadata (program info, parameters)
// - start_color (complete analysis)
// - end_color (complete analysis)
// - gradient_stops (array with positions, colors, luminance)
// - summary (contrast ratios, total distance)
```

### Format Utils Module

Output formatting utilities with precision control:

```rust
use color_rs::format_utils;

// Format as YAML (default) - floating-point values formatted to 3 decimal places
let yaml_output = format_utils::format_as_yaml(&result_data)?;

// Format as TOML - consistent precision formatting
let toml_output = format_utils::format_as_toml(&result_data)?;

// Save to file with automatic extension
format_utils::save_to_file(&result_data, "analysis", "yaml")?;
// Creates: analysis.yaml
```

### Output Filter Module

Selective output filtering system for controlling which parts of color analysis are displayed:

```rust
use color_rs::output_filter::{FilterConfig, FilterEngine, AnalysisOutput};
use color_rs::color::analyze_color;

// Create filter configuration from expression
let filter_config = FilterConfig::from_expression("[input,conversion]")?;

// Create filter engine
let filter_engine = FilterEngine::new(filter_config);

// Perform color analysis
let analysis_result = analyze_color("#FF5733", "delta-e-2000", "lab", None, None)?;

// Apply filtering
let filtered_output = filter_engine.apply(&analysis_result)?;

// Serialize filtered output
match filtered_output {
    AnalysisOutput::Filtered(filtered) => {
        let yaml = filtered.to_yaml()?;
        println!("{}", yaml);
    },
    AnalysisOutput::Unfiltered(unfiltered) => {
        let yaml = unfiltered.to_yaml()?;
        println!("{}", yaml);
    }
}
```

## Data Structures

### ColorAnalysisResult

Complete color analysis output with precision formatting:

```rust
#[derive(Serialize, Deserialize)]
pub struct ColorAnalysisResult {
    pub metadata: AnalysisMetadata,
    pub input: InputData,
    pub conversion: ConversionData,
    pub contrast: ContrastData,
    pub grayscale: GrayscaleData,
    pub color_collections: ColorCollectionData,
    pub color_schemes: ColorSchemeData,
}

#[derive(Serialize, Deserialize)]
pub struct ConversionData {
    pub rgb: [u8; 3],
    #[serde(serialize_with = "serialize_f64_3")]
    pub hsl: [f64; 3],  // 3 decimal places
    pub hex: String,
    #[serde(serialize_with = "serialize_lab")]
    pub lab: [f64; 3],  // Special formatting for LAB
    #[serde(serialize_with = "serialize_lch")]
    pub lch: [f64; 3],  // Special formatting for LCH
    #[serde(serialize_with = "serialize_f64_3")]
    pub xyz: [f64; 3],  // 3 decimal places
}
```

### Error Handling

```rust
use color_rs::ColorError;

#[derive(Debug)]
pub enum ColorError {
    InvalidColor(String),
    InvalidFormat(String),
    ConversionError(String),
    IoError(String),
    SerializationError(String),
}
```

## Advanced Usage Examples

### Color Collection Matching

```rust
use color_rs::color;

// Find closest colors with different distance methods
let result = color::analyze_color(
    "#FF5733",
    "delta-e-2000",  // Most perceptually accurate
    "lab",
    None,
    None
)?;

// Access closest matches
let css_matches = &result.color_collections.css_colors;
let ral_classic = &result.color_collections.ral_classic;
let ral_design = &result.color_collections.ral_design;

for color_match in css_matches {
    println!("CSS: {} - {:.3} distance", 
             color_match.name, 
             color_match.distance);
}
```

### Precision Formatting Control

```rust
use color_rs::precision_utils::{format_f64, format_lab, format_lch, format_wcag_relative_luminance};

// Standard 3 decimal places
let formatted = format_f64(1.23456789); // "1.235"

// LAB a/b components (2 decimal places)
let lab_formatted = format_lab([50.0, 25.123456, -15.987654]); 
// [50.000, 25.12, -15.99]

// LCH chroma component (2 decimal places)
let lch_formatted = format_lch([50.0, 35.123456, 45.0]); 
// [50.000, 35.12, 45.000]

// WCAG relative luminance (4 decimal places)
let wcag_formatted = format_wcag_relative_luminance(0.123456789); // "0.1235"
```

### Thread Safety

All color-rs functions are stateless and thread-safe:

```rust
use std::thread;
use color_rs::color;

let handles: Vec<_> = (0..4).map(|i| {
    thread::spawn(move || {
        color::analyze_color(&format!("#{:06x}", i * 0x111111), "delta-e-2000", "lab", None, None)
    })
}).collect();

for handle in handles {
    let result = handle.join().unwrap()?;
    // Process result...
}
```

## Version 0.15.4 Changes

### Precision Formatting Updates
- **Floating-point values**: Now formatted to 3 decimal places maximum
- **WCAG relative luminance**: Formatted to 4 decimal places for accessibility compliance
- **LAB a/b components**: Formatted to 2 decimal places for visual accuracy
- **LCH chroma**: Formatted to 2 decimal places for perceptual consistency

### Distance Method Changes
- **Default method**: Changed from "delta-e-2000" to "lch" for improved performance
- **LCH distance**: Enhanced algorithm for more accurate color matching

### Serialization Improvements
- **Serde integration**: Custom serialization functions for consistent formatting
- **YAML/TOML output**: Unified precision across all output formats
- **File output**: Consistent formatting in saved files
