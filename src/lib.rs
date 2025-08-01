//! Color-rs: Color gradient calculations using LAB color space
//!
//! This library provides tools for generating perceptually uniform color gradients
//! with CSS cubic-bezier easing functions. It supports multiple output formats
//! including console tables, SVG, and PNG.

pub mod cli;
pub mod color;
pub mod color_distance_strategies;
pub mod color_formatter;
// Template Method Pattern Migration (Milestone 1.2) - Functional Replacement
pub mod color_matching_functional;
// Facade Pattern Migration (Milestone 2.2) - Functional Replacement
pub mod color_ops;
pub mod color_parser;
// Factory Pattern Migration (Milestone 1.3) - Functional Replacement
pub mod color_parser_functional;
pub mod color_schemes;
// Backward Compatibility Layer (Milestone 3.1)
pub mod compat;
pub mod config;
pub mod error;
pub mod file_output;
pub mod format_utils;
pub mod gradient;
// Builder Pattern Optimization (Milestone 2.1) - Functional Replacement
pub mod gradient_functional;
pub mod image;
pub mod output_formats;
pub mod precision_utils;
pub mod utils;

// New GoF Pattern Implementations
// Command Pattern Migration (Milestone 1.4) - Functional Replacement
pub mod command_functional;
pub mod parsing_chain;

// Re-export main types for convenience
pub use cli::{Cli, ColorArgs, Commands, GradientArgs};
pub use color::{ColorInfo, ColorSpace};
pub use color_distance_strategies::{
    DistanceAlgorithm, calculate_distance, available_strategies,
    // Smart constructors and validation
    ValidatedLab, ValidationError, calculate_distance_validated,
    // Lens/optics for functional field access
    LabLens, LightnessLens, ALens, BLens
};
// Template Method Pattern Migration (Milestone 1.2) - Functional Replacement
pub use color_matching_functional::{
    CollectionType, MatchingConfig, 
    match_color_functional, match_color_by_type, match_across_all_collections,
    validate_lab_basic, validate_ral_classic, validate_ral_design,
    post_process_ral_design, extract_hue_from_code
};
// Facade Pattern Migration (Milestone 2.2) - Functional Replacement
pub use color_ops::{
    // Core functions
    wcag_relative, perceived_brightness, relative_luminance,
    delta_e_2000, perceptual_distance, find_closest, delta_e_cie76, delta_e_cie94,
    wcag_ratio, meets_aa_standard, meets_aaa_standard, compliance_level, ratio,
    srgb_to_hsl, srgb_to_lab, srgb_to_lch, srgb_to_hsv,
    hex_to_srgb, srgb_to_hex, rgb_tuple_to_srgb, srgb_to_rgb_tuple,
    analyze_color, compare_colors, ColorAnalysis, ColorComparison,
    ColorProperties, ColorSpaces, PerceptualData, AccessibilityData,
    mix, lab_interpolation, lch_interpolation, linear_rgb,
    create_palette, weighted_mix,
    multiply_blend, screen_blend, overlay_blend,
    // Module access for organized operations
    luminance, distance, contrast, conversion, analysis, mixing
};
// Import ColorSpace with alias to avoid conflict
pub use color_ops::mixing::ColorSpace as MixingColorSpace;
pub use color_parser::{ColorMatch, SearchFilter, UnifiedColorManager, UniversalColor};
// Factory Pattern Migration (Milestone 1.3) - Functional Replacement
pub use color_parser_functional::{
    ParserType, ParsingConfig, ParserCapabilities,
    PreprocessingStep, PostprocessingStep,
    parse_color_functional, get_color_name_functional, get_parser_capabilities,
    fast_parsing_config, comprehensive_parsing_config, strict_parsing_config,
    parse_color_fast, parse_color_comprehensive, parse_color_strict,
    get_color_name_fast, get_color_name_comprehensive, get_color_name_strict,
    AVAILABLE_PARSER_TYPES
};
pub use color_schemes::{
    ColorSchemeBuilder, ColorSchemeCalculator, ColorSchemeResult, ColorSchemeStrategy,
};
// Command Pattern Migration (Milestone 1.4) - Functional Replacement
pub use command_functional::{
    CommandType, ExecutionContext, ExecutionResult,
    PreHookStep, PostHookStep,
    execute_command_functional, get_command_name, get_command_description, supports_undo,
    create_gradient_command, create_analyze_command, create_find_closest_command, create_convert_command,
    execute_command_simple, execute_command_with_validation, execute_command_enhanced,
    AVAILABLE_COMMAND_TYPES
};
pub use error::{ColorError, Result};
pub use gradient::{GradientCalculator, GradientValue};
// Builder Pattern Optimization (Milestone 2.1) - Functional Replacement
pub use gradient_functional::{
    GradientConfig, ColorPair, EasingConfig, PositionRange, ImageOutput, StopConfig, FileOutput,
    GradientValidationError, linear_gradient, smooth_gradient, positioned_gradient
};

pub use image::{ImageFormat, ImageGenerator};
pub use utils::Utils;

/// Main library interface for generating gradients
pub struct ColorRs;

impl ColorRs {
    /// Create a new instance of the color-rs library
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Generate a gradient based on the provided arguments
    pub fn generate_gradient(&self, args: GradientArgs) -> Result<()> {
        // Use functional gradient system (Milestone 2.1b)
        gradient_functional::generate_gradient_functional(args)
    }

    /// Match and convert color between different color spaces  
    pub fn color_match(&self, args: ColorArgs) -> Result<String> {
        let algorithm = crate::color_distance_strategies::DistanceAlgorithm::from_str_or_default(&args.distance_method);

        // Always use enhanced color matching with schemes (new default behavior)
        color::color_match_with_schemes(&args, algorithm)
    }
}

impl Default for ColorRs {
    fn default() -> Self {
        Self::new()
    }
}
