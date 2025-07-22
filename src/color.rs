//! Color operations and conversions for color-rs

use crate::color_formatter::ColorFormatter;
use crate::color_parser::{ColorCollection, UniversalColor};
use crate::color_utils::*;
use crate::config::HEX_COLOR_LENGTH;
use crate::error::{ColorError, Result};
use crate::utils::Utils;
use palette::Lab;
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

        // DUPLICATION ELIMINATED: Direct call to ColorFormatter instead of wrapper
        let _ = ColorFormatter::format_comprehensive_report(lab_color, color_input, &ral_color_name);
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

    // DUPLICATION ELIMINATED: Direct call to ColorFormatter instead of wrapper
    let _ = ColorFormatter::format_comprehensive_report(lab_color, color_input, &color_name);
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

    // Always use structured TOML/YAML output (terminal + optional file)
    format_comprehensive_report_with_structured_output(
        schemes,
        &args.color,
        &color_name,
        strategy,
        args,
    )
}

/// Generate comprehensive report with structured TOML/YAML output for terminal and optional file
/// Generate comprehensive report with file output support
fn format_comprehensive_report_with_structured_output(
    schemes: crate::color_schemes::ColorSchemeResult,
    input: &str,
    color_name: &str,
    strategy: &dyn crate::color_distance_strategies::ColorDistanceStrategy,
    args: &crate::cli::ColorArgs,
) -> Result<String> {
    use crate::color_formatter::ColorFormatter;
    use crate::file_output::FileOutputService;

    // Collect structured data for both terminal and file output
    let mut analysis_data = ColorFormatter::collect_color_analysis_data(
        schemes.base_color,
        input,
        color_name,
        strategy,
    )?;

    // Add color schemes data with selected strategy
    let color_schemes = collect_enhanced_color_schemes_data(&schemes, &args.scheme_strategy);
    analysis_data = analysis_data.with_color_schemes(color_schemes);

    // Determine output format (default to TOML if not specified)
    let format = args
        .output_format
        .as_ref()
        .unwrap_or(&crate::cli::OutputFormat::Yaml);

    // Create output service and generate formatted output
    let formatted_output = match format {
        crate::cli::OutputFormat::Toml => analysis_data.to_toml().map_err(|e| {
            crate::error::ColorError::InvalidArguments(format!(
                "Failed to serialize to TOML: {}",
                e
            ))
        })?,
        crate::cli::OutputFormat::Yaml => analysis_data.to_yaml().map_err(|e| {
            crate::error::ColorError::InvalidArguments(format!(
                "Failed to serialize to YAML: {}",
                e
            ))
        })?,
    };

    // Display structured output to terminal with colorization
    display_colorized_structured_output(&formatted_output, format);

    // Write to file if requested
    if let Some(filename) = &args.output_file {
        use crate::cli::OutputFormat;
        use colored::*;

        match format {
            OutputFormat::Toml => {
                let toml_filename = if filename.ends_with(".toml") {
                    filename.clone()
                } else {
                    format!("{}.toml", filename)
                };
                FileOutputService::write_toml(&analysis_data, &toml_filename)?;
                println!(
                    "Color analysis saved to TOML file: {}",
                    toml_filename.green()
                );
            }
            OutputFormat::Yaml => {
                let yaml_filename = if filename.ends_with(".yaml") || filename.ends_with(".yml") {
                    filename.clone()
                } else {
                    format!("{}.yaml", filename)
                };
                FileOutputService::write_yaml(&analysis_data, &yaml_filename)?;
                println!(
                    "Color analysis saved to YAML file: {}",
                    yaml_filename.green()
                );
            }
        }
    }

    Ok(String::new())
}

/// Display TOML/YAML output to terminal with colorization
fn display_colorized_structured_output(content: &str, format: &crate::cli::OutputFormat) {
    use colored::*;

    println!(
        "{}",
        format!(
            "# {} OUTPUT",
            match format {
                crate::cli::OutputFormat::Toml => "TOML",
                crate::cli::OutputFormat::Yaml => "YAML",
            }
        )
        .bold()
        .blue()
    );

    for line in content.lines() {
        let colored_line = colorize_structured_line(line, format);
        println!("{}", colored_line);
    }
}

/// Colorize a single line of TOML/YAML output
fn colorize_structured_line(line: &str, format: &crate::cli::OutputFormat) -> String {
    use colored::*;

    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];

    match format {
        crate::cli::OutputFormat::Toml => {
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                // Section headers like [metadata], [conversion]
                format!("{}{}", indent, trimmed.bold().cyan())
            } else if trimmed.starts_with("[[") && trimmed.ends_with("]]") {
                // Array section headers like [[color_collections.css_colors]]
                format!("{}{}", indent, trimmed.bold().magenta())
            } else if let Some(eq_pos) = trimmed.find(" = ") {
                // Key = value pairs
                let key = &trimmed[..eq_pos];
                let value = &trimmed[eq_pos + 3..];
                format!(
                    "{}{} = {}",
                    indent,
                    key.green(),
                    crate::output_utils::OutputUtils::colorize_output_value(value)
                )
            } else {
                line.to_string()
            }
        }
        crate::cli::OutputFormat::Yaml => {
            if trimmed.ends_with(':') && !trimmed.contains(' ') {
                // Top-level keys like "metadata:", "conversion:"
                format!("{}{}", indent, trimmed.bold().cyan())
            } else if let Some(colon_pos) = trimmed.find(": ") {
                // Key: value pairs
                let key = &trimmed[..colon_pos + 1];
                let value = &trimmed[colon_pos + 2..];
                format!(
                    "{}{} {}",
                    indent,
                    key.green(),
                    crate::output_utils::OutputUtils::colorize_output_value(value)
                )
            } else if let Some(stripped) = trimmed.strip_prefix("- ") {
                // Array items
                format!(
                    "{}- {}",
                    indent,
                    crate::output_utils::OutputUtils::colorize_output_value(stripped)
                )
            } else {
                line.to_string()
            }
        }
    }
}

/// Colorize values based on their type
/// Collect enhanced color schemes data for new flattened file output
fn collect_enhanced_color_schemes_data(
    schemes: &crate::color_schemes::ColorSchemeResult,
    strategy: &str,
) -> crate::output_formats::ColorSchemes {
    use crate::color_parser::ColorParser;
    use crate::color_utils::ColorUtils;
    use crate::output_formats::{
        CollectionMatch, EnhancedColorSchemeItem, ColorSchemes,
    };
    use palette::Lab;

    // Create parser for color name matching
    let parser = ColorParser::new();

    // Select the appropriate strategy schemes
    let selected_schemes = match strategy {
        "hsl" => (
            schemes.hsl_complementary,
            schemes.hsl_split_complementary,
            schemes.hsl_triadic,
            schemes.hsl_tetradic,
        ),
        "lab" | _ => (
            schemes.lab_complementary,
            schemes.lab_split_complementary,
            schemes.lab_triadic,
            schemes.lab_tetradic,
        ),
    };

    /// Convert a Lab color to an EnhancedColorSchemeItem with full color information
    fn lab_to_enhanced_color_scheme_item(lab: Lab, parser: &ColorParser) -> EnhancedColorSchemeItem {
        let hex = ColorUtils::lab_to_hex(lab);
        let hsl_tuple = ColorUtils::lab_to_hsl_tuple(lab);
        let hsl = format!(
            "hsl({:.1}, {:.2}%, {:.2}%)",
            hsl_tuple.0,
            hsl_tuple.1 * 100.0,
            hsl_tuple.2 * 100.0
        );
        let lch = crate::format_utils::FormatUtils::lab_to_lch(lab);

        // Get color name information with enhanced collection matches
        let (r, g, b) = ColorUtils::lab_to_rgb(lab);
        let (css_match, ral_classic_match, ral_design_match) = get_collection_matches((r, g, b), parser);

        EnhancedColorSchemeItem {
            hex,
            hsl,
            lch,
            css: css_match,
            ral_classic: ral_classic_match,
            ral_design: ral_design_match,
        }
    }

    /// Get collection matches for all color collections
    fn get_collection_matches(rgb: (u8, u8, u8), parser: &ColorParser) -> (Option<CollectionMatch>, Option<CollectionMatch>, Option<CollectionMatch>) {
        let target = UniversalColor::from_rgb([rgb.0, rgb.1, rgb.2]);
        let input_hex = format!("#{:02X}{:02X}{:02X}", rgb.0, rgb.1, rgb.2);

        // CSS collection matches
        let css_match = {
            let css_collection = parser.css_collection();
            let exact_css = css_collection.colors().iter().find(|color| {
                let color_hex = format!(
                    "#{:02X}{:02X}{:02X}",
                    color.color.rgb[0], color.color.rgb[1], color.color.rgb[2]
                );
                color_hex.to_uppercase() == input_hex.to_uppercase()
            });

            if let Some(exact) = exact_css {
                let wcag_luminance = ColorUtils::wcag_relative_luminance_rgb((exact.color.rgb[0], exact.color.rgb[1], exact.color.rgb[2]));
                let exact_hex = format!("#{:02X}{:02X}{:02X}", exact.color.rgb[0], exact.color.rgb[1], exact.color.rgb[2]);
                Some(CollectionMatch {
                    name: exact.metadata.name.clone(),
                    hex: exact_hex,
                    distance: 0.0,
                    wcag_relative_luminance: wcag_luminance,
                })
            } else {
                let nearest_css = css_collection.find_closest(&target, 1, None);
                if let Some(closest) = nearest_css.first() {
                    let wcag_luminance = ColorUtils::wcag_relative_luminance_rgb((closest.entry.color.rgb[0], closest.entry.color.rgb[1], closest.entry.color.rgb[2]));
                    let nearest_hex = format!("#{:02X}{:02X}{:02X}", closest.entry.color.rgb[0], closest.entry.color.rgb[1], closest.entry.color.rgb[2]);
                    Some(CollectionMatch {
                        name: closest.entry.metadata.name.clone(),
                        hex: nearest_hex,
                        distance: closest.distance,
                        wcag_relative_luminance: wcag_luminance,
                    })
                } else {
                    None
                }
            }
        };

        // RAL Classic collection matches
        let ral_classic_match = {
            let ral_classic_matches =
                parser.find_closest_ral_classic([target.rgb[0], target.rgb[1], target.rgb[2]], 1);
            if let Some(closest) = ral_classic_matches.first() {
                let color_hex = format!(
                    "#{:02X}{:02X}{:02X}",
                    closest.entry.color.rgb[0],
                    closest.entry.color.rgb[1],
                    closest.entry.color.rgb[2]
                );
                let distance = if color_hex.to_uppercase() == input_hex.to_uppercase() {
                    0.0
                } else {
                    closest.distance
                };
                let wcag_luminance = ColorUtils::wcag_relative_luminance_rgb((closest.entry.color.rgb[0], closest.entry.color.rgb[1], closest.entry.color.rgb[2]));

                Some(CollectionMatch {
                    name: closest.entry.metadata.name.clone(),
                    hex: color_hex,
                    distance,
                    wcag_relative_luminance: wcag_luminance,
                })
            } else {
                None
            }
        };

        // RAL Design collection matches
        let ral_design_match = {
            let ral_design_matches =
                parser.find_closest_ral_design([target.rgb[0], target.rgb[1], target.rgb[2]], 1);
            if let Some(closest) = ral_design_matches.first() {
                let color_hex = format!(
                    "#{:02X}{:02X}{:02X}",
                    closest.entry.color.rgb[0],
                    closest.entry.color.rgb[1],
                    closest.entry.color.rgb[2]
                );
                let distance = if color_hex.to_uppercase() == input_hex.to_uppercase() {
                    0.0
                } else {
                    closest.distance
                };
                let wcag_luminance = ColorUtils::wcag_relative_luminance_rgb((closest.entry.color.rgb[0], closest.entry.color.rgb[1], closest.entry.color.rgb[2]));

                Some(CollectionMatch {
                    name: closest.entry.metadata.name.clone(),
                    hex: color_hex,
                    distance,
                    wcag_relative_luminance: wcag_luminance,
                })
            } else {
                None
            }
        };

        (css_match, ral_classic_match, ral_design_match)
    }

    ColorSchemes {
        complementary: lab_to_enhanced_color_scheme_item(selected_schemes.0, &parser),
        split_complementary: vec![
            lab_to_enhanced_color_scheme_item(selected_schemes.1.0, &parser),
            lab_to_enhanced_color_scheme_item(selected_schemes.1.1, &parser),
        ],
        triadic: vec![
            lab_to_enhanced_color_scheme_item(selected_schemes.2.0, &parser),
            lab_to_enhanced_color_scheme_item(selected_schemes.2.1, &parser),
        ],
        tetradic: vec![
            lab_to_enhanced_color_scheme_item(selected_schemes.3.0, &parser),
            lab_to_enhanced_color_scheme_item(selected_schemes.3.1, &parser),
            lab_to_enhanced_color_scheme_item(selected_schemes.3.2, &parser),
        ],
    }
}
