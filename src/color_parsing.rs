//! Color Parser - Zero-Cost Abstraction for Color Parsing
//!
//! This module provides enum-based color parsing with zero-cost abstractions.
//! Uses enum dispatch and pure functions for optimal performance.

use crate::color_parser::{ColorParser, UnifiedColorManager, ColorFormat};
use crate::error::Result;

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

/// Preprocessing step function type
pub type PreprocessingFn = fn(&str) -> String;

/// Post-processing step function type
pub type PostprocessingFn = fn(String) -> String;

/// Preprocessing steps using function composition
#[derive(Debug, Clone)]
pub enum PreprocessingStep {
    /// Normalize whitespace and case
    Normalize,
    /// Trim whitespace
    Trim,
    /// Convert to lowercase
    Lowercase,
    /// Remove special characters
    RemoveSpecialChars,
    /// Custom preprocessing function
    Custom(PreprocessingFn),
}

/// Post-processing steps using function composition
#[derive(Debug, Clone)]
pub enum PostprocessingStep {
    /// Capitalize first letter
    Capitalize,
    /// Convert to title case
    TitleCase,
    /// Add prefix
    AddPrefix(String),
    /// Add suffix
    AddSuffix(String),
    /// Custom post-processing function
    Custom(PostprocessingFn),
}

/// Parser capabilities information
#[derive(Debug, Clone)]
pub struct ParserCapabilities {
    pub parser_type: ParserType,
    pub supported_formats: Vec<String>,
    pub collection_count: usize,
    pub color_count: usize,
    pub processing_pipeline_length: usize,
}

impl Default for ParsingConfig {
    fn default() -> Self {
        Self {
            parser_type: ParserType::Full,
            enable_fallback_naming: true,
            color_tolerance: 10.0,
            preprocessing: vec![PreprocessingStep::Trim, PreprocessingStep::Normalize],
            postprocessing: vec![],
        }
    }
}

impl ParsingConfig {
    /// Builder pattern for creating configurations
    pub fn new(parser_type: ParserType) -> Self {
        Self {
            parser_type,
            ..Default::default()
        }
    }

    /// Enable fallback naming
    pub fn with_fallback_naming(mut self, enable: bool) -> Self {
        self.enable_fallback_naming = enable;
        self
    }

    /// Set color tolerance
    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.color_tolerance = tolerance.clamp(0.0, 100.0);
        self
    }

    /// Add preprocessing step
    pub fn with_preprocessing(mut self, step: PreprocessingStep) -> Self {
        self.preprocessing.push(step);
        self
    }

    /// Add post-processing step
    pub fn with_postprocessing(mut self, step: PostprocessingStep) -> Self {
        self.postprocessing.push(step);
        self
    }

    /// Enable strict validation for custom parsers
    pub fn with_strict_validation(mut self) -> Self {
        if let ParserType::Custom { .. } = self.parser_type {
            self.parser_type = ParserType::Custom { strict_validation: true };
        }
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
    let result = match &config.parser_type {
        ParserType::Css => parse_css_color(&processed_input),
        ParserType::Full => parse_full_color(&processed_input),
        ParserType::Custom { strict_validation } => {
            parse_custom_color(&processed_input, *strict_validation)
        }
    };

    result
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

// Preset configurations - pure functions instead of factory methods

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

// Internal parsing functions using pattern matching instead of virtual dispatch

fn parse_css_color(input: &str) -> Result<(palette::Lab, ColorFormat)> {
    let parser = ColorParser::new();
    parser.parse(input)
}

fn parse_full_color(input: &str) -> Result<(palette::Lab, ColorFormat)> {
    let parser = ColorParser::new();
    parser.parse(input)
}

fn parse_custom_color(input: &str, strict_validation: bool) -> Result<(palette::Lab, ColorFormat)> {
    let parser = ColorParser::new();
    
    if strict_validation {
        // Could add additional validation logic here
        parser.parse(input)
    } else {
        parser.parse(input)
    }
}

fn get_css_color_name(rgb: [u8; 3]) -> String {
    let parser = ColorParser::new();
    parser.get_color_name((rgb[0], rgb[1], rgb[2]))
}

fn get_full_color_name(rgb: [u8; 3], _tolerance: f64) -> String {
    // Use unified manager for comprehensive color matching
    if let Ok(unified_manager) = UnifiedColorManager::new() {
        let matches = unified_manager.find_closest_across_all(rgb, 1);
        
        // Find the best match across all collections
        for (_, collection_matches) in &matches {
            if let Some(color_match) = collection_matches.first() {
                return color_match.entry.metadata.name.clone();
            }
        }
    }

    // Fallback to basic parser
    get_css_color_name(rgb)
}

fn get_custom_color_name(rgb: [u8; 3], enable_fallback: bool) -> String {
    let name = get_css_color_name(rgb);
    
    if !enable_fallback && name.starts_with("rgb(") {
        "Unknown".to_string()
    } else {
        name
    }
}

// Preprocessing pipeline functions using function composition

fn apply_preprocessing_pipeline(input: &str, steps: &[PreprocessingStep]) -> String {
    steps.iter().fold(input.to_string(), |acc, step| {
        apply_preprocessing_step(&acc, step)
    })
}

fn apply_preprocessing_step(input: &str, step: &PreprocessingStep) -> String {
    match step {
        PreprocessingStep::Normalize => normalize_input(input),
        PreprocessingStep::Trim => input.trim().to_string(),
        PreprocessingStep::Lowercase => input.to_lowercase(),
        PreprocessingStep::RemoveSpecialChars => remove_special_chars(input),
        PreprocessingStep::Custom(func) => func(input),
    }
}

fn normalize_input(input: &str) -> String {
    input.chars()
        .map(|c| if c.is_whitespace() { ' ' } else { c })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

fn remove_special_chars(input: &str) -> String {
    input.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '(' || *c == ')' || *c == ',' || *c == '#')
        .collect()
}

// Post-processing pipeline functions using function composition

fn apply_postprocessing_pipeline(input: String, steps: &[PostprocessingStep]) -> String {
    steps.iter().fold(input, |acc, step| {
        apply_postprocessing_step(acc, step)
    })
}

fn apply_postprocessing_step(input: String, step: &PostprocessingStep) -> String {
    match step {
        PostprocessingStep::Capitalize => capitalize_first(&input),
        PostprocessingStep::TitleCase => to_title_case(&input),
        PostprocessingStep::AddPrefix(prefix) => format!("{}{}", prefix, input),
        PostprocessingStep::AddSuffix(suffix) => format!("{}{}", input, suffix),
        PostprocessingStep::Custom(func) => func(input),
    }
}

fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn to_title_case(input: &str) -> String {
    input.split_whitespace()
        .map(capitalize_first)
        .collect::<Vec<String>>()
        .join(" ")
}

// Convenience functions - functional equivalents of factory presets

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional_css_parsing() {
        let config = ParsingConfig::new(ParserType::Css);
        let result = parse_color("#ff0000", &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_functional_full_parsing() {
        let config = ParsingConfig::new(ParserType::Full);
        let result = parse_color("red", &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_capabilities() {
        let css_config = ParsingConfig::new(ParserType::Css);
        let capabilities = get_parser_capabilities(&css_config);
        assert_eq!(capabilities.collection_count, 1);
        assert!(capabilities.supported_formats.contains(&"HEX".to_string()));
    }

    #[test]
    fn test_preprocessing_pipeline() {
        let config = ParsingConfig::new(ParserType::Css)
            .with_preprocessing(PreprocessingStep::Trim)
            .with_preprocessing(PreprocessingStep::Lowercase);
        
        let result = parse_color("  #FF0000  ", &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_postprocessing_pipeline() {
        let config = ParsingConfig::new(ParserType::Css)
            .with_postprocessing(PostprocessingStep::TitleCase);
        
        let name = get_color_name([255, 0, 0], &config);
        assert!(!name.is_empty());
    }

    #[test]
    fn test_preset_configurations() {
        // Test fast configuration
        let result = parse_color_fast("#00ff00");
        assert!(result.is_ok());

        // Test comprehensive configuration  
        let result = parse_color_comprehensive("blue");
        assert!(result.is_ok());

        // Test strict configuration
        let result = parse_color_strict("#0000ff");
        assert!(result.is_ok());
    }

    #[test]
    fn test_color_name_functions() {
        let red_rgb = [255, 0, 0];
        
        let fast_name = get_color_name_fast(red_rgb);
        let comprehensive_name = get_color_name_comprehensive(red_rgb);
        let strict_name = get_color_name_strict(red_rgb);
        
        assert!(!fast_name.is_empty());
        assert!(!comprehensive_name.is_empty());
        assert!(!strict_name.is_empty());
    }

    #[test]
    fn test_parser_type_enum_dispatch() {
        // Test that each parser type works without virtual dispatch
        let css_config = ParsingConfig::new(ParserType::Css);
        let full_config = ParsingConfig::new(ParserType::Full);
        let custom_config = ParsingConfig::new(ParserType::Custom { strict_validation: true });

        assert!(parse_color("#ff0000", &css_config).is_ok());
        assert!(parse_color("#00ff00", &full_config).is_ok());
        assert!(parse_color("#0000ff", &custom_config).is_ok());
    }

    #[test]
    fn test_function_composition_preprocessing() {
        let input = "  RGB(255, 0, 0)  ";
        let steps = vec![
            PreprocessingStep::Trim,
            PreprocessingStep::Lowercase,
        ];
        
        let result = apply_preprocessing_pipeline(input, &steps);
        assert_eq!(result, "rgb(255, 0, 0)");
    }

    #[test]
    fn test_function_composition_postprocessing() {
        let input = "red color".to_string();
        let steps = vec![
            PostprocessingStep::TitleCase,
            PostprocessingStep::AddSuffix(" Paint".to_string()),
        ];
        
        let result = apply_postprocessing_pipeline(input, &steps);
        assert_eq!(result, "Red Color Paint");
    }
}
