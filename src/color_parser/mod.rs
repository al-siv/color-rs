//! Integrated color parser module for color-rs
//!
//! This module combines and modernizes functionality from:
//! - css-color-parser-rs (https://github.com/7thSigil/css-color-parser-rs) by Katkov Oleksandr
//! - color-name (https://github.com/annymosse/color-name) by annymosse
//!
//! We are heavily inspired by these libraries and have integrated and modernized
//! their functionality for our use case.

pub mod color_names;
pub mod css_parser;
pub mod ral_data;
pub mod ral_matcher;
pub mod types;

pub use color_names::ColorNameResolver;
pub use ral_matcher::{
    find_closest_ral_classic, find_closest_ral_design, find_closest_ral_colors,
    find_ral_by_name, parse_ral_color, parse_ral_classic_code, parse_ral_design_code,
    RalMatch, RalClassification, RgbColor
};
pub use css_parser::CssColorParser;
pub use types::{ColorFormat, ParsedColor};

use crate::error::{ColorError, Result};
use palette::{IntoColor, Lab, Srgb};

/// Unified color parser that can handle various input formats
pub struct ColorParser {
    css_parser: CssColorParser,
    name_resolver: ColorNameResolver,
}

impl ColorParser {
    /// Create a new color parser
    pub fn new() -> Self {
        Self {
            css_parser: CssColorParser::new(),
            name_resolver: ColorNameResolver::new(),
        }
    }

    /// Parse any color input and return LAB color with format information
    pub fn parse(&self, input: &str) -> Result<(Lab, ColorFormat)> {
        // Try CSS parsing first (handles hex, rgb, rgba, hsl, hsla, named colors)
        if let Ok(parsed) = self.css_parser.parse(input) {
            let lab = self.srgb_to_lab(parsed.r, parsed.g, parsed.b);
            return Ok((lab, parsed.format));
        }

        // If CSS parsing failed, return error
        Err(ColorError::InvalidColor(format!(
            "Unable to parse color: {}",
            input
        )))
    }

    /// Get the closest color name for given RGB values
    pub fn get_color_name(&self, r: u8, g: u8, b: u8) -> String {
        self.name_resolver.find_closest_name([r, g, b])
    }

    /// Convert sRGB values to LAB color space
    fn srgb_to_lab(&self, r: u8, g: u8, b: u8) -> Lab {
        let srgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        srgb.into_color()
    }
}

impl Default for ColorParser {
    fn default() -> Self {
        Self::new()
    }
}
