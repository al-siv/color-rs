//! Chain of Responsibility Pattern for Color Parsing
//!
//! This module implements the Chain of Responsibility design pattern for parsing
//! different color input formats. Each handler in the chain attempts to parse
//! the input and either handles it or passes it to the next handler.

use crate::{color_utils::LegacyColorUtils as ColorUtils, error::ColorError};
use palette::Lab;
use std::sync::Arc;

type Result<T> = std::result::Result<T, ColorError>;

/// Result of a successful color parsing operation
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub lab_color: Lab,
    pub format_name: String,
    pub color_name: Option<String>,
}

/// Trait for color parsing handlers in the chain of responsibility
pub trait ColorParsingHandler: Send + Sync {
    /// Attempts to parse the input string into a color
    /// Returns Ok(Some(result)) if successfully parsed
    /// Returns Ok(None) if this handler cannot parse the input
    /// Returns Err if there was an error during parsing
    fn try_parse(&self, input: &str) -> Result<Option<ParseResult>>;

    /// Returns the name of this parser for debugging/logging
    fn handler_name(&self) -> &str;
}

/// HEX color parser (handles #FF0000, FF0000, #F00 formats)
pub struct HexColorParsingHandler;

impl Default for HexColorParsingHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl HexColorParsingHandler {
    #[must_use] pub const fn new() -> Self {
        Self
    }
}

impl ColorParsingHandler for HexColorParsingHandler {
    fn try_parse(&self, input: &str) -> Result<Option<ParseResult>> {
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

        // Try to parse using the existing color processor
        match ColorUtils::parse_hex_color(&expanded_hex) {
            Ok(lab) => Ok(Some(ParseResult {
                lab_color: lab,
                format_name: "HEX".to_string(),
                color_name: None,
            })),
            Err(_) => Ok(None),
        }
    }

    fn handler_name(&self) -> &'static str {
        "HEX Parser"
    }
}

/// RGB color parser (handles rgb(255,0,0), rgba(255,0,0,1.0) formats)
pub struct RgbColorParsingHandler;

impl Default for RgbColorParsingHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl RgbColorParsingHandler {
    #[must_use] pub const fn new() -> Self {
        Self
    }
}

impl ColorParsingHandler for RgbColorParsingHandler {
    fn try_parse(&self, input: &str) -> Result<Option<ParseResult>> {
        let trimmed = input.trim().to_lowercase();

        // Check if this looks like RGB format
        if !trimmed.starts_with("rgb(") && !trimmed.starts_with("rgba(") {
            return Ok(None);
        }

        // Try to parse using the unified color parser
        let parser = crate::color_parser::ColorParser::new();
        match parser.parse(input) {
            Ok((lab, _)) => Ok(Some(ParseResult {
                lab_color: lab,
                format_name: "RGB".to_string(),
                color_name: None,
            })),
            Err(_) => Ok(None),
        }
    }

    fn handler_name(&self) -> &'static str {
        "RGB Parser"
    }
}

/// CSS Named Color parser (handles named colors like 'red', 'blue')
pub struct CssNamedColorParsingHandler;

impl Default for CssNamedColorParsingHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl CssNamedColorParsingHandler {
    #[must_use] pub const fn new() -> Self {
        Self
    }
}

impl ColorParsingHandler for CssNamedColorParsingHandler {
    fn try_parse(&self, input: &str) -> Result<Option<ParseResult>> {
        let trimmed = input.trim().to_lowercase();

        // Try to parse using CSS color parser
        let parser = crate::color_parser::css_parser::CssColorParser::new();
        match parser.parse(&trimmed) {
            Ok(parsed_color) => {
                // Convert RGB to LAB
                let lab = ColorUtils::rgb_to_lab((parsed_color.r, parsed_color.g, parsed_color.b));
                Ok(Some(ParseResult {
                    lab_color: lab,
                    format_name: "CSS Named".to_string(),
                    color_name: Some(trimmed),
                }))
            }
            Err(_) => Ok(None),
        }
    }

    fn handler_name(&self) -> &'static str {
        "CSS Named Color Parser"
    }
}

/// RAL Color parser (handles RAL 3020, RAL 050 50 78 formats)
pub struct RalColorParsingHandler;

impl Default for RalColorParsingHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl RalColorParsingHandler {
    #[must_use] pub const fn new() -> Self {
        Self
    }
}

impl ColorParsingHandler for RalColorParsingHandler {
    fn try_parse(&self, input: &str) -> Result<Option<ParseResult>> {
        let trimmed = input.trim();

        // Check if this looks like a RAL color code
        if !trimmed.to_uppercase().starts_with("RAL") {
            return Ok(None);
        }

        // For now, return None as RAL parsing requires more complex implementation
        // This maintains the Chain of Responsibility pattern while allowing
        // future implementation of RAL parsing
        Ok(None)
    }

    fn handler_name(&self) -> &'static str {
        "RAL Color Parser"
    }
}

/// Color parsing chain that manages multiple parsing handlers
pub struct ColorParsingChain {
    handlers: Vec<Arc<dyn ColorParsingHandler>>,
}

impl ColorParsingChain {
    /// Creates a new parsing chain with default handlers
    #[must_use] pub fn new() -> Self {
        let handlers: Vec<Arc<dyn ColorParsingHandler>> = vec![
            Arc::new(HexColorParsingHandler::new()),
            Arc::new(RgbColorParsingHandler::new()),
            Arc::new(CssNamedColorParsingHandler::new()),
            Arc::new(RalColorParsingHandler::new()),
        ];

        Self { handlers }
    }

    /// Creates a custom parsing chain with specified handlers
    #[must_use] pub fn with_handlers(handlers: Vec<Arc<dyn ColorParsingHandler>>) -> Self {
        Self { handlers }
    }

    /// Attempts to parse the input using all handlers in sequence
    pub fn parse(&self, input: &str) -> Result<ParseResult> {
        for handler in &self.handlers {
            if let Some(result) = handler.try_parse(input)? {
                return Ok(result);
            }
        }

        Err(ColorError::ParseError(format!(
            "No parser could handle input: {input}"
        )))
    }

    /// Returns the names of all handlers in the chain
    #[must_use]
    pub fn handler_names(&self) -> Vec<&str> {
        self.handlers.iter().map(|h| h.handler_name()).collect()
    }
}

impl Default for ColorParsingChain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_parser() {
        let parser = HexColorParsingHandler::new();

        // Test valid hex colors
        assert!(parser.try_parse("#FF0000").unwrap().is_some());
        assert!(parser.try_parse("FF0000").unwrap().is_some());
        assert!(parser.try_parse("#F00").unwrap().is_some());

        // Test invalid inputs
        assert!(parser.try_parse("rgb(255,0,0)").unwrap().is_none());
        assert!(parser.try_parse("red").unwrap().is_none());
        assert!(parser.try_parse("#GGGGGG").unwrap().is_none());
    }

    #[test]
    fn test_chain_creation() {
        let chain = ColorParsingChain::new();
        assert_eq!(chain.handlers.len(), 4);

        let names = chain.handler_names();
        assert!(names.contains(&"HEX Parser"));
        assert!(names.contains(&"RGB Parser"));
        assert!(names.contains(&"CSS Named Color Parser"));
        assert!(names.contains(&"RAL Color Parser"));
    }

    #[test]
    fn test_chain_parsing() {
        let chain = ColorParsingChain::new();

        // Test hex color parsing
        let result = chain.parse("#FF0000");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.format_name, "HEX");
    }
}
