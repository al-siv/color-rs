# Type System Reference v0.15.4

Comprehensive documentation of the color-rs type system, domain model, and data structures including rationale, invariants, constructors, and conversion patterns.

## Overview

The color-rs type system is designed for:
- **Type Safety**: Prevent invalid color values and operations through compile-time checks
- **Performance**: Minimize allocations and optimize for common use cases
- **Extensibility**: Support multiple color spaces and distance algorithms
- **Precision**: Consistent floating-point formatting across all representations

## Core Domain Types

### ColorInfo

Display-optimized color information for terminal table output.

```rust
#[derive(Tabled)]
pub struct ColorInfo {
    #[tabled(rename = "Color")]
    pub label: String,
    #[tabled(rename = "Hex")]
    pub hex: String,
    #[tabled(rename = "RGB")]
    pub rgb: String,
    #[tabled(rename = "HSL")]
    pub hsl: String,
    #[tabled(rename = "Lab")]
    pub lab: String,
}
```

**Design Rationale**: 
- Pre-formatted strings optimize terminal display performance
- Consistent formatting through centralized precision utilities
- Table-ready structure with appropriate column headers

**Type Invariants**:
- `hex`: Always `#RRGGBB` format (7 characters)
- `rgb`: Always `rgb(r, g, b)` where r,g,b ∈ [0,255]
- `hsl`: Always `hsl(h, s%, l%)` where h ∈ [0,360], s,l ∈ [0,100]
- `lab`: Always `Lab(l, a, b)` with precision formatting

**Constructor Pattern**: Use `ColorProcessor::create_color_info()` for guaranteed formatting consistency.

### UniversalColor

Unified color representation across all collections and formats.

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

**Design Rationale**:
- Unifies CSS, RAL Classic, and RAL Design System+ colors
- Optional `code` field accommodates RAL color codes
- `PartialEq` enables efficient duplicate detection

**Type Invariants**:
- `hex`: Valid hex color string (validated on construction)
- `rgb`: Array of exact RGB values [0,255]
- `collection`: One of "css", "ral_classic", "ral_design"
- `code`: RAL format "RAL XXXX" or None for CSS colors

## Precision Formatting Types

### Formatting Functions (v0.15.4)

```rust
pub fn format_f64(value: f64) -> String;              // 3 decimal places max
pub fn format_lab(lab: [f64; 3]) -> [String; 3];      // L: 3dp, a/b: 2dp  
pub fn format_lch(lch: [f64; 3]) -> [String; 3];      // L: 3dp, C: 2dp, H: 3dp
pub fn format_wcag_relative_luminance(value: f64) -> String; // 4 decimal places
```

**Design Rationale**:
- Consistent precision across all output formats (YAML, TOML, terminal, files)
- Specialized formatting for different color space components
- Serde integration for automatic serialization formatting

**Type Safety**:
- Functions are pure (no side effects)
- Input validation for NaN/infinity values
- Thread-safe for concurrent usage

### Serialization Support

```rust
#[serde(serialize_with = "serialize_f64_3")]
pub hsl: [f64; 3],

#[serde(serialize_with = "serialize_lab")]  
pub lab: [f64; 3],

#[serde(serialize_with = "serialize_lch")]
pub lch: [f64; 3],

#[serde(serialize_with = "serialize_wcag_luminance")]
pub wcag_relative_luminance: f64,
```

**Integration Pattern**: Custom serde serializers ensure consistent precision in YAML/TOML output.

## Color Representation Types

### RGB Representation

```rust
pub type Rgb = [u8; 3];  // Red, Green, Blue [0,255]
```

**Invariants**: Each component is exactly one byte (0-255)
**Usage**: Primary representation for final color values

### LAB Representation

```rust
pub type Lab = [f64; 3];  // L*, a*, b* in CIELAB space
```

**Invariants**: 
- L* ∈ [0,100] (lightness)
- a*, b* ∈ [-128,127] (green-red, blue-yellow)
**Usage**: Perceptually uniform calculations and interpolation

### HSL Representation

```rust
pub type Hsl = [f64; 3];  // Hue, Saturation, Lightness
```

**Invariants**:
- Hue ∈ [0,360) degrees
- Saturation ∈ [0,100] percentage  
- Lightness ∈ [0,100] percentage
**Usage**: Traditional color wheel operations

## Gradient System Types

### GradientStop

Individual point in a gradient sequence.

```rust
#[derive(Debug, Clone, Serialize)]
pub struct GradientStop {
    pub position: u8,           // Position 0-100
    pub rgb: [u8; 3],          // RGB color values
    pub hex: String,           // Hex representation
    #[serde(serialize_with = "serialize_wcag_luminance")]
    pub wcag_luminance: f64,   // WCAG relative luminance (4dp)
}
```

**Design Rationale**:
- `position` as u8 enforces 0-100 range at compile time
- Multiple representations for different use cases
- WCAG luminance for accessibility analysis

**Type Invariants**:
- `position` ∈ [0,100]
- `hex` follows #RRGGBB format
- `wcag_luminance` ∈ [0,1] with 4 decimal places

### GradientMetadata

Metadata and parameters for gradient generation.

```rust
#[derive(Debug, Clone, Serialize)]
pub struct GradientMetadata {
    pub program: String,
    pub version: String,
    pub timestamp: String,
    pub analysis_type: String,
    pub parameters: GradientParameters,
}

#[derive(Debug, Clone, Serialize)]
pub struct GradientParameters {
    pub start_color: String,
    pub end_color: String,
    pub start_position: u8,
    pub end_position: u8,
    #[serde(serialize_with = "serialize_f64_3")]
    pub ease_in: f64,
    #[serde(serialize_with = "serialize_f64_3")]
    pub ease_out: f64,
    pub stops: usize,
    pub simple_stops: bool,
}
```

**Design Pattern**: Separates metadata from data for cleaner serialization and processing.

## Parser System Types

### ColorParserType

Enumeration of available parser strategies.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorParserType {
    Css,         // CSS named colors only
    Full,        // All collections (CSS + RAL)
    Custom,      // User-configured parser
}
```

**Type Safety**: Exhaustive matching prevents unhandled parser types.

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
    fn default() -> Self {
        Self {
            enable_css_colors: true,
            enable_ral_classic: true,
            enable_ral_design: true,
            enable_named_colors: true,
            strict_validation: false,
            cache_enabled: true,
        }
    }
}
```

**Design Pattern**: Builder pattern with sensible defaults for common configurations.

## Distance Strategy Types

### ColorDistanceStrategy Trait

Strategy pattern for pluggable distance algorithms.

```rust
pub trait ColorDistanceStrategy: Send + Sync {
    fn calculate_distance(&self, color1: [f64; 3], color2: [f64; 3]) -> f64;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}
```

**Design Benefits**:
- Thread-safe (`Send + Sync`)
- Pluggable algorithms
- Self-documenting through name/description

### Concrete Strategy Types

```rust
pub struct DeltaE76Strategy;
pub struct DeltaE2000Strategy; 
pub struct EuclideanLabStrategy;
pub struct LchStrategy;  // v0.15.4 default
```

**Type Safety**: Each strategy implements the trait with algorithm-specific optimizations.

## Output Filter System Types

### FilterRule

Enumeration of filtering operations.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum FilterRule {
    IncludeBlock(String),
    IncludeField(String, String),  // (block, field)
    ExcludeBlock(String),
    ExcludeField(String, String),  // (block, field)
    IncludeAll,
}
```

**Type Safety**: Explicit enumeration prevents invalid filter operations.

### FilterConfig

Configuration for output filtering.

```rust
#[derive(Debug, Clone)]
pub struct FilterConfig {
    pub rules: Vec<FilterRule>,
    pub default_behavior: FilterBehavior,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FilterBehavior {
    IncludeAll,
    ExcludeAll,
}
```

**Design Pattern**: Configurable default behavior with explicit rule overrides.

### FilteredOutput Types

```rust
#[derive(Debug, Clone, Serialize)]
pub enum AnalysisOutput {
    Unfiltered(ColorAnalysisOutput),
    Filtered(FilteredColorAnalysisOutput),
}

#[derive(Debug, Clone, Serialize)]
pub struct FilteredColorAnalysisOutput {
    pub metadata: ProgramMetadata,  // Always included
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<InputInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion: Option<ColorFormats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast: Option<FilteredContrastData>,
    // ... other optional fields
}
```

**Serialization Strategy**: Optional fields use `skip_serializing_if` for clean output.

## Error Types

### ColorError

Comprehensive error enumeration with context.

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
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
}
```

**Benefits**:
- Automatic `std::error::Error` implementation via `thiserror`
- Contextual error messages
- Automatic conversion from `std::io::Error`

### Result Type Alias

```rust
pub type Result<T> = std::result::Result<T, ColorError>;
```

**Convenience**: Reduces boilerplate in function signatures.

## Type Safety and Invariants

### Compile-Time Guarantees

1. **Range Safety**: `u8` for positions ensures 0-255 range
2. **Color Validity**: Construction through validated parsers
3. **Thread Safety**: `Send + Sync` bounds where needed
4. **Memory Safety**: Owned types prevent lifetime issues

### Runtime Validations

```rust
impl UniversalColor {
    pub fn new(name: String, hex: String, collection: String) -> Result<Self> {
        // Validate hex format
        if !hex.starts_with('#') || hex.len() != 7 {
            return Err(ColorError::ParseError(format!("Invalid hex: {}", hex)));
        }
        
        // Parse RGB values
        let rgb = hex_to_rgb(&hex)?;
        
        Ok(Self {
            name,
            hex,
            rgb,
            collection,
            code: None,
        })
    }
}
```

**Pattern**: Validation in constructors ensures invariants are maintained.

## Constructor Patterns

### Builder Pattern

```rust
impl GradientBuilder {
    pub fn new() -> Self { /* ... */ }
    
    pub fn start_color<S: AsRef<str>>(mut self, color: S) -> Self {
        self.start_color = color.as_ref().to_string();
        self
    }
    
    pub fn ease_in_out(mut self, ease_in: f64, ease_out: f64) -> Self {
        self.ease_in = ease_in;
        self.ease_out = ease_out;
        self
    }
    
    pub fn build(self) -> Result<GradientArgs> { /* validation */ }
}
```

**Benefits**: Fluent API with validation at build time.

### Factory Pattern

```rust
impl ColorParserFactory {
    pub fn create_parser(parser_type: ColorParserType) -> Result<Box<dyn ColorParserTrait>> {
        match parser_type {
            ColorParserType::Css => Ok(Box::new(CssParser::new()?)),
            ColorParserType::Full => Ok(Box::new(FullParser::new()?)),
            ColorParserType::Custom => Err(ColorError::InvalidArguments("Custom parser requires config".into())),
        }
    }
}
```

**Benefits**: Centralized creation logic with type safety.

## Conversion Traits

### Standard Conversions

```rust
impl From<[u8; 3]> for UniversalColor {
    fn from(rgb: [u8; 3]) -> Self {
        let hex = format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]);
        Self {
            name: hex.clone(),
            hex,
            rgb,
            collection: "generated".to_string(),
            code: None,
        }
    }
}

impl TryFrom<&str> for UniversalColor {
    type Error = ColorError;
    
    fn try_from(hex: &str) -> Result<Self> {
        UniversalColor::new("unnamed".to_string(), hex.to_string(), "manual".to_string())
    }
}
```

**Design Philosophy**: 
- `From` for infallible conversions
- `TryFrom` for conversions that may fail
- Consistent error handling through `ColorError`

### Custom Display Implementations

```rust
impl fmt::Display for UniversalColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.hex)
    }
}

impl fmt::Display for GradientStop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%: {} (luminance: {:.4})", 
               self.position, 
               self.hex, 
               self.wcag_luminance)
    }
}
```

**Consistency**: All types have meaningful string representations.

## Memory Management

### Owned vs Borrowed Types

The type system prefers owned types (`String`, `Vec`) over borrowed types (`&str`, `&[T]`) for:
- **API Simplicity**: No lifetime parameters in public interfaces
- **Thread Safety**: Owned data can cross thread boundaries
- **Caching**: Internal caches can store owned data

### Zero-Copy Where Possible

```rust
pub fn analyze_color_borrowed(input: &str) -> Result<ColorAnalysisResult> {
    // Process without unnecessary cloning
}

pub fn analyze_color_owned(input: String) -> Result<ColorAnalysisResult> {
    analyze_color_borrowed(&input)
}
```

**Pattern**: Internal functions use borrowed types, public API accepts both.

## Version 0.15.4 Type System Updates

### Precision Type Integration

- **Serde Attributes**: All floating-point fields now use custom serializers
- **Formatting Consistency**: Unified precision across all output formats
- **Type Safety**: Compile-time guarantees for formatting behavior

### Enhanced Error Context

```rust
#[derive(Debug, thiserror::Error)]
pub enum PrecisionError {
    #[error("Invalid precision value: {value}, expected range: {range}")]
    InvalidPrecision { value: f64, range: String },
    
    #[error("Formatting error: {0}")]
    FormatError(String),
}
```

**Improvement**: More specific error types for precision-related operations.

This type system provides a solid foundation for color-rs operations while maintaining safety, performance, and extensibility.
