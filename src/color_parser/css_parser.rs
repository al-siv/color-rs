//! CSS Color Parser
//!
//! Modernized and integrated version of css-color-parser-rs
//! Original: https://github.com/7thSigil/css-color-parser-rs
//! Authors: Dean McNamee, Katkov Oleksandr

use super::csv_loader::CsvLoader;
use super::parse_utils::ParseUtils;
use super::types::{ColorFormat, ParsedColor};
use crate::color_utils::*;
use crate::error::{ColorError, Result};
use palette::*;
use std::collections::HashMap;

/// CSS color parser that handles various CSS color formats
pub struct CssColorParser {
    named_colors: HashMap<String, (u8, u8, u8)>,
}

impl CssColorParser {
    /// Create a new CSS color parser
    pub fn new() -> Self {
        Self {
            named_colors: Self::create_named_css_colors_from_data(),
        }
    }

    /// Parse a CSS color string
    pub fn parse(&self, input: &str) -> Result<ParsedColor> {
        let input = input.trim();
        if input.is_empty() {
            return Err(ColorError::InvalidColor("Empty color string".to_string()));
        }

        // Remove all whitespace and convert to lowercase
        let mut cleaned = input.replace(' ', "");
        cleaned.make_ascii_lowercase();

        // Check for named colors first
        if let Some(&(r, g, b)) = self.named_colors.get(&cleaned) {
            return Ok(ParsedColor::from_rgb(r, g, b, ColorFormat::Named));
        }

        // Check for hex colors
        if cleaned.starts_with('#') {
            return self.parse_hex(&cleaned);
        }

        // Check for functional notation (rgb, rgba, hsl, hsla)
        if let Some(open_paren) = cleaned.find('(') {
            if let Some(close_paren) = cleaned.find(')') {
                if close_paren + 1 == cleaned.len() && close_paren > open_paren {
                    return self.parse_functional(&cleaned, open_paren, close_paren);
                }
            }
        }

        Err(ColorError::InvalidColor(format!(
            "Unrecognized color format: {}",
            input
        )))
    }

    /// Parse hex color (#rgb or #rrggbb)
    fn parse_hex(&self, input: &str) -> Result<ParsedColor> {
        let hex_part = &input[1..]; // Remove #

        match hex_part.len() {
            3 => {
                // #rgb format
                let r_hex = &hex_part[0..1];
                let g_hex = &hex_part[1..2];
                let b_hex = &hex_part[2..3];

                let r = ParseUtils::parse_hex_component(&format!("{}{}", r_hex, r_hex))?;
                let g = ParseUtils::parse_hex_component(&format!("{}{}", g_hex, g_hex))?;
                let b = ParseUtils::parse_hex_component(&format!("{}{}", b_hex, b_hex))?;

                Ok(ParsedColor::from_rgb(r, g, b, ColorFormat::Hex))
            }
            6 => {
                // #rrggbb format
                let r = ParseUtils::parse_hex_component(&hex_part[0..2])?;
                let g = ParseUtils::parse_hex_component(&hex_part[2..4])?;
                let b = ParseUtils::parse_hex_component(&hex_part[4..6])?;

                Ok(ParsedColor::from_rgb(r, g, b, ColorFormat::Hex))
            }
            _ => Err(ColorError::InvalidColor(
                "Invalid hex color length".to_string(),
            )),
        }
    }

    /// Parse functional notation (rgb, rgba, hsl, hsla)
    fn parse_functional(
        &self,
        input: &str,
        open_paren: usize,
        close_paren: usize,
    ) -> Result<ParsedColor> {
        let function_name = &input[..open_paren];
        let params_str = &input[open_paren + 1..close_paren];
        let params: Vec<&str> = params_str.split(',').collect();

        match function_name {
            "rgb" => {
                if params.len() != 3 {
                    return Err(ColorError::InvalidColor(
                        "RGB requires 3 parameters".to_string(),
                    ));
                }
                let (r, g, b) = self.parse_rgb_params(&params)?;
                Ok(ParsedColor::from_rgb(r, g, b, ColorFormat::Rgb))
            }
            "rgba" => {
                if params.len() != 4 {
                    return Err(ColorError::InvalidColor(
                        "RGBA requires 4 parameters".to_string(),
                    ));
                }
                let (r, g, b) = self.parse_rgb_params(&params)?;
                let a = ParseUtils::parse_alpha(params[3])?;
                Ok(ParsedColor::new(r, g, b, a, ColorFormat::Rgba))
            }
            "hsl" => {
                if params.len() != 3 {
                    return Err(ColorError::InvalidColor(
                        "HSL requires 3 parameters".to_string(),
                    ));
                }
                let (r, g, b) = self.parse_hsl_params(&params)?;
                Ok(ParsedColor::from_rgb(r, g, b, ColorFormat::Hsl))
            }
            "hsla" => {
                if params.len() != 4 {
                    return Err(ColorError::InvalidColor(
                        "HSLA requires 4 parameters".to_string(),
                    ));
                }
                let (r, g, b) = self.parse_hsl_params(&params[..3])?;
                let a = ParseUtils::parse_alpha(params[3])?;
                Ok(ParsedColor::new(r, g, b, a, ColorFormat::Hsla))
            }
            "lch" => {
                if params.len() != 3 {
                    return Err(ColorError::InvalidColor(
                        "LCH requires 3 parameters".to_string(),
                    ));
                }
                let (r, g, b) = self.parse_lch_params(&params)?;
                Ok(ParsedColor::from_rgb(r, g, b, ColorFormat::Lch))
            }
            _ => Err(ColorError::InvalidColor(format!(
                "Unknown function: {}",
                function_name
            ))),
        }
    }

    /// Parse RGB parameters
    fn parse_rgb_params(&self, params: &[&str]) -> Result<(u8, u8, u8)> {
        if params.len() != 3 {
            return Err(ColorError::InvalidColor(
                "Expected 3 RGB parameters".to_string(),
            ));
        }

        let r = ParseUtils::parse_color_component(params[0])?;
        let g = ParseUtils::parse_color_component(params[1])?;
        let b = ParseUtils::parse_color_component(params[2])?;

        Ok((r, g, b))
    }

    /// Parse HSL parameters and convert to RGB
    fn parse_hsl_params(&self, params: &[&str]) -> Result<(u8, u8, u8)> {
        if params.len() != 3 {
            return Err(ColorError::InvalidColor(
                "Expected 3 HSL parameters".to_string(),
            ));
        }

        let h = ParseUtils::parse_hue(params[0])?;
        let s = ParseUtils::parse_percentage(params[1])?;
        let l = ParseUtils::parse_percentage(params[2])?;

        // Normalize hue to 0-1 range
        let h_norm = (((h % 360.0) + 360.0) % 360.0) / 360.0;
        let hsl: Hsl = Hsl::new(h_norm as f32, s as f32, l as f32);

        // Convert HSL to RGB using the reliable color_utils implementation
        let (r, g, b) = ColorUtils::srgb_to_rgb(hsl.into_color());
        Ok((r, g, b))
    }

    /// Parse LCH parameters and convert to RGB
    fn parse_lch_params(&self, params: &[&str]) -> Result<(u8, u8, u8)> {
        if params.len() != 3 {
            return Err(ColorError::InvalidColor(
                "Expected 3 LCH parameters".to_string(),
            ));
        }

        let l: f32 = params[0]
            .trim()
            .parse()
            .map_err(|_| ColorError::InvalidColor("Invalid LCH L value".to_string()))?;
        let c: f32 = params[1]
            .trim()
            .parse()
            .map_err(|_| ColorError::InvalidColor("Invalid LCH C value".to_string()))?;
        let h: f32 = params[2]
            .trim()
            .parse()
            .map_err(|_| ColorError::InvalidColor("Invalid LCH H value".to_string()))?;

        // Convert LCH to LAB using color_utils
        let lch = palette::Lch::new(l, c, h);
        let lab = ColorUtils::lch_to_lab(lch);
        
        // Convert LAB to RGB
        let (r, g, b) = ColorUtils::lab_to_rgb(lab);
        Ok((r, g, b))
    }

    /// Create named colors map from CSS colors CSV file
    fn create_named_css_colors_from_data() -> HashMap<String, (u8, u8, u8)> {
        let mut css_colors = HashMap::new();

        // Load from CSV file, fallback to empty map on error
        if let Ok(csv_colors) = CsvLoader::load_colors_from_csv("color-table/css-colors.csv") {
            for entry in csv_colors {
                if let Ok(rgb) = CsvLoader::hex_to_rgb(&entry.hex) {
                    // Use the lowercase code as the key for CSS compatibility
                    css_colors.insert(entry.code.to_lowercase(), (rgb[0], rgb[1], rgb[2]));
                }
            }
        }

        css_colors
    }
}

impl Default for CssColorParser {
    fn default() -> Self {
        Self::new()
    }
}
