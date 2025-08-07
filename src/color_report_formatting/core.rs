//! Core formatting functions
//!
//! Contains the main data collection and formatting functions for color analysis output.
//! Handles structured data preparation and format conversion.

use crate::cli::{ColorArgs, OutputFormat};
use crate::color_distance_strategies::DistanceAlgorithm;
use crate::color_formatter::ColorFormatter;
use crate::color_schemes::ColorSchemeResult;
use crate::error::{ColorError, Result};
use crate::output_formats::ColorAnalysisOutput;
use palette::{Hsl, IntoColor, Lab, Srgb};

/// Convert LAB to hex color string
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // Safe: values clamped to [0.0, 255.0] range
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
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // Safe: values clamped to [0.0, 255.0] range
pub fn lab_to_rgb(lab: Lab) -> (u8, u8, u8) {
    let srgb: Srgb = lab.into_color();
    (
        (srgb.red * 255.0).round().clamp(0.0, 255.0) as u8,
        (srgb.green * 255.0).round().clamp(0.0, 255.0) as u8,
        (srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8,
    )
}

/// Convert LAB to HSL tuple
#[must_use]
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
#[must_use]
pub fn rgb_to_srgb(rgb: (u8, u8, u8)) -> Srgb {
    Srgb::new(
        f32::from(rgb.0) / 255.0,
        f32::from(rgb.1) / 255.0,
        f32::from(rgb.2) / 255.0,
    )
}

/// Convert RGB tuple to LAB
#[must_use]
pub fn rgb_to_lab(rgb: (u8, u8, u8)) -> Lab {
    let srgb = rgb_to_srgb(rgb);
    srgb.into_color()
}

/// Collect and structure analysis data for output
///
/// # Errors
/// Returns an error if color scheme analysis or data collection fails
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
    let color_schemes = super::utilities::collect_enhanced_color_schemes_data(
        schemes,
        &args.scheme_strategy,
        algorithm,
    );
    analysis_data = analysis_data.with_color_schemes(color_schemes);

    Ok(analysis_data)
}

/// Generate formatted output based on the selected format
///
/// # Errors
/// Returns an error if output serialization fails for the selected format
pub fn generate_formatted_output(
    analysis_data: &ColorAnalysisOutput,
    format: &OutputFormat,
) -> Result<String> {
    match format {
        OutputFormat::Toml => analysis_data
            .to_toml()
            .map_err(|e| ColorError::InvalidArguments(format!("Failed to serialize to TOML: {e}"))),
        OutputFormat::Yaml => analysis_data
            .to_yaml()
            .map_err(|e| ColorError::InvalidArguments(format!("Failed to serialize to YAML: {e}"))),
    }
}
