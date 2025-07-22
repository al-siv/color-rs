# Color-rs API Guide

This document provides comprehensive reference for the color-rs library's public API, covering all types, traits, and functions re-exported from the crate root.

## Table of Contents

- [Getting Started](#getting-started)
- [Core Types](#core-types)
- [Color Operations](#color-operations)
- [Gradient System](#gradient-system)
- [Parser System](#parser-system)
- [Design Patterns](#design-patterns)
- [Error Handling](#error-handling)
- [Usage Examples](#usage-examples)

## Getting Started

Add color-rs to your `Cargo.toml`:

```toml
[dependencies]
color-rs = "0.14.0"
```

### Basic Usage

```rust
use color_rs::{ColorRs, cli::GradientArgs};

fn main() -> color_rs::Result<()> {
    let color_rs = ColorRs::new();
    
    // Simple gradient generation
    let args = GradientArgs {
        start_color: "red".to_string(),
        end_color: "blue".to_string(),
        // ... other fields with defaults
    };
    
    color_rs.generate_gradient(args)?;
    Ok(())
}
```

## Core Types

### ColorRs

The main library interface providing high-level operations.

```rust
pub struct ColorRs;

impl ColorRs {
    /// Create a new instance of the color-rs library
    pub fn new() -> Self;
    
    /// Generate a gradient based on the provided arguments
    pub fn generate_gradient(&self, args: GradientArgs) -> Result<()>;
    
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
