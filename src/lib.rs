//! Color-rs: Color gradient calculations using LAB color space
//!
//! This library provides tools for generating perceptually uniform color gradients
//! with CSS cubic-bezier easing functions. It supports multiple output formats
//! including console tables, SVG, and PNG.

pub mod cli;
pub mod clock;
pub mod color;
pub mod color_distance_strategies;
pub mod color_formatter;
pub mod color_report_formatting;
// Template Method Pattern Migration (Milestone 1.2) - Modern Alternative
pub mod color_matching;
// Facade Pattern Migration (Milestone 2.2) - Modern Alternative
pub mod color_ops;
pub mod color_parser;
// Factory Pattern Migration (Milestone 1.3) - Modern Alternative
pub mod color_parsing;
pub mod color_schemes;
// Scheme Configuration - Functional builder patterns for color schemes
pub mod scheme_config;
// Backward Compatibility Layer (Milestone 3.1)
pub mod compat;
pub mod config;
pub mod error;
pub mod file_output;
pub mod format_utils;
pub mod gradient;
// Gradient Configuration - Functional gradient building patterns
pub mod gradient_config;
pub mod image;
pub mod output_formats;
// Performance validation for Milestone 7.2
pub mod performance_validation;
pub mod precision_utils;
pub mod utils;

// Functional Programming Modules
// Command Execution - Functional command processing patterns
pub mod command_execution;
pub mod parsing_chain;

// Re-export main types for convenience
pub use cli::{Cli, ColorArgs, Commands, GradientArgs};
pub use color::{ColorInfo, ColorSpace};
pub use color_distance_strategies::{
    // Core types from functional enum dispatch
    DistanceAlgorithm, ValidatedLab, ValidationError,
    // Main legacy compatibility function
    calculate_distance,
    // Smart constructors and validation
    SmartConstructors, ColorSource, ValidationConstraints,
    // Lens/optics for immutable field access
    LabLens, LightnessLens, ALens, BLens,
    // Algorithm utilities
    filter_fast_algorithms, filter_perceptual_algorithms, recommend_algorithm,
    // Batch operations
    BatchValidator,
    // Legacy compatibility
    array_to_validated_lab, validated_lab_to_array,
};
// Color Matching - Functional pattern matching across collections
pub use color_matching::{
    CollectionType, MatchingConfig, 
    match_color, match_color_by_type, match_across_all_collections,
    validate_lab_basic, validate_ral_classic, validate_ral_design,
    post_process_ral_design, extract_hue_from_code
};
// Color Operations - Comprehensive functional color operations
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
// Factory Pattern Migration (Milestone 1.3) - Modern Alternative
pub use color_parsing::{
    ParserType, ParsingConfig, ParserCapabilities,
    PreprocessingStep, PostprocessingStep,
    parse_color, get_color_name, get_parser_capabilities,
    fast_parsing_config, comprehensive_parsing_config, strict_parsing_config,
    parse_color_fast, parse_color_comprehensive, parse_color_strict,
    get_color_name_fast, get_color_name_comprehensive, get_color_name_strict,
    AVAILABLE_PARSER_TYPES
};
pub use color_schemes::{
    ColorSchemeBuilder, ColorSchemeCalculator, ColorSchemeResult, ColorSchemeStrategy,
    HslColorSchemeStrategy, LabColorSchemeStrategy,
    adjust_color_relative_luminance, adjust_color_lab_luminance,
    preserve_wcag_relative_luminance, preserve_lab_luminance,
};
// Scheme Configuration - Functional builder pattern for color schemes
pub use scheme_config::{
    ColorSchemeConfig, ConfigError, ColorSchemeCalculator as ModernColorSchemeCalculator,
    LuminanceConfig, calculate_color_schemes, 
    standard, with_relative_luminance_preservation, with_lab_luminance_preservation,
    with_target_relative_luminance, with_target_lab_luminance, complex_config
};
// Command Execution - Functional command processing and validation
pub use command_execution::{
    CommandType, ExecutionContext, ExecutionResult,
    PreHookStep, PostHookStep,
    execute_command, get_command_name, get_command_description, supports_undo,
    create_gradient_command, create_analyze_command, create_find_closest_command, create_convert_command,
    execute_command_simple, execute_command_with_validation, execute_command_enhanced,
    AVAILABLE_COMMAND_TYPES
};
pub use error::{ColorError, Result};
pub use gradient::{GradientCalculator, GradientValue};
// Gradient Configuration - Functional gradient configuration builders
pub use gradient_config::{
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
    /// Generate gradient with specified arguments
    /// 
    /// # Errors
    /// Returns error if gradient generation fails due to invalid parameters,
    /// file system errors, or color processing issues.
    pub fn generate_gradient(&self, args: GradientArgs) -> Result<()> {
        // Use gradient configuration system (Milestone 2.1b)
        gradient_config::generate_gradient(args)
    }

    /// Match and convert color between different color spaces  
    /// Match and analyze colors with specified arguments
    /// 
    /// # Errors
    /// Returns error if color matching fails due to invalid input,
    /// color parsing issues, or analysis computation problems.
    pub fn color_match(&self, args: &ColorArgs) -> Result<String> {
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
