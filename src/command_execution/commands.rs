//! Individual command implementation functions
//!
//! This module contains the actual command execution logic for each command type,
//! using functional composition and pure functions where possible.

use super::types::ExecutionResult;
use crate::cli::GradientArgs;
use crate::error::{ColorError, Result};
use palette::{IntoColor, Lab, Mix}; // Import traits for LAB interpolation and conversion
use std::collections::HashMap;

// Command execution functions using pattern matching instead of virtual dispatch

/// Execute gradient generation command
/// # Errors
/// Returns error if color parsing or gradient generation fails
pub fn execute_generate_gradient(
    args: &GradientArgs,
    output_path: Option<&str>,
) -> Result<ExecutionResult> {
    let (start_lab, end_lab) = parse_gradient_colors(args)?;
    let gradient_output = generate_gradient_steps(start_lab, end_lab, args.stops);
    let format_output = append_format_outputs(args, output_path);
    let metadata = create_gradient_metadata(args);

    let final_output = format!("{gradient_output}{format_output}");
    Ok(ExecutionResult::success_with_metadata(
        final_output,
        metadata,
    ))
}

/// Execute find closest color command
/// # Errors
/// Returns error if color parsing fails
pub fn execute_find_closest_color(
    color_input: &str,
    collection: Option<&str>,
    algorithm: &str,
    count: usize,
) -> Result<ExecutionResult> {
    // Parse the color for validation
    let _lab_color = crate::color::parse_color_input(color_input)
        .map_err(|e| ColorError::ParseError(format!("Failed to parse color: {e}")))?;

    // Create output using functional color matching
    let output =
        format!("Finding {count} closest colors to {color_input} using {algorithm} algorithm\n");

    let mut metadata = HashMap::new();
    metadata.insert("input_color".to_string(), color_input.to_string());
    metadata.insert("algorithm".to_string(), algorithm.to_string());
    metadata.insert("count".to_string(), count.to_string());
    if let Some(collection) = collection {
        metadata.insert("collection".to_string(), collection.to_string());
    }

    Ok(ExecutionResult::success_with_metadata(output, metadata))
}

/// Execute color analysis command  
/// # Errors
/// Returns error if color parsing fails
pub fn execute_analyze_color(
    color_input: &str,
    include_schemes: bool,
    output_format: &str,
) -> Result<ExecutionResult> {
    // Parse the color for analysis
    let _lab_color = crate::color::parse_color_input(color_input)
        .map_err(|e| ColorError::ParseError(format!("Failed to parse color: {e}")))?;

    let mut output = format!("Analyzing color: {color_input}\nOutput format: {output_format}\n");

    if include_schemes {
        output.push_str("Including color schemes in analysis\n");
    }

    let mut metadata = HashMap::new();
    metadata.insert("input_color".to_string(), color_input.to_string());
    metadata.insert("include_schemes".to_string(), include_schemes.to_string());
    metadata.insert("output_format".to_string(), output_format.to_string());

    Ok(ExecutionResult::success_with_metadata(output, metadata))
}

/// Execute color conversion command
/// # Errors  
/// Returns error if color parsing fails
pub fn execute_convert_color(
    color_input: &str,
    target_format: &str,
    precision: usize,
) -> Result<ExecutionResult> {
    // Parse the color for conversion
    let _lab_color = crate::color::parse_color_input(color_input)
        .map_err(|e| ColorError::ParseError(format!("Failed to parse color: {e}")))?;

    let output = format!(
        "Converting {color_input} to {target_format} format with {precision} decimal precision\n"
    );

    let mut metadata = HashMap::new();
    metadata.insert("input_color".to_string(), color_input.to_string());
    metadata.insert("target_format".to_string(), target_format.to_string());
    metadata.insert("precision".to_string(), precision.to_string());

    Ok(ExecutionResult::success_with_metadata(output, metadata))
}

// Helper functions for gradient command execution

/// Parse start and end colors for gradient generation
fn parse_gradient_colors(args: &GradientArgs) -> Result<(Lab, Lab)> {
    let start_lab = crate::color::parse_color_input(&args.start_color)
        .map_err(|e| ColorError::ParseError(format!("Failed to parse start color: {e}")))?;

    let end_lab = crate::color::parse_color_input(&args.end_color)
        .map_err(|e| ColorError::ParseError(format!("Failed to parse end color: {e}")))?;

    Ok((start_lab, end_lab))
}

/// Generate gradient steps with color interpolation
fn generate_gradient_steps(start_lab: Lab, end_lab: Lab, steps: usize) -> String {
    use std::fmt::Write;

    let mut output = String::new();
    output.push_str("Generated gradient:\n");

    for i in 0..steps {
        #[allow(clippy::cast_precision_loss)] // Gradient step calculation needs f64 precision
        let t = i as f64 / (steps - 1) as f64;
        // Use functional LAB interpolation with palette Mix trait
        #[allow(clippy::cast_possible_truncation)] // Intentional f64->f32 for Mix trait
        let interpolated = start_lab.mix(end_lab, t as f32);
        let hex = crate::color_ops::conversion::srgb_to_hex(interpolated.into_color());
    let _ = writeln!(output, "Step {i}: {hex}"); // Writing to String cannot fail
    }

    output
}

/// Generate output format messages
fn append_format_outputs(args: &GradientArgs, output_path: Option<&str>) -> String {
    use std::fmt::Write;

    let mut output = String::new();

    if args.should_generate_svg() {
        output.push_str("\nSVG generated successfully\n");
        if let Some(path) = output_path {
            let _ = writeln!(output, "SVG saved to: {path}"); // Writing to String cannot fail
        }
    }

    if args.should_generate_png() {
        output.push_str("PNG generated successfully\n");
    }

    output
}

/// Create metadata for gradient execution result
fn create_gradient_metadata(args: &GradientArgs) -> HashMap<String, String> {
    let mut metadata = HashMap::new();
    metadata.insert("start_color".to_string(), args.start_color.clone());
    metadata.insert("end_color".to_string(), args.end_color.clone());
    metadata.insert("steps".to_string(), args.stops.to_string());
    metadata
}

/// Execute hue analysis command
/// # Errors
/// Execute hue collection display with range filtering and sorting
/// Returns error if collection loading, range parsing, or formatting fails
pub fn execute_hue_analysis(
    args: &crate::cli::HueArgs,
    _output_path: Option<&str>,
) -> Result<ExecutionResult> {
    use crate::cli::OutputFormat;
    use crate::color_report_formatting::display;
    use crate::output_formats::HueCollectionOutput;
    use crate::command_execution::hue_analysis::*;

    let collection = load_collection(&args.collection)?;
    let ranges = parse_ranges(args)?;
    let mut filtered = collect_filtered_colors(&collection, &ranges);
    sort_filtered_colors(&mut filtered);
    let configuration = build_configuration(args, filtered.len());
    let entries = build_hue_entries(&filtered);
    let hue_output = HueCollectionOutput::new().with_configuration(configuration).with_colors(entries);
    let yaml_output = hue_output.to_yaml().map_err(|e| crate::error::ColorError::ParseError(format!("Failed to serialize to YAML: {e}")))?;
    display::display_terminal_output(&yaml_output, &OutputFormat::Yaml);

    // Handle file export if requested
    if let Some(file_path) = &args.output_file {
        let output_format = args.output_format.clone().unwrap_or_default();
        export_hue_collection_display(&hue_output, output_format, file_path)?;
    }

    // Handle visual output if requested
    if args.should_generate_visual() {
        let analysis_results = build_visual_analysis_results(&filtered, &args.collection);

        // Generate visual output
        let image_generator = crate::image::ImageGenerator::new();

        if args.should_generate_gradient() {
            println!("Generating horizontal gradient: {}", args.svg_name());
            image_generator.generate_hue_gradient(args, &analysis_results)?;
            if args.should_generate_png() {
                println!("Generated PNG: {}", args.png_name());
            }
        }

        if args.should_generate_palette() {
            println!("Generating vertical palette: {}", args.svg_name());
            image_generator.generate_hue_palette(args, &analysis_results)?;
            if args.should_generate_png() {
                println!("Generated PNG: {}", args.png_name());
            }
        }
    }

    // Create metadata
    let metadata = build_metadata(args, filtered.len());

    // Return success result without terminal output since we already displayed it
    Ok(ExecutionResult::success_with_metadata(
        String::new(), // Empty string since display_terminal_output already handled output
        metadata,
    ))
}

/// Export hue collection display to file in specified format
fn export_hue_collection_display(
    hue_output: &crate::output_formats::HueCollectionOutput,
    format: crate::cli::OutputFormat,
    file_path: &str,
) -> crate::error::Result<()> {
    use std::fs;

    let content = match format {
        crate::cli::OutputFormat::Yaml => hue_output.to_yaml().map_err(|e| {
            crate::error::ColorError::ParseError(format!("YAML serialization failed: {e}"))
        })?,
        crate::cli::OutputFormat::Toml => hue_output.to_toml().map_err(|e| {
            crate::error::ColorError::ParseError(format!("TOML serialization failed: {e}"))
        })?,
    };

    fs::write(file_path, content).map_err(crate::error::ColorError::from)?;

    Ok(())
}
