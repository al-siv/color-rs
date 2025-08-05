//! Gradient Configuration Types
//!
//! This module defines all the core types used for gradient configuration,
//! including color pairs, easing configurations, position ranges, and output formats.

use crate::cli::OutputFormat;
use crate::error::ColorError;

/// Immutable gradient configuration
///
/// Unlike the traditional Builder pattern, this configuration is immutable
/// and uses smart constructors to ensure validity at creation time.
#[derive(Debug, Clone, PartialEq)]
pub struct GradientConfig {
    pub(crate) colors: ColorPair,
    pub(crate) easing: EasingConfig,
    pub(crate) position_range: PositionRange,
    pub(crate) image_output: ImageOutput,
    pub(crate) stop_config: StopConfig,
    pub(crate) file_output: Option<FileOutput>,
}

/// Validated color pair for gradient endpoints
#[derive(Debug, Clone, PartialEq)]
pub struct ColorPair {
    pub(crate) start: String,
    pub(crate) end: String,
}

/// Easing configuration using preset configurations
#[derive(Debug, Clone, PartialEq)]
pub struct EasingConfig {
    pub(crate) ease_in: f64,
    pub(crate) ease_out: f64,
}

/// Position range for gradient
#[derive(Debug, Clone, PartialEq)]
pub struct PositionRange {
    pub(crate) start: u8,
    pub(crate) end: u8,
}

/// Image output configuration
#[derive(Debug, Clone, PartialEq)]
pub struct ImageOutput {
    pub(crate) svg_filename: Option<String>,
    pub(crate) png_filename: Option<String>,
    pub(crate) width: u32,
    pub(crate) show_legend: bool,
}

/// Stop configuration for gradient calculation
#[derive(Debug, Clone, PartialEq)]
pub enum StopConfig {
    /// Fixed number of steps
    Steps(u8),
    /// Intelligent stops with specified count
    IntelligentStops(usize),
    /// Equal distribution with specified count
    EqualStops(usize),
}

/// File output configuration
#[derive(Debug, Clone, PartialEq)]
pub struct FileOutput {
    pub(crate) format: OutputFormat,
    pub(crate) filename: String,
}

/// Gradient validation errors
#[derive(Debug, Clone, PartialEq)]
pub enum GradientValidationError {
    InvalidColorFormat(String),
    InvalidEasingValue(f64),
    InvalidPositionRange(u8, u8),
    InvalidStepValue(u8),
    InvalidWidth(u32),
    EmptyFilename,
}

impl std::fmt::Display for GradientValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GradientValidationError::InvalidColorFormat(msg) => write!(f, "Invalid color format: {}", msg),
            GradientValidationError::InvalidEasingValue(val) => write!(f, "Invalid easing value: {}. Must be between 0.0 and 1.0", val),
            GradientValidationError::InvalidPositionRange(start, end) => write!(f, "Invalid position range: start={}, end={}. Start must be less than end and both must be 0-100", start, end),
            GradientValidationError::InvalidStepValue(step) => write!(f, "Invalid step value: {}. Must be greater than 0", step),
            GradientValidationError::InvalidWidth(width) => write!(f, "Invalid width: {}. Must be greater than 0", width),
            GradientValidationError::EmptyFilename => write!(f, "Filename cannot be empty"),
        }
    }
}

impl std::error::Error for GradientValidationError {}

impl From<GradientValidationError> for ColorError {
    fn from(error: GradientValidationError) -> Self {
        ColorError::InvalidArguments(error.to_string())
    }
}
