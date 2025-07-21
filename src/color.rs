//! Color operations and conversions for color-rs

use crate::color_formatter::ColorFormatter;

use crate::color_utils::*;
use crate::config::HEX_COLOR_LENGTH;
use crate::error::{ColorError, Result};
use palette::{Lab, Srgb};
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
    /// Create color information structure for a given LAB color
    pub fn create_color_info(label: String, lab: Lab) -> ColorInfo {
        let hex = ColorUtils::lab_to_hex(lab);
        let rgb = ColorUtils::lab_to_rgb(lab);
        let hsl = ColorUtils::lab_to_hsl_tuple(lab);

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
        use crate::config::{
            COLUMN_WIDTH, HEADER_BASE_COLORS, LABEL_GRADIENT_END_COLOR, LABEL_GRADIENT_START_COLOR,
        };
        use colored::*;
        use tabled::{
            Table,
            settings::{Alignment, Style, object::Columns},
        };

        let color_data = vec![
            Self::create_color_info(LABEL_GRADIENT_START_COLOR.to_string(), start_lab),
            Self::create_color_info(LABEL_GRADIENT_END_COLOR.to_string(), end_lab),
        ];

        println!("{}", HEADER_BASE_COLORS.bold().to_uppercase());
        let mut table = Table::new(color_data);
        table.with(Style::rounded());
        table.modify(Columns::first(), Alignment::right());
        println!("{}", table);
        println!();

        // Calculate WCAG contrast ratio
        let start_srgb: Srgb = ColorUtils::lab_to_srgb(start_lab);
        let end_srgb: Srgb = ColorUtils::lab_to_srgb(end_lab);
        let wcag_contrast =
            crate::color_utils::ColorUtils::wcag_contrast_ratio(start_srgb, end_srgb);
        let lab_delta_e = crate::color_utils::ColorUtils::lab_contrast_ratio(start_lab, end_lab);

        println!(
            "{} {:>7.2}",
            format!("{:>width$}", "Contrast (WCAG):", width = COLUMN_WIDTH)
                .bold()
                .green(),
            wcag_contrast
        );
        println!(
            "{} {:>7.2}",
            format!("{:>width$}", "Contrast (Lab):", width = COLUMN_WIDTH)
                .bold()
                .green(),
            lab_delta_e
        );
        println!();
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
pub fn color_match(color_input: &str) {
    // First, try to parse as RAL code (RAL Classic or Design System+)
    if let Some(ral_match) = try_parse_ral_color(color_input) {
        // Convert RAL match to LAB color for comprehensive analysis
        let hex_without_hash = ral_match.hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex_without_hash[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex_without_hash[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex_without_hash[4..6], 16).unwrap_or(0);

        let srgb = ColorUtils::rgb_to_srgb((r, g, b));
        let lab_color: Lab = ColorUtils::srgb_to_lab(srgb);

        // Use the RAL color name as the color name
        let ral_color_name = format!("{} ({})", ral_match.name, ral_match.code);

        format_comprehensive_report_with_unified_collections(
            lab_color,
            color_input,
            &ral_color_name,
        );
    }

    // Parse the input color
    let (lab_color, _format) = match parse_color_with_parser(color_input) {
        Ok((lab, format)) => (lab, format),
        Err(e) => {
            println!("Error parsing color: {}", e);
            return;
        }
    };

    // Get color name
    let color_name = match get_color_name_for_lab(lab_color) {
        Ok(name) => name,
        Err(e) => {
            println!("Error getting color name: {}", e);
            return;
        }
    };

    // Generate comprehensive report including RAL matches
    format_comprehensive_report_with_unified_collections(lab_color, color_input, &color_name);
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
    let (r, g, b) = ColorUtils::lab_to_rgb(lab_color);
    let parser = ColorParser::new();
    Ok(parser.get_color_name((r, g, b)))
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
    fn test_color_match() {
        // Test that color_match doesn't panic
        color_match("#FF5733");
        color_match("rgb(255, 87, 51)");
        color_match("red");
    }

    #[test]
    fn test_color_match_various_formats() {
        // Test that color_match doesn't panic for various formats
        color_match("#FF0000");
        color_match("rgb(0, 255, 0)");
        color_match("red");
        color_match("hsl(240, 100%, 50%)");
    }

    #[test]
    fn test_color_match_grayscale() {
        // Test that color_match doesn't panic for grayscale
        color_match("#808080");
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
) {
    // Use the new unified approach that includes all collections in one section
    ColorFormatter::format_comprehensive_report(lab_color, input, color_name);
}

/// Match and convert a color to all formats with comprehensive output using a custom strategy
pub fn color_match_with_strategy(
    color_input: &str,
    strategy: &dyn crate::color_distance_strategies::ColorDistanceStrategy,
) {
    // First, try to parse as RAL code (RAL Classic or Design System+)
    if let Some(ral_match) = try_parse_ral_color(color_input) {
        // Convert RAL match to LAB color for comprehensive analysis
        let hex_without_hash = ral_match.hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex_without_hash[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex_without_hash[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex_without_hash[4..6], 16).unwrap_or(0);

        let srgb = ColorUtils::rgb_to_srgb((r, g, b));
        let lab_color: Lab = ColorUtils::srgb_to_lab(srgb);

        // Use the RAL color name as the color name
        let ral_color_name = format!("{} ({})", ral_match.name, ral_match.code);

        match ColorFormatter::format_comprehensive_report_with_strategy(
            lab_color,
            color_input,
            &ral_color_name,
            strategy,
        ) {
            Ok(_) => return,
            Err(e) => {
                println!("Error generating report: {}", e);
                return;
            }
        }
    }

    // Parse the input color
    let (lab_color, _format) = match parse_color_with_parser(color_input) {
        Ok((lab, format)) => (lab, format),
        Err(e) => {
            println!("Error parsing color: {}", e);
            return;
        }
    };

    // Get color name
    let color_name = match get_color_name_for_lab(lab_color) {
        Ok(name) => name,
        Err(e) => {
            println!("Error getting color name: {}", e);
            return;
        }
    };

    // Generate comprehensive report including RAL matches
    match ColorFormatter::format_comprehensive_report_with_strategy(
        lab_color,
        color_input,
        &color_name,
        strategy,
    ) {
        Ok(report) => println!("{}", report),
        Err(e) => println!("Error generating report: {}", e),
    }
}

/// Enhanced color matching with color schemes and luminance adjustments
pub fn color_match_with_schemes(
    args: &crate::cli::ColorArgs,
    strategy: &dyn crate::color_distance_strategies::ColorDistanceStrategy,
) -> Result<String> {
    // Parse the input color
    let (lab_color, _format) = parse_color_with_parser(&args.color)?;

    // Get color name
    let color_name = get_color_name_for_lab(lab_color)?;

    // Build color scheme calculator based on arguments
    let mut scheme_builder = crate::color_schemes::ColorSchemeBuilder::new();

    // Configure luminance handling
    if let Some(relative_lum) = args.relative_luminance {
        scheme_builder = scheme_builder.with_target_relative_luminance(relative_lum);
    } else if args.relative_luminance.is_none() && args.luminance.is_some() {
        // When the flag is present without value, preserve luminance in color schemes
        scheme_builder = scheme_builder.preserve_relative_luminance();
    }

    if let Some(lab_lum) = args.luminance {
        scheme_builder = scheme_builder.with_target_lab_luminance(lab_lum);
    } else if args.luminance.is_none() {
        // When no luminance is specified, preserve relative luminance for better visual results
        scheme_builder = scheme_builder.preserve_lab_luminance();
    }

    // Calculate color schemes
    let calculator = scheme_builder.build();
    let schemes = calculator.calculate(lab_color)?;

    // Generate comprehensive report with color schemes
    format_comprehensive_report_with_schemes(schemes, &args.color, &color_name, strategy)
}

/// Generate comprehensive report with color schemes included
fn format_comprehensive_report_with_schemes(
    schemes: crate::color_schemes::ColorSchemeResult,
    input: &str,
    color_name: &str,
    strategy: &dyn crate::color_distance_strategies::ColorDistanceStrategy,
) -> Result<String> {
    // Use the strategy-aware ColorFormatter to generate the report
    ColorFormatter::format_comprehensive_report_with_strategy(
        schemes.base_color,
        input,
        color_name,
        strategy,
    )?;

    // Now add the color schemes section
    crate::color_formatter::ColorFormatter::format_color_schemes(&schemes);

    // Combine both reports
    Ok(String::new())
}
