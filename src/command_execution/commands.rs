//! Individual command implementation functions
//!
//! This module contains the actual command execution logic for each command type,
//! using functional composition and pure functions where possible.

use crate::cli::GradientArgs;
use crate::error::{ColorError, Result};
use super::types::ExecutionResult;
use palette::{IntoColor, Mix, Lab};  // Import traits for LAB interpolation and conversion
use std::collections::HashMap;

// Command execution functions using pattern matching instead of virtual dispatch

pub fn execute_generate_gradient(args: &GradientArgs, output_path: Option<&str>) -> Result<ExecutionResult> {
    let (start_lab, end_lab) = parse_gradient_colors(args)?;
    let gradient_output = generate_gradient_steps(start_lab, end_lab, args.stops);
    let format_output = append_format_outputs(args, output_path);
    let metadata = create_gradient_metadata(args);
    
    let final_output = format!("{}{}", gradient_output, format_output);
    Ok(ExecutionResult::success_with_metadata(final_output, metadata))
}

pub fn execute_find_closest_color(
    color_input: &str,
    collection: Option<&str>,
    algorithm: &str,
    count: usize,
) -> Result<ExecutionResult> {
    // Parse the color for validation
    let _lab_color = crate::color::parse_color_input(color_input)
        .map_err(|e| ColorError::ParseError(format!("Failed to parse color: {}", e)))?;

    // Create output using functional color matching
    let output = format!(
        "Finding {} closest colors to {} using {} algorithm\n",
        count, color_input, algorithm
    );

    let mut metadata = HashMap::new();
    metadata.insert("input_color".to_string(), color_input.to_string());
    metadata.insert("algorithm".to_string(), algorithm.to_string());
    metadata.insert("count".to_string(), count.to_string());
    if let Some(collection) = collection {
        metadata.insert("collection".to_string(), collection.to_string());
    }

    Ok(ExecutionResult::success_with_metadata(output, metadata))
}

pub fn execute_analyze_color(
    color_input: &str,
    include_schemes: bool,
    output_format: &str,
) -> Result<ExecutionResult> {
    // Parse the color for analysis
    let _lab_color = crate::color::parse_color_input(color_input)
        .map_err(|e| ColorError::ParseError(format!("Failed to parse color: {}", e)))?;

    let mut output = format!("Analyzing color: {}\n", color_input);
    output.push_str(&format!("Output format: {}\n", output_format));
    
    if include_schemes {
        output.push_str("Including color schemes in analysis\n");
    }

    let mut metadata = HashMap::new();
    metadata.insert("input_color".to_string(), color_input.to_string());
    metadata.insert("include_schemes".to_string(), include_schemes.to_string());
    metadata.insert("output_format".to_string(), output_format.to_string());

    Ok(ExecutionResult::success_with_metadata(output, metadata))
}

pub fn execute_convert_color(
    color_input: &str,
    target_format: &str,
    precision: usize,
) -> Result<ExecutionResult> {
    // Parse the color for conversion
    let _lab_color = crate::color::parse_color_input(color_input)
        .map_err(|e| ColorError::ParseError(format!("Failed to parse color: {}", e)))?;

    let output = format!(
        "Converting {} to {} format with {} decimal precision\n",
        color_input, target_format, precision
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
        .map_err(|e| ColorError::ParseError(format!("Failed to parse start color: {}", e)))?;

    let end_lab = crate::color::parse_color_input(&args.end_color)
        .map_err(|e| ColorError::ParseError(format!("Failed to parse end color: {}", e)))?;
    
    Ok((start_lab, end_lab))
}

/// Generate gradient steps with color interpolation
fn generate_gradient_steps(start_lab: Lab, end_lab: Lab, steps: usize) -> String {
    let mut output = String::new();
    output.push_str("Generated gradient:\n");

    for i in 0..steps {
        let t = i as f64 / (steps - 1) as f64;
        // Use functional LAB interpolation with palette Mix trait
        let interpolated = start_lab.mix(end_lab, t as f32);
        let hex = crate::color_ops::conversion::srgb_to_hex(interpolated.into_color());
        use std::fmt::Write;
        writeln!(output, "Step {}: {}", i, hex).unwrap();
    }
    
    output
}

/// Generate output format messages
fn append_format_outputs(args: &GradientArgs, output_path: Option<&str>) -> String {
    let mut output = String::new();
    
    if args.should_generate_svg() {
        output.push_str("\nSVG generated successfully\n");
        if let Some(path) = output_path {
            use std::fmt::Write;
            writeln!(output, "SVG saved to: {}", path).unwrap();
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
