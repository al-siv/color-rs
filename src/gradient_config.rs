//! Gradient Configuration
//!
//! This module provides immutable alternatives to the traditional Builder pattern.
//! It uses smart constructors, compile-time validation, and value types to ensure
//! type safety and composable configuration.

use crate::cli::{GradientArgs, OutputFormat};
use crate::config::*;
use crate::error::{ColorError, Result};

/// Immutable gradient configuration
///
/// Unlike the traditional Builder pattern, this configuration is immutable
/// and uses smart constructors to ensure validity at creation time.
///
/// # Example
/// ```rust
/// use color_rs::gradient_config::{GradientConfig, ColorPair, EasingConfig};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let config = GradientConfig::new(
///     ColorPair::new("#FF0000", "#0000FF")?,
///     EasingConfig::ease_in_out()
/// )?
/// .with_svg_output("gradient.svg")?
/// .with_steps(10)?;
///
/// let args = config.to_gradient_args();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GradientConfig {
    colors: ColorPair,
    easing: EasingConfig,
    position_range: PositionRange,
    image_output: ImageOutput,
    stop_config: StopConfig,
    file_output: Option<FileOutput>,
}

/// Validated color pair for gradient endpoints
#[derive(Debug, Clone, PartialEq)]
pub struct ColorPair {
    start: String,
    end: String,
}

/// Easing configuration using preset configurations
#[derive(Debug, Clone, PartialEq)]
pub struct EasingConfig {
    ease_in: f64,
    ease_out: f64,
}

/// Position range with validation
#[derive(Debug, Clone, PartialEq)]
pub struct PositionRange {
    start: u8,
    end: u8,
}

/// Image output configuration
#[derive(Debug, Clone, PartialEq)]
pub struct ImageOutput {
    svg: Option<String>,
    png: Option<String>,
    width: u32,
    show_legend: bool,
}

/// Stop configuration enum for type-safe alternatives
#[derive(Debug, Clone, PartialEq)]
pub enum StopConfig {
    /// Output gradient values every X percent
    Steps(u8),
    /// Number of intelligent gradient stops
    IntelligentStops(usize),
    /// Number of equally spaced stops
    EqualStops(usize),
}

/// File output configuration
#[derive(Debug, Clone, PartialEq)]
pub struct FileOutput {
    format: OutputFormat,
    filename: String,
}

/// Validation errors for gradient configuration
#[derive(Debug, Clone, PartialEq)]
pub enum GradientValidationError {
    InvalidColorFormat(String),
    InvalidPositionRange { start: u8, end: u8 },
    InvalidEasing { value: f64, field: String },
    InvalidStepValue(u8),
    InvalidWidth(u32),
    EmptyFilename,
}

impl std::fmt::Display for GradientValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidColorFormat(color) => write!(f, "Invalid color format: {}", color),
            Self::InvalidPositionRange { start, end } => {
                write!(f, "Invalid position range: start {} >= end {}", start, end)
            }
            Self::InvalidEasing { value, field } => {
                write!(f, "Invalid {} easing value: {} (must be 0.0-1.0)", field, value)
            }
            Self::InvalidStepValue(step) => write!(f, "Invalid step value: {} (must be > 0)", step),
            Self::InvalidWidth(width) => write!(f, "Invalid width: {} (must be > 0)", width),
            Self::EmptyFilename => write!(f, "Filename cannot be empty"),
        }
    }
}

impl std::error::Error for GradientValidationError {}

impl From<GradientValidationError> for ColorError {
    fn from(error: GradientValidationError) -> Self {
        ColorError::InvalidArguments(error.to_string())
    }
}

// Smart Constructors and Validation Functions

impl ColorPair {
    /// Create a new color pair with basic validation
    ///
    /// # Errors
    /// Returns `GradientValidationError` if colors are empty
    pub fn new(start: &str, end: &str) -> std::result::Result<Self, GradientValidationError> {
        if start.trim().is_empty() {
            return Err(GradientValidationError::InvalidColorFormat(
                "Start color cannot be empty".to_string(),
            ));
        }
        if end.trim().is_empty() {
            return Err(GradientValidationError::InvalidColorFormat(
                "End color cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            start: start.to_string(),
            end: end.to_string(),
        })
    }

    /// Get the start color
    pub fn start(&self) -> &str {
        &self.start
    }

    /// Get the end color
    pub fn end(&self) -> &str {
        &self.end
    }
}

impl EasingConfig {
    /// Create a custom easing configuration
    ///
    /// # Errors
    /// Returns `GradientValidationError` if values are not in range 0.0-1.0
    pub fn new(ease_in: f64, ease_out: f64) -> std::result::Result<Self, GradientValidationError> {
        if !(BEZIER_MIN..=BEZIER_MAX).contains(&ease_in) {
            return Err(GradientValidationError::InvalidEasing {
                value: ease_in,
                field: "ease_in".to_string(),
            });
        }
        if !(BEZIER_MIN..=BEZIER_MAX).contains(&ease_out) {
            return Err(GradientValidationError::InvalidEasing {
                value: ease_out,
                field: "ease_out".to_string(),
            });
        }

        Ok(Self { ease_in, ease_out })
    }

    /// Linear easing (no acceleration/deceleration)
    pub fn linear() -> Self {
        Self {
            ease_in: 0.0,
            ease_out: 1.0,
        }
    }

    /// Ease-in (slow start, fast end)
    pub fn ease_in_preset() -> Self {
        Self {
            ease_in: 0.42,
            ease_out: 1.0,
        }
    }

    /// Ease-out (fast start, slow end)
    pub fn ease_out_preset() -> Self {
        Self {
            ease_in: 0.0,
            ease_out: 0.58,
        }
    }

    /// Ease-in-out (slow start and end, fast middle)
    pub fn ease_in_out() -> Self {
        Self {
            ease_in: 0.42,
            ease_out: 0.58,
        }
    }

    /// Default easing from config
    pub fn default_config() -> Self {
        Self {
            ease_in: 0.65,
            ease_out: 0.35,
        }
    }

    /// Get ease-in value
    pub fn ease_in_value(&self) -> f64 {
        self.ease_in
    }

    /// Get ease-out value
    pub fn ease_out_value(&self) -> f64 {
        self.ease_out
    }
}

impl PositionRange {
    /// Create a new position range with validation
    ///
    /// # Errors
    /// Returns `GradientValidationError` if start >= end or values > 100
    pub fn new(start: u8, end: u8) -> std::result::Result<Self, GradientValidationError> {
        if start > MAX_PERCENTAGE || end > MAX_PERCENTAGE {
            return Err(GradientValidationError::InvalidPositionRange { start, end });
        }
        if start >= end {
            return Err(GradientValidationError::InvalidPositionRange { start, end });
        }

        Ok(Self { start, end })
    }

    /// Default position range (0% to 100%)
    pub fn full_range() -> Self {
        Self { start: 0, end: 100 }
    }

    /// Get start position
    pub fn start(&self) -> u8 {
        self.start
    }

    /// Get end position
    pub fn end(&self) -> u8 {
        self.end
    }
}

impl ImageOutput {
    /// Create image output with no images
    pub fn none() -> Self {
        Self {
            svg: None,
            png: None,
            width: 1000,
            show_legend: true,
        }
    }

    /// Create SVG output configuration
    pub fn svg(filename: &str, width: u32, show_legend: bool) -> std::result::Result<Self, GradientValidationError> {
        if width == 0 {
            return Err(GradientValidationError::InvalidWidth(width));
        }
        if filename.trim().is_empty() {
            return Err(GradientValidationError::EmptyFilename);
        }

        Ok(Self {
            svg: Some(filename.to_string()),
            png: None,
            width,
            show_legend,
        })
    }

    /// Create PNG output configuration
    pub fn png(filename: &str, width: u32, show_legend: bool) -> std::result::Result<Self, GradientValidationError> {
        if width == 0 {
            return Err(GradientValidationError::InvalidWidth(width));
        }
        if filename.trim().is_empty() {
            return Err(GradientValidationError::EmptyFilename);
        }

        Ok(Self {
            svg: None,
            png: Some(filename.to_string()),
            width,
            show_legend,
        })
    }

    /// Create both SVG and PNG output configuration
    pub fn both(svg_name: &str, png_name: &str, width: u32, show_legend: bool) -> std::result::Result<Self, GradientValidationError> {
        if width == 0 {
            return Err(GradientValidationError::InvalidWidth(width));
        }
        if svg_name.trim().is_empty() || png_name.trim().is_empty() {
            return Err(GradientValidationError::EmptyFilename);
        }

        Ok(Self {
            svg: Some(svg_name.to_string()),
            png: Some(png_name.to_string()),
            width,
            show_legend,
        })
    }

    /// Check if SVG output is enabled
    pub fn has_svg(&self) -> bool {
        self.svg.is_some()
    }

    /// Check if PNG output is enabled
    pub fn has_png(&self) -> bool {
        self.png.is_some()
    }

    /// Get SVG filename
    pub fn svg_filename(&self) -> Option<&str> {
        self.svg.as_deref()
    }

    /// Get PNG filename
    pub fn png_filename(&self) -> Option<&str> {
        self.png.as_deref()
    }

    /// Get width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Check if legend should be shown
    pub fn show_legend(&self) -> bool {
        self.show_legend
    }
}

impl StopConfig {
    /// Create step-based configuration
    ///
    /// # Errors
    /// Returns `GradientValidationError` if step is 0
    pub fn steps(step: u8) -> std::result::Result<Self, GradientValidationError> {
        if step == 0 {
            return Err(GradientValidationError::InvalidStepValue(step));
        }
        Ok(Self::Steps(step))
    }

    /// Create intelligent stops configuration
    pub fn intelligent_stops(count: usize) -> Self {
        Self::IntelligentStops(count)
    }

    /// Create equal stops configuration
    pub fn equal_stops(count: usize) -> Self {
        Self::EqualStops(count)
    }

    /// Default configuration (5 intelligent stops)
    pub fn default_config() -> Self {
        Self::IntelligentStops(5)
    }
}

impl FileOutput {
    /// Create file output configuration
    ///
    /// # Errors
    /// Returns `GradientValidationError` if filename is empty
    pub fn new(format: OutputFormat, filename: &str) -> std::result::Result<Self, GradientValidationError> {
        if filename.trim().is_empty() {
            return Err(GradientValidationError::EmptyFilename);
        }

        Ok(Self {
            format,
            filename: filename.to_string(),
        })
    }

    /// Get format
    pub fn format(&self) -> &OutputFormat {
        &self.format
    }

    /// Get filename
    pub fn filename(&self) -> &str {
        &self.filename
    }
}

// Main Gradient Configuration Implementation

impl GradientConfig {
    /// Create a new gradient configuration with required parameters
    ///
    /// # Errors
    /// Returns `ColorError` if color pair or easing configuration is invalid
    pub fn new(
        colors: ColorPair,
        easing: EasingConfig,
    ) -> Result<Self> {
        Ok(Self {
            colors,
            easing,
            position_range: PositionRange::full_range(),
            image_output: ImageOutput::none(),
            stop_config: StopConfig::default_config(),
            file_output: None,
        })
    }

    /// Create a basic gradient configuration with defaults
    ///
    /// # Errors
    /// Returns `ColorError` if color validation fails
    pub fn basic(start_color: &str, end_color: &str) -> Result<Self> {
        let colors = ColorPair::new(start_color, end_color)?;
        let easing = EasingConfig::default_config();
        Self::new(colors, easing)
    }

    /// Update position range (immutable)
    ///
    /// # Errors
    /// Returns `ColorError` if position range is invalid
    pub fn with_position_range(self, range: PositionRange) -> Result<Self> {
        Ok(Self {
            position_range: range,
            ..self
        })
    }

    /// Update easing configuration (immutable)
    pub fn with_easing(self, easing: EasingConfig) -> Self {
        Self { easing, ..self }
    }

    /// Update stop configuration (immutable)
    pub fn with_stop_config(self, stop_config: StopConfig) -> Self {
        Self { stop_config, ..self }
    }

    /// Add SVG output (immutable)
    ///
    /// # Errors
    /// Returns `ColorError` if filename validation fails
    pub fn with_svg_output(self, filename: &str) -> Result<Self> {
        if filename.trim().is_empty() {
            return Err(GradientValidationError::EmptyFilename.into());
        }

        let mut image_output = self.image_output.clone();
        image_output.svg = Some(filename.to_string());
        
        Ok(Self { image_output, ..self })
    }

    /// Add PNG output (immutable)
    ///
    /// # Errors
    /// Returns `ColorError` if filename validation fails
    pub fn with_png_output(self, filename: &str) -> Result<Self> {
        if filename.trim().is_empty() {
            return Err(GradientValidationError::EmptyFilename.into());
        }

        let mut image_output = self.image_output.clone();
        image_output.png = Some(filename.to_string());
        
        Ok(Self { image_output, ..self })
    }

    /// Add both SVG and PNG output (immutable)
    ///
    /// # Errors
    /// Returns `ColorError` if filename validation fails
    pub fn with_both_outputs(self, svg_name: &str, png_name: &str) -> Result<Self> {
        let image_output = ImageOutput::both(svg_name, png_name, self.image_output.width, self.image_output.show_legend)?;
        Ok(Self { image_output, ..self })
    }

    /// Update image width (immutable)
    ///
    /// # Errors
    /// Returns `ColorError` if width is invalid
    pub fn with_width(self, width: u32) -> Result<Self> {
        if width == 0 {
            return Err(GradientValidationError::InvalidWidth(width).into());
        }

        let image_output = Self::update_image_width(self.image_output, width);
        Ok(Self { image_output, ..self })
    }

    /// Toggle legend display (immutable)
    pub fn with_legend(self, show_legend: bool) -> Self {
        let image_output = Self::update_image_legend(self.image_output, show_legend);
        Self { image_output, ..self }
    }

    /// Add file output (immutable)
    ///
    /// # Errors
    /// Returns `ColorError` if file output configuration is invalid
    pub fn with_file_output(self, file_output: FileOutput) -> Self {
        Self {
            file_output: Some(file_output),
            ..self
        }
    }

    /// Convenience method to add steps configuration
    ///
    /// # Errors
    /// Returns `ColorError` if step value is invalid
    pub fn with_steps(self, step: u8) -> Result<Self> {
        let stop_config = StopConfig::steps(step)?;
        Ok(self.with_stop_config(stop_config))
    }

    /// Convenience method to add intelligent stops
    pub fn with_intelligent_stops(self, count: usize) -> Self {
        let stop_config = StopConfig::intelligent_stops(count);
        self.with_stop_config(stop_config)
    }

    /// Convenience method to add equal stops
    pub fn with_equal_stops(self, count: usize) -> Self {
        let stop_config = StopConfig::equal_stops(count);
        self.with_stop_config(stop_config)
    }

    /// Helper function to update image width while preserving other settings
    fn update_image_width(mut image_output: ImageOutput, width: u32) -> ImageOutput {
        image_output.width = width;
        image_output
    }

    /// Helper function to update legend setting while preserving other settings
    fn update_image_legend(mut image_output: ImageOutput, show_legend: bool) -> ImageOutput {
        image_output.show_legend = show_legend;
        image_output
    }

    /// Convert to `GradientArgs` for CLI compatibility
    pub fn to_gradient_args(self) -> GradientArgs {
        let (step, stops, stops_simple) = match self.stop_config {
            StopConfig::Steps(s) => (Some(s), 5, false),
            StopConfig::IntelligentStops(count) => (None, count, false),
            StopConfig::EqualStops(count) => (None, count, true),
        };

        GradientArgs {
            start_color: self.colors.start,
            end_color: self.colors.end,
            start_position: self.position_range.start,
            end_position: self.position_range.end,
            ease_in: self.easing.ease_in,
            ease_out: self.easing.ease_out,
            svg: self.image_output.svg,
            png: self.image_output.png,
            no_legend: !self.image_output.show_legend,
            width: self.image_output.width,
            step,
            stops,
            stops_simple,
            output_format: self.file_output.as_ref().map(|f| f.format.clone()),
            output_file: self.file_output.map(|f| f.filename),
            func_filter: None,
        }
    }

    /// Create `GradientConfig` from CLI `GradientArgs` (CLI integration)
    pub fn from_gradient_args(args: GradientArgs) -> crate::error::Result<Self> {
        // Create color pair from CLI arguments
        let colors = ColorPair::new(&args.start_color, &args.end_color)?;

        // Create easing configuration
        let easing = EasingConfig::new(args.ease_in, args.ease_out)?;

        // Create position range
        let position_range = PositionRange::new(args.start_position, args.end_position)?;

        // Create stop configuration
        let stop_config = if let Some(step) = args.step {
            StopConfig::Steps(step)
        } else if args.stops_simple {
            StopConfig::EqualStops(args.stops)
        } else {
            StopConfig::IntelligentStops(args.stops)
        };

        // Create file output configuration
        let file_output = match (args.output_format, args.output_file) {
            (Some(format), Some(filename)) => Some(FileOutput::new(format, &filename)?),
            (Some(format), None) => {
                // Generate default filename based on format
                let default_name = match format {
                    OutputFormat::Toml => "gradient.toml",
                    OutputFormat::Yaml => "gradient.yaml",
                };
                Some(FileOutput::new(format, default_name)?)
            }
            _ => None,
        };

        // Create base configuration
        let base_config = Self::new(colors, easing)?;
        
        // Apply position range
        let positioned_config = base_config.with_position_range(position_range)?;
        
        // Apply stop configuration
        let stop_configured = positioned_config.with_stop_config(stop_config);

        // Apply image output configuration using appropriate methods
        let image_configured = match (&args.svg, &args.png) {
            (Some(svg_name), Some(png_name)) => {
                // Both formats specified
                stop_configured.with_both_outputs(svg_name, png_name)?
            }
            (Some(svg_name), None) => {
                // SVG only
                stop_configured.with_svg_output(svg_name)?
            }
            (None, Some(png_name)) => {
                // PNG only
                stop_configured.with_png_output(png_name)?
            }
            (None, None) => {
                // No image output - keep as is
                stop_configured
            }
        };

        // Apply width and legend settings
        let sized_config = image_configured.with_width(args.width)?;
        let legend_config = sized_config.with_legend(!args.no_legend);
        
        // Apply file output if specified
        let final_config = if let Some(file_out) = file_output {
            legend_config.with_file_output(file_out)
        } else {
            legend_config
        };

    Ok(final_config)
}

    /// Get color pair
    pub fn colors(&self) -> &ColorPair {
        &self.colors
    }

    /// Get easing configuration
    pub fn easing(&self) -> &EasingConfig {
        &self.easing
    }

    /// Get position range
    pub fn position_range(&self) -> &PositionRange {
        &self.position_range
    }

    /// Get image output configuration
    pub fn image_output(&self) -> &ImageOutput {
        &self.image_output
    }

    /// Get stop configuration
    pub fn stop_config(&self) -> &StopConfig {
        &self.stop_config
    }

    /// Get file output configuration
    pub fn file_output(&self) -> Option<&FileOutput> {
        self.file_output.as_ref()
    }
}

/// Convenience functions for common gradient configurations

/// Create a simple linear gradient
///
/// # Errors
/// Returns `ColorError` if color validation fails
pub fn linear_gradient(start_color: &str, end_color: &str) -> Result<GradientConfig> {
    let colors = ColorPair::new(start_color, end_color).map_err(ColorError::from)?;
    let easing = EasingConfig::linear();
    GradientConfig::new(colors, easing)
}

/// Create an ease-in-out gradient
///
/// # Errors
/// Returns `ColorError` if color validation fails
pub fn smooth_gradient(start_color: &str, end_color: &str) -> Result<GradientConfig> {
    let colors = ColorPair::new(start_color, end_color).map_err(ColorError::from)?;
    let easing = EasingConfig::ease_in_out();
    GradientConfig::new(colors, easing)
}

/// Create a gradient with custom position range
///
/// # Errors
/// Returns `ColorError` if validation fails
pub fn positioned_gradient(
    start_color: &str,
    end_color: &str,
    start_pos: u8,
    end_pos: u8,
) -> Result<GradientConfig> {
    let colors = ColorPair::new(start_color, end_color).map_err(ColorError::from)?;
    let easing = EasingConfig::default_config();
    let position_range = PositionRange::new(start_pos, end_pos).map_err(ColorError::from)?;
    
    GradientConfig::new(colors, easing)?
        .with_position_range(position_range)
}

/// Generate gradient using modern approach (Milestone 2.1b integration)
pub fn generate_gradient(args: GradientArgs) -> Result<()> {
    // Create gradient configuration from CLI arguments
    let config = GradientConfig::from_gradient_args(args)?;
    
    // TODO: Implement actual gradient generation using the gradient config
    // For now, delegate to the old system to maintain functionality
    // This will be replaced with modern implementation later
    
    // Convert back to GradientArgs and use existing generation
    let gradient_args = config.to_gradient_args();
    crate::gradient::generate_gradient(gradient_args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_pair_creation() {
        let pair = ColorPair::new("#FF0000", "#0000FF").unwrap();
        assert_eq!(pair.start(), "#FF0000");
        assert_eq!(pair.end(), "#0000FF");
    }

    #[test]
    fn test_color_pair_validation() {
        assert!(ColorPair::new("", "#0000FF").is_err());
        assert!(ColorPair::new("#FF0000", "").is_err());
        assert!(ColorPair::new("  ", "#0000FF").is_err());
    }

    #[test]
    fn test_easing_presets() {
        let linear = EasingConfig::linear();
        assert_eq!(linear.ease_in_value(), 0.0);
        assert_eq!(linear.ease_out_value(), 1.0);

        let ease_in_out = EasingConfig::ease_in_out();
        assert_eq!(ease_in_out.ease_in_value(), 0.42);
        assert_eq!(ease_in_out.ease_out_value(), 0.58);
    }

    #[test]
    fn test_easing_validation() {
        assert!(EasingConfig::new(-0.1, 0.5).is_err());
        assert!(EasingConfig::new(0.5, 1.1).is_err());
        assert!(EasingConfig::new(0.0, 1.0).is_ok());
    }

    #[test]
    fn test_position_range() {
        let range = PositionRange::new(20, 80).unwrap();
        assert_eq!(range.start(), 20);
        assert_eq!(range.end(), 80);

        // Invalid ranges
        assert!(PositionRange::new(80, 20).is_err()); // start >= end
        assert!(PositionRange::new(50, 101).is_err()); // end > 100
    }

    #[test]
    fn test_image_output() {
        let svg_output = ImageOutput::svg("test.svg", 800, true).unwrap();
        assert!(svg_output.has_svg());
        assert!(!svg_output.has_png());
        assert_eq!(svg_output.svg_filename(), Some("test.svg"));
        assert_eq!(svg_output.width(), 800);
        assert!(svg_output.show_legend());

        // Invalid width
        assert!(ImageOutput::svg("test.svg", 0, true).is_err());
        // Empty filename
        assert!(ImageOutput::svg("", 800, true).is_err());
    }

    #[test]
    fn test_stop_config() {
        let steps = StopConfig::steps(10).unwrap();
        assert!(matches!(steps, StopConfig::Steps(10)));

        let intelligent = StopConfig::intelligent_stops(7);
        assert!(matches!(intelligent, StopConfig::IntelligentStops(7)));

        // Invalid step
        assert!(StopConfig::steps(0).is_err());
    }

    #[test]
    fn test_gradient_config_basic() {
        let colors = ColorPair::new("#FF0000", "#0000FF").unwrap();
        let easing = EasingConfig::ease_in_out();
        let config = GradientConfig::new(colors, easing).unwrap();

        assert_eq!(config.colors().start(), "#FF0000");
        assert_eq!(config.colors().end(), "#0000FF");
        assert_eq!(config.easing().ease_in_value(), 0.42);
        assert_eq!(config.easing().ease_out_value(), 0.58);
    }

    #[test]
    fn test_gradient_config_immutable_updates() {
        let original = GradientConfig::basic("#FF0000", "#0000FF").unwrap();
        
        let with_svg = original.clone().with_svg_output("test.svg").unwrap();
        assert!(with_svg.image_output().has_svg());
        assert!(!original.image_output().has_svg()); // Original unchanged

        let with_steps = original.clone().with_steps(5).unwrap();
        assert!(matches!(with_steps.stop_config(), StopConfig::Steps(5)));
        assert!(matches!(original.stop_config(), StopConfig::IntelligentStops(5))); // Original unchanged
    }

    #[test]
    fn test_gradient_config_to_args() {
        let config = GradientConfig::basic("#FF0000", "#0000FF")
            .unwrap()
            .with_svg_output("test.svg")
            .unwrap()
            .with_steps(10)
            .unwrap()
            .with_width(800)
            .unwrap();

        let args = config.to_gradient_args();
        assert_eq!(args.start_color, "#FF0000");
        assert_eq!(args.end_color, "#0000FF");
        assert_eq!(args.svg, Some("test.svg".to_string()));
        assert_eq!(args.step, Some(10));
        assert_eq!(args.width, 800);
    }

    #[test]
    fn test_convenience_functions() {
        let linear = linear_gradient("#FF0000", "#0000FF").unwrap();
        assert_eq!(linear.easing().ease_in_value(), 0.0);
        assert_eq!(linear.easing().ease_out_value(), 1.0);

        let smooth = smooth_gradient("red", "blue").unwrap();
        assert_eq!(smooth.easing().ease_in_value(), 0.42);
        assert_eq!(smooth.easing().ease_out_value(), 0.58);

        let positioned = positioned_gradient("red", "blue", 20, 80).unwrap();
        assert_eq!(positioned.position_range().start(), 20);
        assert_eq!(positioned.position_range().end(), 80);
    }

    #[test]
    fn test_gradient_composition() {
        // Test that we can chain operations
        let config = GradientConfig::basic("#FF0000", "#0000FF")
            .and_then(|c| c.with_svg_output("gradient.svg"))
            .and_then(|c| c.with_steps(5))
            .and_then(|c| c.with_width(1200))
            .map(|c| c.with_legend(false))
            .unwrap();

        assert!(config.image_output().has_svg());
        assert_eq!(config.image_output().width(), 1200);
        assert!(!config.image_output().show_legend());
        assert!(matches!(config.stop_config(), StopConfig::Steps(5)));
    }

    #[test]
    fn test_error_propagation() {
        // Test that validation errors are properly propagated
        let result = GradientConfig::basic("", "#0000FF");
        assert!(result.is_err());

        let result = GradientConfig::basic("#FF0000", "#0000FF")
            .unwrap()
            .with_steps(0);
        assert!(result.is_err());

        let result = GradientConfig::basic("#FF0000", "#0000FF")
            .unwrap()
            .with_width(0);
        assert!(result.is_err());
    }
}
