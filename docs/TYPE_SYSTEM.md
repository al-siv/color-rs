# Color-rs Type System

This document describes the domain model of color-rs, including rationale for each struct/enum, invariants enforced by the type system, constructors vs. direct field access, and From/Into conversions.

## Table of Contents

- [Core Domain Types](#core-domain-types)
- [Color Representation](#color-representation)
- [Gradient System Types](#gradient-system-types)
- [Parser System Types](#parser-system-types)
- [Error Types](#error-types)
- [Type Safety and Invariants](#type-safety-and-invariants)
- [Constructor Patterns](#constructor-patterns)
- [Conversion Traits](#conversion-traits)

## Core Domain Types

### ColorInfo

Represents comprehensive color information across multiple color spaces, optimized for display in tables.

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

**Rationale**: Designed for terminal table output with the `tabled` crate. Fields are pre-formatted strings rather than raw values to optimize for display performance and consistent formatting.

**Invariants**:
- `hex` field always follows format `#RRGGBB` (6 characters plus #)
- `rgb` field follows format `rgb(r, g, b)` where r,g,b are 0-255
- `hsl` field follows format `hsl(h, s%, l%)` where h is 0-360, s,l are 0-100%
- `lab` field follows format `Lab(l, a, b)` with appropriate precision

**Constructor**: Use `ColorProcessor::create_color_info()` rather than direct field access to ensure consistent formatting.

### ColorSpace

Enumeration of supported color spaces for internal calculations.

```rust
#[derive(Debug, Clone, Copy)]
pub enum ColorSpace {
    Srgb,
    Lab, 
    Hsl,
}
```

**Rationale**: Provides type-safe enumeration of color spaces. Uses `Copy` trait for efficient passing by value since it's a simple enum.

**Invariants**: All variants correspond to color spaces supported by the `palette` library.

### UniversalColor

Unified representation for colors from any collection, serving as the canonical color type.

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

**Rationale**: Abstracts over different color collections (CSS, RAL Classic, RAL Design+) with a common interface. The `PartialEq` implementation allows for color comparison across collections.

**Invariants**:
- `hex` is always uppercase format `#RRGGBB`
- `rgb` array elements are valid u8 values (0-255)
- `name` is non-empty string
- `collection` identifies the source collection ("CSS", "RAL Classic", "RAL Design+")
- `code` is optional collection-specific identifier (e.g., "RAL 3020", "H040L50C70")

**Constructor**: Prefer factory methods from specific collections rather than direct construction.

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

**Rationale**: Combines matched color with its calculated distance and source collection for ranking results.

**Invariants**:
- `distance` is always non-negative (≥ 0.0)
- `collection_type` matches the collection field in the contained UniversalColor
- Lower distance values indicate closer matches

**Constructor**: Created by color matching algorithms; not intended for direct construction.

## Color Representation

### ParsedColor

Intermediate representation during color parsing with format detection.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedColor {
    pub r: u8,           // Red component (0-255)
    pub g: u8,           // Green component (0-255) 
    pub b: u8,           // Blue component (0-255)
    pub a: f64,          // Alpha component (0.0-1.0)
    pub format: ColorFormat,
}
```

**Rationale**: Bridges the gap between string input and internal color representation, preserving both parsed values and original format information.

**Invariants**:
- RGB values are constrained to u8 range (0-255) by type system
- Alpha value must be in range 0.0-1.0 (enforced by constructor validation)
- Format enum matches the detected input format

**Constructors**:
```rust
impl ParsedColor {
    pub fn new(r: u8, g: u8, b: u8, a: f64, format: ColorFormat) -> Self;
    pub fn from_rgb(r: u8, g: u8, b: u8, format: ColorFormat) -> Self;
    pub fn rgb(&self) -> (u8, u8, u8);
}
```

### ColorFormat

Enumeration of detected color input formats.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorFormat {
    Hex,    // #rgb or #rrggbb
    Rgb,    // rgb(r,g,b)
    Rgba,   // rgba(r,g,b,a)
    Hsl,    // hsl(h,s%,l%)
    Hsla,   // hsla(h,s%,l%,a)
    Named,  // red, blue, etc.
}
```

**Rationale**: Type-safe format detection enables different parsing strategies and preserves input format information for error reporting.

**Invariants**: Each variant corresponds to a supported input format with specific parsing rules.

## Gradient System Types

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

**Rationale**: Combines position and color information for gradient stops. Includes multiple color representations for different output needs (terminal tables, CSS, image generation).

**Invariants**:
- `position` is constrained to 0-100 by u8 type
- `rgb` values are valid 0-255 range
- `hex` follows `#RRGGBB` format
- `css_percentage` follows `XX%` format matching position
- `lab` values correspond to the RGB color

**Constructor**: Created by `GradientCalculator` methods; ensures all representations are consistent.

### GradientArgs

Command-line arguments for gradient generation, with validation methods.

```rust
#[derive(Args, Clone)]
pub struct GradientArgs {
    pub start_color: String,
    pub end_color: String,
    pub start_position: u8,      // 0-100
    pub end_position: u8,        // 0-100  
    pub ease_in: f64,           // 0.0-1.0
    pub ease_out: f64,          // 0.0-1.0
    pub svg: bool,
    pub png: bool,
    pub no_legend: bool,
    pub width: u32,
    pub svg_name: String,
    pub png_name: String,
    pub grad_step: u8,          // 1-100
    pub grad_stops: Option<usize>,
    pub grad_stops_simple: Option<usize>,
}
```

**Rationale**: Captures all CLI arguments with appropriate types. Uses `u8` for percentages to enforce 0-100 range at type level.

**Invariants**:
- Position values are 0-100 (enforced by u8)
- `start_position < end_position` (validated by `validate()` method)
- Easing values are 0.0-1.0 (validated by `validate()` method)
- Only one of `grad_step`, `grad_stops`, `grad_stops_simple` is used

**Validation**: The `validate()` method enforces logical constraints that cannot be expressed in the type system.

### ColorMatchArgs

Arguments for color matching operations.

```rust
#[derive(Args, Clone)]
pub struct ColorMatchArgs {
    pub color: String,
    pub distance_method: String,
}
```

**Rationale**: Simple structure for color analysis commands. Uses string types to defer validation to parsing stage.

**Invariants**: Color parsing and distance method validation occur during execution, not at type level.

## Parser System Types

### ColorParserType

Enumeration of available parser configurations.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorParserType {
    Css,     // Basic CSS color parser
    Full,    // Full-featured parser with all collections
    Custom,  // Custom parser with user configuration
}
```

**Rationale**: Provides type-safe factory pattern for creating different parser implementations with varying capabilities.

### ColorParserConfig

Configuration for custom color parsers with feature flags.

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
```

**Rationale**: Builder-like configuration allowing selective feature enablement. All options are boolean flags for simplicity.

**Default Implementation**: Provides sensible defaults enabling most features.

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
```

**Rationale**: Provides fine-grained control over search operations across color collections.

**Default Implementation**: Enables all collections with fuzzy, case-insensitive matching.

## Error Types

### ColorError

Comprehensive error enumeration covering all failure modes.

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

**Rationale**: Uses `thiserror` for automatic `Error` trait implementation and formatted error messages. Provides specific error categories for different failure modes.

**Invariants**: 
- Each variant includes context-appropriate error message
- `IoError` automatically converts from `std::io::Error` via `#[from]`
- All error messages are human-readable and actionable

### Result Type

Type alias for convenient error handling throughout the codebase.

```rust
pub type Result<T> = std::result::Result<T, ColorError>;
```

**Rationale**: Reduces boilerplate and provides consistent error handling across the API.

## Type Safety and Invariants

### Range Constraints

The type system enforces several important constraints:

1. **RGB Values**: `u8` type automatically constrains to 0-255 range
2. **Percentages**: `u8` type constrains positions to 0-100 (validated semantically)
3. **Alpha Values**: `f64` allows 0.0-1.0 range (validated in constructors)

### Validation Patterns

```rust
// Type-level constraints
pub struct GradientArgs {
    pub start_position: u8,  // 0-100 by type
    pub end_position: u8,    // 0-100 by type
}

// Runtime validation for logical constraints
impl GradientArgs {
    pub fn validate(&self) -> Result<()> {
        if self.start_position >= self.end_position {
            return Err(ColorError::InvalidArguments(
                "Start position must be less than end position".to_string()
            ));
        }
        // ... additional validations
    }
}
```

### Immutability Patterns

Most types use owned data (`String`, `Vec`) rather than borrowed data to avoid lifetime complications:

```rust
// Preferred: Owned data
pub struct ColorMatch {
    pub color: UniversalColor,  // Owned
    pub distance: f64,
    pub collection_type: String, // Owned
}

// Avoided: Borrowed data would require lifetimes
pub struct ColorMatch<'a> {
    pub color: &'a UniversalColor,  // Would complicate API
    pub collection_type: &'a str,
}
```

## Constructor Patterns

### Factory Methods

Prefer factory methods over direct construction for complex types:

```rust
impl ColorProcessor {
    /// Factory method ensuring consistent formatting
    pub fn create_color_info(label: String, lab: Lab) -> ColorInfo {
        // Ensures all fields are consistently formatted
    }
}

impl GradientBuilder {
    /// Builder pattern for complex configuration
    pub fn new() -> Self { /* ... */ }
    pub fn start_color<S: AsRef<str>>(self, color: S) -> Self { /* ... */ }
    pub fn build(self) -> Result<GradientArgs> { /* ... */ }
}
```

### Direct Field Access

Allow direct field access for simple data types where invariants are enforced by the type system:

```rust
// Safe for direct access - u8 constrains range
let args = GradientArgs {
    start_position: 20,  // Type system ensures 0-255, semantically 0-100
    end_position: 80,
    // ...
};
```

### Custom Constructors

Provide convenience constructors for common use cases:

```rust
impl ParsedColor {
    pub fn new(r: u8, g: u8, b: u8, a: f64, format: ColorFormat) -> Self;
    
    /// Convenience constructor for opaque colors
    pub fn from_rgb(r: u8, g: u8, b: u8, format: ColorFormat) -> Self {
        Self::new(r, g, b, 1.0, format)
    }
}
```

## Conversion Traits

### From/Into Implementations

The codebase primarily uses the `palette` library's conversion traits rather than implementing custom `From`/`Into` conversions:

```rust
use palette::{FromColor, IntoColor, Lab, Srgb, Hsl};

// Using palette's conversion traits
let rgb = Srgb::new(1.0, 0.0, 0.0);
let lab: Lab = Lab::from_color(rgb);
let hsl: Hsl = lab.into_color();
```

### Custom Conversions

Custom conversions are implemented where additional logic is needed:

```rust
impl ColorProcessor {
    /// Custom conversion with validation and formatting
    pub fn lab_to_hex(lab: Lab) -> String {
        let rgb: Srgb = lab.into_color();
        let r = (rgb.red * 255.0).round() as u8;
        let g = (rgb.green * 255.0).round() as u8;
        let b = (rgb.blue * 255.0).round() as u8;
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}
```

### Conversion Guidelines

1. **Use `palette` conversions** for standard color space conversions
2. **Implement custom conversions** when additional formatting or validation is needed
3. **Prefer explicit conversion methods** over trait implementations for clarity
4. **Avoid lossy conversions** without explicit acknowledgment in method names

## Design Principles

### Type-Driven Design

The type system is designed to:

1. **Prevent Invalid States**: Use appropriate types to make invalid states unrepresentable
2. **Encode Business Rules**: Constraints like position ordering are validated at appropriate layers
3. **Minimize Runtime Errors**: Prefer compile-time safety where possible
4. **Provide Clear APIs**: Types should be self-documenting through their design

### Performance Considerations

1. **Owned Data**: Simplifies lifetimes at the cost of some memory usage
2. **Copy Types**: Simple enums implement `Copy` for efficient passing
3. **String Formatting**: Pre-formatted strings in `ColorInfo` optimize display performance
4. **Validation Separation**: Type-level constraints for performance, runtime validation for correctness

### Error Handling Strategy

1. **Early Validation**: Validate inputs as early as possible in the pipeline
2. **Contextual Errors**: Provide specific error types with actionable messages
3. **Graceful Degradation**: Where possible, continue operation with partial results
4. **Type Safety**: Use `Result` types consistently rather than panicking
