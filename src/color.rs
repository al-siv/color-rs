//! Color operations and conversions for color-rs

use palette::{FromColor, Hsl, IntoColor, Lab, Srgb};
use tabled::Tabled;
use crate::config::{HEX_COLOR_LENGTH, RGB_MAX};
use crate::error::{ColorError, Result};

/// Color information for display in tables
#[derive(Tabled)]
pub struct ColorInfo {
    #[tabled(rename = "Color")]
    pub label: String,
    #[tabled(rename = "Hex")]
    pub hex: String,
    #[tabled(rename = "RGB")]
    pub rgb: String,
    #[tabled(rename = "HSL")]
    pub hsl: String,
    #[tabled(rename = "Lab")]
    pub lab: String,
}

/// Supported color spaces
#[derive(Debug, Clone, Copy)]
pub enum ColorSpace {
    Srgb,
    Lab,
    Hsl,
}

/// Color operations and conversions
pub struct ColorProcessor;

impl ColorProcessor {
    /// Parse a hex color string into LAB color space
    pub fn parse_hex_color(hex: &str) -> Result<Lab> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != HEX_COLOR_LENGTH {
            return Err(ColorError::InvalidColor(
                "Invalid HEX color format. Expected format: #RRGGBB".to_string(),
            ));
        }

        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;

        let rgb = Srgb::new(r as f32 / RGB_MAX as f32, g as f32 / RGB_MAX as f32, b as f32 / RGB_MAX as f32);
        Ok(Lab::from_color(rgb))
    }

    /// Convert LAB color to hex string
    pub fn lab_to_hex(lab: Lab) -> String {
        let rgb: Srgb = lab.into_color();
        let r = (rgb.red * RGB_MAX as f32).round() as u8;
        let g = (rgb.green * RGB_MAX as f32).round() as u8;
        let b = (rgb.blue * RGB_MAX as f32).round() as u8;
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    /// Convert LAB color to RGB values
    pub fn lab_to_rgb_values(lab: Lab) -> (u8, u8, u8) {
        let rgb: Srgb = lab.into_color();
        let r = (rgb.red * RGB_MAX as f32).round() as u8;
        let g = (rgb.green * RGB_MAX as f32).round() as u8;
        let b = (rgb.blue * RGB_MAX as f32).round() as u8;
        (r, g, b)
    }

    /// Convert LAB color to HSL values
    pub fn lab_to_hsl_values(lab: Lab) -> (f32, f32, f32) {
        let hsl: Hsl = lab.into_color();
        (
            hsl.hue.into_positive_degrees(),
            hsl.saturation,
            hsl.lightness,
        )
    }

    /// Create color information structure for a given LAB color
    pub fn create_color_info(label: String, lab: Lab) -> ColorInfo {
        let hex = Self::lab_to_hex(lab);
        let rgb = Self::lab_to_rgb_values(lab);
        let hsl = Self::lab_to_hsl_values(lab);

        ColorInfo {
            label,
            hex,
            rgb: format!("RGB({}, {}, {})", rgb.0, rgb.1, rgb.2),
            hsl: format!(
                "HSL({:.1}°, {:.1}%, {:.1}%)",
                hsl.0,
                hsl.1 * 100.0,
                hsl.2 * 100.0
            ),
            lab: format!(
                "Lab({:.1}, {:.1}, {:.1})",
                lab.l, lab.a, lab.b
            ),
        }
    }

    /// Print color information table
    pub fn print_color_info_table(start_lab: Lab, end_lab: Lab) {
        use colored::*;
        use tabled::{Table, settings::{Alignment, Style, object::Columns}};

        let color_data = vec![
            Self::create_color_info("Start Color".to_string(), start_lab),
            Self::create_color_info("End Color".to_string(), end_lab),
        ];

        println!(
            "{}",
            " Color Information: "
                .bold()
                .to_uppercase()
                .black()
                .on_bright_white()
        );
        let mut table = Table::new(color_data);
        table.with(Style::rounded());
        table.modify(Columns::first(), Alignment::right());
        println!("{}", table);
        println!();
    }

    /// Interpolate between two LAB colors
    pub fn interpolate_lab(start: Lab, end: Lab, t: f64) -> Lab {
        let t = t as f32;
        Lab::new(
            start.l + (end.l - start.l) * t,
            start.a + (end.a - start.a) * t,
            start.b + (end.b - start.b) * t,
        )
    }

    /// Validate a hex color string
    pub fn validate_hex_color(hex: &str) -> Result<()> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != HEX_COLOR_LENGTH {
            return Err(ColorError::InvalidColor(
                "Invalid HEX color format. Expected format: #RRGGBB".to_string(),
            ));
        }

        // Check if all characters are valid hex digits
        for ch in hex.chars() {
            if !ch.is_ascii_hexdigit() {
                return Err(ColorError::InvalidColor(
                    "Invalid hex color: contains non-hexadecimal characters".to_string(),
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color() {
        let lab = ColorProcessor::parse_hex_color("#FF0000").unwrap();
        assert!(lab.l > 50.0 && lab.l < 55.0); // Red should have lightness around 53
    }

    #[test]
    fn test_invalid_hex_color() {
        assert!(ColorProcessor::parse_hex_color("#ZZZZZZ").is_err());
        assert!(ColorProcessor::parse_hex_color("#FF00").is_err());
    }

    #[test]
    fn test_lab_to_hex() {
        let lab = Lab::new(53.2, 80.1, 67.2); // Approximately red
        let hex = ColorProcessor::lab_to_hex(lab);
        assert!(hex.starts_with('#'));
        assert_eq!(hex.len(), 7);
    }

    #[test]
    fn test_color_interpolation() {
        let red = Lab::new(53.2, 80.1, 67.2);
        let blue = Lab::new(32.3, 79.2, -107.9);
        let mid = ColorProcessor::interpolate_lab(red, blue, 0.5);
        
        // Middle color should be between red and blue
        assert!(mid.l > blue.l && mid.l < red.l);
    }
}
