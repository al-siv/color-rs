//! Simplified gradient generation module
//!
//! Cleaned up from over-engineered pattern implementation to basic functionality

pub mod calculator;
pub mod easing;
pub mod output;

// Simple re-exports for basic functionality
pub use calculator::{GradientCalculator, GradientValue, UnifiedGradientStop};
pub use easing::{CubicBezierEasing, EasingStrategy, EasingType, LinearEasing};

/// Simplified gradient generation function for CLI interface
pub fn generate_gradient(args: crate::cli::GradientArgs) -> crate::error::Result<()> {
    use crate::color_distance_strategies::{DistanceAlgorithm, calculate_distance};
    use crate::color_parser::css_parser::CssColorParser;
    use crate::color_parser::unified_manager::UnifiedColorManager;
    use crate::image::ImageGenerator;
    use crate::output_formats::{
        ColorCollectionMatches, ColorInfo, ContrastAnalysis, EnhancedGradientAnalysisOutput,
        EnhancedGradientStop, GradientAnalysisOutput, GradientColors, GradientConfiguration,
        GradientStop, NestedColorInfo, ProgramMetadata,
    };
    use palette::{Lab, Srgb, IntoColor};

    // Helper functions for functional color operations
    let lab_to_hex = |lab: Lab| -> String {
        let srgb: Srgb = lab.into_color();
        format!("#{:02x}{:02x}{:02x}", 
            (srgb.red * 255.0) as u8,
            (srgb.green * 255.0) as u8,
            (srgb.blue * 255.0) as u8
        )
    };

    let wcag_relative_luminance_rgb = |rgb: (u8, u8, u8)| -> f64 {
        let (r, g, b) = (rgb.0 as f64 / 255.0, rgb.1 as f64 / 255.0, rgb.2 as f64 / 255.0);
        let to_linear = |c: f64| if c <= 0.03928 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) };
        0.2126 * to_linear(r) + 0.7152 * to_linear(g) + 0.0722 * to_linear(b)
    };

    let get_contrast_assessment = |rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)| -> (f32, String) {
        let l1 = wcag_relative_luminance_rgb(rgb1);
        let l2 = wcag_relative_luminance_rgb(rgb2);
        let ratio = if l1 > l2 { (l1 + 0.05) / (l2 + 0.05) } else { (l2 + 0.05) / (l1 + 0.05) };
        let level = if ratio >= 7.0 { "AAA" } else if ratio >= 4.5 { "AA" } else if ratio >= 3.0 { "AA Large" } else { "Fail" };
        (ratio as f32, level.to_string())
    };

    // Parse colors using CSS parser to support named colors
    let css_parser = CssColorParser::new();
    let start_color = css_parser.parse(&args.start_color)?;
    let end_color = css_parser.parse(&args.end_color)?;

    // Convert to LAB color space using palette
    let start_srgb = Srgb::new(
        start_color.r as f32 / 255.0,
        start_color.g as f32 / 255.0,
        start_color.b as f32 / 255.0
    );
    let start_lab: Lab = start_srgb.into_color();
    
    let end_srgb = Srgb::new(
        end_color.r as f32 / 255.0,
        end_color.g as f32 / 255.0,
        end_color.b as f32 / 255.0
    );
    let end_lab: Lab = end_srgb.into_color();

    // Generate images if requested
    let image_gen = ImageGenerator::new();
    if args.should_generate_svg() {
        image_gen.generate_svg(&args, start_lab, end_lab)?;
        println!("SVG gradient saved to: {}", args.svg_name());
    }
    if args.should_generate_png() {
        image_gen.generate_png(&args, start_lab, end_lab)?;
        println!("PNG gradient saved to: {}", args.png_name());
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
    let start_end_distance = calculate_distance(DistanceAlgorithm::DeltaE2000, start_lab, end_lab);

    // Calculate relative contrast between start and end colors using functional implementation
    let (relative_contrast, _contrast_level) = get_contrast_assessment(
        (start_color.r, start_color.g, start_color.b),
        (end_color.r, end_color.g, end_color.b),
    );

    // Calculate WCAG21 relative luminance for start and end colors
    let start_luminance =
        wcag_relative_luminance_rgb((start_color.r, start_color.g, start_color.b));
    let end_luminance =
        wcag_relative_luminance_rgb((end_color.r, end_color.g, end_color.b));

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

    // Generate gradient stops using unified calculation
    let unified_stops = GradientCalculator::calculate_unified_gradient(
        start_lab,
        end_lab,
        args.start_position,
        args.end_position,
        args.ease_in,
        args.ease_out,
        steps,
        args.stops_simple,
    );

    // Convert unified stops to old format for YAML output
    let mut gradient_stops = Vec::new();
    for stop in &unified_stops {
        let hex = lab_to_hex(stop.lab_color);
        let luminance = wcag_relative_luminance_rgb(stop.rgb_color);

        // Calculate color distance from start_color using Delta E 2000
        let distance = calculate_distance(DistanceAlgorithm::DeltaE2000, start_lab, stop.lab_color) as f32;

        // Find closest color names
        let closest_css = color_manager
            .find_closest_css_colors([stop.rgb_color.0, stop.rgb_color.1, stop.rgb_color.2], 1);
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

        let gradient_stop = GradientStop {
            position: stop.position as u32,
            hex: hex.clone(),
            rgb: format!(
                "rgb({}, {}, {})",
                stop.rgb_color.0, stop.rgb_color.1, stop.rgb_color.2
            ),
            lab: format!(
                "lab({:.2}, {:.2}, {:.2})",
                stop.lab_color.l, stop.lab_color.a, stop.lab_color.b
            ),
            lch: format!(
                "lch({:.2}, {:.2}, {:.1})",
                stop.lab_color.l,
                stop.lab_color.a.hypot(stop.lab_color.b),
                stop.lab_color.b.atan2(stop.lab_color.a).to_degrees()
            ),
            wcag21_relative_luminance: luminance,
            distance,
            color_name,
        };

        gradient_stops.push(gradient_stop);
    }

    // Generate enhanced gradient stops with nested color structure using unified data
    let mut enhanced_gradient_stops = Vec::new();
    for stop in &unified_stops {
        let hex = lab_to_hex(stop.lab_color);
        let luminance = wcag_relative_luminance_rgb(stop.rgb_color);

        // Calculate color distance from start_color using Delta E 2000
        let distance = calculate_distance(DistanceAlgorithm::DeltaE2000, start_lab, stop.lab_color) as f32;

        // Get color collections for this stop
        let stop_collections =
            find_color_collections([stop.rgb_color.0, stop.rgb_color.1, stop.rgb_color.2]);

        let enhanced_stop = EnhancedGradientStop {
            position: stop.position as u32,
            color: NestedColorInfo {
                hex: hex.clone(),
                rgb: format!(
                    "rgb({}, {}, {})",
                    stop.rgb_color.0, stop.rgb_color.1, stop.rgb_color.2
                ),
                lab: format!(
                    "lab({:.2}, {:.2}, {:.2})",
                    stop.lab_color.l, stop.lab_color.a, stop.lab_color.b
                ),
                lch: format!(
                    "lch({:.2}, {:.2}, {:.1})",
                    stop.lab_color.l,
                    stop.lab_color.a.hypot(stop.lab_color.b),
                    stop.lab_color.b.atan2(stop.lab_color.a).to_degrees()
                ),
                wcag21_relative_luminance: luminance,
                distance,
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
                hex: lab_to_hex(start_lab),
                rgb: format!(
                    "rgb({}, {}, {})",
                    start_color.r, start_color.g, start_color.b
                ),
                lab: format!(
                    "lab({:.2}, {:.2}, {:.2})",
                    start_lab.l, start_lab.a, start_lab.b
                ),
                lch: format!(
                    "lch({:.2}, {:.2}, {:.1})",
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
                hex: lab_to_hex(end_lab),
                rgb: format!("rgb({}, {}, {})", end_color.r, end_color.g, end_color.b),
                lab: format!("lab({:.2}, {:.2}, {:.2})", end_lab.l, end_lab.a, end_lab.b),
                lch: format!(
                    "lch({:.2}, {:.2}, {:.1})",
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
                hex: lab_to_hex(start_lab),
                rgb: format!(
                    "rgb({}, {}, {})",
                    start_color.r, start_color.g, start_color.b
                ),
                lab: format!(
                    "lab({:.2}, {:.2}, {:.2})",
                    start_lab.l, start_lab.a, start_lab.b
                ),
                lch: format!(
                    "lch({:.2}, {:.2}, {:.1})",
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
                hex: lab_to_hex(end_lab),
                rgb: format!("rgb({}, {}, {})", end_color.r, end_color.g, end_color.b),
                lab: format!("lab({:.2}, {:.2}, {:.2})", end_lab.l, end_lab.a, end_lab.b),
                lch: format!(
                    "lch({:.2}, {:.2}, {:.1})",
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
        // Test passes if compilation succeeds
    }
}
