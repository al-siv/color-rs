//! CSS Color Parser
//!
//! Modernized and integrated version of css-color-parser-rs
//! Original: https://github.com/7thSigil/css-color-parser-rs
//! Authors: Dean McNamee, Katkov Oleksandr

use super::color_names::COLOR_DATA;
use super::types::{ColorFormat, ParsedColor};
use crate::error::{ColorError, Result};
use std::collections::HashMap;
use std::str::FromStr;

/// CSS color parser that handles various CSS color formats
pub struct CssColorParser {
    named_colors: HashMap<String, (u8, u8, u8)>,
}

impl CssColorParser {
    /// Create a new CSS color parser
    pub fn new() -> Self {
        Self {
            named_colors: Self::create_named_colors_from_data(),
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

                let r = u8::from_str_radix(&format!("{}{}", r_hex, r_hex), 16)
                    .map_err(|_| ColorError::InvalidColor("Invalid hex color".to_string()))?;
                let g = u8::from_str_radix(&format!("{}{}", g_hex, g_hex), 16)
                    .map_err(|_| ColorError::InvalidColor("Invalid hex color".to_string()))?;
                let b = u8::from_str_radix(&format!("{}{}", b_hex, b_hex), 16)
                    .map_err(|_| ColorError::InvalidColor("Invalid hex color".to_string()))?;

                Ok(ParsedColor::from_rgb(r, g, b, ColorFormat::Hex))
            }
            6 => {
                // #rrggbb format
                let r = u8::from_str_radix(&hex_part[0..2], 16)
                    .map_err(|_| ColorError::InvalidColor("Invalid hex color".to_string()))?;
                let g = u8::from_str_radix(&hex_part[2..4], 16)
                    .map_err(|_| ColorError::InvalidColor("Invalid hex color".to_string()))?;
                let b = u8::from_str_radix(&hex_part[4..6], 16)
                    .map_err(|_| ColorError::InvalidColor("Invalid hex color".to_string()))?;

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
                let (r, g, b) = self.parse_rgb_params(&params[..3])?;
                let a = self.parse_alpha(params[3])?;
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
                let a = self.parse_alpha(params[3])?;
                Ok(ParsedColor::new(r, g, b, a, ColorFormat::Hsla))
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

        let r = self.parse_color_component(params[0])?;
        let g = self.parse_color_component(params[1])?;
        let b = self.parse_color_component(params[2])?;

        Ok((r, g, b))
    }

    /// Parse HSL parameters and convert to RGB
    fn parse_hsl_params(&self, params: &[&str]) -> Result<(u8, u8, u8)> {
        if params.len() != 3 {
            return Err(ColorError::InvalidColor(
                "Expected 3 HSL parameters".to_string(),
            ));
        }

        let h = f32::from_str(params[0].trim())
            .map_err(|_| ColorError::InvalidColor("Invalid hue value".to_string()))?;
        let s = self.parse_percentage(params[1])?;
        let l = self.parse_percentage(params[2])?;

        // Normalize hue to 0-1 range
        let h_norm = (((h % 360.0) + 360.0) % 360.0) / 360.0;

        // Convert HSL to RGB using the reliable color_utils implementation
        use crate::color_utils::ColorUtils;
        let (r, g, b) = ColorUtils::hsl_to_rgb(h_norm, s, l);
        Ok((r, g, b))
    }

    /// Parse color component (0-255 or percentage)
    fn parse_color_component(&self, value: &str) -> Result<u8> {
        let value = value.trim();

        if value.ends_with('%') {
            let percentage_str = &value[..value.len() - 1];
            let percentage = f32::from_str(percentage_str)
                .map_err(|_| ColorError::InvalidColor("Invalid percentage".to_string()))?;
            Ok(((percentage / 100.0 * 255.0).round().clamp(0.0, 255.0)) as u8)
        } else {
            let int_val = u32::from_str(value)
                .map_err(|_| ColorError::InvalidColor("Invalid color component".to_string()))?;
            Ok((int_val.clamp(0, 255)) as u8)
        }
    }

    /// Parse percentage value (returns 0.0-1.0)
    fn parse_percentage(&self, value: &str) -> Result<f32> {
        let value = value.trim();

        if value.ends_with('%') {
            let percentage_str = &value[..value.len() - 1];
            let percentage = f32::from_str(percentage_str)
                .map_err(|_| ColorError::InvalidColor("Invalid percentage".to_string()))?;
            Ok((percentage / 100.0).clamp(0.0, 1.0))
        } else {
            // Allow float values without % for convenience
            let float_val = f32::from_str(value)
                .map_err(|_| ColorError::InvalidColor("Invalid percentage".to_string()))?;
            Ok(float_val.clamp(0.0, 1.0))
        }
    }

    /// Parse alpha value (0.0-1.0)
    fn parse_alpha(&self, value: &str) -> Result<f32> {
        let value = value.trim();
        let alpha = f32::from_str(value)
            .map_err(|_| ColorError::InvalidColor("Invalid alpha value".to_string()))?;
        Ok(alpha.clamp(0.0, 1.0))
    }

    /// Create named colors map from unified COLOR_DATA
    fn create_named_colors_from_data() -> HashMap<String, (u8, u8, u8)> {
        let mut colors = HashMap::new();

        // Use COLOR_DATA as single source of truth, converting to lowercase for CSS compatibility
        for &(name, [r, g, b]) in COLOR_DATA {
            colors.insert(name.to_lowercase(), (r, g, b));
        }

        colors
    }
}

impl Default for CssColorParser {
    fn default() -> Self {
        Self::new()
    }
}
