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

/// Match and convert a color to all formats with comprehensive output
pub fn color_match(color_input: &str) -> Result<String> {
    use crate::color_parser::ColorParser;
    use palette::{Srgb, Hsl, Oklch, Xyz};
    use std::fmt::Write;
    
    // Create color parser instance
    let parser = ColorParser::new();
    
    // Parse the input color using the integrated parser
    let (lab_color, _format) = parser.parse(color_input)
        .map_err(|e| ColorError::InvalidColor(format!("Failed to parse color '{}': {}", color_input, e)))?;
    
    // Convert LAB back to sRGB for display and name lookup
    let srgb: Srgb = lab_color.into_color();
    let r = (srgb.red * 255.0).round() as u8;
    let g = (srgb.green * 255.0).round() as u8;
    let b = (srgb.blue * 255.0).round() as u8;
    
    // Find color name
    let color_name = parser.get_color_name(r, g, b);
    
    // Calculate grayscale equivalent using LAB L* component
    let grayscale_l = lab_color.l;
    let grayscale_rgb = (grayscale_l / 100.0 * 255.0).round() as u8;
    
    let mut output = String::new();
    
    // Comprehensive output - always detailed
    write!(output, "Color Analysis for: {}\n", color_input).map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    write!(output, "{}\n", "──────────────────────────────────────────────────").map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    
    // Original input
    write!(output, "Input: {}\n", color_input).map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    write!(output, "Name: {}\n", color_name).map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    write!(output, "\n").map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    
    // All format conversions
    write!(output, "Format Conversions:\n").map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    
    // sRGB values (0-255)
    write!(output, "• RGB:    rgb({}, {}, {})\n", r, g, b).map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    
    // Hex
    write!(output, "• Hex:    #{:02x}{:02x}{:02x}\n", r, g, b).map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    
    // HSL
    let hsl: Hsl = lab_color.into_color();
    write!(output, "• HSL:    hsl({:.0}, {:.1}%, {:.1}%)\n", 
            hsl.hue.into_positive_degrees(),
            hsl.saturation * 100.0,
            hsl.lightness * 100.0).map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    
    // LAB
    write!(output, "• LAB:    lab({:.2}, {:.2}, {:.2})\n", lab_color.l, lab_color.a, lab_color.b).map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    
    // XYZ
    let xyz: Xyz = lab_color.into_color();
    write!(output, "• XYZ:    xyz({:.3}, {:.3}, {:.3})\n", xyz.x, xyz.y, xyz.z).map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    
    // OKLCH
    let oklch: Oklch = lab_color.into_color();
    write!(output, "• OKLCH:  oklch({:.3}, {:.3}, {:.1}°)\n", 
            oklch.l, oklch.chroma, oklch.hue.into_positive_degrees()).map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    
    write!(output, "\n").map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    
    // Additional information
    write!(output, "Additional Information:\n").map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    write!(output, "• Grayscale: rgb({}, {}, {}) #{:02x}{:02x}{:02x} (LAB L* = {:.1})\n", 
            grayscale_rgb, grayscale_rgb, grayscale_rgb, grayscale_rgb, grayscale_rgb, grayscale_rgb, grayscale_l).map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    
    // Luminance and contrast information
    let luminance = 0.299 * (r as f32 / 255.0) + 
                   0.587 * (g as f32 / 255.0) + 
                   0.114 * (b as f32 / 255.0);
    write!(output, "• Relative Luminance: {:.3}\n", luminance).map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    
    let brightness = if luminance > 0.5 { "Light" } else { "Dark" };
    write!(output, "• Brightness: {}", brightness).map_err(|e| ColorError::InvalidColor(e.to_string()))?;
    
    Ok(output.trim_end().to_string())
}

/// Parse color input from various formats
pub fn parse_color_input(input: &str) -> Result<Lab> {
    let input = input.trim();
    
    // Try hex format first
    if input.starts_with('#') {
        let hex = &input[1..];
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16)
                .map_err(|_| ColorError::InvalidColor("Invalid hex color".to_string()))?;
            let g = u8::from_str_radix(&hex[2..4], 16)
                .map_err(|_| ColorError::InvalidColor("Invalid hex color".to_string()))?;
            let b = u8::from_str_radix(&hex[4..6], 16)
                .map_err(|_| ColorError::InvalidColor("Invalid hex color".to_string()))?;
            
            let srgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
            return Ok(srgb.into_color());
        }
    }
    
    // Try rgb format
    if input.starts_with("rgb(") && input.ends_with(')') {
        let content = &input[4..input.len()-1];
        let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
        if parts.len() == 3 {
            let r: u8 = parts[0].parse()
                .map_err(|_| ColorError::InvalidColor("Invalid RGB value".to_string()))?;
            let g: u8 = parts[1].parse()
                .map_err(|_| ColorError::InvalidColor("Invalid RGB value".to_string()))?;
            let b: u8 = parts[2].parse()
                .map_err(|_| ColorError::InvalidColor("Invalid RGB value".to_string()))?;
            
            let srgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
            return Ok(srgb.into_color());
        }
    }
    
    Err(ColorError::InvalidColor(format!("Unrecognized color format: {}", input)))
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

    #[test]
    fn test_color_match() {
        // Test comprehensive output (always detailed now)
        let output = color_match("#FF5733").unwrap();
        assert!(output.contains("Color Analysis for: #FF5733"));
        assert!(output.contains("Format Conversions:"));
        assert!(output.contains("Additional Information:"));
        assert!(output.contains("• RGB:    rgb(255, 87, 51)"));
        assert!(output.contains("• Hex:    #ff5733"));
        assert!(output.contains("• HSL:"));
        assert!(output.contains("• LAB:"));
        assert!(output.contains("• XYZ:"));
        assert!(output.contains("• OKLCH:"));
        assert!(output.contains("• Grayscale:"));
        assert!(output.contains("• Relative Luminance:"));
        assert!(output.contains("• Brightness:"));
    }

    #[test]
    fn test_color_match_various_formats() {
        // Test hex input
        let hex_result = color_match("#FF0000").unwrap();
        assert!(hex_result.contains("rgb(255, 0, 0)"));

        // Test RGB input
        let rgb_result = color_match("rgb(0, 255, 0)").unwrap();
        assert!(rgb_result.contains("rgb(0, 255, 0)"));

        // Test named color input
        let named_result = color_match("red").unwrap();
        assert!(named_result.contains("rgb(255, 0, 0)"));
        assert!(named_result.contains("Name: Red"));

        // Test HSL input
        let hsl_result = color_match("hsl(240, 100%, 50%)").unwrap();
        assert!(hsl_result.contains("rgb(0, 0, 255)"));
    }

    #[test]
    fn test_color_match_grayscale() {
        let result = color_match("#808080").unwrap();
        assert!(result.contains("Grayscale: rgb("));
        assert!(result.contains("#808080")); // Should include HEX format for grayscale
        
        // For gray color, grayscale should be close to the original
        assert!(result.contains("LAB L*"));
    }

    #[test]
    fn test_parse_color_input() {
        let lab_from_hex = parse_color_input("#FF5733").unwrap();
        let lab_from_rgb = parse_color_input("rgb(255, 87, 51)").unwrap();
        
        assert!((lab_from_hex.l - lab_from_rgb.l).abs() < 0.01);
        assert!((lab_from_hex.a - lab_from_rgb.a).abs() < 0.01);
        assert!((lab_from_hex.b - lab_from_rgb.b).abs() < 0.01);
    }
}
