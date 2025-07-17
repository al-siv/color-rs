//! Color operations and conversions for color-rs

use crate::color_formatter::ColorFormatter;
use crate::config::{HEX_COLOR_LENGTH, RGB_MAX};
use crate::error::{ColorError, Result};
use palette::{FromColor, Hsl, IntoColor, Lab, Srgb};
use tabled::Tabled;

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

        let rgb = Srgb::new(
            r as f32 / RGB_MAX as f32,
            g as f32 / RGB_MAX as f32,
            b as f32 / RGB_MAX as f32,
        );
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
            rgb: format!("rgb({}, {}, {})", rgb.0, rgb.1, rgb.2),
            hsl: format!(
                "hsl({:.1}, {:.1}%, {:.1}%)",
                hsl.0,
                hsl.1 * 100.0,
                hsl.2 * 100.0
            ),
            lab: format!("lab({:.1}, {:.1}, {:.1})", lab.l, lab.a, lab.b),
        }
    }

    /// Print color information table
    pub fn print_color_info_table(start_lab: Lab, end_lab: Lab) {
        use colored::*;
        use tabled::{
            Table,
            settings::{Alignment, Style, object::Columns},
        };

        let color_data = vec![
            Self::create_color_info("Start Color".to_string(), start_lab),
            Self::create_color_info("End Color".to_string(), end_lab),
        ];

        println!(
            "{}",
            " GRADIENT VALUES:  "
                .bold()
                .to_uppercase()
                .black()
                .on_white()
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

    /// Format color information for table display
    pub fn format_color_info(lab_color: Lab, label: &str) -> ColorInfo {
        ColorFormatter::format_color_info(lab_color, label)
    }
}

/// Match and convert a color to all formats with comprehensive output
pub fn color_match(color_input: &str) -> Result<String> {
    // First, try to parse as RAL code (RAL Classic or Design System+)
    if let Some(ral_match) = try_parse_ral_color(color_input) {
        // Convert RAL match to LAB color for comprehensive analysis
        let hex_without_hash = ral_match.hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex_without_hash[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex_without_hash[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex_without_hash[4..6], 16).unwrap_or(0);

        let srgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        let lab_color: Lab = srgb.into_color();

        // Use the RAL color name as the color name
        let ral_color_name = format!("{} ({})", ral_match.name, ral_match.code);

        return format_comprehensive_report_with_unified_collections(
            lab_color,
            color_input,
            &ral_color_name,
        );
    }

    // Parse the input color
    let (lab_color, _format) = parse_color_with_parser(color_input)?;

    // Get color name
    let color_name = get_color_name_for_lab(lab_color)?;

    // Generate comprehensive report including RAL matches
    format_comprehensive_report_with_unified_collections(lab_color, color_input, &color_name)
}

/// Parse color input using the integrated parser
fn parse_color_with_parser(color_input: &str) -> Result<(Lab, crate::color_parser::ColorFormat)> {
    use crate::color_parser::ColorParser;

    let parser = ColorParser::new();
    parser.parse(color_input).map_err(|e| {
        ColorError::InvalidColor(format!("Failed to parse color '{}': {}", color_input, e))
    })
}

/// Get color name for a LAB color
fn get_color_name_for_lab(lab_color: Lab) -> Result<String> {
    use crate::color_parser::ColorParser;

    // Convert LAB back to sRGB for name lookup
    let srgb: Srgb = lab_color.into_color();
    let r = (srgb.red * 255.0).round() as u8;
    let g = (srgb.green * 255.0).round() as u8;
    let b = (srgb.blue * 255.0).round() as u8;

    let parser = ColorParser::new();
    Ok(parser.get_color_name(r, g, b))
}

/// Parse color input from various formats
pub fn parse_color_input(input: &str) -> Result<Lab> {
    let (lab, _format) = parse_color_with_parser(input)?;
    Ok(lab)
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
        // Test comprehensive output with new unified format
        let output = color_match("#FF5733").unwrap();
        assert!(output.contains("COLOR ANALYSIS"));
        assert!(output.contains("FORMAT CONVERSIONS"));
        assert!(output.contains("ADDITIONAL INFORMATION"));
        assert!(output.contains("COLOR COLLECTIONS"));
        assert!(output.contains("rgb(255, 87, 51)"));
        assert!(output.contains("#FF5733"));
        assert!(output.contains("HSL:"));
        assert!(output.contains("LAB:"));
        assert!(output.contains("XYZ:"));
        assert!(output.contains("OKLCH:"));
        assert!(output.contains("Grayscale:"));
        assert!(output.contains("WCAG Relative Luminance:"));
        assert!(output.contains("Brightness:"));
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
        assert!(named_result.contains("CSS Colors") && named_result.contains("red"));

        // Test HSL input
        let hsl_result = color_match("hsl(240, 100%, 50%)").unwrap();
        assert!(hsl_result.contains("rgb(0, 0, 255)"));
    }

    #[test]
    fn test_color_match_grayscale() {
        let result = color_match("#808080").unwrap();
        assert!(result.contains("Grayscale:"));
        assert!(result.contains("#808080")); // Should include HEX format for grayscale

        // For gray color, LAB should be present
        assert!(result.contains("LAB:"));
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

/// Try to parse input as RAL color code or name
fn try_parse_ral_color(input: &str) -> Option<crate::color_parser::RalMatch> {
    use crate::color_parser::parse_ral_color;

    // The parse_ral_color function now handles CSS color filtering internally
    parse_ral_color(input)
}

/// Generate comprehensive report using the unified collection approach
fn format_comprehensive_report_with_unified_collections(
    lab_color: Lab,
    input: &str,
    color_name: &str,
) -> Result<String> {
    // Use the new unified approach that includes all collections in one section
    ColorFormatter::format_comprehensive_report(lab_color, input, color_name)
}

/// Match and convert a color to all formats with comprehensive output using a custom strategy
pub fn color_match_with_strategy(
    color_input: &str,
    strategy: &dyn crate::color_distance_strategies::ColorDistanceStrategy,
) -> Result<String> {
    // First, try to parse as RAL code (RAL Classic or Design System+)
    if let Some(ral_match) = try_parse_ral_color(color_input) {
        // Convert RAL match to LAB color for comprehensive analysis
        let hex_without_hash = ral_match.hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex_without_hash[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex_without_hash[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex_without_hash[4..6], 16).unwrap_or(0);

        let srgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        let lab_color: Lab = srgb.into_color();

        // Use the RAL color name as the color name
        let ral_color_name = format!("{} ({})", ral_match.name, ral_match.code);

        return format_comprehensive_report_with_unified_collections_strategy(
            lab_color,
            color_input,
            &ral_color_name,
            strategy,
        );
    }

    // Parse the input color
    let (lab_color, _format) = parse_color_with_parser(color_input)?;

    // Get color name
    let color_name = get_color_name_for_lab(lab_color)?;

    // Generate comprehensive report including RAL matches
    format_comprehensive_report_with_unified_collections_strategy(
        lab_color,
        color_input,
        &color_name,
        strategy,
    )
}

/// Enhanced color matching with color schemes and luminance adjustments
pub fn color_match_with_schemes(
    args: &crate::cli::ColorMatchArgs,
    strategy: &dyn crate::color_distance_strategies::ColorDistanceStrategy,
) -> Result<String> {
    // Parse the input color
    let (lab_color, _format) = parse_color_with_parser(&args.color)?;
    
    // Get color name
    let color_name = get_color_name_for_lab(lab_color)?;
    
    // Build color scheme calculator based on arguments
    let mut scheme_builder = crate::color_schemes::ColorSchemeBuilder::new();
    
    // Configure strategy based on schema flags
    if args.schema_lab {
        scheme_builder = scheme_builder.with_lab_strategy();
    } else if args.schema_hsl {
        scheme_builder = scheme_builder.with_hsl_strategy();
    } else {
        // Default to HSL strategy
        scheme_builder = scheme_builder.with_hsl_strategy();
    }
    
    // Configure output format (Lab if any lab-related flags are used)
    if args.schema_lab || args.relative_luminance.is_some() || args.luminance.is_some() {
        scheme_builder = scheme_builder.with_lab_output();
    }
    
    // Configure luminance handling
    if let Some(relative_lum) = args.relative_luminance {
        scheme_builder = scheme_builder.with_target_relative_luminance(relative_lum);
    } else if args.relative_luminance.is_none() && 
              (args.luminance.is_some() || args.schema_lab) {
        // When the flag is present without value, preserve luminance in color schemes
        scheme_builder = scheme_builder.preserve_relative_luminance();
    }
    
    if let Some(lab_lum) = args.luminance {
        scheme_builder = scheme_builder.with_target_lab_luminance(lab_lum);
    } else if args.luminance.is_none() && args.schema_lab {
        // When the flag is present without value, preserve luminance in color schemes
        scheme_builder = scheme_builder.preserve_lab_luminance();
    }
    
    // Calculate color schemes
    let calculator = scheme_builder.build();
    let schemes = calculator.calculate(lab_color)?;
    
    // Generate comprehensive report with color schemes
    format_comprehensive_report_with_schemes(
        schemes,
        &args.color,
        &color_name,
        strategy,
    )
}

/// Generate comprehensive report with color schemes included
fn format_comprehensive_report_with_schemes(
    schemes: crate::color_schemes::ColorSchemeResult,
    input: &str,
    color_name: &str,
    strategy: &dyn crate::color_distance_strategies::ColorDistanceStrategy,
) -> Result<String> {
    // First generate the basic comprehensive report for the base color
    let base_report = format_comprehensive_report_with_unified_collections_strategy(
        schemes.base_color,
        input,
        color_name,
        strategy,
    )?;
    
    // Now add the color schemes section
    let scheme_section = crate::color_formatter::ColorFormatter::format_color_schemes(&schemes)?;
    
    // Combine both reports
    Ok(format!("{}\n{}", base_report, scheme_section))
}

/// Generate comprehensive report using the unified collection approach with strategy
fn format_comprehensive_report_with_unified_collections_strategy(
    lab_color: Lab,
    input: &str,
    color_name: &str,
    strategy: &dyn crate::color_distance_strategies::ColorDistanceStrategy,
) -> Result<String> {
    // Use the strategy-aware ColorFormatter to generate the report
    ColorFormatter::format_comprehensive_report_with_strategy(
        lab_color, input, color_name, strategy,
    )
}
