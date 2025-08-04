//! Color Report Formatting Functions
//!
//! This module contains functional decomposition of the long `format_comprehensive_report_with_structured_output`
//! function, breaking it down into focused, composable functions with clear separation of concerns.

use crate::cli::{ColorArgs, OutputFormat};
use crate::color_formatter::ColorFormatter;
use crate::color_schemes::ColorSchemeResult;
use crate::error::{ColorError, Result};
use crate::output_formats::{CollectionMatch, ColorSchemes, EnhancedColorSchemeItem, ColorAnalysisOutput};
use crate::color_distance_strategies::DistanceAlgorithm;
use colored::Colorize;
use std::fs::File;
use std::io::Write;
use palette::{Hsl, IntoColor, Lab, Srgb};

/// Helper functions for color space conversions

/// Convert LAB to hex string
pub fn lab_to_hex(lab: Lab) -> String {
    let srgb: Srgb = lab.into_color();
    format!(
        "#{:02X}{:02X}{:02X}",
        (srgb.red * 255.0).round().clamp(0.0, 255.0) as u8,
        (srgb.green * 255.0).round().clamp(0.0, 255.0) as u8,
        (srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8,
    )
}

/// Convert LAB to RGB tuple
pub fn lab_to_rgb(lab: Lab) -> (u8, u8, u8) {
    let srgb: Srgb = lab.into_color();
    (
        (srgb.red * 255.0).round().clamp(0.0, 255.0) as u8,
        (srgb.green * 255.0).round().clamp(0.0, 255.0) as u8,
        (srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8,
    )
}

/// Convert LAB to HSL tuple
pub fn lab_to_hsl_tuple(lab: Lab) -> (f64, f64, f64) {
    let srgb: Srgb = lab.into_color();
    let hsl: Hsl = srgb.into_color();
    (
        hsl.hue.into_positive_degrees() as f64,
        hsl.saturation as f64,
        hsl.lightness as f64,
    )
}

/// Convert RGB tuple to Srgb
pub fn rgb_to_srgb(rgb: (u8, u8, u8)) -> Srgb {
    Srgb::new(
        rgb.0 as f32 / 255.0,
        rgb.1 as f32 / 255.0,
        rgb.2 as f32 / 255.0,
    )
}

/// Convert RGB tuple to LAB
pub fn rgb_to_lab(rgb: (u8, u8, u8)) -> Lab {
    let srgb = rgb_to_srgb(rgb);
    srgb.into_color()
}

/// Collect and structure analysis data for output
pub fn collect_analysis_data(
    schemes: &ColorSchemeResult,
    input: &str,
    color_name: &str,
    algorithm: DistanceAlgorithm,
    args: &ColorArgs,
) -> Result<ColorAnalysisOutput> {
    // Collect structured data for both terminal and file output
    let mut analysis_data = ColorFormatter::collect_color_analysis_data(
        schemes.base_color,
        input,
        color_name,
        algorithm,
    )?;

    // Add color schemes data with selected strategy
    let color_schemes = collect_enhanced_color_schemes_data(schemes, &args.scheme_strategy, algorithm);
    analysis_data = analysis_data.with_color_schemes(color_schemes);

    Ok(analysis_data)
}

/// Generate formatted output based on the selected format
pub fn generate_formatted_output(
    analysis_data: &ColorAnalysisOutput,
    format: &OutputFormat,
) -> Result<String> {
    match format {
        OutputFormat::Toml => analysis_data.to_toml().map_err(|e| {
            ColorError::InvalidArguments(format!("Failed to serialize to TOML: {e}"))
        }),
        OutputFormat::Yaml => analysis_data.to_yaml().map_err(|e| {
            ColorError::InvalidArguments(format!("Failed to serialize to YAML: {e}"))
        }),
    }
}

/// Display formatted output to terminal with colorization
pub fn display_terminal_output(formatted_output: &str, format: &OutputFormat) {
    for line in formatted_output.lines() {
        let colored_line = colorize_structured_line(line, format);
        println!("{colored_line}");
    }
}

/// Write analysis data to file in the specified format
pub fn write_output_file(
    analysis_data: &ColorAnalysisOutput,
    filename: &str,
    format: &OutputFormat,
) -> Result<()> {
    match format {
        OutputFormat::Toml => write_toml_file(analysis_data, filename),
        OutputFormat::Yaml => write_yaml_file(analysis_data, filename),
    }
}

/// Write analysis data to TOML file
fn write_toml_file(
    analysis_data: &ColorAnalysisOutput,
    filename: &str,
) -> Result<()> {
    let toml_filename = ensure_file_extension(filename, "toml");
    let toml_content = analysis_data.to_toml().map_err(|e| {
        ColorError::InvalidArguments(format!("Failed to serialize to TOML: {e}"))
    })?;
    
    write_file_content(&toml_filename, &toml_content)?;
    println!("Color analysis saved to TOML file: {}", toml_filename.green());
    Ok(())
}

/// Write analysis data to YAML file
fn write_yaml_file(
    analysis_data: &ColorAnalysisOutput,
    filename: &str,
) -> Result<()> {
    let yaml_filename = ensure_yaml_extension(filename);
    let yaml_content = analysis_data.to_yaml().map_err(|e| {
        ColorError::InvalidArguments(format!("Failed to serialize to YAML: {e}"))
    })?;
    
    write_file_content(&yaml_filename, &yaml_content)?;
    println!("Color analysis saved to YAML file: {}", yaml_filename.green());
    Ok(())
}

/// Ensure filename has the correct extension
fn ensure_file_extension(filename: &str, extension: &str) -> String {
    if std::path::Path::new(filename)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case(extension))
    {
        filename.to_string()
    } else {
        format!("{filename}.{extension}")
    }
}

/// Ensure filename has YAML extension (supports both .yaml and .yml)
fn ensure_yaml_extension(filename: &str) -> String {
    if std::path::Path::new(filename)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("yaml"))
        || std::path::Path::new(filename)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("yml"))
    {
        filename.to_string()
    } else {
        format!("{filename}.yaml")
    }
}

/// Write content to file with error handling
fn write_file_content(filename: &str, content: &str) -> Result<()> {
    let mut file = File::create(filename).map_err(|e| {
        ColorError::InvalidArguments(format!("Failed to create file {filename}: {e}"))
    })?;
    
    file.write_all(content.as_bytes()).map_err(|e| {
        ColorError::InvalidArguments(format!("Failed to write to file {filename}: {e}"))
    })?;
    
    Ok(())
}

/// Colorize a single line of TOML/YAML output
pub fn colorize_structured_line(line: &str, format: &OutputFormat) -> String {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];

    match format {
        OutputFormat::Toml => colorize_toml_line(indent, trimmed),
        OutputFormat::Yaml => colorize_yaml_line(indent, trimmed),
    }
}

/// Colorize TOML format lines
fn colorize_toml_line(indent: &str, trimmed: &str) -> String {
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
        format!("{}{} = {}", indent, key.green(), value)
    } else {
        format!("{indent}{trimmed}")
    }
}

/// Colorize YAML format lines
fn colorize_yaml_line(indent: &str, trimmed: &str) -> String {
    if trimmed.ends_with(':') && !trimmed.contains(' ') {
        // Top-level keys like "metadata:", "conversion:"
        format!("{}{}", indent, trimmed.bold().cyan())
    } else if let Some(colon_pos) = trimmed.find(": ") {
        // Key: value pairs
        let key = &trimmed[..=colon_pos];
        let value = &trimmed[colon_pos + 2..];
        format!("{}{} {}", indent, key.green(), value)
    } else if let Some(stripped) = trimmed.strip_prefix("- ") {
        // Array items
        format!("{indent}- {stripped}")
    } else {
        format!("{indent}{trimmed}")
    }
}

/// Collect enhanced color schemes data for new flattened file output
pub fn collect_enhanced_color_schemes_data(
    schemes: &ColorSchemeResult,
    strategy: &str,
    distance_algorithm: DistanceAlgorithm,
) -> ColorSchemes {
    use crate::color_parser::unified_manager::UnifiedColorManager;
    use palette::Lab;

    // Create manager for color matching with strategy support
    let manager = UnifiedColorManager::new().unwrap_or_default();

    /// Convert a Lab color to an `EnhancedColorSchemeItem` with full color information
    fn lab_to_enhanced_item(
        color: Lab,
        manager: &UnifiedColorManager,
        distance_algorithm: DistanceAlgorithm,
    ) -> EnhancedColorSchemeItem {
        use crate::color_parser::UniversalColor;

        let hex = lab_to_hex(color);
        let hsl_tuple = lab_to_hsl_tuple(color);
        let hsl = format!(
            "hsl({:.1}, {:.2}%, {:.2}%)",
            hsl_tuple.0,
            hsl_tuple.1 * 100.0,
            hsl_tuple.2 * 100.0
        );
        let lch = crate::format_utils::FormatUtils::lab_to_lch(color);

        // Get color name information with enhanced collection matches
        let (r, g, b) = lab_to_rgb(color);
        let target = UniversalColor::from_rgb([r, g, b]);

        // Get collection matches
        let css_match = get_closest_css_match(&target, manager, distance_algorithm);
        let ral_classic_match = get_closest_ral_classic_match(&target, manager, distance_algorithm);
        let ral_design_match = get_closest_ral_design_match(&target, manager, distance_algorithm);

        EnhancedColorSchemeItem {
            hex,
            hsl,
            lch,
            css: css_match,
            ral_classic: ral_classic_match,
            ral_design: ral_design_match,
        }
    }

    // Select the appropriate strategy schemes
    let selected_schemes = match strategy {
        "hsl" => (
            schemes.hsl_complementary,
            schemes.hsl_split_complementary,
            schemes.hsl_triadic,
            schemes.hsl_tetradic,
        ),
        _ => (
            schemes.lab_complementary,
            schemes.lab_split_complementary,
            schemes.lab_triadic,
            schemes.lab_tetradic,
        ),
    };

    ColorSchemes {
        complementary: lab_to_enhanced_item(selected_schemes.0, &manager, distance_algorithm),
        split_complementary: vec![
            lab_to_enhanced_item(selected_schemes.1.0, &manager, distance_algorithm),
            lab_to_enhanced_item(selected_schemes.1.1, &manager, distance_algorithm),
        ],
        triadic: vec![
            lab_to_enhanced_item(selected_schemes.2.0, &manager, distance_algorithm),
            lab_to_enhanced_item(selected_schemes.2.1, &manager, distance_algorithm),
        ],
        tetradic: vec![
            lab_to_enhanced_item(selected_schemes.3.0, &manager, distance_algorithm),
            lab_to_enhanced_item(selected_schemes.3.1, &manager, distance_algorithm),
            lab_to_enhanced_item(selected_schemes.3.2, &manager, distance_algorithm),
        ],
    }
}

/// Get closest CSS collection match using distance strategy
fn get_closest_css_match(
    target: &crate::color_parser::UniversalColor,
    manager: &crate::color_parser::unified_manager::UnifiedColorManager,
    distance_algorithm: DistanceAlgorithm,
) -> Option<CollectionMatch> {
    let rgb = [target.rgb[0], target.rgb[1], target.rgb[2]];
    let matches = manager.find_closest_css_colors_with_algorithm(rgb, 1, distance_algorithm);
    
    if let Some(closest) = matches.first() {
        let target_lab = rgb_to_lab((target.rgb[0], target.rgb[1], target.rgb[2]));
        let closest_lab = rgb_to_lab((
            closest.entry.color.rgb[0],
            closest.entry.color.rgb[1],
            closest.entry.color.rgb[2],
        ));
        let distance = crate::color_distance_strategies::calculate_distance(distance_algorithm, target_lab, closest_lab);
        let srgb = rgb_to_srgb((
            closest.entry.color.rgb[0],
            closest.entry.color.rgb[1],
            closest.entry.color.rgb[2],
        ));
        let wcag_relative_luminance = crate::color_ops::luminance::wcag_relative(srgb);
        Some(CollectionMatch {
            name: closest.entry.metadata.name.clone(),
            hex: format!(
                "#{:02X}{:02X}{:02X}",
                closest.entry.color.rgb[0],
                closest.entry.color.rgb[1],
                closest.entry.color.rgb[2]
            ),
            distance,
            wcag_relative_luminance,
        })
    } else {
        None
    }
}

/// Get closest RAL Classic collection match using distance strategy
fn get_closest_ral_classic_match(
    target: &crate::color_parser::UniversalColor,
    manager: &crate::color_parser::unified_manager::UnifiedColorManager,
    distance_algorithm: DistanceAlgorithm,
) -> Option<CollectionMatch> {
    let rgb = [target.rgb[0], target.rgb[1], target.rgb[2]];
    let matches = manager.find_closest_ral_classic_with_algorithm(rgb, 1, distance_algorithm);
    
    if let Some(closest) = matches.first() {
        let target_lab = rgb_to_lab((target.rgb[0], target.rgb[1], target.rgb[2]));
        let closest_lab = rgb_to_lab((
            closest.entry.color.rgb[0],
            closest.entry.color.rgb[1],
            closest.entry.color.rgb[2],
        ));
        let distance = crate::color_distance_strategies::calculate_distance(distance_algorithm, target_lab, closest_lab);
        let srgb = rgb_to_srgb((
            closest.entry.color.rgb[0],
            closest.entry.color.rgb[1],
            closest.entry.color.rgb[2],
        ));
        let wcag_relative_luminance = crate::color_ops::luminance::wcag_relative(srgb);
        Some(CollectionMatch {
            name: closest.entry.metadata.name.clone(),
            hex: format!(
                "#{:02X}{:02X}{:02X}",
                closest.entry.color.rgb[0],
                closest.entry.color.rgb[1],
                closest.entry.color.rgb[2]
            ),
            distance,
            wcag_relative_luminance,
        })
    } else {
        None
    }
}

/// Get closest RAL Design collection match using distance strategy
fn get_closest_ral_design_match(
    target: &crate::color_parser::UniversalColor,
    manager: &crate::color_parser::unified_manager::UnifiedColorManager,
    distance_algorithm: DistanceAlgorithm,
) -> Option<CollectionMatch> {
    let target_rgb = extract_target_rgb(target);
    let matches = find_ral_design_matches(manager, target_rgb, distance_algorithm);
    
    matches.first().map(|closest| {
        create_collection_match_from_ral_entry(target, closest, distance_algorithm)
    })
}

/// Extract RGB values from target color as array
fn extract_target_rgb(target: &crate::color_parser::UniversalColor) -> [u8; 3] {
    [target.rgb[0], target.rgb[1], target.rgb[2]]
}

/// Find closest RAL Design matches using the specified algorithm
fn find_ral_design_matches(
    manager: &crate::color_parser::unified_manager::UnifiedColorManager,
    target_rgb: [u8; 3],
    distance_algorithm: DistanceAlgorithm,
) -> Vec<crate::color_parser::collections::ColorMatch> {
    manager.find_closest_ral_design_with_algorithm(target_rgb, 1, distance_algorithm)
}

/// Create a CollectionMatch from a RAL Design entry
fn create_collection_match_from_ral_entry(
    target: &crate::color_parser::UniversalColor,
    closest: &crate::color_parser::collections::ColorMatch,
    distance_algorithm: DistanceAlgorithm,
) -> CollectionMatch {
    let target_lab = convert_target_to_lab(target);
    let closest_lab = convert_ral_entry_to_lab(closest);
    let distance = calculate_color_distance(distance_algorithm, target_lab, closest_lab);
    let wcag_luminance = calculate_wcag_luminance(closest);
    let hex_string = format_rgb_as_hex(closest);

    CollectionMatch {
        name: closest.entry.metadata.name.clone(),
        hex: hex_string,
        distance,
        wcag_relative_luminance: wcag_luminance,
    }
}

/// Convert target color to LAB color space
fn convert_target_to_lab(target: &crate::color_parser::UniversalColor) -> Lab<palette::white_point::D65> {
    rgb_to_lab((target.rgb[0], target.rgb[1], target.rgb[2]))
}

/// Convert RAL entry color to LAB color space
fn convert_ral_entry_to_lab(closest: &crate::color_parser::collections::ColorMatch) -> Lab<palette::white_point::D65> {
    rgb_to_lab((
        closest.entry.color.rgb[0],
        closest.entry.color.rgb[1],
        closest.entry.color.rgb[2],
    ))
}

/// Calculate distance between two LAB colors
fn calculate_color_distance(
    algorithm: DistanceAlgorithm,
    target_lab: Lab<palette::white_point::D65>,
    closest_lab: Lab<palette::white_point::D65>,
) -> f64 {
    crate::color_distance_strategies::calculate_distance(algorithm, target_lab, closest_lab)
}

/// Calculate WCAG relative luminance for RAL entry
fn calculate_wcag_luminance(closest: &crate::color_parser::collections::ColorMatch) -> f64 {
    let srgb = rgb_to_srgb((
        closest.entry.color.rgb[0],
        closest.entry.color.rgb[1],
        closest.entry.color.rgb[2],
    ));
    crate::color_ops::luminance::wcag_relative(srgb)
}

/// Format RGB values as hex string
fn format_rgb_as_hex(closest: &crate::color_parser::collections::ColorMatch) -> String {
    format!(
        "#{:02X}{:02X}{:02X}",
        closest.entry.color.rgb[0],
        closest.entry.color.rgb[1],
        closest.entry.color.rgb[2]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_file_extension() {
        assert_eq!(ensure_file_extension("test", "toml"), "test.toml");
        assert_eq!(ensure_file_extension("test.toml", "toml"), "test.toml");
        assert_eq!(ensure_file_extension("test.TOML", "toml"), "test.TOML");
    }

    #[test]
    fn test_ensure_yaml_extension() {
        assert_eq!(ensure_yaml_extension("test"), "test.yaml");
        assert_eq!(ensure_yaml_extension("test.yaml"), "test.yaml");
        assert_eq!(ensure_yaml_extension("test.yml"), "test.yml");
        assert_eq!(ensure_yaml_extension("test.YAML"), "test.YAML");
    }

    #[test]
    fn test_toml_line_colorization() {
        let line = "key = value";
        let result = colorize_toml_line("", line);
        // Test that the function processes the line without panicking
        assert!(result.contains("key"));
        assert!(result.contains("value"));
    }

    #[test]
    fn test_yaml_line_colorization() {
        let line = "key: value";
        let result = colorize_yaml_line("", line);
        // Test that the function processes the line without panicking
        assert!(result.contains("key"));
        assert!(result.contains("value"));
    }
}
