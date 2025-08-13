//! Color operations and conversions for color-rs

use crate::color_formatter::ColorFormatter;
use crate::color_report_formatting::{lab_to_hex, lab_to_hsl_tuple, lab_to_rgb, rgb_to_srgb};
use crate::config::HEX_COLOR_LENGTH;
use crate::error::{ColorError, Result};
use crate::utils::Utils;
use palette::{IntoColor, Lab};
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
    #[must_use]
    pub fn create_color_info(label: String, lab: Lab) -> ColorInfo {
        let hex = lab_to_hex(lab);
        let rgb = lab_to_rgb(lab);
        let hsl = lab_to_hsl_tuple(lab);

        ColorInfo {
            label,
            hex,
            rgb: Utils::rgb_to_string(rgb.0, rgb.1, rgb.2),
            hsl: format!(
                "hsl({:.1}, {:.1}%, {:.1}%)",
                hsl.0,
                hsl.1 * 100.0,
                hsl.2 * 100.0
            ),
            lab: format!("lab({:.1}, {:.1}, {:.1})", lab.l, lab.a, lab.b),
        }
    }

    /// Validate a hex color string
    ///
    /// # Errors
    ///
    /// Returns `ColorError::InvalidColor` if:
    /// - Hex string is not exactly 6 characters long (excluding '#')
    /// - Hex string contains non-hexadecimal characters
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
    #[must_use]
    pub fn format_color_info(lab_color: Lab, label: &str) -> ColorInfo {
        ColorFormatter::format_color_info(lab_color, label)
    }
}

/// Match and convert a color to all formats with comprehensive output
pub fn color_match(color_input: &str) {
    // Attempt RAL direct match path (previously triggered deprecated comprehensive report output).
    if let Some(ral_match) = try_parse_ral_color(color_input) {
        let hex_without_hash = ral_match.hex.trim_start_matches('#');
        if hex_without_hash.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex_without_hash[0..2], 16),
                u8::from_str_radix(&hex_without_hash[2..4], 16),
                u8::from_str_radix(&hex_without_hash[4..6], 16),
            ) {
                let srgb = rgb_to_srgb((r, g, b));
                let lab_color: Lab = srgb.into_color();
                let ral_color_name = format!("{} ({})", ral_match.name, ral_match.code);
                // Minimal output: show converted info (placeholder for future structured output refactor)
                let info = ColorFormatter::format_color_info(lab_color, &ral_color_name);
                println!("{} | {} | {} | {} | {}", info.label, info.hex, info.rgb, info.hsl, info.lab);
            }
        }
    }

    // Parse the input color
    let (lab_color, _format) = match parse_color_with_parser(color_input) {
        Ok((lab, format)) => (lab, format),
        Err(e) => {
            println!("Error parsing color: {e}");
            return;
        }
    };

    // Get color name
    let color_name = get_color_name_for_lab(lab_color);

    // Produce a single-line informational summary replacing deprecated comprehensive output.
    let info = ColorFormatter::format_color_info(lab_color, &color_name);
    println!("{} | {} | {} | {} | {}", info.label, info.hex, info.rgb, info.hsl, info.lab);
}

/// Parse color input using the integrated parser
fn parse_color_with_parser(color_input: &str) -> Result<(Lab, crate::color_parser::ColorFormat)> {
    use crate::color_parser::ColorParser;

    let parser = ColorParser::new();
    parser.parse(color_input).map_err(|e| {
        ColorError::InvalidColor(format!("Failed to parse color '{color_input}': {e}"))
    })
}

/// Get color name for a LAB color
fn get_color_name_for_lab(lab_color: Lab) -> String {
    use crate::color_parser::ColorParser;

    // Convert LAB back to sRGB for name lookup
    let (r, g, b) = lab_to_rgb(lab_color);
    let parser = ColorParser::new();
    parser.get_color_name((r, g, b))
}

/// Parse color input from various formats
///
/// # Errors
///
/// Returns error if the input cannot be parsed as any supported color format
/// (HEX, RGB, HSL, LAB, CSS color name, or RAL color).
pub fn parse_color_input(input: &str) -> Result<Lab> {
    let (lab, _format) = parse_color_with_parser(input)?;
    Ok(lab)
}

/// Try to parse input as RAL color code or name
fn try_parse_ral_color(input: &str) -> Option<crate::color_parser::RalMatch> {
    use crate::color_parser::parse_ral_color;

    // The parse_ral_color function now handles CSS color filtering internally
    parse_ral_color(input)
}

/// Generate comprehensive report using the unified collection approach
/// Enhanced color matching with color schemes and luminance adjustments using modern `scheme_config`
///
/// # Errors
///
/// Returns an error if:
/// - The input color cannot be parsed
/// - Color scheme calculation fails
/// - Output serialization fails
pub fn color_match_with_schemes(
    args: &crate::cli::ColorArgs,
    algorithm: crate::color_distance_strategies::DistanceAlgorithm,
) -> Result<String> {
    // Parse the input color
    let (lab_color, _format) = parse_color_with_parser(&args.color)?;

    // Get color name
    let color_name = get_color_name_for_lab(lab_color);

    // Build color scheme configuration using modern immutable approach
    let scheme_config = build_scheme_config_from_args(args)?;

    // Calculate color schemes using modern approach
    let schemes = crate::scheme_config::calculate_color_schemes(scheme_config, lab_color)?;

    // Always use structured TOML/YAML output (terminal + optional file)
    format_comprehensive_report_with_structured_output(
        &schemes,
        &args.color,
        &color_name,
        algorithm,
        args,
    )
}

/// Build `ColorSchemeConfig` from command line arguments using modern immutable pattern
fn build_scheme_config_from_args(
    args: &crate::cli::ColorArgs,
) -> Result<crate::scheme_config::ColorSchemeConfig> {
    use crate::scheme_config::ColorSchemeConfig;

    // Start with default configuration
    let mut config = ColorSchemeConfig::default();

    // Configure luminance handling based on arguments
    if let Some(relative_lum) = args.relative_luminance {
        config = config.set_target_relative_luminance(relative_lum)?;
    } else if args.relative_luminance.is_none() && args.luminance.is_some() {
        // When the flag is present without value, preserve luminance in color schemes
        config = config.preserve_relative_luminance()?;
    }

    if let Some(lab_lum) = args.luminance {
        config = config.set_target_lab_luminance(lab_lum)?;
    } else if args.luminance.is_none() {
        // When no luminance is specified, preserve lab luminance for better visual results
        config = config.preserve_lab_luminance()?;
    }

    Ok(config)
}

/// Generate comprehensive report with structured TOML/YAML output for terminal and optional file
/// Generate comprehensive report with file output support
fn format_comprehensive_report_with_structured_output(
    schemes: &crate::color_schemes::ColorSchemeResult,
    input: &str,
    color_name: &str,
    algorithm: crate::color_distance_strategies::DistanceAlgorithm,
    args: &crate::cli::ColorArgs,
) -> Result<String> {
    use crate::color_report_formatting::{
        collect_analysis_data, display_terminal_output, generate_formatted_output,
        write_output_file,
    };

    // Collect and structure analysis data
    let analysis_data = collect_analysis_data(schemes, input, color_name, algorithm, args)?;

    // Determine output format (default to YAML if not specified)
    let format = args
        .output_format
        .as_ref()
        .unwrap_or(&crate::cli::OutputFormat::Yaml);

    // Generate formatted output
    let formatted_output = generate_formatted_output(&analysis_data, format)?;

    // Display structured output to terminal with colorization
    display_terminal_output(&formatted_output, format);

    // Write to file if requested
    if let Some(filename) = &args.output_file {
        write_output_file(&analysis_data, filename, format)?;
    }

    Ok(String::new())
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
