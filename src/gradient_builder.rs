//! Gradient Builder Pattern implementation
//!
//! This module provides a fluent Builder interface for creating gradient configurations,
//! allowing for more flexible and readable gradient creation.

use crate::cli::GradientArgs;
use crate::config::*;
use crate::error::{ColorError, Result};

/// Builder for gradient configuration using the Builder pattern
///
/// This provides a fluent interface for creating gradient configurations
/// without requiring all parameters upfront.
///
/// # Example
/// ```rust
/// use color_rs::GradientBuilder;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let gradient = GradientBuilder::new()
///     .start_color("#FF0000")
///     .end_color("#0000FF")
///     .ease_in(0.42)
///     .ease_out(0.58)
///     .steps(10)
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct GradientBuilder {
    start_color: Option<String>,
    end_color: Option<String>,
    start_position: u8,
    end_position: u8,
    ease_in: f64,
    ease_out: f64,
    svg: bool,
    png: bool,
    no_legend: bool,
    width: u32,
    svg_name: String,
    png_name: String,
    step: Option<u8>,
    stops: usize,
    stops_simple: bool,
    output_format: Option<crate::cli::OutputFormat>,
    output_file: Option<String>,
}

impl Default for GradientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl GradientBuilder {
    /// Create a new gradient builder with default values
    pub fn new() -> Self {
        Self {
            start_color: None,
            end_color: None,
            start_position: 0,
            end_position: 100,
            ease_in: 0.65,
            ease_out: 0.35,
            svg: false,
            png: false,
            no_legend: false,
            width: 1000,
            svg_name: DEFAULT_SVG_NAME.to_string(),
            png_name: DEFAULT_PNG_NAME.to_string(),
            step: None,
            stops: 5,
            stops_simple: false,
            output_format: None,
            output_file: None,
        }
    }

    /// Set the starting color
    pub fn start_color<S: Into<String>>(mut self, color: S) -> Self {
        self.start_color = Some(color.into());
        self
    }

    /// Set the ending color
    pub fn end_color<S: Into<String>>(mut self, color: S) -> Self {
        self.end_color = Some(color.into());
        self
    }

    /// Set the start position as percentage (0-100)
    pub fn start_position(mut self, position: u8) -> Self {
        self.start_position = position;
        self
    }

    /// Set the end position as percentage (0-100)
    pub fn end_position(mut self, position: u8) -> Self {
        self.end_position = position;
        self
    }

    /// Set the ease-in control point (0.0-1.0)
    pub fn ease_in(mut self, ease: f64) -> Self {
        self.ease_in = ease;
        self
    }

    /// Set the ease-out control point (0.0-1.0)
    pub fn ease_out(mut self, ease: f64) -> Self {
        self.ease_out = ease;
        self
    }

    /// Set both ease-in and ease-out to the same value
    pub fn ease(mut self, ease: f64) -> Self {
        self.ease_in = ease;
        self.ease_out = ease;
        self
    }

    /// Configure for linear easing (no acceleration/deceleration)
    pub fn linear(mut self) -> Self {
        self.ease_in = 0.0;
        self.ease_out = 1.0;
        self
    }

    /// Configure for ease-in (slow start, fast end)
    pub fn ease_in_preset(mut self) -> Self {
        self.ease_in = 0.42;
        self.ease_out = 1.0;
        self
    }

    /// Configure for ease-out (fast start, slow end)
    pub fn ease_out_preset(mut self) -> Self {
        self.ease_in = 0.0;
        self.ease_out = 0.58;
        self
    }

    /// Configure for ease-in-out (slow start and end, fast middle)
    pub fn ease_in_out(mut self) -> Self {
        self.ease_in = 0.42;
        self.ease_out = 0.58;
        self
    }

    /// Enable SVG output
    pub fn svg(mut self) -> Self {
        self.svg = true;
        self
    }

    /// Enable PNG output
    pub fn png(mut self) -> Self {
        self.png = true;
        self
    }

    /// Enable both SVG and PNG output
    pub fn images(mut self) -> Self {
        self.svg = true;
        self.png = true;
        self
    }

    /// Disable legend on generated images
    pub fn no_legend(mut self) -> Self {
        self.no_legend = true;
        self
    }

    /// Set image width in pixels
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Set SVG filename
    pub fn svg_filename<S: Into<String>>(mut self, name: S) -> Self {
        self.svg_name = name.into();
        self
    }

    /// Set PNG filename
    pub fn png_filename<S: Into<String>>(mut self, name: S) -> Self {
        self.png_name = name.into();
        self
    }

    /// Set gradient step percentage (conflicts with stops methods)
    pub fn steps(mut self, step: u8) -> Self {
        self.step = Some(step);
        self.stops = 5; // Reset to default
        self.stops_simple = false;
        self
    }

    /// Set number of intelligent gradient stops (conflicts with other stop methods)  
    pub fn intelligent_stops(mut self, stops: usize) -> Self {
        self.stops = stops;
        self.step = None;
        self.stops_simple = false;
        self
    }

    /// Set number of equally spaced stops (conflicts with other stop methods)
    pub fn equal_stops(mut self, stops: usize) -> Self {
        self.stops = stops;
        self.step = None;
        self.stops_simple = true;
        self
    }

    /// Set output format and file
    pub fn output(mut self, format: crate::cli::OutputFormat, filename: String) -> Self {
        self.output_format = Some(format);
        self.output_file = Some(filename);
        self
    }

    /// Set output file (format will default to TOML)
    pub fn output_file<S: Into<String>>(mut self, filename: S) -> Self {
        self.output_file = Some(filename.into());
        if self.output_format.is_none() {
            self.output_format = Some(crate::cli::OutputFormat::Toml);
        }
        self
    }

    /// Validate the current configuration
    pub fn validate(&self) -> Result<()> {
        // Check required fields
        if self.start_color.is_none() {
            return Err(ColorError::InvalidArguments(
                "Start color is required".to_string(),
            ));
        }
        if self.end_color.is_none() {
            return Err(ColorError::InvalidArguments(
                "End color is required".to_string(),
            ));
        }

        // Validate position bounds
        if self.start_position > MAX_PERCENTAGE || self.end_position > MAX_PERCENTAGE {
            return Err(ColorError::InvalidArguments(
                "Positions must be between 0 and 100".to_string(),
            ));
        }

        // Validate position order
        if self.start_position >= self.end_position {
            return Err(ColorError::InvalidArguments(
                "Start position must be less than end position".to_string(),
            ));
        }

        // Validate ease values
        if self.ease_in < BEZIER_MIN || self.ease_in > BEZIER_MAX {
            return Err(ColorError::InvalidArguments(
                "Ease-in value must be between 0.0 and 1.0".to_string(),
            ));
        }

        if self.ease_out < BEZIER_MIN || self.ease_out > BEZIER_MAX {
            return Err(ColorError::InvalidArguments(
                "Ease-out value must be between 0.0 and 1.0".to_string(),
            ));
        }

        // Validate --no-legend usage
        if self.no_legend && !self.svg && !self.png {
            return Err(ColorError::InvalidArguments(
                "--no-legend can only be used with --svg or --png".to_string(),
            ));
        }

        // Validate width
        if self.width == 0 {
            return Err(ColorError::InvalidArguments(
                "Width must be greater than 0".to_string(),
            ));
        }

        // Validate step
        if let Some(step) = self.step {
            if step == 0 {
                return Err(ColorError::InvalidArguments(
                    "Gradient step must be greater than 0".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Build the gradient configuration
    ///
    /// # Errors
    /// Returns an error if required fields are missing or values are invalid
    pub fn build(self) -> Result<GradientArgs> {
        self.validate()?;

        Ok(GradientArgs {
            start_color: self.start_color.unwrap(), // Safe because validate() checks this
            end_color: self.end_color.unwrap(),     // Safe because validate() checks this
            start_position: self.start_position,
            end_position: self.end_position,
            ease_in: self.ease_in,
            ease_out: self.ease_out,
            svg: self.svg,
            png: self.png,
            no_legend: self.no_legend,
            width: self.width,
            svg_name: self.svg_name,
            png_name: self.png_name,
            step: self.step,
            stops: self.stops,
            stops_simple: self.stops_simple,
            output_format: self.output_format,
            output_file: self.output_file,
            func_filter: None, // Gradient builder doesn't support filter expressions yet
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_builder_basic() {
        let result = GradientBuilder::new()
            .start_color("#FF0000")
            .end_color("#0000FF")
            .build();

        assert!(result.is_ok());
        let args = result.unwrap();
        assert_eq!(args.start_color, "#FF0000");
        assert_eq!(args.end_color, "#0000FF");
        assert_eq!(args.ease_in, 0.65);
        assert_eq!(args.ease_out, 0.35);
    }

    #[test]
    fn test_gradient_builder_fluent_interface() {
        let result = GradientBuilder::new()
            .start_color("red")
            .end_color("blue")
            .ease_in_out()
            .svg()
            .width(500)
            .steps(10)
            .build();

        assert!(result.is_ok());
        let args = result.unwrap();
        assert_eq!(args.start_color, "red");
        assert_eq!(args.end_color, "blue");
        assert_eq!(args.ease_in, 0.42);
        assert_eq!(args.ease_out, 0.58);
        assert!(args.svg);
        assert_eq!(args.width, 500);
        assert_eq!(args.step, Some(10));
    }

    #[test]
    fn test_gradient_builder_validation_missing_colors() {
        let result = GradientBuilder::new().build();
        assert!(result.is_err());

        let result = GradientBuilder::new().start_color("#FF0000").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_gradient_builder_validation_invalid_positions() {
        let result = GradientBuilder::new()
            .start_color("#FF0000")
            .end_color("#0000FF")
            .start_position(50)
            .end_position(25)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_gradient_builder_validation_invalid_ease() {
        let result = GradientBuilder::new()
            .start_color("#FF0000")
            .end_color("#0000FF")
            .ease_in(-0.5)
            .build();
        assert!(result.is_err());

        let result = GradientBuilder::new()
            .start_color("#FF0000")
            .end_color("#0000FF")
            .ease_out(1.5)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_gradient_builder_presets() {
        let linear = GradientBuilder::new()
            .start_color("#FF0000")
            .end_color("#0000FF")
            .linear()
            .build()
            .unwrap();
        assert_eq!(linear.ease_in, 0.0);
        assert_eq!(linear.ease_out, 1.0);

        let ease_in = GradientBuilder::new()
            .start_color("#FF0000")
            .end_color("#0000FF")
            .ease_in_preset()
            .build()
            .unwrap();
        assert_eq!(ease_in.ease_in, 0.42);
        assert_eq!(ease_in.ease_out, 1.0);
    }

    #[test]
    fn test_gradient_builder_stops_conflicts() {
        // Test that setting different stop types properly clears others
        let builder = GradientBuilder::new()
            .start_color("#FF0000")
            .end_color("#0000FF")
            .steps(5)
            .intelligent_stops(10);

        let args = builder.build().unwrap();
        assert_eq!(args.stops, 10);
        assert_eq!(args.step, None); // Should be None when cleared
        assert!(!args.stops_simple);
    }
}
