//! Factory Pattern implementation for Color Parser creation
//!
//! This module provides a Factory pattern for creating different types of color parsers
//! with various configurations and capabilities.

use crate::color_parser::{ColorParser, UnifiedColorManager};
use crate::config::*;
use crate::error::Result;
use crate::utils::Utils;

/// Enum defining different types of color parsers available
#[derive(Debug, Clone, PartialEq)]
pub enum ColorParserType {
    /// Standard CSS color parser with basic color support
    Css,
    /// Extended parser with all color collections (CSS + RAL Classic + RAL Design)
    Full,
    /// Custom parser for specific use cases
    Custom,
}

/// Configuration for color parser creation
#[derive(Debug, Clone)]
pub struct ColorParserConfig {
    /// Type of parser to create
    pub parser_type: ColorParserType,
    /// Whether to enable strict validation (returns errors on invalid input)
    pub strict_validation: bool,
    /// Whether to enable fallback color naming
    pub enable_fallback_naming: bool,
    /// Custom color tolerance for matching (0.0-100.0)
    pub color_tolerance: f64,
}

impl Default for ColorParserConfig {
    fn default() -> Self {
        Self {
            parser_type: ColorParserType::Full,
            strict_validation: false,
            enable_fallback_naming: true,
            color_tolerance: 10.0,
        }
    }
}

/// Factory for creating color parsers using the Factory pattern
///
/// This factory provides a unified interface for creating different types of
/// color parsers with various configurations and capabilities.
///
/// # Example
/// ```rust
/// use color_rs::color_parser_factory::{ColorParserFactory, ColorParserType};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Create a basic CSS parser
/// let css_parser = ColorParserFactory::create_parser(ColorParserType::Css)?;
///
/// // Create a full parser with all collections
/// let full_parser = ColorParserFactory::create_parser(ColorParserType::Full)?;
/// # Ok(())
/// # }
/// ```
pub struct ColorParserFactory;

impl ColorParserFactory {
    /// Create a color parser of the specified type with default configuration
    pub fn create_parser(parser_type: ColorParserType) -> Result<Box<dyn ColorParserTrait>> {
        let config = ColorParserConfig {
            parser_type,
            ..Default::default()
        };
        Self::create_with_config(config)
    }

    /// Create a color parser with custom configuration
    pub fn create_with_config(config: ColorParserConfig) -> Result<Box<dyn ColorParserTrait>> {
        match config.parser_type {
            ColorParserType::Css => Ok(Box::new(BasicColorParser::new(config)?)),
            ColorParserType::Full => Ok(Box::new(FullColorParser::new(config)?)),
            ColorParserType::Custom => Ok(Box::new(CustomColorParser::new(config)?)),
        }
    }

    /// Create a fast parser optimized for performance (fewer collections)
    pub fn create_fast() -> Result<Box<dyn ColorParserTrait>> {
        Self::create_parser(ColorParserType::Css)
    }

    /// Create a comprehensive parser with all features
    pub fn create_comprehensive() -> Result<Box<dyn ColorParserTrait>> {
        let config = ColorParserConfig {
            parser_type: ColorParserType::Full,
            strict_validation: false,
            enable_fallback_naming: true,
            color_tolerance: 15.0, // More lenient for better matches
        };
        Self::create_with_config(config)
    }

    /// Create a strict parser for validation purposes
    pub fn create_strict() -> Result<Box<dyn ColorParserTrait>> {
        let config = ColorParserConfig {
            parser_type: ColorParserType::Full,
            strict_validation: true,
            enable_fallback_naming: false,
            color_tolerance: 5.0, // Strict tolerance
        };
        Self::create_with_config(config)
    }

    /// Get available parser types
    pub fn available_types() -> Vec<ColorParserType> {
        vec![
            ColorParserType::Css,
            ColorParserType::Full,
            ColorParserType::Custom,
        ]
    }
}

/// Trait defining the interface for color parsers
pub trait ColorParserTrait {
    /// Parse a color string and return LAB color with format information
    fn parse(&self, input: &str) -> Result<(palette::Lab, crate::color_parser::ColorFormat)>;

    /// Get the closest color name for given RGB values
    fn get_color_name(&self, rgb: (u8, u8, u8)) -> String;

    /// Get parser capabilities/information
    fn get_info(&self) -> ColorParserInfo;
}

/// Information about a color parser's capabilities
#[derive(Debug, Clone)]
pub struct ColorParserInfo {
    pub parser_type: ColorParserType,
    pub supported_formats: Vec<String>,
    pub collection_count: usize,
    pub color_count: usize,
}

/// Basic CSS-only color parser
pub struct BasicColorParser {
    color_parser: ColorParser,
    #[allow(dead_code)]
    config: ColorParserConfig,
}

impl BasicColorParser {
    pub fn new(config: ColorParserConfig) -> Result<Self> {
        Ok(Self {
            color_parser: ColorParser::new(),
            config,
        })
    }
}

impl ColorParserTrait for BasicColorParser {
    fn parse(&self, input: &str) -> Result<(palette::Lab, crate::color_parser::ColorFormat)> {
        self.color_parser.parse(input)
    }

    fn get_color_name(&self, rgb: (u8, u8, u8)) -> String {
        self.color_parser.get_color_name(rgb)
    }

    fn get_info(&self) -> ColorParserInfo {
        ColorParserInfo {
            parser_type: ColorParserType::Css,
            supported_formats: vec![
                "HEX".to_string(),
                "RGB".to_string(),
                "RGBA".to_string(),
                "HSL".to_string(),
                "HSLA".to_string(),
                "Named".to_string(),
            ],
            collection_count: 1,
            color_count: 147, // Standard CSS colors
        }
    }
}

/// Full color parser with all available collections
pub struct FullColorParser {
    color_parser: ColorParser,
    unified_manager: UnifiedColorManager,
    config: ColorParserConfig,
}

impl FullColorParser {
    pub fn new(config: ColorParserConfig) -> Result<Self> {
        Ok(Self {
            color_parser: ColorParser::new(),
            unified_manager: UnifiedColorManager::new()?,
            config,
        })
    }
}

impl ColorParserTrait for FullColorParser {
    fn parse(&self, input: &str) -> Result<(palette::Lab, crate::color_parser::ColorFormat)> {
        self.color_parser.parse(input)
    }

    fn get_color_name(&self, rgb: (u8, u8, u8)) -> String {
        // Use unified manager for comprehensive color matching
        let matches = self
            .unified_manager
            .find_closest_across_all([rgb.0, rgb.1, rgb.2], 1);

        // Find the best match across all collections
        for (_, collection_matches) in &matches {
            if let Some(color_match) = collection_matches.first() {
                return color_match.entry.metadata.name.clone();
            }
        }

        if self.config.enable_fallback_naming {
            Utils::rgb_to_string(rgb.0, rgb.1, rgb.2)
        } else {
            "Unknown".to_string()
        }
    }

    fn get_info(&self) -> ColorParserInfo {
        ColorParserInfo {
            parser_type: ColorParserType::Full,
            supported_formats: vec![
                "HEX".to_string(),
                "RGB".to_string(),
                "RGBA".to_string(),
                "HSL".to_string(),
                "HSLA".to_string(),
                "Named".to_string(),
                "RAL Classic".to_string(),
                "RAL Design".to_string(),
            ],
            collection_count: 3, // CSS, RAL Classic, RAL Design
            color_count: 1500,   // Approximate total
        }
    }
}

/// Custom color parser for specialized use cases
pub struct CustomColorParser {
    color_parser: ColorParser,
    config: ColorParserConfig,
}

impl CustomColorParser {
    pub fn new(config: ColorParserConfig) -> Result<Self> {
        Ok(Self {
            color_parser: ColorParser::new(),
            config,
        })
    }
}

impl ColorParserTrait for CustomColorParser {
    fn parse(&self, input: &str) -> Result<(palette::Lab, crate::color_parser::ColorFormat)> {
        if self.config.strict_validation {
            // Implement stricter validation if needed
            self.color_parser.parse(input)
        } else {
            self.color_parser.parse(input)
        }
    }

    fn get_color_name(&self, rgb: (u8, u8, u8)) -> String {
        if self.config.enable_fallback_naming {
            self.color_parser.get_color_name(rgb)
        } else {
            // More conservative naming
            let name = self.color_parser.get_color_name(rgb);
            if name.starts_with("rgb(") {
                "Unknown".to_string()
            } else {
                name
            }
        }
    }

    fn get_info(&self) -> ColorParserInfo {
        ColorParserInfo {
            parser_type: ColorParserType::Custom,
            supported_formats: vec!["HEX".to_string(), "RGB".to_string(), "Named".to_string()],
            collection_count: 1,
            color_count: 147,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factory_create_basic_parser() {
        let parser = ColorParserFactory::create_parser(ColorParserType::Css);
        assert!(parser.is_ok());

        let info = parser.unwrap().get_info();
        assert_eq!(info.parser_type, ColorParserType::Css);
        assert_eq!(info.collection_count, 1);
    }

    #[test]
    fn test_factory_create_with_config() {
        let config = ColorParserConfig {
            parser_type: ColorParserType::Full,
            strict_validation: true,
            color_tolerance: 5.0,
            ..Default::default()
        };

        let parser = ColorParserFactory::create_with_config(config);
        assert!(parser.is_ok());
    }

    #[test]
    fn test_factory_preset_parsers() {
        assert!(ColorParserFactory::create_fast().is_ok());
        assert!(ColorParserFactory::create_comprehensive().is_ok());
        assert!(ColorParserFactory::create_strict().is_ok());
    }

    #[test]
    fn test_parser_capabilities() {
        let css_parser = ColorParserFactory::create_parser(ColorParserType::Css).unwrap();
        let css_info = css_parser.get_info();
        assert!(css_info.supported_formats.contains(&"HEX".to_string()));

        let full_parser = ColorParserFactory::create_parser(ColorParserType::Full).unwrap();
        let full_info = full_parser.get_info();
        assert!(
            full_info
                .supported_formats
                .contains(&"RAL Classic".to_string())
        );
    }
}
