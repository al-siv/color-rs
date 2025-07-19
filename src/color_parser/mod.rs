//! Integrated color parser module for color-rs
//!
//! This module combines and modernizes functionality from:
//! - css-color-parser-rs (https://github.com/7thSigil/css-color-parser-rs) by Katkov Oleksandr
//! - color-name (https://github.com/annymosse/color-name) by annymosse
//!
//! We are heavily inspired by these libraries and have integrated and modernized
//! their functionality for our use case.

pub mod css_parser;
pub mod csv_loader;
pub mod parse_utils;
pub mod ral_matcher;
pub mod types;

// New unified collection system
pub mod collections;
pub mod compat;
pub mod css_collection;
pub mod ral_classic_collection;
pub mod ral_design_collection;
pub mod unified_manager;

pub use css_parser::CssColorParser;
pub use ral_matcher::*;
pub use types::{ColorFormat, ParsedColor};

// New unified collection system exports
pub use collections::*;
pub use css_collection::CssColorCollection;
pub use ral_classic_collection::RalClassicCollection;
pub use ral_design_collection::RalDesignCollection;
pub use unified_manager::UnifiedColorManager;

use crate::color_utils::*;
use crate::error::{ColorError, Result};
use palette::Lab;

/// Unified color parser that can handle various input formats
pub struct ColorParser {
    css_parser: CssColorParser,
    css_collection: CssColorCollection,
}

impl ColorParser {
    /// Create a new color parser
    pub fn new() -> Self {
        Self {
            css_parser: CssColorParser::new(),
            css_collection: CssColorCollection::new().unwrap_or_else(|_| {
                // Fallback: create empty collection if CSV loading fails
                CssColorCollection::new().unwrap()
            }),
        }
    }

    /// Parse any color input and return LAB color with format information
    pub fn parse(&self, input: &str) -> Result<(Lab, ColorFormat)> {
        let input = input.trim();

        // Try CSS parsing first (handles hex, rgb, rgba, hsl, hsla, named colors)
        if let Ok(parsed) = self.css_parser.parse(input) {
            let lab = ColorUtils::rgb_to_lab((parsed.r, parsed.g, parsed.b));
            return Ok((lab, parsed.format));
        }

        // Try RAL color parsing (RAL codes and RAL named colors)
        if let Some(ral_match) = ral_matcher::parse_ral_color(input) {
            // Parse hex color from RAL match
            if let Ok(parsed) = self.css_parser.parse(&ral_match.hex) {
                let lab = ColorUtils::rgb_to_lab((parsed.r, parsed.g, parsed.b));
                return Ok((lab, ColorFormat::Named)); // Treat RAL colors as named colors
            }
        }

        // Try RAL named color search (for colors like "luminous orange")
        let ral_matches = ral_matcher::find_ral_by_name(input);
        if !ral_matches.is_empty() {
            let best_match = &ral_matches[0];
            if let Ok(parsed) = self.css_parser.parse(&best_match.hex) {
                let lab = ColorUtils::rgb_to_lab((parsed.r, parsed.g, parsed.b));
                return Ok((lab, ColorFormat::Named));
            }
        }

        // Try hex color without # symbol
        if self.is_hex_without_hash(input) {
            let hex_with_hash = format!("#{}", input);
            if let Ok(parsed) = self.css_parser.parse(&hex_with_hash) {
                let lab = ColorUtils::rgb_to_lab((parsed.r, parsed.g, parsed.b));
                return Ok((lab, ColorFormat::Hex));
            }
        }

        // Try LAB color parsing (lab(L, a, b))
        if let Ok(lab) = self.parse_lab_color(input) {
            return Ok((lab, ColorFormat::Lab));
        }

        // If all parsing methods failed, return error
        Err(ColorError::InvalidColor(format!(
            "Unable to parse color: {}",
            input
        )))
    }

    /// Check if input looks like a hex color without # symbol
    fn is_hex_without_hash(&self, input: &str) -> bool {
        input.len() == 6 && input.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Parse LAB color in the format lab(L, a, b)
    fn parse_lab_color(&self, input: &str) -> Result<Lab> {
        let input = input.trim().to_lowercase();

        if input.starts_with("lab(") && input.ends_with(')') {
            let content = &input[4..input.len() - 1]; // Remove "lab(" and ")"
            let parts: Vec<&str> = content.split(',').collect();

            if parts.len() == 3 {
                let l: f32 = parts[0]
                    .trim()
                    .parse()
                    .map_err(|_| ColorError::InvalidColor("Invalid LAB L value".to_string()))?;
                let a: f32 = parts[1]
                    .trim()
                    .parse()
                    .map_err(|_| ColorError::InvalidColor("Invalid LAB a value".to_string()))?;
                let b: f32 = parts[2]
                    .trim()
                    .parse()
                    .map_err(|_| ColorError::InvalidColor("Invalid LAB b value".to_string()))?;

                return Ok(Lab::new(l, a, b));
            }
        }

        Err(ColorError::InvalidColor(
            "Invalid LAB color format".to_string(),
        ))
    }

    /// Get the closest color name for given RGB values
    pub fn get_color_name(&self, rgb: (u8, u8, u8)) -> String {
        let target = UniversalColor::from_rgb([rgb.0, rgb.1, rgb.2]);
        let matches = self.css_collection.find_closest(&target, 1, None);

        if let Some(closest) = matches.first() {
            closest.entry.metadata.name.clone()
        } else {
            format!("rgb({}, {}, {})", rgb.0, rgb.1, rgb.2) // Fallback to RGB notation
        }
    }
}

impl Default for ColorParser {
    fn default() -> Self {
        Self::new()
    }
}
