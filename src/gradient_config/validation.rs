//! Gradient Configuration Validation
//!
//! This module contains all the smart constructors and validation logic
//! for gradient configuration types.

use super::types::*;
use crate::config::bezier_presets;

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
    /// Create new easing configuration with validation
    ///
    /// # Errors
    /// Returns `GradientValidationError` if values are not in [0.0, 1.0] range
    pub fn new(ease_in: f64, ease_out: f64) -> std::result::Result<Self, GradientValidationError> {
        if !(0.0..=1.0).contains(&ease_in) {
            return Err(GradientValidationError::InvalidEasingValue(ease_in));
        }
        if !(0.0..=1.0).contains(&ease_out) {
            return Err(GradientValidationError::InvalidEasingValue(ease_out));
        }

        Ok(Self { ease_in, ease_out })
    }

    /// Linear easing (no acceleration)
    pub fn linear() -> Self {
        Self {
            ease_in: 0.0,
            ease_out: 1.0,
        }
    }

    /// Ease preset
    pub fn ease() -> Self {
        Self {
            ease_in: bezier_presets::EASE.0,
            ease_out: bezier_presets::EASE.1,
        }
    }

    /// Ease-in preset
    pub fn ease_in() -> Self {
        Self {
            ease_in: bezier_presets::EASE_IN.0,
            ease_out: bezier_presets::EASE_IN.1,
        }
    }

    /// Ease-out preset
    pub fn ease_out() -> Self {
        Self {
            ease_in: bezier_presets::EASE_OUT.0,
            ease_out: bezier_presets::EASE_OUT.1,
        }
    }

    /// Ease-in-out preset (smooth curve)
    pub fn ease_in_out() -> Self {
        Self {
            ease_in: bezier_presets::EASE_IN_OUT.0,
            ease_out: bezier_presets::EASE_IN_OUT.1,
        }
    }

    /// Default easing configuration
    pub fn default_config() -> Self {
        Self::linear()
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
    /// Create new position range with validation
    ///
    /// # Errors
    /// Returns `GradientValidationError` if range is invalid
    pub fn new(start: u8, end: u8) -> std::result::Result<Self, GradientValidationError> {
        if start >= end {
            return Err(GradientValidationError::InvalidPositionRange(start, end));
        }
        if end > 100 {
            return Err(GradientValidationError::InvalidPositionRange(start, end));
        }

        Ok(Self { start, end })
    }

    /// Default position range (0-100)
    pub fn full_range() -> Self {
        Self { start: 0, end: 100 }
    }

    /// Center position range (25-75)
    pub fn center_range() -> Self {
        Self { start: 25, end: 75 }
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
    /// Create SVG output configuration
    ///
    /// # Errors
    /// Returns `GradientValidationError` if filename is empty or width is 0
    pub fn svg(
        filename: &str,
        width: u32,
        show_legend: bool,
    ) -> std::result::Result<Self, GradientValidationError> {
        if filename.trim().is_empty() {
            return Err(GradientValidationError::EmptyFilename);
        }
        if width == 0 {
            return Err(GradientValidationError::InvalidWidth(width));
        }

        Ok(Self {
            svg_filename: Some(filename.to_string()),
            png_filename: None,
            width,
            show_legend,
        })
    }

    /// Create PNG output configuration
    ///
    /// # Errors
    /// Returns `GradientValidationError` if filename is empty or width is 0
    pub fn png(
        filename: &str,
        width: u32,
        show_legend: bool,
    ) -> std::result::Result<Self, GradientValidationError> {
        if filename.trim().is_empty() {
            return Err(GradientValidationError::EmptyFilename);
        }
        if width == 0 {
            return Err(GradientValidationError::InvalidWidth(width));
        }

        Ok(Self {
            svg_filename: None,
            png_filename: Some(filename.to_string()),
            width,
            show_legend,
        })
    }

    /// Create both SVG and PNG output configuration
    ///
    /// # Errors
    /// Returns `GradientValidationError` if filenames are empty or width is 0
    pub fn both(
        svg_filename: &str,
        png_filename: &str,
        width: u32,
        show_legend: bool,
    ) -> std::result::Result<Self, GradientValidationError> {
        if svg_filename.trim().is_empty() || png_filename.trim().is_empty() {
            return Err(GradientValidationError::EmptyFilename);
        }
        if width == 0 {
            return Err(GradientValidationError::InvalidWidth(width));
        }

        Ok(Self {
            svg_filename: Some(svg_filename.to_string()),
            png_filename: Some(png_filename.to_string()),
            width,
            show_legend,
        })
    }

    /// No image output
    pub fn none() -> Self {
        Self {
            svg_filename: None,
            png_filename: None,
            width: 800, // Default width
            show_legend: true,
        }
    }

    /// Default image output configuration
    pub fn default_config() -> Self {
        Self::none()
    }

    /// Check if SVG output is configured
    pub fn has_svg(&self) -> bool {
        self.svg_filename.is_some()
    }

    /// Check if PNG output is configured
    pub fn has_png(&self) -> bool {
        self.png_filename.is_some()
    }

    /// Get SVG filename
    pub fn svg_filename(&self) -> Option<&str> {
        self.svg_filename.as_deref()
    }

    /// Get PNG filename
    pub fn png_filename(&self) -> Option<&str> {
        self.png_filename.as_deref()
    }

    /// Get image width
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
    pub fn new(
        format: crate::cli::OutputFormat,
        filename: &str,
    ) -> std::result::Result<Self, GradientValidationError> {
        if filename.trim().is_empty() {
            return Err(GradientValidationError::EmptyFilename);
        }

        Ok(Self {
            format,
            filename: filename.to_string(),
        })
    }

    /// Get the output format
    pub fn format(&self) -> &crate::cli::OutputFormat {
        &self.format
    }

    /// Get the filename
    pub fn filename(&self) -> &str {
        &self.filename
    }
}
