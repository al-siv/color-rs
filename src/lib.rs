//! Color-rs: Color gradient calculations using LAB color space
//!
//! This library provides tools for generating perceptually uniform color gradients
//! with CSS cubic-bezier easing functions. It supports multiple output formats
//! including console tables, SVG, and PNG.

pub mod cli;
pub use cli_range::Range; // Re-export Range for backward compatibility (tests import color_rs::cli::Range)
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
pub mod config;
pub mod error;
pub mod file_output;
pub mod format_utils;
pub mod gradient;
// Gradient Configuration - Functional gradient building patterns
pub mod gradient_config;
pub mod image;
pub mod image_core; // Pure image/SVG construction utilities (functional core)
pub mod logger;
pub mod output_formats;
pub mod output_formats_gradient; // Extracted gradient output models (Milestone 4 size refactor)
pub mod precision_utils;
pub mod serialization; // Centralized structured serialization helpers (Milestone 4)
pub mod utils;
// Internal developer analysis utilities (non-production) for size & function length gates

// Functional Programming Modules
// Command Execution - Functional command processing patterns
pub mod command_execution;
pub mod parsing_chain;

// Re-export main types for convenience
pub use cli::{Cli, ColorArgs, Commands, GradientArgs, HueArgs};
pub use color::{ColorInfo, ColorSpace};
pub use color_distance_strategies::{
    ALens,
    BLens,
    // Batch operations
    BatchValidator,
    ColorSource,
    // Core types from functional enum dispatch
    DistanceAlgorithm,
    // Lens/optics for immutable field access
    LabLens,
    LightnessLens,
    // Smart constructors and validation
    SmartConstructors,
    ValidatedLab,
    ValidationConstraints,
    ValidationError,
    // Legacy compatibility
    array_to_validated_lab,
    // Main legacy compatibility function
    calculate_distance,
    // Algorithm utilities
    filter_fast_algorithms,
    filter_perceptual_algorithms,
    recommend_algorithm,
    validated_lab_to_array,
};
pub use color_ops::analysis::hue::{
    ColorCollectionType, HueAnalysisOptions, HueAnalysisResult, SortCriteria,
};
// Color Matching - Functional pattern matching across collections
pub use color_matching::{
    CollectionType, MatchingConfig, extract_hue_from_code, match_across_all_collections,
    match_color, match_color_by_type, post_process_ral_design, validate_lab_basic,
    validate_ral_classic, validate_ral_design,
};
// Color Operations - Comprehensive functional color operations
pub use color_ops::{
    AccessibilityData,
    ColorAnalysis,
    ColorComparison,
    ColorProperties,
    ColorSpaces,
    PerceptualData,
    analysis,
    analyze_color,
    compare_colors,
    compliance_level,
    contrast,
    conversion,
    create_palette,
    delta_e_2000,
    delta_e_cie76,
    delta_e_cie94,
    distance,
    find_closest,
    hex_to_srgb,
    lab_interpolation,
    lch_interpolation,
    linear_rgb,
    // Module access for organized operations
    luminance,
    meets_aa_standard,
    meets_aaa_standard,
    mix,
    mixing,
    multiply_blend,
    overlay_blend,
    perceived_brightness,
    perceptual_distance,
    ratio,
    relative_luminance,
    rgb_tuple_to_srgb,
    screen_blend,
    srgb_to_hex,
    srgb_to_hsl,
    srgb_to_hsv,
    srgb_to_lab,
    srgb_to_lch,
    srgb_to_rgb_tuple,
    wcag_ratio,
    // Core functions
    wcag_relative,
    weighted_mix,
};
// Import ColorSpace with alias to avoid conflict
pub use color_ops::mixing::ColorSpace as MixingColorSpace;
pub use color_parser::{ColorMatch, SearchFilter, UnifiedColorManager, UniversalColor};
// Factory Pattern Migration (Milestone 1.3) - Modern Alternative
pub use color_parsing::{
    AVAILABLE_PARSER_TYPES, ParserCapabilities, ParserType, ParsingConfig, PostprocessingStep,
    PreprocessingStep, comprehensive_parsing_config, fast_parsing_config, get_color_name,
    get_color_name_comprehensive, get_color_name_fast, get_color_name_strict,
    get_parser_capabilities, parse_color, parse_color_comprehensive, parse_color_fast,
    parse_color_strict, strict_parsing_config,
};
pub use color_schemes::{
    ColorSchemeBuilder, ColorSchemeCalculator, ColorSchemeResult, ColorSchemeStrategy,
    HslColorSchemeStrategy, LabColorSchemeStrategy, adjust_color_lab_luminance,
    adjust_color_relative_luminance, preserve_lab_luminance, preserve_wcag_relative_luminance,
};
// Scheme Configuration - Functional builder pattern for color schemes
pub use scheme_config::{
    ColorSchemeCalculator as ModernColorSchemeCalculator, ColorSchemeConfig, ConfigError,
    LuminanceConfig, calculate_color_schemes, complex_config, standard,
    with_lab_luminance_preservation, with_relative_luminance_preservation,
    with_target_lab_luminance, with_target_relative_luminance,
};
// Command Execution - Functional command processing and validation
pub use command_execution::{
    AVAILABLE_COMMAND_TYPES, CommandType, ExecutionContext, ExecutionResult, PostHookStep,
    PreHookStep, create_analyze_command, create_convert_command, create_find_closest_command,
    create_gradient_command, execute_command, execute_command_enhanced, execute_command_simple,
    execute_command_with_validation, get_command_description, get_command_name, supports_undo,
};
pub use error::{ColorError, Result};
pub use gradient::{GradientCalculator, GradientValue};
// Gradient Configuration - Functional gradient configuration builders
pub use gradient_config::{
    ColorPair, EasingConfig, FileOutput, GradientConfig, GradientValidationError, ImageOutput,
    PositionRange, StopConfig, linear_gradient, positioned_gradient, smooth_gradient,
};

pub use image::{ImageFormat, ImageGenerator};
pub use utils::Utils;

/// Main library interface for color analysis, gradient generation, and color space conversions
///
/// `ColorRs` provides a comprehensive API for working with colors, including:
/// - Color matching across multiple color collections (CSS, RAL Classic, RAL Design)
/// - Gradient generation with various color space interpolations
/// - Hue analysis and color harmony pattern discovery
/// - Perceptually accurate color distance calculations using LAB Delta E
/// - Color space conversions between RGB, HSL, HSV, LAB, LCH, and more
///
/// The library emphasizes functional programming patterns, immutability, and robust error handling.
/// All color operations use scientifically accurate algorithms for professional-grade results.
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use color_rs::ColorRs;
///
/// let color_rs = ColorRs::new();
/// // Now you can use color_rs for various color operations
/// ```
///
/// ## Color Matching
/// ```rust
/// use color_rs::{ColorRs, cli::ColorArgs};
///
/// let color_rs = ColorRs::new();
/// let args = ColorArgs {
///     color: "#FF6B35".to_string(),
///     distance_method: "lab".to_string(),
///     scheme_strategy: "lab".to_string(),
///     relative_luminance: None,
///     luminance: None,
///     output_format: None,
///     output_file: None,
///     func_filter: None,
/// };
///
/// let matches = color_rs.color_match(&args)?;
/// # Ok::<(), color_rs::error::ColorError>(())
/// ```
///
/// ## Gradient Generation
/// ```rust
/// use color_rs::{ColorRs, cli::GradientArgs};
///
/// let color_rs = ColorRs::new();
/// let args = GradientArgs {
///     start_color: "#FF0000".to_string(),
///     end_color: "#0000FF".to_string(),
///     start_position: 0,
///     end_position: 100,
///     ease_in: 0.4,
///     ease_out: 0.6,
///     svg: None,
///     png: None,
///     vectorized_text: true,
///     no_legend: false,
///     width: 1000,
///     step: None,
///     stops: 5,
///     stops_simple: false,
///     output_format: None,
///     output_file: Some("gradient.svg".to_string()),
///     func_filter: None,
/// };
///
/// color_rs.generate_gradient(args)?;
/// # Ok::<(), color_rs::error::ColorError>(())
/// ```
///
/// ## Hue Analysis
/// ```rust
/// use color_rs::{ColorRs, cli::HueArgs};
///
/// let color_rs = ColorRs::new();
/// let args = HueArgs {
///     collection: "css".to_string(),
///     hue_range: Some("[0...60]".to_string()), // Warm colors
///     lightness_range: None,
///     chroma_range: None,
///     grad: false,
///     pal: false, // Changed to false to avoid requiring SVG filename
///     svg: None,
///     png: None,
///     vectorized_text: true,
///     width: 1000,
///     no_labels: false,
///     output_format: None,
///     output_file: None,
///     color_height: None,
///     font_size: 12,
///     border_width: 0, // No borders for analysis-only mode
///     border_color: "white".to_string(),
///     header_text: None,
/// };
///
/// color_rs.analyze_hue(&args)?;
/// # Ok::<(), color_rs::error::ColorError>(())
/// ```
pub struct ColorRs;

impl ColorRs {
    /// Create a new instance of the color-rs library
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Generate a gradient image from specified color arguments
    ///
    /// Creates a color gradient by interpolating between multiple colors in the specified
    /// color space (sRGB, LAB, or LCH). The gradient can be rendered in various formats
    /// including SVG, PNG, and text-based representations.
    ///
    /// # Arguments
    /// * `args` - Gradient generation arguments including colors, output format, and visual options
    ///
    /// # Returns
    /// `Ok(())` on successful gradient generation, or an error if generation fails
    ///
    /// # Errors
    /// Returns error if:
    /// - Input colors are invalid or cannot be parsed
    /// - Insufficient colors provided for gradient (minimum 2 required)
    /// - Color space conversion fails
    /// - File I/O operations fail during output generation
    /// - Image rendering fails
    ///
    /// # Examples
    /// ```rust
    /// use color_rs::{ColorRs, cli::GradientArgs};
    ///
    /// let color_rs = ColorRs::new();
    /// let args = GradientArgs {
    ///     start_color: "#FF0000".to_string(),
    ///     end_color: "#0000FF".to_string(),
    ///     start_position: 0,
    ///     end_position: 100,
    ///     ease_in: 0.4,
    ///     ease_out: 0.6,
    ///     svg: None,
    ///     png: None,
    ///     vectorized_text: true,
    ///     no_legend: false,
    ///     width: 800,
    ///     step: None,
    ///     stops: 5,
    ///     stops_simple: false,
    ///     output_format: None,
    ///     output_file: Some("gradient.svg".to_string()),
    ///     func_filter: None,
    /// };
    ///
    /// color_rs.generate_gradient(args)?;
    /// # Ok::<(), color_rs::error::ColorError>(())
    /// ```
    pub fn generate_gradient(&self, args: GradientArgs) -> Result<()> {
        // Use gradient configuration system (Milestone 2.1b)
        gradient_config::generate_gradient(args)
    }

    /// Find the closest matching colors from color collections
    ///
    /// Searches through specified color collections (CSS colors, RAL Classic, RAL Design)
    /// to find colors that most closely match the input color. Uses perceptually accurate
    /// LAB Delta E color distance calculations for precise matching.
    ///
    /// # Arguments
    /// * `args` - Color matching arguments including target color, collections, and output options
    ///
    /// # Returns
    /// A formatted string containing the matching results, or an error if matching fails
    ///
    /// # Errors
    /// Returns error if:
    /// - Input color cannot be parsed or is invalid
    /// - Color collection files cannot be loaded
    /// - Color space conversions fail
    /// - Distance calculations encounter mathematical errors
    ///
    /// # Examples
    /// ```rust
    /// use color_rs::{ColorRs, cli::ColorArgs};
    ///
    /// let color_rs = ColorRs::new();
    /// let args = ColorArgs {
    ///     color: "#FF5733".to_string(),
    ///     distance_method: "lab".to_string(),
    ///     scheme_strategy: "lab".to_string(),
    ///     relative_luminance: None,
    ///     luminance: None,
    ///     output_format: None,
    ///     output_file: None,
    ///     func_filter: None,
    /// };
    ///
    /// let matches = color_rs.color_match(&args)?;
    /// println!("{}", matches);
    /// # Ok::<(), color_rs::error::ColorError>(())
    /// ```
    pub fn color_match(&self, args: &ColorArgs) -> Result<String> {
        let algorithm = crate::color_distance_strategies::DistanceAlgorithm::from_str_or_default(
            &args.distance_method,
        );

        // Always use enhanced color matching with schemes (new default behavior)
        color::color_match_with_schemes(args, algorithm)
    }

    /// Analyze hue relationships and color harmony patterns
    ///
    /// Performs comprehensive hue analysis on color collections, finding colors within
    /// specified hue ranges and applying various filtering criteria. Supports color harmony
    /// theory analysis, hue distribution studies, and visual palette generation.
    ///
    /// # Arguments
    /// * `args` - Hue analysis arguments including target hue, tolerance, filtering criteria, and output options
    ///
    /// # Returns
    /// `Ok(())` on successful analysis completion, or an error if analysis fails
    ///
    /// # Errors
    /// Returns error if:
    /// - Invalid hue range specifications (must be 0-360 degrees)
    /// - Color collection loading fails
    /// - Filter criteria are invalid or contradictory
    /// - Mathematical calculations encounter errors
    /// - File output operations fail
    ///
    /// # Examples
    /// ```rust
    /// use color_rs::{ColorRs, cli::HueArgs};
    ///
    /// let color_rs = ColorRs::new();
    /// let args = HueArgs {
    ///     collection: "css".to_string(),
    ///     hue_range: Some("[120...180]".to_string()), // Green to cyan spectrum
    ///     lightness_range: Some("[50...80]".to_string()),
    ///     chroma_range: Some("[30...70]".to_string()),
    ///     grad: false,
    ///     pal: false, // Changed to false to avoid requiring SVG filename
    ///     svg: None,
    ///     png: None,
    ///     vectorized_text: true,
    ///     width: 1000,
    ///     no_labels: false,
    ///     output_format: None,
    ///     output_file: Some("hue_analysis.yaml".to_string()),
    ///     color_height: None,
    ///     font_size: 12,
    ///     border_width: 0, // No borders for analysis-only mode
    ///     border_color: "white".to_string(),
    ///     header_text: None,
    /// };
    ///
    /// color_rs.analyze_hue(&args)?;
    /// # Ok::<(), color_rs::error::ColorError>(())
    /// ```
    pub fn analyze_hue(&self, args: &HueArgs) -> Result<()> {
        // Validate arguments first
        args.validate()?;

        let result = command_execution::execute_hue_analysis(args, None)?;
        println!("{}", result.output);
        Ok(())
    }
}

impl Default for ColorRs {
    fn default() -> Self {
        Self::new()
    }
}

pub mod cli_range; // Extracted range utilities (Milestone 4 Phase 4.1)
