//! Color-rs: Professional color gradient calculations using LAB color space
//!
//! This library provides tools for generating perceptually uniform color gradients
//! with CSS cubic-bezier easing functions. It supports multiple output formats
//! including console tables, SVG, and PNG.

pub mod cli;
pub mod color;
pub mod color_distance_strategies;
pub mod color_formatter;
pub mod color_matching_template;
pub mod color_operations_facade;
pub mod color_parser;
pub mod color_parser_factory;
pub mod color_schemes;
pub mod color_utils;
pub mod config;
pub mod error;
pub mod gradient;
pub mod gradient_builder;
pub mod image;

// New GoF Pattern Implementations
pub mod command_pattern;
pub mod formatter_strategies;
pub mod parsing_chain;

// Re-export main types for convenience
pub use cli::{Cli, ColorMatchArgs, Commands, GradientArgs};
pub use color::{ColorInfo, ColorSpace};
pub use color_distance_strategies::{ColorDistanceStrategy, available_strategies, create_strategy};
pub use color_matching_template::{ColorMatchingTemplate, UnifiedColorMatcher};
pub use color_operations_facade::{ColorAnalysis, ColorOperationsFacade};
pub use color_parser::{ColorMatch, SearchFilter, UnifiedColorManager, UniversalColor};
pub use color_parser_factory::{
    ColorParserConfig, ColorParserFactory, ColorParserTrait, ColorParserType,
};
pub use color_schemes::{
    ColorSchemeBuilder, ColorSchemeCalculator, ColorSchemeResult, ColorSchemeStrategy,
};
pub use color_utils::ColorUtils;
pub use error::{ColorError, Result};
pub use gradient::{GradientCalculator, GradientValue};
pub use gradient_builder::GradientBuilder;
pub use image::{ImageFormat, ImageGenerator};

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
        let strategy = crate::color_distance_strategies::create_strategy(&args.distance_method);

        // Always use enhanced color matching with schemes (new default behavior)
        color::color_match_with_schemes(&args, strategy.as_ref())
    }
}

impl Default for ColorRs {
    fn default() -> Self {
        Self::new()
    }
}
