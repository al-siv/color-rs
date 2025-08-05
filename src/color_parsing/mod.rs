//! Color parsing module
//!
//! This module has been decomposed into focused submodules following functional
//! programming principles and single responsibility design.
//!
//! ## Submodule Organization
//! - `parsers` - Individual parser implementations (CSS, Full, Custom)
//! - `pipeline` - Processing pipeline functions (preprocessing/postprocessing)
//! - `utilities` - Configuration and convenience functions
//!
//! ## Main Functions
//! - `parse_color()` - Main parsing interface with configuration
//! - `get_color_name()` - Get color name with configuration
//! - `parse_color_fast()` - Quick CSS-only parsing
//! - `parse_color_comprehensive()` - Full feature parsing
//!
//! ## Example Usage
//! ```rust
//! use color_rs::color_parsing;
//!
//! // Quick parsing
//! let (lab, format) = color_parsing::parse_color_fast("#FF0000")?;
//! 
//! // Comprehensive parsing with configuration
//! let config = color_parsing::comprehensive_parsing_config();
//! let (lab, format) = color_parsing::parse_color("red", &config)?;
//! 
//! // Get color names
//! let name = color_parsing::get_color_name_comprehensive([255, 0, 0]);
//! ```

pub mod parsers;
pub mod pipeline;
pub mod utilities;

// Re-export main types and functions for backward compatibility
pub use utilities::{
    ParserType,
    ParsingConfig,
    ParserCapabilities,
    parse_color,
    get_color_name,
    get_parser_capabilities,
    fast_parsing_config,
    comprehensive_parsing_config,
    strict_parsing_config,
    parse_color_fast,
    parse_color_comprehensive,
    parse_color_strict,
    get_color_name_fast,
    get_color_name_comprehensive,
    get_color_name_strict,
    AVAILABLE_PARSER_TYPES,
};

pub use pipeline::{
    PreprocessingStep,
    PostprocessingStep,
    apply_preprocessing_pipeline,
    apply_postprocessing_pipeline,
};

pub use parsers::{
    parse_css_color,
    parse_full_color,
    parse_custom_color,
    get_css_color_name,
    get_full_color_name,
    get_custom_color_name,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_configurations() {
        // Test fast configuration
        let fast_config = fast_parsing_config();
        assert!(matches!(fast_config.parser_type, ParserType::Css));
        assert_eq!(fast_config.preprocessing.len(), 1);
        
        // Test comprehensive configuration
        let comp_config = comprehensive_parsing_config();
        assert!(matches!(comp_config.parser_type, ParserType::Full));
        assert!(comp_config.enable_fallback_naming);
        
        // Test strict configuration
        let strict_config = strict_parsing_config();
        assert!(matches!(strict_config.parser_type, ParserType::Custom { strict_validation: true }));
        assert!(!strict_config.enable_fallback_naming);
    }

    #[test]
    fn test_parser_capabilities() {
        let css_config = ParsingConfig::new(ParserType::Css);
        let capabilities = get_parser_capabilities(&css_config);
        
        assert!(matches!(capabilities.parser_type, ParserType::Css));
        assert_eq!(capabilities.collection_count, 1);
        assert!(capabilities.supported_formats.contains(&"HEX".to_string()));
    }

    #[test]
    fn test_convenience_functions() {
        // Test that convenience functions work
        let result = parse_color_fast("#FF0000");
        assert!(result.is_ok());
        
        let name = get_color_name_fast([255, 0, 0]);
        assert!(!name.is_empty());
    }

    #[test]
    fn test_configuration_builder() {
        let config = ParsingConfig::new(ParserType::Css)
            .with_tolerance(20.0)
            .with_fallback_naming(false)
            .with_preprocessing(PreprocessingStep::Trim)
            .with_postprocessing(PostprocessingStep::Capitalize);
            
        assert_eq!(config.color_tolerance, 20.0);
        assert!(!config.enable_fallback_naming);
        assert_eq!(config.preprocessing.len(), 1);
        assert_eq!(config.postprocessing.len(), 1);
    }

    #[test] 
    fn test_module_re_exports() {
        // Verify all expected functions are accessible
        let _config = fast_parsing_config();
        let _caps = get_parser_capabilities(&_config);
        let _name = get_color_name_fast([128, 128, 128]);
        
        // Test pipeline functions
        let input = "  test  ";
        let processed = apply_preprocessing_pipeline(input, &[PreprocessingStep::Trim]);
        assert_eq!(processed, "test");
    }
}
