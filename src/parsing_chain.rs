//! Color Parsing System
//!
//! This module implements color parsing using enum dispatch
//! instead of trait objects for zero-cost abstractions.

use crate::error::ColorError;
use crate::color_parser::collections::ColorCollection;
use crate::config::math_constants;
use palette::{IntoColor, Lab, Srgb};

type Result<T> = std::result::Result<T, ColorError>;

/// Result of a successful color parsing operation
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub lab_color: Lab,
    pub format_name: String,
    pub color_name: Option<String>,
}

/// Color parser using enum dispatch for zero-cost abstractions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorParser {
    /// HEX color parser (handles #FF0000, FF0000, #F00 formats)
    Hex,
    /// RGB color parser (handles rgb(255, 0, 0) formats)
    Rgb,
    /// CSS named color parser (handles "red", "blue", etc.)
    CssNamed,
    /// RAL color parser (handles RAL color codes)
    Ral,
}

impl ColorParser {
    /// Attempts to parse the input string into a color
    /// Returns Ok(Some(result)) if successfully parsed
    /// Returns Ok(None) if this parser cannot parse the input
    /// Returns Err if there was an error during parsing
    pub fn try_parse(&self, input: &str) -> Result<Option<ParseResult>> {
        match self {
            Self::Hex => Self::parse_hex(input),
            Self::Rgb => Self::parse_rgb(input),
            Self::CssNamed => Self::parse_css_named(input),
            Self::Ral => Self::parse_ral(input),
        }
    }

    /// Returns the name of this parser for debugging/logging
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Hex => "HEX Parser",
            Self::Rgb => "RGB Parser", 
            Self::CssNamed => "CSS Named Parser",
            Self::Ral => "RAL Parser",
        }
    }

    /// HEX color parsing logic
    fn parse_hex(input: &str) -> Result<Option<ParseResult>> {
        let trimmed = input.trim();

        // Check if this looks like a hex color
        let hex_pattern = if let Some(stripped) = trimmed.strip_prefix('#') {
            stripped
        } else {
            trimmed
        };

        // Must be 3 or 6 hex characters
        if hex_pattern.len() != 3 && hex_pattern.len() != 6 {
            return Ok(None);
        }

        // Check if all characters are hex
        if !hex_pattern.chars().all(|c| c.is_ascii_hexdigit()) {
            return Ok(None);
        }

        // Convert 3-character hex to 6-character hex
        let expanded_hex = if hex_pattern.len() == 3 {
            format!(
                "#{}{}{}{}{}{}",
                hex_pattern.chars().nth(0).unwrap(),
                hex_pattern.chars().nth(0).unwrap(),
                hex_pattern.chars().nth(1).unwrap(),
                hex_pattern.chars().nth(1).unwrap(),
                hex_pattern.chars().nth(2).unwrap(),
                hex_pattern.chars().nth(2).unwrap()
            )
        } else {
            format!("#{hex_pattern}")
        };

        // Try to parse using hex conversion
        match crate::color_ops::conversion::hex_to_srgb(&expanded_hex) {
            Ok(srgb) => {
                let lab: Lab = srgb.into_color();
                Ok(Some(ParseResult {
                    lab_color: lab,
                    format_name: "HEX".to_string(),
                    color_name: None,
                }))
            }
            Err(_) => Ok(None),
        }
    }

    /// RGB color parsing logic
    fn parse_rgb(input: &str) -> Result<Option<ParseResult>> {
        let trimmed = input.trim().to_lowercase();

        // Check if it starts with "rgb("
        if !trimmed.starts_with("rgb(") || !trimmed.ends_with(')') {
            return Ok(None);
        }

        // Extract the values between parentheses
        let values_str = &trimmed[4..trimmed.len() - 1];
        let parts: Vec<&str> = values_str.split(',').map(|s| s.trim()).collect();

        if parts.len() != 3 {
            return Ok(None);
        }

        // Parse RGB values
        let r: u8 = match parts[0].parse().ok() {
            Some(val) => val,
            None => return Ok(None),
        };
        let g: u8 = match parts[1].parse().ok() {
            Some(val) => val,
            None => return Ok(None),
        };
        let b: u8 = match parts[2].parse().ok() {
            Some(val) => val,
            None => return Ok(None),
        };

        let srgb = Srgb::new(
            f32::from(r) / math_constants::RGB_MAX_VALUE as f32,
            f32::from(g) / math_constants::RGB_MAX_VALUE as f32,
            f32::from(b) / math_constants::RGB_MAX_VALUE as f32,
        );
        let lab: Lab = srgb.into_color();

        Ok(Some(ParseResult {
            lab_color: lab,
            format_name: "RGB".to_string(),
            color_name: None,
        }))
    }

    /// CSS named color parsing logic
    fn parse_css_named(input: &str) -> Result<Option<ParseResult>> {
        let trimmed = input.trim().to_lowercase();

        // Use the CSS collection to find the color
        let css_collection = match crate::color_parser::css_collection::CssColorCollection::new() {
            Ok(collection) => collection,
            Err(_) => return Ok(None),
        };

        match css_collection.find_by_name(&trimmed) {
            Some(color_entry) => {
                let lab = Lab::new(
                    color_entry.color.lab[0],
                    color_entry.color.lab[1],
                    color_entry.color.lab[2],
                );
                Ok(Some(ParseResult {
                    lab_color: lab,
                    format_name: "CSS".to_string(),
                    color_name: Some(trimmed),
                }))
            }
            None => Ok(None),
        }
    }

    /// RAL color parsing logic
    fn parse_ral(input: &str) -> Result<Option<ParseResult>> {
        let trimmed = input.trim().to_uppercase();

        // Try RAL Classic first
        if let Ok(ral_classic) = crate::color_parser::ral_classic_collection::RalClassicCollection::new() {
            if let Some(color_entry) = ral_classic.find_by_name(&trimmed) {
                let lab = Lab::new(
                    color_entry.color.lab[0],
                    color_entry.color.lab[1],
                    color_entry.color.lab[2],
                );
                return Ok(Some(ParseResult {
                    lab_color: lab,
                    format_name: "RAL Classic".to_string(),
                    color_name: Some(trimmed),
                }));
            }
        }

        // Try RAL Design
        if let Ok(ral_design) = crate::color_parser::ral_design_collection::RalDesignCollection::new() {
            if let Some(color_entry) = ral_design.find_by_name(&trimmed) {
                let lab = Lab::new(
                    color_entry.color.lab[0],
                    color_entry.color.lab[1],
                    color_entry.color.lab[2],
                );
                return Ok(Some(ParseResult {
                    lab_color: lab,
                    format_name: "RAL Design".to_string(),
                    color_name: Some(trimmed),
                }));
            }
        }

        Ok(None)
    }
}

/// Color parsing chain using enum dispatch
pub struct ColorParsingChain {
    parsers: Vec<ColorParser>,
}

impl Default for ColorParsingChain {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorParsingChain {
    /// Creates a new parsing chain with default parsers
    #[must_use]
    pub fn new() -> Self {
        let parsers = vec![
            ColorParser::Hex,
            ColorParser::Rgb,
            ColorParser::CssNamed,
            ColorParser::Ral,
        ];

        Self { parsers }
    }

    /// Creates a custom parsing chain with specified parsers
    #[must_use]
    pub fn with_parsers(parsers: Vec<ColorParser>) -> Self {
        Self { parsers }
    }

    /// Attempts to parse the input using all parsers in sequence
    pub fn parse(&self, input: &str) -> Result<ParseResult> {
        for parser in &self.parsers {
            if let Some(result) = parser.try_parse(input)? {
                return Ok(result);
            }
        }

        Err(ColorError::ParseError(format!(
            "Could not parse '{}' as any supported color format",
            input
        )))
    }

    /// Get the names of all parsers in this chain
    #[must_use]
    pub fn parser_names(&self) -> Vec<&'static str> {
        self.parsers.iter().map(|p| p.name()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_parsing() {
        let parser = ColorParser::Hex;
        
        // Test valid hex colors
        assert!(parser.try_parse("#FF0000").unwrap().is_some());
        assert!(parser.try_parse("FF0000").unwrap().is_some());
        assert!(parser.try_parse("#F00").unwrap().is_some());
        assert!(parser.try_parse("F00").unwrap().is_some());
        
        // Test invalid formats
        assert!(parser.try_parse("rgb(255,0,0)").unwrap().is_none());
        assert!(parser.try_parse("red").unwrap().is_none());
        assert!(parser.try_parse("#GG0000").unwrap().is_none());
    }

    #[test]
    fn test_rgb_parsing() {
        let parser = ColorParser::Rgb;
        
        // Test valid RGB colors
        assert!(parser.try_parse("rgb(255, 0, 0)").unwrap().is_some());
        assert!(parser.try_parse("rgb(255,0,0)").unwrap().is_some());
        
        // Test invalid formats
        assert!(parser.try_parse("#FF0000").unwrap().is_none());
        assert!(parser.try_parse("red").unwrap().is_none());
        assert!(parser.try_parse("rgb(256,0,0)").unwrap().is_none());
    }

    #[test]
    fn test_parsing_chain() {
        let chain = ColorParsingChain::new();
        
        // Test that chain can parse different formats
        assert!(chain.parse("#FF0000").is_ok());
        assert!(chain.parse("rgb(255, 0, 0)").is_ok());
        assert!(chain.parse("red").is_ok());
        
        // Test failure case
        assert!(chain.parse("invalid_color").is_err());
    }

    #[test]
    fn test_parser_names() {
        let chain = ColorParsingChain::new();
        let names = chain.parser_names();
        
        assert_eq!(names.len(), 4);
        assert!(names.contains(&"HEX Parser"));
        assert!(names.contains(&"RGB Parser"));
        assert!(names.contains(&"CSS Named Parser"));
        assert!(names.contains(&"RAL Parser"));
    }
}
