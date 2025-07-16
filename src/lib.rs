//! Color-rs: Professional color gradient calculations using LAB color space
//!
//! This library provides tools for generating perceptually uniform color gradients
//! with CSS cubic-bezier easing functions. It supports multiple output formats
//! including console tables, SVG, and PNG.

pub mod cli;
pub mod color;
pub mod color_formatter;
pub mod color_parser;
pub mod color_utils;
pub mod config;
pub mod error;
pub mod gradient;
pub mod image;
pub mod utils;

// Re-export main types for convenience
pub use cli::{Cli, ColorMatchArgs, Commands, GradientArgs};
pub use color::{ColorInfo, ColorSpace};
pub use color_parser::{ColorMatch, SearchFilter, UnifiedColorManager, UniversalColor};
pub use color_utils::ColorUtils;
pub use error::{ColorError, Result};
pub use gradient::{GradientCalculator, GradientValue};
pub use image::{ImageFormat, ImageGenerator};

/// Current version of the color-rs library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Formatting constant: width for formatted columns in the output
pub const COLUMN_WIDTH: usize = 30;

/// Main library interface for generating gradients
pub struct ColorRs;

impl ColorRs {
    /// Create a new instance of the color-rs library
    pub fn new() -> Self {
        Self
    }

    /// Generate a gradient based on the provided arguments
    pub fn generate_gradient(&self, args: GradientArgs) -> Result<()> {
        gradient::generate_gradient(args)
    }

    /// Match and convert color between different color spaces
    pub fn color_match(&self, args: ColorMatchArgs) -> Result<String> {
        color::color_match(&args.color)
    }
}

impl Default for ColorRs {
    fn default() -> Self {
        Self::new()
    }
}
