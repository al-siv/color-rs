//! Integrated color parser module for color-rs
//!
//! This module combines and modernizes functionality from:
//! - css-color-parser-rs (<https://github.com/7thSigil/css-color-parser-rs>) by Katkov Oleksandr
//! - color-name (<https://github.com/annymosse/color-name>) by annymosse
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
pub mod css_collection;
pub mod ral_classic_collection;
pub mod ral_design_collection;
pub mod unified_manager;

pub use css_parser::CssColorParser;
pub use ral_matcher::*;
pub use types::{ColorFormat, ColorParseResult, ParsedColor};

// New unified collection system exports
pub use collections::*;
pub use css_collection::CssColorCollection;
pub use ral_classic_collection::RalClassicCollection;
pub use ral_design_collection::RalDesignCollection;
pub use unified_manager::UnifiedColorManager;

use crate::error::{ColorError, Result};
use palette::{IntoColor, Lab, Lch, Srgb};

/// Helper function to convert RGB tuple to LAB using functional palette approach
fn rgb_to_lab(rgb: (u8, u8, u8)) -> Lab {
    let srgb = Srgb::new(
        f32::from(rgb.0) / 255.0,
        f32::from(rgb.1) / 255.0,
        f32::from(rgb.2) / 255.0,
    );
    srgb.into_color()
}

/// Helper function to convert LAB to RGB tuple using functional palette approach
fn lab_to_rgb(lab: Lab) -> (u8, u8, u8) {
    let srgb: Srgb = lab.into_color();
    (
        (srgb.red * 255.0).round().clamp(0.0, 255.0) as u8,
        (srgb.green * 255.0).round().clamp(0.0, 255.0) as u8,
        (srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8,
    )
}

/// Unified color parser that can handle various input formats
pub struct ColorParser {
    css_parser: CssColorParser,
    css_collection: CssColorCollection,
    unified_manager: UnifiedColorManager,
}

impl ColorParser {
    /// Create a new color parser
    #[must_use]
    pub fn new() -> Self {
        let unified_manager = UnifiedColorManager::new().unwrap_or_else(|_| {
            // Fallback to default if creation fails
            UnifiedColorManager::default()
        });

        Self {
            css_parser: CssColorParser::new(),
            css_collection: CssColorCollection::new().unwrap_or_else(|_| {
                // Fallback: create empty collection if CSV loading fails
                CssColorCollection::empty()
            }),
            unified_manager,
        }
    }

    /// Parse any color input and return LAB color with format information
    pub fn parse(&self, input: &str) -> Result<(Lab, ColorFormat)> {
        let input = input.trim();

        // Try LCH parsing first for direct LAB conversion (avoids RGB roundtrip)
        if let Ok(lab) = Self::parse_lch_color(input) {
            return Ok((lab, ColorFormat::Lch));
        }

        // Try CSS parsing (handles hex, rgb, rgba, hsl, hsla, named colors)
        if let Ok(parsed) = self.css_parser.parse(input) {
            let lab = rgb_to_lab((parsed.r, parsed.g, parsed.b));
            return Ok((lab, parsed.format));
        }

        // Try RAL color parsing (RAL codes and RAL named colors)
        if let Some(ral_match) = ral_matcher::parse_ral_color(input) {
            // Parse hex color from RAL match
            if let Ok(parsed) = self.css_parser.parse(&ral_match.hex) {
                let lab = rgb_to_lab((parsed.r, parsed.g, parsed.b));
                return Ok((lab, ColorFormat::Named)); // Treat RAL colors as named colors
            }
        }

        // Try RAL named color search (for colors like "luminous orange")
        let ral_matches = ral_matcher::find_ral_by_name(input);
        if !ral_matches.is_empty() {
            let best_match = &ral_matches[0];
            if let Ok(parsed) = self.css_parser.parse(&best_match.hex) {
                let lab = rgb_to_lab((parsed.r, parsed.g, parsed.b));
                return Ok((lab, ColorFormat::Named));
            }
        }

        // Try hex color without # symbol
        if self.is_hex_without_hash(input) {
            let hex_with_hash = format!("#{input}");
            if let Ok(parsed) = self.css_parser.parse(&hex_with_hash) {
                let lab = rgb_to_lab((parsed.r, parsed.g, parsed.b));
                return Ok((lab, ColorFormat::Hex));
            }
        }

        // Try LAB color parsing (lab(L, a, b))
        if let Ok(lab) = Self::parse_lab_color(input) {
            return Ok((lab, ColorFormat::Lab));
        }

        // If all parsing methods failed, return error
        Err(ColorError::InvalidColor(format!(
            "Unable to parse color: {input}"
        )))
    }

    /// Check if input looks like a hex color without # symbol
    fn is_hex_without_hash(&self, input: &str) -> bool {
        input.len() == 6 && input.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Parse LAB color in the format lab(L, a, b)
    fn parse_lab_color(input: &str) -> Result<Lab> {
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

    /// Parse LCH color in the format lch(L, C, H) - direct to LAB conversion
    fn parse_lch_color(input: &str) -> Result<Lab> {
        let input = input.trim().to_lowercase();

        if input.starts_with("lch(") && input.ends_with(')') {
            let content = &input[4..input.len() - 1]; // Remove "lch(" and ")"
            let parts: Vec<&str> = content.split(',').collect();

            if parts.len() == 3 {
                let l: f32 = parts[0]
                    .trim()
                    .parse()
                    .map_err(|_| ColorError::InvalidColor("Invalid LCH L value".to_string()))?;
                let c: f32 = parts[1]
                    .trim()
                    .parse()
                    .map_err(|_| ColorError::InvalidColor("Invalid LCH C value".to_string()))?;
                let h: f32 = parts[2]
                    .trim()
                    .parse()
                    .map_err(|_| ColorError::InvalidColor("Invalid LCH H value".to_string()))?;

                // Convert LCH directly to LAB (no RGB roundtrip)
                let lch = Lch::new(l, c, h);
                let lab: Lab = lch.into_color();
                return Ok(lab);
            }
        }

        Err(ColorError::InvalidColor(
            "Invalid LCH color format".to_string(),
        ))
    }

    /// Get the closest color name for given RGB values
    #[must_use]
    pub fn get_color_name(&self, rgb: (u8, u8, u8)) -> String {
        let target = UniversalColor::from_rgb([rgb.0, rgb.1, rgb.2]);
        let matches = self.css_collection.find_closest(&target, 1, None);

        if let Some(closest) = matches.first() {
            closest.entry.metadata.name.clone()
        } else {
            format!("rgb({}, {}, {})", rgb.0, rgb.1, rgb.2) // Fallback to RGB notation
        }
    }

    /// Get access to the CSS color collection
    #[must_use]
    pub const fn css_collection(&self) -> &CssColorCollection {
        &self.css_collection
    }

    /// Get access to the unified color manager for all collections
    #[must_use]
    pub const fn unified_manager(&self) -> &UnifiedColorManager {
        &self.unified_manager
    }

    /// Find closest colors from all collections (CSS, RAL Classic, RAL Design)
    #[must_use]
    pub fn find_closest_all_collections(
        &self,
        rgb: [u8; 3],
        max_results: usize,
    ) -> Vec<(String, Vec<ColorMatch>)> {
        self.unified_manager
            .find_closest_across_all(rgb, max_results)
    }

    /// Find closest RAL Classic colors
    #[must_use]
    pub fn find_closest_ral_classic(&self, rgb: [u8; 3], max_results: usize) -> Vec<ColorMatch> {
        self.unified_manager
            .find_closest_ral_classic(rgb, max_results)
    }

    /// Find closest RAL Design colors
    #[must_use]
    pub fn find_closest_ral_design(&self, rgb: [u8; 3], max_results: usize) -> Vec<ColorMatch> {
        self.unified_manager
            .find_closest_ral_design(rgb, max_results)
    }
}

impl Default for ColorParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Comprehensive color parsing function
pub fn parse_color_comprehensive(input: &str) -> Result<ColorParseResult> {
    let parser = ColorParser::new();
    match parser.parse(input) {
        Ok((lab, _format)) => {
            // Convert back to ParsedColor for compatibility
            let (r, g, b) = lab_to_rgb(lab);
            let color = ParsedColor::from_rgb(r, g, b, ColorFormat::Hex);
            Ok(ColorParseResult::Single(color))
        }
        Err(e) => Err(e),
    }
}
