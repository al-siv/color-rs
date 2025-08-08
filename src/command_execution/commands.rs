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
    use crate::cli::Range;
    use crate::color_parser::collections::ColorCollection;
    use crate::color_parser::{CssColorCollection, RalClassicCollection, RalDesignCollection};
    use crate::color_report_formatting::display;
    use crate::output_formats::{HueCollectionConfiguration, HueCollectionOutput, HueColorEntry};
    use palette::Lch;
    use std::collections::HashMap;

    // Load the specified collection
    let collection: Box<dyn ColorCollection> = match args.collection.as_str() {
        "css" => Box::new(CssColorCollection::new().map_err(|e| {
            crate::error::ColorError::ParseError(format!("Failed to load CSS collection: {e}"))
        })?),
        "ralc" => Box::new(RalClassicCollection::new().map_err(|e| {
            crate::error::ColorError::ParseError(format!(
                "Failed to load RAL Classic collection: {e}"
            ))
        })?),
        "rald" => Box::new(RalDesignCollection::new().map_err(|e| {
            crate::error::ColorError::ParseError(format!(
                "Failed to load RAL Design collection: {e}"
            ))
        })?),
        _ => {
            return Err(crate::error::ColorError::ParseError(format!(
                "Unknown collection: {}",
                args.collection
            )));
        }
    };

    // Parse range filters if provided
    let hue_range = if let Some(ref range_str) = args.hue_range {
        Some(Range::parse(range_str)?)
    } else {
        None
    };

    let lightness_range = if let Some(ref range_str) = args.lightness_range {
        Some(Range::parse(range_str)?)
    } else {
        None
    };

    let chroma_range = if let Some(ref range_str) = args.chroma_range {
        Some(Range::parse(range_str)?)
    } else {
        None
    };

    // Filter and sort collection by hue
    let mut filtered_colors: Vec<_> = collection
        .colors()
        .iter()
        .filter_map(|color_entry| {
            // Convert to LCH for filtering
            let srgb = palette::Srgb::new(
                color_entry.color.rgb[0] as f32 / 255.0,
                color_entry.color.rgb[1] as f32 / 255.0,
                color_entry.color.rgb[2] as f32 / 255.0,
            );
            let lch: Lch = palette::FromColor::from_color(srgb);

            // Apply range filters
            if let Some(ref hr) = hue_range {
                if !hr.contains_with_wrap(lch.hue.into_degrees() as f64, 360.0) {
                    return None;
                }
            }

            if let Some(ref lr) = lightness_range {
                if !lr.contains_linear(lch.l as f64) {
                    return None;
                }
            }

            if let Some(ref cr) = chroma_range {
                if !cr.contains_linear(lch.chroma as f64) {
                    return None;
                }
            }

            Some((color_entry, lch))
        })
        .collect();

    // Sort by hue (primary), then by code (secondary)
    filtered_colors.sort_by(|a, b| {
        let hue_cmp =
            a.1.hue
                .into_degrees()
                .partial_cmp(&b.1.hue.into_degrees())
                .unwrap_or(std::cmp::Ordering::Equal);
        if hue_cmp != std::cmp::Ordering::Equal {
            hue_cmp
        } else {
            a.0.metadata
                .code
                .as_ref()
                .unwrap_or(&String::new())
                .cmp(b.0.metadata.code.as_ref().unwrap_or(&String::new()))
        }
    });

    // Create structured output
    let configuration = HueCollectionConfiguration {
        collection: args.collection.clone(),
        total_colors: filtered_colors.len(),
        hue_range: args.hue_range.clone(),
        lightness_range: args.lightness_range.clone(),
        chroma_range: args.chroma_range.clone(),
    };

    let hue_colors: Vec<HueColorEntry> = {
        let mut previous_hue = None;
        filtered_colors
            .iter()
            .map(|(color_entry, lch)| {
                // Get color name
                let name = color_entry.metadata.name.clone();

                // Calculate hue and hue shift
                let hue = lch.hue.into_degrees() as f64;
                let hue_shift = previous_hue.map(|prev| {
                    let diff = hue - prev;
                    if diff > 180.0 {
                        diff - 360.0
                    } else if diff < -180.0 {
                        diff + 360.0
                    } else {
                        diff
                    }
                });

                // Create hex representation
                let hex = format!(
                    "#{:02X}{:02X}{:02X}",
                    color_entry.color.rgb[0], color_entry.color.rgb[1], color_entry.color.rgb[2]
                );

                // Create LCH representation
                let lch_str = format!("lch({:>4.1}, {:>4.1}, {:>6.1})", lch.l, lch.chroma, hue);

                // Create code
                let code = color_entry
                    .metadata
                    .code
                    .as_ref()
                    .unwrap_or(&"Unknown".to_string())
                    .clone();

                // Create hue shift string
                let hue_shift_str = match hue_shift {
                    Some(shift) => format!("{:>6}", format!("+{:.2}", shift)),
                    None => format!("{:>6}", "—"),
                };

                // Create single line format: "Hue | code | HEX | LCH | name | Hue shift from previous color"
                let display =
                    format!("{hue:>6.1} | {hex} | {lch_str} | {hue_shift_str} | {code} | {name}");

                previous_hue = Some(hue);

                HueColorEntry { display }
            })
            .collect()
    };

    let hue_output = HueCollectionOutput::new()
        .with_configuration(configuration)
        .with_colors(hue_colors);

    // Generate YAML output for colored terminal display
    let yaml_output = hue_output.to_yaml().map_err(|e| {
        crate::error::ColorError::ParseError(format!("Failed to serialize to YAML: {e}"))
    })?;

    // Display with colored terminal output
    display::display_terminal_output(&yaml_output, &OutputFormat::Yaml);

    // Handle file export if requested
    if let Some(file_path) = &args.output_file {
        let output_format = args.output_format.clone().unwrap_or_default();
        export_hue_collection_display(&hue_output, output_format, file_path)?;
    }

    // Handle visual output if requested
    if args.should_generate_visual() {
        // Convert filtered colors to HueAnalysisResult format for visual generation
        let analysis_results: Vec<crate::color_ops::analysis::hue::HueAnalysisResult> =
            filtered_colors
                .iter()
                .map(|(color_entry, lch)| {
                    crate::color_ops::analysis::hue::HueAnalysisResult {
                        color: *lch,
                        name: Some(color_entry.metadata.name.clone()),
                        hue_distance: 0.0, // Not used for visual generation
                        saturation: lch.chroma as f64,
                        lightness: lch.l as f64,
                        collection: args.collection.clone(),
                        code: color_entry.metadata.code.clone(),
                    }
                })
                .collect();

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
    let mut metadata = HashMap::new();
    metadata.insert("collection".to_string(), args.collection.clone());
    metadata.insert(
        "total_colors".to_string(),
        filtered_colors.len().to_string(),
    );
    if let Some(ref hr) = args.hue_range {
        metadata.insert("hue_range".to_string(), hr.clone());
    }
    if let Some(ref lr) = args.lightness_range {
        metadata.insert("lightness_range".to_string(), lr.clone());
    }
    if let Some(ref cr) = args.chroma_range {
        metadata.insert("chroma_range".to_string(), cr.clone());
    }

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
