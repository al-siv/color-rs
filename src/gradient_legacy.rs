//! Gradient generation and interpolation using LAB color space
//!
//! This module provides a legacy interface to the new modular gradient system.
//! All functionality has been refactored into separate modules using design patterns.

use crate::cli::GradientArgs;
use crate::error::{ColorError, Result};
use crate::color_parser::ColorParser;

// Re-export the new modular gradient system
pub use crate::gradient::{
    GradientFacade, GradientService, GradientWorkflowBuilder,
    GradientValue, OutputFormat, EasingType
};

/// Main gradient generation function using the new modular system
pub fn generate_gradient(args: GradientArgs) -> Result<()> {
    // Validate arguments
    args.validate()?;

    // Parse colors using unified color parser
    let parser = ColorParser::new();

    let (start_lab, _) = parser.parse(&args.start_color).map_err(|e| {
        ColorError::InvalidColor(format!(
            "Failed to parse start color '{}': {}",
            args.start_color, e
        ))
    })?;
    let (end_lab, _) = parser.parse(&args.end_color).map_err(|e| {
        ColorError::InvalidColor(format!(
            "Failed to parse end color '{}': {}",
            args.end_color, e
        ))
    })?;

    // Generate images if requested
    if args.should_generate_svg() || args.should_generate_png() {
        use crate::image::ImageGenerator;
        let generator = ImageGenerator::new();

        if args.should_generate_svg() {
            generator.generate_svg(&args, start_lab, end_lab)?;
            use colored::*;
            println!(
                "{} {}\n",
                "SVG gradient saved to:".green().bold(),
                args.svg_name
            );
        }

        if args.should_generate_png() {
            generator.generate_png(&args, start_lab, end_lab)?;
            use colored::*;
            println!(
                "{} {}\n",
                "PNG gradient saved to:".green().bold(),
                args.png_name
            );
        }
    }

    // Convert CLI args to new gradient request format
    let easing_type = if args.ease_in == 0.0 && args.ease_out == 1.0 {
        EasingType::Linear
    } else {
        EasingType::CubicBezier
    };

    let output_format = match &args.output_format {
        Some(crate::cli::OutputFormat::Json) => OutputFormat::Json,
        Some(crate::cli::OutputFormat::Csv) => OutputFormat::Table, // CSV handled separately
        _ => OutputFormat::Table,
    };

    // Build gradient request using the new system
    let request = GradientWorkflowBuilder::new()
        .colors(&args.start_color, &args.end_color)
        .stops(args.stops)
        .positions(args.start_position, args.end_position)
        .easing(easing_type, args.ease_in, args.ease_out)
        .intelligent_stops(args.intelligent)
        .output_format(output_format)
        .output_file(args.output_file.clone())
        .show_preview(true)
        .build();

    // Execute workflow using new system
    let workflow = crate::gradient::StandardGradientWorkflow;
    let response = workflow.execute_workflow(&request)?;

    // Handle output based on format
    if let Some(format) = &args.output_format {
        match format {
            crate::cli::OutputFormat::Json => {
                if let Some(ref filename) = args.output_file {
                    std::fs::write(filename, &response.output)?;
                    println!("JSON output saved to: {}", filename);
                } else {
                    println!("{}", response.output);
                }
            }
            crate::cli::OutputFormat::Csv => {
                // Convert to CSV format
                let csv_output = create_csv_output(&response.values)?;
                if let Some(ref filename) = args.output_file {
                    std::fs::write(filename, csv_output)?;
                    println!("CSV output saved to: {}", filename);
                } else {
                    println!("{}", csv_output);
                }
            }
            _ => {
                // Display table format with filtering if needed
                let filtered_values = if args.filtering_enabled() {
                    apply_gradient_filtering(&response.values, &args)?
                } else {
                    response.values
                };

                if args.analysis {
                    create_gradient_analysis_output(&filtered_values, &args)?;
                } else {
                    display_gradient_output(&filtered_values, &args)?;
                }
            }
        }
    } else {
        // Default table output
        let filtered_values = if args.filtering_enabled() {
            apply_gradient_filtering(&response.values, &args)?
        } else {
            response.values
        };

        if args.analysis {
            create_gradient_analysis_output(&filtered_values, &args)?;
        } else {
            display_gradient_output(&filtered_values, &args)?;
        }
    }

    Ok(())
}

/// Convert gradient values to CSV format
fn create_csv_output(values: &[GradientValue]) -> Result<String> {
    let mut csv = String::from("Position,Hex,RGB,WCAG Luminance\n");
    for value in values {
        csv.push_str(&format!(
            "{},{},{},{}\n",
            value.position, value.hex, value.rgb, value.wcag_luminance
        ));
    }
    Ok(csv)
}

/// Apply gradient-specific filtering
fn apply_gradient_filtering(
    values: &[GradientValue], 
    args: &GradientArgs
) -> Result<Vec<GradientValue>> {
    let mut filtered = values.to_vec();

    // Apply contrast filters
    if let Some(min_contrast) = args.min_contrast {
        filtered.retain(|value| {
            if let Ok(luminance) = value.wcag_luminance.parse::<f64>() {
                luminance >= min_contrast
            } else {
                true
            }
        });
    }

    if let Some(max_contrast) = args.max_contrast {
        filtered.retain(|value| {
            if let Ok(luminance) = value.wcag_luminance.parse::<f64>() {
                luminance <= max_contrast
            } else {
                true
            }
        });
    }

    // Apply position filters
    if let Some(min_pos) = args.min_position {
        filtered.retain(|value| {
            if let Ok(pos) = value.position.trim_end_matches('%').parse::<f64>() {
                pos >= min_pos
            } else {
                true
            }
        });
    }

    if let Some(max_pos) = args.max_position {
        filtered.retain(|value| {
            if let Ok(pos) = value.position.trim_end_matches('%').parse::<f64>() {
                pos <= max_pos
            } else {
                true
            }
        });
    }

    Ok(filtered)
}

/// Display gradient output with formatting
fn display_gradient_output(values: &[GradientValue], args: &GradientArgs) -> Result<()> {
    use crate::gradient::GradientOutputUtils;
    
    if args.show_colors || args.colorize {
        GradientOutputUtils::print_gradient_with_colors(values)?;
    } else {
        GradientOutputUtils::display_gradient_table(values)?;
    }
    
    Ok(())
}

/// Create analysis output with statistics
fn create_gradient_analysis_output(values: &[GradientValue], _args: &GradientArgs) -> Result<()> {
    use colored::*;
    
    println!("{}", "=== Gradient Analysis ===".cyan().bold());
    println!("Total stops: {}", values.len());
    
    if !values.is_empty() {
        // Calculate statistics
        let luminances: Vec<f64> = values
            .iter()
            .filter_map(|v| v.wcag_luminance.parse().ok())
            .collect();
            
        if !luminances.is_empty() {
            let min_lum = luminances.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max_lum = luminances.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            let avg_lum = luminances.iter().sum::<f64>() / luminances.len() as f64;
            
            println!("Luminance range: {:.3} - {:.3}", min_lum, max_lum);
            println!("Average luminance: {:.3}", avg_lum);
            println!("Contrast ratio: {:.2}:1", max_lum / min_lum.max(0.001));
        }
    }
    
    println!("\n{}", "=== Gradient Values ===".cyan().bold());
    display_gradient_output(values, &GradientArgs::default())?;
    
    Ok(())
}

/// Legacy gradient calculator - now delegates to new modular system
pub struct GradientCalculator;

impl GradientCalculator {
    /// Legacy method - use new GradientService instead
    #[deprecated(note = "Use GradientService::generate_simple_gradient instead")]
    pub fn generate_gradient_values(
        start_lab: palette::Lab,
        end_lab: palette::Lab,
        num_stops: usize,
        start_position: u8,
        end_position: u8,
        ease_in: f64,
        ease_out: f64,
    ) -> Result<Vec<GradientValue>> {
        use crate::color_utils::ColorUtils;
        
        // Convert LAB to hex for new system
        let start_hex = format!("#{:02X}{:02X}{:02X}", 
            ColorUtils::lab_to_rgb(start_lab).0,
            ColorUtils::lab_to_rgb(start_lab).1,
            ColorUtils::lab_to_rgb(start_lab).2
        );
        let end_hex = format!("#{:02X}{:02X}{:02X}", 
            ColorUtils::lab_to_rgb(end_lab).0,
            ColorUtils::lab_to_rgb(end_lab).1,
            ColorUtils::lab_to_rgb(end_lab).2
        );
        
        // Use new system
        GradientService::generate_eased_gradient(
            &start_hex,
            &end_hex,
            num_stops,
            EasingType::CubicBezier,
            ease_in,
            ease_out,
        )
    }
}
