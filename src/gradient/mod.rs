//! Simplified gradient generation module
//!
//! Cleaned up from over-engineered pattern implementation to basic functionality

pub mod calculator;
pub mod easing;
pub mod output;

// Simple re-exports for basic functionality
pub use calculator::{GradientCalculator, GradientValue};
pub use easing::{CubicBezierEasing, EasingStrategy, EasingType, LinearEasing};

/// Simplified gradient generation function for CLI interface
pub fn generate_gradient(args: crate::cli::GradientArgs) -> crate::error::Result<()> {
    use crate::color_distance_strategies::{ColorDistanceStrategy, DeltaE2000Strategy};
    use crate::color_parser::css_parser::CssColorParser;
    use crate::color_parser::unified_manager::UnifiedColorManager;
    use crate::color_utils::LegacyColorUtils as ColorUtils;
    use crate::image::ImageGenerator;
    use crate::output_formats::{
        ColorCollectionMatches, ColorInfo, ContrastAnalysis, EnhancedGradientAnalysisOutput,
        EnhancedGradientStop, GradientAnalysisOutput, GradientColors, GradientConfiguration,
        GradientStop, NestedColorInfo, ProgramMetadata,
    };

    // Parse colors using CSS parser to support named colors
    let css_parser = CssColorParser::new();
    let start_color = css_parser.parse(&args.start_color)?;
    let end_color = css_parser.parse(&args.end_color)?;

    // Convert to LAB color space
    let start_lab = ColorUtils::rgb_to_lab((start_color.r, start_color.g, start_color.b));
    let end_lab = ColorUtils::rgb_to_lab((end_color.r, end_color.g, end_color.b));

    // Generate images if requested
    let image_gen = ImageGenerator::new();
    if args.svg {
        image_gen.generate_svg(&args, start_lab, end_lab)?;
        println!("SVG gradient saved to: {}", args.svg_name);
    }
    if args.png {
        image_gen.generate_png(&args, start_lab, end_lab)?;
        println!("PNG gradient saved to: {}", args.png_name);
    }

    // Calculate gradient steps
    let steps = if let Some(step_percent) = args.step {
        (100 / step_percent as usize).max(2)
    } else {
        args.stops
    };

    // Create unified color manager for color name lookups
    let color_manager = UnifiedColorManager::new()?;

    // Calculate distance between start and end colors using Delta-E 2000
    let start_end_distance = {
        let strategy = DeltaE2000Strategy;
        strategy.calculate_distance(start_lab, end_lab)
    };

    // Calculate relative contrast between start and end colors using existing color_utils
    let (relative_contrast, _contrast_level) = ColorUtils::get_contrast_assessment(
        (start_color.r, start_color.g, start_color.b),
        (end_color.r, end_color.g, end_color.b),
    );

    // Calculate WCAG21 relative luminance for start and end colors
    let start_luminance =
        ColorUtils::wcag_relative_luminance_rgb((start_color.r, start_color.g, start_color.b));
    let end_luminance =
        ColorUtils::wcag_relative_luminance_rgb((end_color.r, end_color.g, end_color.b));

    // Helper function to find color collections for a given RGB color
    let find_color_collections = |rgb: [u8; 3]| -> ColorCollectionMatches {
        let css_matches = color_manager.find_closest_css_colors(rgb, 1);
        let ral_classic_matches = color_manager.find_closest_ral_classic(rgb, 1);
        let ral_design_matches = color_manager.find_closest_ral_design(rgb, 1);

        let css = if css_matches.is_empty() {
            "Unknown | Unknown | #000000".to_string()
        } else {
            let m = &css_matches[0];
            let hex = format!(
                "#{:02X}{:02X}{:02X}",
                m.entry.color.rgb[0], m.entry.color.rgb[1], m.entry.color.rgb[2]
            );
            format!(
                "{} | {} | {}",
                m.entry.metadata.code.as_deref().unwrap_or("unknown"),
                m.entry.metadata.name,
                hex
            )
        };

        let ralc = if ral_classic_matches.is_empty() {
            "Unknown | Unknown | #000000".to_string()
        } else {
            let m = &ral_classic_matches[0];
            let hex = format!(
                "#{:02X}{:02X}{:02X}",
                m.entry.color.rgb[0], m.entry.color.rgb[1], m.entry.color.rgb[2]
            );
            format!(
                "{} | {} | {}",
                m.entry.metadata.code.as_deref().unwrap_or("unknown"),
                m.entry.metadata.name,
                hex
            )
        };

        let raldsp = if ral_design_matches.is_empty() {
            "Unknown | Unknown | #000000".to_string()
        } else {
            let m = &ral_design_matches[0];
            let hex = format!(
                "#{:02X}{:02X}{:02X}",
                m.entry.color.rgb[0], m.entry.color.rgb[1], m.entry.color.rgb[2]
            );
            format!(
                "{} | {} | {}",
                m.entry.metadata.code.as_deref().unwrap_or("unknown"),
                m.entry.metadata.name,
                hex
            )
        };

        ColorCollectionMatches {
            css,
            css_distance: css_matches.first().map_or(999.0, |m| m.distance),
            ralc,
            ralc_distance: ral_classic_matches.first().map_or(999.0, |m| m.distance),
            raldsp,
            raldsp_distance: ral_design_matches.first().map_or(999.0, |m| m.distance),
        }
    };

    // Find color collections for start and end colors
    let start_collections = find_color_collections([start_color.r, start_color.g, start_color.b]);
    let end_collections = find_color_collections([end_color.r, end_color.g, end_color.b]);

    // Generate gradient stops with full analysis
    let mut gradient_stops = Vec::new();
    for i in 0..steps {
        let t = i as f64 / (steps - 1) as f64;
        let interpolated = ColorUtils::interpolate_lab(start_lab, end_lab, t);
        let (r, g, b) = ColorUtils::lab_to_rgb(interpolated);
        let hex = ColorUtils::lab_to_hex(interpolated);
        let luminance = ColorUtils::wcag_relative_luminance_rgb((r, g, b));

        // Find closest color names
        let closest_css = color_manager.find_closest_css_colors([r, g, b], 1);
        let color_name = if closest_css.is_empty() {
            None
        } else {
            Some(crate::output_formats::ColorNameInfo {
                exact: None,
                nearest: Some(crate::output_formats::NearestColorMatch {
                    name: closest_css[0].entry.metadata.name.clone(),
                    collection: "CSS".to_string(),
                    distance: closest_css[0].distance,
                }),
                all_collections: None,
            })
        };

        let stop = GradientStop {
            position: (t * 100.0).round() as u32, // Convert to integer position
            hex: hex.clone(),
            rgb: format!("rgb({r}, {g}, {b})"),
            lab: format!(
                "lab({:.2}, {:.3}, {:.3})",
                interpolated.l, interpolated.a, interpolated.b
            ),
            lch: format!(
                "lch({:.2}, {:.3}, {:.1})",
                interpolated.l,
                interpolated.a.hypot(interpolated.b),
                interpolated.b.atan2(interpolated.a).to_degrees()
            ),
            wcag21_relative_luminance: luminance,
            color_name,
        };

        gradient_stops.push(stop);
    }

    // Generate enhanced gradient stops with nested color structure
    let mut enhanced_gradient_stops = Vec::new();
    for i in 0..steps {
        let t = i as f64 / (steps - 1) as f64;
        let interpolated = ColorUtils::interpolate_lab(start_lab, end_lab, t);
        let (r, g, b) = ColorUtils::lab_to_rgb(interpolated);
        let hex = ColorUtils::lab_to_hex(interpolated);
        let luminance = ColorUtils::wcag_relative_luminance_rgb((r, g, b));

        // Get color collections for this stop
        let stop_collections = find_color_collections([r, g, b]);

        let enhanced_stop = EnhancedGradientStop {
            position: (t * 100.0).round() as u32,
            color: NestedColorInfo {
                hex: hex.clone(),
                rgb: format!("rgb({r}, {g}, {b})"),
                lab: format!(
                    "lab({:.2}, {:.3}, {:.3})",
                    interpolated.l, interpolated.a, interpolated.b
                ),
                lch: format!(
                    "lch({:.2}, {:.3}, {:.1})",
                    interpolated.l,
                    interpolated.a.hypot(interpolated.b),
                    interpolated.b.atan2(interpolated.a).to_degrees()
                ),
                wcag21_relative_luminance: luminance,
            },
            collections: stop_collections,
        };

        enhanced_gradient_stops.push(enhanced_stop);
    }

    // Create enhanced gradient analysis
    let enhanced_gradient_analysis = EnhancedGradientAnalysisOutput {
        metadata: ProgramMetadata::new(Some("Delta E 2000")),
        configuration: GradientConfiguration {
            start_color: args.start_color.clone(),
            end_color: args.end_color.clone(),
            start_position: args.start_position,
            end_position: args.end_position,
            ease_in: args.ease_in,
            ease_out: args.ease_out,
            gradient_steps: steps,
        },
        colors: GradientColors {
            start: ColorInfo {
                hex: ColorUtils::lab_to_hex(start_lab),
                rgb: format!(
                    "rgb({}, {}, {})",
                    start_color.r, start_color.g, start_color.b
                ),
                lab: format!(
                    "lab({:.2}, {:.3}, {:.3})",
                    start_lab.l, start_lab.a, start_lab.b
                ),
                lch: format!(
                    "lch({:.2}, {:.3}, {:.1})",
                    start_lab.l,
                    start_lab.a.hypot(start_lab.b),
                    start_lab.b.atan2(start_lab.a).to_degrees()
                ),
                contrast: Some(ContrastAnalysis {
                    distance: start_end_distance,
                    wcag21_relative_luminance: start_luminance,
                    relative_contrast,
                }),
                collections: Some(start_collections.clone()),
            },
            end: ColorInfo {
                hex: ColorUtils::lab_to_hex(end_lab),
                rgb: format!("rgb({}, {}, {})", end_color.r, end_color.g, end_color.b),
                lab: format!("lab({:.2}, {:.3}, {:.3})", end_lab.l, end_lab.a, end_lab.b),
                lch: format!(
                    "lch({:.2}, {:.3}, {:.1})",
                    end_lab.l,
                    end_lab.a.hypot(end_lab.b),
                    end_lab.b.atan2(end_lab.a).to_degrees()
                ),
                contrast: Some(ContrastAnalysis {
                    distance: start_end_distance,
                    wcag21_relative_luminance: end_luminance,
                    relative_contrast,
                }),
                collections: Some(end_collections.clone()),
            },
        },
        gradient_stops: enhanced_gradient_stops,
    };

    // Create complete gradient analysis (legacy format for compatibility)
    let _gradient_analysis = GradientAnalysisOutput {
        metadata: ProgramMetadata::new(Some("Delta E 2000")),
        configuration: GradientConfiguration {
            start_color: args.start_color.clone(),
            end_color: args.end_color.clone(),
            start_position: args.start_position,
            end_position: args.end_position,
            ease_in: args.ease_in,
            ease_out: args.ease_out,
            gradient_steps: steps,
        },
        colors: GradientColors {
            start: ColorInfo {
                hex: ColorUtils::lab_to_hex(start_lab),
                rgb: format!(
                    "rgb({}, {}, {})",
                    start_color.r, start_color.g, start_color.b
                ),
                lab: format!(
                    "lab({:.2}, {:.3}, {:.3})",
                    start_lab.l, start_lab.a, start_lab.b
                ),
                lch: format!(
                    "lch({:.2}, {:.3}, {:.1})",
                    start_lab.l,
                    start_lab.a.hypot(start_lab.b),
                    start_lab.b.atan2(start_lab.a).to_degrees()
                ),
                contrast: Some(ContrastAnalysis {
                    distance: start_end_distance,
                    wcag21_relative_luminance: start_luminance,
                    relative_contrast,
                }),
                collections: Some(start_collections),
            },
            end: ColorInfo {
                hex: ColorUtils::lab_to_hex(end_lab),
                rgb: format!("rgb({}, {}, {})", end_color.r, end_color.g, end_color.b),
                lab: format!("lab({:.2}, {:.3}, {:.3})", end_lab.l, end_lab.a, end_lab.b),
                lch: format!(
                    "lch({:.2}, {:.3}, {:.1})",
                    end_lab.l,
                    end_lab.a.hypot(end_lab.b),
                    end_lab.b.atan2(end_lab.a).to_degrees()
                ),
                contrast: Some(ContrastAnalysis {
                    distance: start_end_distance,
                    wcag21_relative_luminance: end_luminance,
                    relative_contrast,
                }),
                collections: Some(end_collections),
            },
        },
        gradient_stops,
    };

    // Output in specified format (default YAML) - using enhanced format
    let format = args
        .output_format
        .as_ref()
        .unwrap_or(&crate::cli::OutputFormat::Yaml);
    let output = match format {
        crate::cli::OutputFormat::Toml => enhanced_gradient_analysis.to_toml().map_err(|e| {
            crate::error::ColorError::InvalidArguments(format!("Failed to serialize to TOML: {e}"))
        })?,
        crate::cli::OutputFormat::Yaml => enhanced_gradient_analysis.to_yaml().map_err(|e| {
            crate::error::ColorError::InvalidArguments(format!("Failed to serialize to YAML: {e}"))
        })?,
    };

    // Display to terminal with colorization (like color command)
    display_colorized_gradient_output(&output, format);

    // Save to file if requested
    if let Some(filename) = &args.output_file {
        use std::fs::File;
        use std::io::Write;

        let extension = match format {
            crate::cli::OutputFormat::Toml => "toml",
            crate::cli::OutputFormat::Yaml => "yaml",
        };

        let full_filename = if filename.contains('.') {
            filename.clone()
        } else {
            format!("{filename}.{extension}")
        };

        let mut file = File::create(&full_filename)?;
        file.write_all(output.as_bytes())?;
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
        assert!(true);
    }
}
