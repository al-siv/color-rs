//! Core formatting functions
//!
//! Contains the main data collection and formatting functions for color analysis output.
//! Handles structured data preparation and format conversion.

use crate::cli::{ColorArgs, OutputFormat};
use crate::color_formatter::ColorFormatter;
use crate::color_schemes::ColorSchemeResult;
use crate::error::{ColorError, Result};
use crate::output_formats::ColorAnalysisOutput;
use crate::color_distance_strategies::DistanceAlgorithm;
use palette::{Hsl, IntoColor, Lab, Srgb};

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
    let color_schemes = super::utilities::collect_enhanced_color_schemes_data(schemes, &args.scheme_strategy, algorithm);
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
