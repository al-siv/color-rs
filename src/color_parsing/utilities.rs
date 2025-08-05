//! Parsing utilities and configuration
//!
//! Provides configuration structures, preset functions, and convenience utilities
//! for color parsing operations. Uses functional composition for flexible configuration.

use crate::color_parser::ColorFormat;
use crate::error::Result;
use super::pipeline::{PreprocessingStep, PostprocessingStep, apply_preprocessing_pipeline, apply_postprocessing_pipeline};
use super::parsers::{parse_css_color, parse_full_color, parse_custom_color, get_css_color_name, get_full_color_name, get_custom_color_name};

/// Parser type configuration using enum dispatch (replaces trait objects)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParserType {
    /// Basic CSS color parser - fast, minimal collections
    Css,
    /// Comprehensive parser with all collections (CSS + RAL Classic + RAL Design)
    Full,
    /// Custom parser with configurable validation rules
    Custom { strict_validation: bool },
}

/// Parsing configuration using functional composition
#[derive(Debug, Clone)]
pub struct ParsingConfig {
    /// Parser type determines parsing strategy
    pub parser_type: ParserType,
    /// Enable fallback naming for unrecognized colors
    pub enable_fallback_naming: bool,
    /// Color tolerance for matching (0.0-100.0)
    pub color_tolerance: f64,
    /// Preprocessing functions pipeline
    pub preprocessing: Vec<PreprocessingStep>,
    /// Post-processing functions pipeline  
    pub postprocessing: Vec<PostprocessingStep>,
}

/// Parser capabilities information
#[derive(Debug, Clone)]
pub struct ParserCapabilities {
    /// Parser type
    pub parser_type: ParserType,
    /// Supported color formats
    pub supported_formats: Vec<String>,
    /// Number of color collections
    pub collection_count: usize,
    /// Total number of named colors
    pub color_count: usize,
    /// Processing pipeline complexity
    pub processing_pipeline_length: usize,
}

impl ParsingConfig {
    /// Create new parsing configuration with specified parser type
    pub fn new(parser_type: ParserType) -> Self {
        Self {
            parser_type,
            enable_fallback_naming: true,
            color_tolerance: 10.0,
            preprocessing: Vec::new(),
            postprocessing: Vec::new(),
        }
    }

    /// Add preprocessing step to pipeline
    pub fn with_preprocessing(mut self, step: PreprocessingStep) -> Self {
        self.preprocessing.push(step);
        self
    }

    /// Add postprocessing step to pipeline
    pub fn with_postprocessing(mut self, step: PostprocessingStep) -> Self {
        self.postprocessing.push(step);
        self
    }

    /// Set color tolerance
    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.color_tolerance = tolerance;
        self
    }

    /// Set fallback naming behavior
    pub fn with_fallback_naming(mut self, enable: bool) -> Self {
        self.enable_fallback_naming = enable;
        self
    }
}

/// Main functional parsing interface - replaces factory methods
pub fn parse_color(
    input: &str,
    config: &ParsingConfig,
) -> Result<(palette::Lab, ColorFormat)> {
    // Apply preprocessing pipeline
    let processed_input = apply_preprocessing_pipeline(input, &config.preprocessing);
    
    // Parse based on parser type using pattern matching (not trait dispatch)
    match &config.parser_type {
        ParserType::Css => parse_css_color(&processed_input),
        ParserType::Full => parse_full_color(&processed_input),
        ParserType::Custom { strict_validation } => {
            parse_custom_color(&processed_input, *strict_validation)
        }
    }
}

/// Get color name using functional approach - replaces trait method
pub fn get_color_name(
    rgb: [u8; 3],
    config: &ParsingConfig,
) -> String {
    let name = match &config.parser_type {
        ParserType::Css => get_css_color_name(rgb),
        ParserType::Full => get_full_color_name(rgb, config.color_tolerance),
        ParserType::Custom { .. } => get_custom_color_name(rgb, config.enable_fallback_naming),
    };

    // Apply post-processing pipeline
    apply_postprocessing_pipeline(name, &config.postprocessing)
}

/// Get parser capabilities - pure function instead of trait method
pub fn get_parser_capabilities(config: &ParsingConfig) -> ParserCapabilities {
    let (supported_formats, collection_count, color_count) = match &config.parser_type {
        ParserType::Css => (
            vec!["HEX", "RGB", "RGBA", "HSL", "HSLA", "Named"]
                .into_iter()
                .map(String::from)
                .collect(),
            1,
            147,
        ),
        ParserType::Full => (
            vec![
                "HEX", "RGB", "RGBA", "HSL", "HSLA", "Named", 
                "RAL Classic", "RAL Design"
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            3,
            1500,
        ),
        ParserType::Custom { .. } => (
            vec!["HEX", "RGB", "Named"]
                .into_iter()
                .map(String::from)
                .collect(),
            1,
            147,
        ),
    };

    ParserCapabilities {
        parser_type: config.parser_type.clone(),
        supported_formats,
        collection_count,
        color_count,
        processing_pipeline_length: config.preprocessing.len() + config.postprocessing.len(),
    }
}

/// Fast parsing configuration (CSS only)
pub fn fast_parsing_config() -> ParsingConfig {
    ParsingConfig::new(ParserType::Css)
        .with_preprocessing(PreprocessingStep::Trim)
}

/// Comprehensive parsing configuration (all collections)
pub fn comprehensive_parsing_config() -> ParsingConfig {
    ParsingConfig::new(ParserType::Full)
        .with_tolerance(15.0)
        .with_fallback_naming(true)
        .with_preprocessing(PreprocessingStep::Trim)
        .with_preprocessing(PreprocessingStep::Normalize)
        .with_postprocessing(PostprocessingStep::TitleCase)
}

/// Strict parsing configuration (validation focused)
pub fn strict_parsing_config() -> ParsingConfig {
    ParsingConfig::new(ParserType::Custom { strict_validation: true })
        .with_tolerance(5.0)
        .with_fallback_naming(false)
        .with_preprocessing(PreprocessingStep::Trim)
        .with_preprocessing(PreprocessingStep::Normalize)
        .with_preprocessing(PreprocessingStep::RemoveSpecialChars)
}

/// Available parser types - compile-time constant
pub const AVAILABLE_PARSER_TYPES: &[ParserType] = &[
    ParserType::Css,
    ParserType::Full,
    ParserType::Custom { strict_validation: false },
];

/// Convenience functions - functional equivalents of factory presets
/// Parse color using fast configuration (CSS only)
pub fn parse_color_fast(input: &str) -> Result<(palette::Lab, ColorFormat)> {
    parse_color(input, &fast_parsing_config())
}

/// Parse color using comprehensive configuration (all collections)
pub fn parse_color_comprehensive(input: &str) -> Result<(palette::Lab, ColorFormat)> {
    parse_color(input, &comprehensive_parsing_config())
}

/// Parse color using strict configuration (validation focused)
pub fn parse_color_strict(input: &str) -> Result<(palette::Lab, ColorFormat)> {
    parse_color(input, &strict_parsing_config())
}

/// Get color name using fast configuration
pub fn get_color_name_fast(rgb: [u8; 3]) -> String {
    get_color_name(rgb, &fast_parsing_config())
}

/// Get color name using comprehensive configuration
pub fn get_color_name_comprehensive(rgb: [u8; 3]) -> String {
    get_color_name(rgb, &comprehensive_parsing_config())
}

/// Get color name using strict configuration
pub fn get_color_name_strict(rgb: [u8; 3]) -> String {
    get_color_name(rgb, &strict_parsing_config())
}
