//! Simplified gradient generation module
//!
//! Cleaned up from over-engineered pattern implementation to basic functionality

// generation helpers module declared below

pub mod calculator;
pub mod easing;
pub mod output;
pub mod generation; // newly extracted pure helpers

// Functional replacements for OOP patterns
pub mod gradient_formatter;
pub mod gradient_stops;
pub mod unified_calculator;

// Simple re-exports for basic functionality
pub use calculator::{
    CalculationAlgorithm, EqualSpacingCalculator, GradientCalculator, GradientValue,
    IntelligentStopCalculator, UnifiedGradientStop, cubic_bezier_ease,
};
pub use easing::{EasingFactory, EasingFunction, EasingType};

// Functional re-exports
pub use gradient_formatter::{EventCallbacks, GradientFormat, format_gradient_with_callbacks};
pub use gradient_stops::{GradientStopCalculator, StopCalculationStrategy};
pub use unified_calculator::{calculate_unified_gradient_cfg, GradientCalculationConfig};

/// Simplified gradient generation function for CLI interface
pub fn generate_gradient(args: crate::cli::GradientArgs) -> crate::error::Result<()> {
    use crate::image::ImageGenerator;
    use crate::color_parser::unified_manager::UnifiedColorManager;

    // Parse & compute core data
    let base = crate::gradient::generation::parse_base_colors(&args)?;
    let metrics = crate::gradient::generation::compute_metrics(&base);
    let manager = UnifiedColorManager::new()?;

    // Optional image outputs (side effects isolated)
    if args.should_generate_svg() || args.should_generate_png() {
        let img = ImageGenerator::new();
        if args.should_generate_svg() { img.generate_svg(&args, base.start_lab, base.end_lab)?; println!("SVG gradient saved to: {}", args.svg_name()); }
        if args.should_generate_png() { img.generate_png(&args, base.start_lab, base.end_lab)?; println!("PNG gradient saved to: {}", args.png_name()); }
    }

    // Gradient calculation & stop construction
    let unified = crate::gradient::generation::unified_stops(&args, &base);
    let start_collections = crate::gradient::generation::find_color_collections(&manager, base.start_rgb);
    let end_collections = crate::gradient::generation::find_color_collections(&manager, base.end_rgb);
    let legacy_stops = crate::gradient::generation::build_gradient_stops(&unified, &base, &manager);
    let enhanced_stops = crate::gradient::generation::build_enhanced_stops(&unified, &base, &manager);
    let (_legacy, enhanced) = crate::gradient::generation::assemble_outputs(&args, &base, &metrics, start_collections, end_collections, legacy_stops, enhanced_stops);

    // Serialize & output
    let format = args.output_format.as_ref().unwrap_or(&crate::cli::OutputFormat::Yaml);
    let output = match format {
        crate::cli::OutputFormat::Toml => enhanced.to_toml().map_err(|e| crate::error::ColorError::InvalidArguments(format!("Failed to serialize to TOML: {e}")))?,
        crate::cli::OutputFormat::Yaml => enhanced.to_yaml().map_err(|e| crate::error::ColorError::InvalidArguments(format!("Failed to serialize to YAML: {e}")))?,
    };
    display_colorized_gradient_output(&output, format);

    if let Some(filename) = &args.output_file {
        use std::fs::File; use std::io::Write;
        let extension = match format { crate::cli::OutputFormat::Toml => "toml", crate::cli::OutputFormat::Yaml => "yaml" };
        let full_filename = if filename.contains('.') { filename.clone() } else { format!("{filename}.{extension}") };
        File::create(&full_filename)?.write_all(output.as_bytes())?;
        println!("Gradient analysis saved to: {full_filename}");
    }
    Ok(())
}

/// Display TOML/YAML output to terminal with colorization (copied from color.rs)
fn display_colorized_gradient_output(content: &str, format: &crate::cli::OutputFormat) {
    for line in content.lines() {
        let colored_line = colorize_structured_line(line, format);
        println!("{colored_line}");
    }
}

/// Colorize a single line of TOML/YAML output (copied from color.rs)
fn colorize_structured_line(line: &str, format: &crate::cli::OutputFormat) -> String {
    use colored::Colorize;

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
                format!("{}{} = {}", indent, key.green(), value)
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
                let key = &trimmed[..=colon_pos];
                let value = &trimmed[colon_pos + 2..];
                format!("{}{} {}", indent, key.green(), value)
            } else if let Some(stripped) = trimmed.strip_prefix("- ") {
                // Array items
                format!("{indent}- {stripped}")
            } else {
                line.to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_gradient() {
        // Basic test - just ensure types are accessible
        let _easing_type = EasingType::Linear;
        // Test passes if compilation succeeds
    }
}
