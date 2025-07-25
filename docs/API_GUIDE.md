# Color-rs API Guide v0.14.1

Rust library API reference for color analysis, gradient generation, and color space conversions.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
color-rs = "0.14.1"
```

## Basic Library Usage

```rust
use color_rs::{color, gradient, format_utils, ColorError};

fn main() -> Result<(), ColorError> {
    // Color analysis
    let color_result = color::analyze_color(
        "#FF5733",
        "delta-e-2000",
        "lab",
        None,
        None
    )?;
    
    // Gradient generation
    let gradient_result = gradient::generate_gradient(
        "red",
        "blue", 
        0,     // start position
        100,   // end position
        0.65,  // ease in
        0.35,  // ease out
        5,     // stops
        false, // simple stops
        None,  // step override
        false  // generate images
    )?;
    
    Ok(())
}
```

## Core Modules

### color Module

Primary color analysis functionality:

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

### gradient Module

Gradient generation with LAB interpolation:

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

### format_utils Module

Output formatting utilities:

```rust
use color_rs::format_utils;

// Format as YAML (default)
let yaml_output = format_utils::format_as_yaml(&result_data)?;

// Format as TOML
let toml_output = format_utils::format_as_toml(&result_data)?;

// Save to file with automatic extension
format_utils::save_to_file(&result_data, "analysis", "yaml")?;
// Creates: analysis.yaml
```

### output_filter Module

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

#### FilterConfig API

```rust
use color_rs::output_filter::{FilterConfig, FilterRule};

// Parse from expression string
let config = FilterConfig::from_expression("[contrast.wcag21_relative_luminance]")?;

// Manual configuration
let mut config = FilterConfig::new();
config.rules.push(FilterRule::IncludeField("contrast".to_string(), "wcag21_relative_luminance".to_string()));

// Check inclusion rules
if config.should_include_block("contrast") {
    // Block will be included
}

if config.should_include_field("contrast", "wcag21_relative_luminance") {
    // Field will be included
}
```

#### Filter Expression Syntax

Supported filter expressions:

```rust
// Block filtering
"[input]"                          // Show only input block
"[input,conversion,contrast]"      // Show multiple blocks
"[all]"                           // Show all blocks (default)

// Field filtering  
"[contrast.wcag21_relative_luminance]"    // Show only specific field
"[grayscale.lch0,grayscale.lch0_hex]"     // Show multiple fields
"[conversion.hex,conversion.rgb]"          // Show conversion subset

// Exclusion filtering
"[all,!color_collections]"               // Show all except color collections
"[contrast,!contrast.brightness]"        // Show contrast except brightness
"[!color_schemes,!grayscale]"            // Exclude multiple blocks

// Mixed filtering
"[input,contrast.wcag21_relative_luminance]"  // Block + field combination
```

## Color Data Structures

### ColorAnalysisResult

Complete color analysis output:

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
pub struct AnalysisMetadata {
    pub program: String,
    pub version: String,
    pub timestamp: String,
    pub analysis_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConversionData {
    pub rgb: [u8; 3],
    pub hsl: [f64; 3],
    pub hex: String,
    pub lab: [f64; 3],
    pub lch: [f64; 3],
    pub xyz: [f64; 3],
}

#[derive(Serialize, Deserialize)]
pub struct ContrastData {
    pub wcag_relative_luminance: f64,
    pub contrast_vs_white: f64,
    pub contrast_vs_black: f64,
}
```

### Filtered Output Structures

Structures for selective output filtering:

```rust
#[derive(Debug, Clone, Serialize)]
pub enum AnalysisOutput {
    /// Regular unfiltered output
    Unfiltered(ColorAnalysisOutput),
    /// Filtered output with optional blocks
    Filtered(FilteredColorAnalysisOutput),
}

#[derive(Debug, Clone, Serialize)]
pub struct FilteredColorAnalysisOutput {
    /// Program metadata (always included)
    pub metadata: ProgramMetadata,
    /// Input information (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<InputInfo>,
    /// Color format conversions (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion: Option<ColorFormats>,
    /// Contrast and luminance information (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast: Option<FilteredContrastData>,
    /// Grayscale variations (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grayscale: Option<FilteredGrayscaleData>,
    /// Color collection matches (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_collections: Option<ColorCollections>,
    /// Color schemes (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_schemes: Option<ColorSchemes>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FilteredContrastData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wcag21_relative_luminance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast_vs_white: Option<ContrastInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast_vs_black: Option<ContrastInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<BrightnessInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FilteredGrayscaleData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch0_hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch2_hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch4_hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch6_hex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lch6: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilterRule {
    /// Include a specific block (e.g., "input", "conversion")
    IncludeBlock(String),
    /// Include a specific field within a block (e.g., "contrast.wcag21_relative_luminance")
    IncludeField(String, String), // (block, field)
    /// Exclude a specific block
    ExcludeBlock(String),
    /// Exclude a specific field within a block
    ExcludeField(String, String), // (block, field)
    /// Include all blocks (default behavior)
    IncludeAll,
}
```

### GradientResult

Gradient generation output:

```rust
#[derive(Serialize, Deserialize)]
pub struct GradientResult {
    pub metadata: GradientMetadata,
    pub start_color: ColorAnalysisResult,
    pub end_color: ColorAnalysisResult,
    pub gradient_stops: Vec<GradientStop>,
    pub summary: GradientSummary,
}

#[derive(Serialize, Deserialize)]
pub struct GradientStop {
    pub position: u8,
    pub rgb: [u8; 3],
    pub hex: String,
    pub wcag_luminance: f64,
}

#[derive(Serialize, Deserialize)]
pub struct GradientMetadata {
    pub program: String,
    pub version: String,
    pub timestamp: String,
    pub analysis_type: String,
    pub parameters: GradientParameters,
}
```

## Error Handling

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

impl std::fmt::Display for ColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ColorError::InvalidColor(msg) => write!(f, "Invalid color: {}", msg),
            ColorError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            ColorError::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
            ColorError::IoError(msg) => write!(f, "I/O error: {}", msg),
            ColorError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for ColorError {}
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
    println!("CSS: {} - {:.2} distance", 
             color_match.name, 
             color_match.distance);
}
```

### Color Scheme Generation

```rust
// Generate LAB-based schemes (perceptually uniform)
let lab_result = color::analyze_color("#FF5733", "delta-e-2000", "lab", None, None)?;
let lab_schemes = &lab_result.color_schemes;

// Generate HSL-based schemes (traditional color wheel)
let hsl_result = color::analyze_color("#FF5733", "delta-e-2000", "hsl", None, None)?;
let hsl_schemes = &hsl_result.color_schemes;

println!("LAB Complementary: {:?}", lab_schemes.complementary);
println!("HSL Complementary: {:?}", hsl_schemes.complementary);
```

### Gradient with Custom Easing

```rust
use color_rs::gradient;

// Create ease-in-out gradient
let result = gradient::generate_gradient(
    "red",
    "blue",
    0,      // full range
    100,
    0.42,   // ease-in (CSS ease-in-out)
    0.58,   // ease-out (CSS ease-in-out)
    8,      // 8 intelligent stops
    false,  // use curve derivatives
    None,   // no step override
    true    // generate SVG/PNG
)?;

// Access gradient stops
for stop in &result.gradient_stops {
    println!("{}%: {} (luminance: {:.3})", 
             stop.position, 
             stop.hex, 
             stop.wcag_luminance);
}
```

### Luminance Adjustment

```rust
// Replace color with specific WCAG relative luminance
let adjusted = color::analyze_color(
    "#FF5733",
    "delta-e-2000",
    "lab",
    Some(0.5),  // Target WCAG luminance
    None
)?;

// Replace color with specific LAB lightness
let lab_adjusted = color::analyze_color(
    "#FF5733", 
    "delta-e-2000",
    "lab",
    None,
    Some(60.0)  // Target LAB L* value
)?;
```

### File Output

```rust
use color_rs::{color, format_utils};

// Analyze color and save as TOML
let result = color::analyze_color("#FF5733", "delta-e-2000", "lab", None, None)?;
format_utils::save_to_file(&result, "color-analysis", "toml")?;
// Creates: color-analysis.toml

// Generate gradient and save as YAML
let gradient = gradient::generate_gradient("red", "blue", 0, 100, 0.65, 0.35, 5, false, None, false)?;
format_utils::save_to_file(&gradient, "red-blue-gradient", "yaml")?;
// Creates: red-blue-gradient.yaml
```

## Integration Patterns

### Batch Processing

```rust
use color_rs::{color, format_utils};

fn process_color_palette(colors: Vec<&str>) -> Result<(), ColorError> {
    for (i, color_input) in colors.iter().enumerate() {
        let result = color::analyze_color(
            color_input,
            "delta-e-2000",
            "lab", 
            None,
            None
        )?;
        
        format_utils::save_to_file(
            &result, 
            &format!("color-{:02}", i),
            "yaml"
        )?;
    }
    Ok(())
}

// Process a brand palette
process_color_palette(vec!["#FF5733", "#33C4FF", "#7209B7"])?;
```

### Color Validation

```rust
use color_rs::color;

fn validate_accessibility(color: &str, min_contrast: f64) -> Result<bool, ColorError> {
    let result = color::analyze_color(color, "delta-e-2000", "lab", None, None)?;
    
    let white_contrast = result.contrast.contrast_vs_white;
    let black_contrast = result.contrast.contrast_vs_black;
    
    Ok(white_contrast >= min_contrast || black_contrast >= min_contrast)
}

// Check WCAG AA compliance (4.5:1 ratio)
let is_accessible = validate_accessibility("#FF5733", 4.5)?;
```

### Design System Integration

```rust
use color_rs::{color, gradient};

struct DesignSystem {
    primary: String,
    secondary: String,
}

impl DesignSystem {
    fn analyze_colors(&self) -> Result<(ColorAnalysisResult, ColorAnalysisResult), ColorError> {
        let primary = color::analyze_color(&self.primary, "delta-e-2000", "lab", None, None)?;
        let secondary = color::analyze_color(&self.secondary, "delta-e-2000", "lab", None, None)?;
        Ok((primary, secondary))
    }
    
    fn generate_brand_gradient(&self) -> Result<GradientResult, ColorError> {
        gradient::generate_gradient(
            &self.primary,
            &self.secondary,
            0, 100,
            0.65, 0.35,
            6,
            false,
            None,
            true  // Generate images for design assets
        )
    }
}
```

## Performance Considerations

- **Color Space Conversions**: Optimized using the palette library
- **Distance Calculations**: Delta E 2000 is most accurate but slower than Delta E 76
- **Collection Matching**: RAL Design System+ has 1825+ colors vs RAL Classic's 213
- **Memory Usage**: Structured output is memory-efficient with minimal allocation
- **File I/O**: Automatic buffering for large YAML/TOML outputs

## Thread Safety

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
    
    /// Match and convert color between different color spaces
    pub fn color_match(&self, args: ColorMatchArgs) -> Result<String>;
}

impl Default for ColorRs {
    fn default() -> Self;
}
```

### ColorInfo

Represents comprehensive color information across multiple color spaces.

```rust
#[derive(Debug, Clone)]
pub struct ColorInfo {
    pub rgb: [u8; 3],
    pub hex: String,
    pub hsl: [f64; 3],
    pub lab: [f64; 3],
    pub xyz: [f64; 3],
    pub oklch: [f64; 3],
    pub luminance: f64,
    pub name: Option<String>,
}
```

### ColorSpace

Enumeration of supported color spaces for conversions.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSpace {
    Rgb,
    Hsl,
    Lab,
    Xyz,
    Oklch,
}
```

### GradientValue

Represents a single point in a gradient with position and color information.

```rust
#[derive(Debug, Clone)]
pub struct GradientValue {
    pub position: u8,           // Percentage position (0-100)
    pub rgb: [u8; 3],          // RGB values
    pub hex: String,           // HEX representation
    pub lab: [f64; 3],         // LAB color space values
    pub css_percentage: String, // CSS-compatible percentage
}
```

### UniversalColor

Unified representation for colors from any collection or format.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct UniversalColor {
    pub name: String,
    pub hex: String,
    pub rgb: [u8; 3],
    pub collection: String,
    pub code: Option<String>,
}
```

### ColorMatch

Result of color matching operations with distance calculations.

```rust
#[derive(Debug, Clone)]
pub struct ColorMatch {
    pub color: UniversalColor,
    pub distance: f64,
    pub collection_type: String,
}
```

## Color Operations

### ColorUtils

Core utility functions for color space conversions and calculations.

```rust
pub struct ColorUtils;

impl ColorUtils {
    /// Convert RGB to LAB color space
    pub fn rgb_to_lab(rgb: [u8; 3]) -> [f64; 3];
    
    /// Convert LAB to RGB color space
    pub fn lab_to_rgb(lab: [f64; 3]) -> [u8; 3];
    
    /// Convert RGB to HSL color space
    pub fn rgb_to_hsl(rgb: [u8; 3]) -> [f64; 3];
    
    /// Convert HSL to RGB color space
    pub fn hsl_to_rgb(hsl: [f64; 3]) -> [u8; 3];
    
    /// Convert RGB to XYZ color space
    pub fn rgb_to_xyz(rgb: [u8; 3]) -> [f64; 3];
    
    /// Convert RGB to OKLCH color space
    pub fn rgb_to_oklch(rgb: [u8; 3]) -> [f64; 3];
    
    /// Calculate relative luminance for WCAG compliance
    pub fn calculate_luminance(rgb: [u8; 3]) -> f64;
    
    /// Calculate contrast ratio between two colors
    pub fn contrast_ratio(color1: [u8; 3], color2: [u8; 3]) -> f64;
    
    /// Convert RGB values to HEX string
    pub fn rgb_to_hex(rgb: [u8; 3]) -> String;
    
    /// Parse HEX string to RGB values
    pub fn hex_to_rgb(hex: &str) -> color_rs::Result<[u8; 3]>;
}
```

### ColorOperationsFacade

Simplified facade providing easy access to common color operations.

```rust
pub struct ColorOperationsFacade;

impl ColorOperationsFacade {
    /// Create a new facade instance
    pub fn new() -> Self;
    
    /// Convert HEX color to RGB
    pub fn hex_to_rgb(&self, hex: &str) -> Result<[u8; 3]>;
    
    /// Calculate contrast ratio between two colors
    pub fn calculate_contrast(&self, color1: &str, color2: &str) -> Result<f64>;
    
    /// Perform comprehensive color analysis
    pub fn analyze_color(&self, color: &str) -> Result<ColorAnalysis>;
    
    /// Mix two colors with specified ratio (0.0-1.0)
    pub fn mix_colors(&self, color1: &str, color2: &str, ratio: f64) -> Result<String>;
}
```

### ColorAnalysis

Comprehensive analysis result containing all color information.

```rust
#[derive(Debug, Clone)]
pub struct ColorAnalysis {
    pub input: String,
    pub rgb: [u8; 3],
    pub hex: String,
    pub hsl: [f64; 3],
    pub lab: [f64; 3],
    pub xyz: [f64; 3],
    pub oklch: [f64; 3],
    pub luminance: f64,
    pub wcag_aa_normal: bool,     // WCAG AA compliance for normal text
    pub wcag_aa_large: bool,      // WCAG AA compliance for large text
    pub wcag_aaa_normal: bool,    // WCAG AAA compliance for normal text
    pub wcag_aaa_large: bool,     // WCAG AAA compliance for large text
    pub closest_css: Option<ColorMatch>,
    pub closest_ral_classic: Option<ColorMatch>,
    pub closest_ral_design: Option<ColorMatch>,
}
```

## Gradient System

### GradientCalculator

Core gradient calculation engine using LAB color space and cubic-bezier easing.

```rust
pub struct GradientCalculator;

impl GradientCalculator {
    /// Create a new gradient calculator
    pub fn new() -> Self;
    
    /// Calculate gradient values between two colors
    pub fn calculate(
        &self,
        start_color: [f64; 3],  // LAB color space
        end_color: [f64; 3],    // LAB color space
        steps: Vec<f64>,        // Position values (0.0-1.0)
        ease_in: f64,           // Cubic-bezier control point
        ease_out: f64,          // Cubic-bezier control point
    ) -> Vec<GradientValue>;
    
    /// Generate equally spaced gradient stops
    pub fn equal_stops(
        &self,
        start_color: [f64; 3],
        end_color: [f64; 3],
        count: usize,
        ease_in: f64,
        ease_out: f64,
    ) -> Vec<GradientValue>;
    
    /// Generate mathematically placed stops based on curve analysis
    pub fn intelligent_stops(
        &self,
        start_color: [f64; 3],
        end_color: [f64; 3],
        count: usize,
        ease_in: f64,
        ease_out: f64,
    ) -> Vec<GradientValue>;
}
```

### GradientBuilder

Fluent builder pattern for gradient configuration with method chaining.

```rust
pub struct GradientBuilder {
    // Private fields
}

impl GradientBuilder {
    /// Create a new gradient builder
    pub fn new() -> Self;
    
    /// Set starting color (any supported format)
    pub fn start_color<S: AsRef<str>>(self, color: S) -> Self;
    
    /// Set ending color (any supported format)
    pub fn end_color<S: AsRef<str>>(self, color: S) -> Self;
    
    /// Set both control points for custom easing
    pub fn ease_in_out(self, ease_in: f64, ease_out: f64) -> Self;
    
    /// Set ease-in control point
    pub fn ease_in(self, value: f64) -> Self;
    
    /// Set ease-out control point  
    pub fn ease_out(self, value: f64) -> Self;
    
    /// Apply linear easing (no curve)
    pub fn linear(self) -> Self;
    
    /// Apply CSS ease preset (0.25, 1.0)
    pub fn ease(self) -> Self;
    
    /// Apply CSS ease-in preset (0.42, 1.0)
    pub fn ease_in_preset(self) -> Self;
    
    /// Apply CSS ease-out preset (0.0, 0.58)
    pub fn ease_out_preset(self) -> Self;
    
    /// Apply CSS ease-in-out preset (0.42, 0.58)
    pub fn ease_in_out_preset(self) -> Self;
    
    /// Set gradient step percentage
    pub fn steps(self, step_percent: u8) -> Self;
    
    /// Set number of equal stops
    pub fn equal_stops(self, count: usize) -> Self;
    
    /// Set number of mathematically distributed stops
    pub fn intelligent_stops(self, count: usize) -> Self;
    
    /// Enable SVG output
    pub fn svg(self) -> Self;
    
    /// Enable PNG output
    pub fn png(self) -> Self;
    
    /// Enable both image outputs
    pub fn images(self) -> Self;
    
    /// Set image width
    pub fn width(self, width: u32) -> Self;
    
    /// Disable legends on images
    pub fn no_legend(self) -> Self;
    
    /// Build final GradientArgs
    pub fn build(self) -> Result<GradientArgs>;
}
```

## Parser System

### ColorParserFactory

Factory for creating different types of color parsers with various capabilities.

```rust
pub struct ColorParserFactory;

impl ColorParserFactory {
    /// Create a parser of the specified type
    pub fn create_parser(parser_type: ColorParserType) -> Result<Box<dyn ColorParserTrait>>;
    
    /// Create a fast parser optimized for speed
    pub fn create_fast() -> Result<Box<dyn ColorParserTrait>>;
    
    /// Create a comprehensive parser with all features
    pub fn create_comprehensive() -> Result<Box<dyn ColorParserTrait>>;
    
    /// Create a strict parser with validation
    pub fn create_strict() -> Result<Box<dyn ColorParserTrait>>;
    
    /// Create a custom parser with specific configuration
    pub fn create_custom(config: ColorParserConfig) -> Result<Box<dyn ColorParserTrait>>;
}
```

### ColorParserType

Enumeration of available parser types.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorParserType {
    /// Basic CSS color parser
    Css,
    /// Full-featured parser with all collections
    Full,
    /// Custom parser with user configuration
    Custom,
}
```

### ColorParserTrait

Trait defining the interface for color parsers.

```rust
pub trait ColorParserTrait: Send + Sync {
    /// Parse a color string to LAB color space and format
    fn parse(&self, input: &str) -> Result<([f64; 3], String)>;
    
    /// Get color name from RGB values
    fn get_color_name(&self, r: u8, g: u8, b: u8) -> Option<String>;
    
    /// Check if parser supports a specific format
    fn supports_format(&self, format: &str) -> bool;
    
    /// Get parser information and capabilities
    fn get_info(&self) -> ColorParserInfo;
}
```

### ColorParserConfig

Configuration for custom color parsers.

```rust
#[derive(Debug, Clone)]
pub struct ColorParserConfig {
    pub enable_css_colors: bool,
    pub enable_ral_classic: bool,
    pub enable_ral_design: bool,
    pub enable_named_colors: bool,
    pub strict_validation: bool,
    pub cache_enabled: bool,
}

impl Default for ColorParserConfig {
    fn default() -> Self;
}
```

### UnifiedColorManager

Central manager for all color parsing operations across collections.

```rust
pub struct UnifiedColorManager {
    // Private fields
}

impl UnifiedColorManager {
    /// Create a new unified color manager
    pub fn new() -> Result<Self>;
    
    /// Parse any color format to UniversalColor
    pub fn parse_color(&self, input: &str) -> Result<UniversalColor>;
    
    /// Find closest matching colors across all collections
    pub fn find_closest_matches(
        &self,
        target: &UniversalColor,
        max_results: usize,
    ) -> Vec<ColorMatch>;
    
    /// Search colors by name across collections
    pub fn search_by_name(&self, name: &str, filter: SearchFilter) -> Vec<UniversalColor>;
    
    /// Get all available colors from specified collections
    pub fn get_all_colors(&self, filter: SearchFilter) -> Vec<UniversalColor>;
}
```

### SearchFilter

Filter for specifying which color collections to search.

```rust
#[derive(Debug, Clone)]
pub struct SearchFilter {
    pub css_colors: bool,
    pub ral_classic: bool,
    pub ral_design: bool,
    pub exact_match: bool,
    pub case_sensitive: bool,
}

impl Default for SearchFilter {
    fn default() -> Self;
}
```

## Distance Calculation

### ColorDistanceStrategy

Strategy pattern for pluggable color distance algorithms.

```rust
pub trait ColorDistanceStrategy: Send + Sync {
    /// Calculate distance between two LAB colors
    fn calculate_distance(&self, color1: [f64; 3], color2: [f64; 3]) -> f64;
    
    /// Get strategy name
    fn name(&self) -> &str;
    
    /// Get strategy description
    fn description(&self) -> &str;
}
```

### Available Strategies

```rust
/// Get list of available distance calculation strategies
pub fn available_strategies() -> Vec<String>;

/// Create a distance strategy by name
pub fn create_strategy(name: &str) -> Box<dyn ColorDistanceStrategy>;
```

Available strategy names:
- `"delta-e-76"` - Fast CIE Delta E 1976 formula
- `"delta-e-2000"` - Industry-standard CIE Delta E 2000
- `"euclidean-lab"` - Simple Euclidean distance in LAB space

## Template Method Pattern

### ColorMatchingTemplate

Template method pattern for standardized color matching algorithms.

```rust
pub trait ColorMatchingTemplate {
    /// Find closest color using template method
    fn find_closest_color(&self, target: &str) -> Result<ColorMatch>;
    
    /// Template method steps (implemented by concrete classes)
    fn parse_input(&self, input: &str) -> Result<UniversalColor>;
    fn get_collection(&self) -> &[UniversalColor];
    fn calculate_distance(&self, color1: &UniversalColor, color2: &UniversalColor) -> f64;
    fn format_result(&self, color: &UniversalColor, distance: f64) -> ColorMatch;
}
```

### UnifiedColorMatcher

Concrete implementation of ColorMatchingTemplate that searches all collections.

```rust
pub struct UnifiedColorMatcher {
    // Private fields
}

impl UnifiedColorMatcher {
    /// Create a new unified color matcher
    pub fn new() -> Result<Self>;
}

impl ColorMatchingTemplate for UnifiedColorMatcher {
    fn find_closest_color(&self, target: &str) -> Result<ColorMatch>;
    // Other methods implemented
}
```

## Image Generation

### ImageGenerator

High-level interface for generating gradient images.

```rust
pub struct ImageGenerator;

impl ImageGenerator {
    /// Generate SVG image from gradient values
    pub fn generate_svg(
        gradient: &[GradientValue],
        width: u32,
        filename: &str,
        show_legend: bool,
    ) -> Result<()>;
    
    /// Generate PNG image from gradient values
    pub fn generate_png(
        gradient: &[GradientValue],
        width: u32,
        filename: &str,
        show_legend: bool,
    ) -> Result<()>;
}
```

### ImageFormat

Enumeration of supported image formats.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Svg,
    Png,
}
```

## Error Handling

### ColorError

Comprehensive error type covering all failure modes.

```rust
#[derive(Debug, thiserror::Error)]
pub enum ColorError {
    #[error("Color parsing error: {0}")]
    ParseError(String),
    
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
    
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Collection error: {0}")]
    CollectionError(String),
    
    #[error("Calculation error: {0}")]
    CalculationError(String),
    
    #[error("Image generation error: {0}")]
    ImageError(String),
}
```

### Result Type

Convenient type alias for color-rs operations.

```rust
pub type Result<T> = std::result::Result<T, ColorError>;
```

## CLI Integration

### Command Structures

For CLI integration, the library exports the command argument structures:

```rust
/// Main CLI structure
#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands
#[derive(Subcommand)]
pub enum Commands {
    Gradient(GradientArgs),
    ColorMatch(ColorMatchArgs),
}

/// Arguments for gradient generation
#[derive(Args, Clone)]
pub struct GradientArgs {
    pub start_color: String,
    pub end_color: String,
    pub start_position: u8,
    pub end_position: u8,
    pub ease_in: f64,
    pub ease_out: f64,
    pub svg: bool,
    pub png: bool,
    pub no_legend: bool,
    pub width: u32,
    pub svg_name: String,
    pub png_name: String,
    pub grad_step: u8,
    pub grad_stops: Option<usize>,
    pub grad_stops_simple: Option<usize>,
}

/// Arguments for color matching
#[derive(Args, Clone)]
pub struct ColorMatchArgs {
    pub color: String,
    pub distance_method: String,
}
```

## Constants

### Version Information

```rust
/// Current version of the color-rs library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Formatting constant: width for formatted columns in output
pub const COLUMN_WIDTH: usize = 30;
```

## Generic Parameters and Lifetimes

Most types in the color-rs API avoid complex generic parameters and lifetimes for simplicity. Where generics are used:

- **Builder Pattern**: `GradientBuilder` methods accept `AsRef<str>` for flexible string input
- **Strategy Pattern**: `Box<dyn Trait>` for type erasure with owned trait objects
- **Collections**: Use owned data structures (`Vec`, `String`) to avoid lifetime complications

## Blanket Implementations

- **Default**: Implemented for main types (`ColorRs`, `GradientBuilder`, etc.) for convenient initialization
- **Clone**: Available for all data types to support flexible usage patterns
- **Debug**: Implemented for all public types to aid debugging
- **Send + Sync**: All core types are thread-safe for concurrent usage
