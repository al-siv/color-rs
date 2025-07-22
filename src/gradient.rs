//! Gradient generation and interpolation using LAB color space

use crate::cli::GradientArgs;
use crate::color_utils::ColorUtils;
use crate::config::*;
use crate::error::{ColorError, Result};
use crate::utils::Utils;
use kurbo::{CubicBez, ParamCurve, Point};
use palette::Lab;
use tabled::Tabled;

/// Gradient value for display in tables
#[derive(Tabled, Clone)]
pub struct GradientValue {
    #[tabled(rename = "Position")]
    pub position: String,
    #[tabled(rename = "Hex")]
    pub hex: String,
    #[tabled(rename = "RGB")]
    pub rgb: String,
    #[tabled(rename = "WCAG Luminance")]
    pub wcag_luminance: String,
}

/// Gradient calculation and generation
pub struct GradientCalculator;

impl GradientCalculator {
    /// Cubic Bezier easing function using kurbo library
    /// Implements cubic-bezier(x1, 0, x2, 1) easing functions
    /// This matches CSS timing functions like ease-in-out: cubic-bezier(0.42, 0, 0.58, 1)
    pub fn cubic_bezier_ease(t: f64, x1: f64, x2: f64) -> f64 {
        if t <= 0.0 {
            return 0.0;
        }
        if t >= 1.0 {
            return 1.0;
        }

        // Clamp control points to valid range
        let x1 = x1.clamp(BEZIER_MIN, BEZIER_MAX);
        let x2 = x2.clamp(BEZIER_MIN, BEZIER_MAX);

        // Create cubic bezier curve with control points (0,0), (x1,0), (x2,1), (1,1)
        // This matches cubic-bezier specification
        let curve = CubicBez::new(
            Point::new(0.0, 0.0), // Start point
            Point::new(x1, 0.0),  // First control point (x1, 0)
            Point::new(x2, 1.0),  // Second control point (x2, 1)
            Point::new(1.0, 1.0), // End point
        );

        // Use binary search to find parameter where x-coordinate equals target
        Self::solve_cubic_bezier_for_x(&curve, t)
    }

    /// Binary search to find parameter t where curve.eval(t).x == target_x
    /// This replaces custom Newton-Raphson implementation with a robust binary search
    fn solve_cubic_bezier_for_x(curve: &CubicBez, target_x: f64) -> f64 {
        let mut low: f64 = 0.0;
        let mut high: f64 = 1.0;
        let tgt: f64 = target_x;

        for _ in 0..MAX_ITERATIONS {
            let mid = (low + high) * 0.5;
            let point = curve.eval(mid);
            let current_x = point.x;

            if (current_x - tgt).abs() < EPSILON {
                return point.y.clamp(0.0, 1.0);
            }

            if current_x < tgt {
                low = mid;
            } else {
                high = mid;
            }
        }

        // If we didn't converge, evaluate at the midpoint
        let point = curve.eval((low + high) * 0.5);
        point.y.clamp(0.0, 1.0)
    }

    /// Calculate intelligent gradient stop positions based on cubic-bezier curve derivatives
    /// Places more stops where the curve changes rapidly to prevent visual banding
    pub fn calculate_intelligent_stops(num_stops: usize, ease_in: f64, ease_out: f64) -> Vec<f64> {
        if num_stops == 0 {
            return vec![];
        }
        if num_stops == 1 {
            return vec![0.5];
        }

        let x1 = ease_in.clamp(BEZIER_MIN, BEZIER_MAX);
        let x2 = ease_out.clamp(BEZIER_MIN, BEZIER_MAX);

        // Create cubic bezier curve
        let curve = CubicBez::new(
            Point::new(0.0, 0.0),
            Point::new(x1, 0.0),
            Point::new(x2, 1.0),
            Point::new(1.0, 1.0),
        );

        let mut cumulative_importance = vec![0.0; INTELLIGENT_STOP_SAMPLE_POINTS + 1];

        for i in 0..INTELLIGENT_STOP_SAMPLE_POINTS {
            let t = i as f64 / INTELLIGENT_STOP_SAMPLE_POINTS as f64;
            let dt = 1.0 / INTELLIGENT_STOP_SAMPLE_POINTS as f64;

            // Calculate derivative magnitude using numerical differentiation
            let current_point = curve.eval(t);
            let next_point = curve.eval((t + dt).min(1.0));

            let dy = next_point.y - current_point.y;
            // Only consider color space changes (y-axis), ignore time progression (x-axis)
            // This focuses on where the easing function changes rapidly in terms of output value
            let derivative_magnitude = dy.abs();

            // Accumulate importance (areas where curve changes rapidly get higher weight)
            cumulative_importance[i + 1] = cumulative_importance[i] + derivative_magnitude;
        }

        let total_importance = cumulative_importance[INTELLIGENT_STOP_SAMPLE_POINTS];
        if total_importance == 0.0 {
            // Fallback to equal spacing if no variation
            return (0..num_stops)
                .map(|i| i as f64 / (num_stops - 1).max(1) as f64)
                .collect();
        }

        // Distribute stops based on cumulative importance
        let mut stops = Vec::new();
        for i in 0..num_stops {
            let target_importance = (i as f64 / (num_stops - 1).max(1) as f64) * total_importance;

            // Binary search to find the t value corresponding to target importance
            let mut low = 0;
            let mut high = INTELLIGENT_STOP_SAMPLE_POINTS;

            while high - low > 1 {
                let mid = (low + high) / 2;
                if cumulative_importance[mid] < target_importance {
                    low = mid;
                } else {
                    high = mid;
                }
            }

            let t = low as f64 / INTELLIGENT_STOP_SAMPLE_POINTS as f64;
            stops.push(t);
        }

        stops
    }

    /// Calculate intelligent gradient stop positions with integer percentages
    /// Returns positions as integer percentages to avoid CSS formatting issues
    pub fn calculate_intelligent_stops_integer(
        num_stops: usize,
        ease_in: f64,
        ease_out: f64,
        start_pos: u8,
        end_pos: u8,
    ) -> Vec<u8> {
        if num_stops == 0 {
            return vec![];
        }
        if num_stops == 1 {
            return vec![(start_pos + end_pos) / 2];
        }

        // Get floating point positions first
        let float_positions = Self::calculate_intelligent_stops(num_stops, ease_in, ease_out);
        let span = end_pos - start_pos;

        // Convert to integer positions and remove duplicates
        let mut integer_positions: Vec<u8> = float_positions
            .iter()
            .map(|&t| {
                let pos = start_pos as f64 + t * span as f64;
                pos.round() as u8
            })
            .collect();

        // Remove duplicates while preserving order
        integer_positions.dedup();

        // Ensure first and last positions are included
        if !integer_positions.contains(&start_pos) {
            integer_positions.insert(0, start_pos);
        }
        if !integer_positions.contains(&end_pos) {
            integer_positions.push(end_pos);
        }

        // Remove duplicates again after adding endpoints
        integer_positions.dedup();
        integer_positions.sort();

        integer_positions
    }

    /// Generate gradient values based on the provided arguments
    pub fn generate_gradient_values(
        args: &GradientArgs,
        start_lab: Lab,
        end_lab: Lab,
    ) -> Result<Vec<GradientValue>> {
        let mut gradient_values = Vec::new();

        // Default behavior is now intelligent stops (stops)
        // First check if step is explicitly provided
        if let Some(step) = args.step {
            // Use step-based generation when explicitly requested
            let mut position = args.start_position;
            while position <= args.end_position {
                let normalized_t = (position - args.start_position) as f64
                    / (args.end_position - args.start_position) as f64;

                let smooth_t = Self::cubic_bezier_ease(normalized_t, args.ease_in, args.ease_out);
                let interpolated_lab = ColorUtils::interpolate_lab(start_lab, end_lab, smooth_t);
                let hex_color = ColorUtils::lab_to_hex(interpolated_lab);
                let rgb_values = ColorUtils::lab_to_rgb(interpolated_lab);
                let wcag_luminance = ColorUtils::wcag_relative_luminance_rgb((
                    rgb_values.0,
                    rgb_values.1,
                    rgb_values.2,
                ));

                gradient_values.push(GradientValue {
                    position: format!("{}%", position),
                    hex: hex_color,
                    rgb: Utils::rgb_to_string(rgb_values.0, rgb_values.1, rgb_values.2),
                    wcag_luminance: format!("{:.3}", wcag_luminance),
                });

                position += step;
                if position > args.end_position && position - step < args.end_position {
                    // Ensure we always include the end position
                    position = args.end_position;
                } else if position > args.end_position {
                    break;
                }
            }
        } else if args.stops_simple {
            // Simple equal spacing when --stops-simple flag is used
            let num_stops = args.stops;
            for i in 0..num_stops {
                let t = if num_stops == 1 {
                    0.5
                } else {
                    i as f64 / (num_stops - 1) as f64
                };

                let position_float = args.start_position as f64
                    + t * (args.end_position - args.start_position) as f64;
                let position = position_float.round() as u8;
                let normalized_t = (position - args.start_position) as f64
                    / (args.end_position - args.start_position) as f64;
                let smooth_t = Self::cubic_bezier_ease(normalized_t, args.ease_in, args.ease_out);
                let interpolated_lab = ColorUtils::interpolate_lab(start_lab, end_lab, smooth_t);
                let hex_color = ColorUtils::lab_to_hex(interpolated_lab);
                let rgb_values = ColorUtils::lab_to_rgb(interpolated_lab);
                let wcag_luminance = ColorUtils::wcag_relative_luminance_rgb((
                    rgb_values.0,
                    rgb_values.1,
                    rgb_values.2,
                ));

                gradient_values.push(GradientValue {
                    position: format!("{}%", position),
                    hex: hex_color,
                    rgb: Utils::rgb_to_string(rgb_values.0, rgb_values.1, rgb_values.2),
                    wcag_luminance: format!("{:.3}", wcag_luminance),
                });
            }

            // Remove duplicates based on position
            gradient_values.dedup_by(|a, b| a.position == b.position);
        } else {
            // Default behavior: intelligent stop placement (stops)
            let stop_positions = Self::calculate_intelligent_stops_integer(
                args.stops,
                args.ease_in,
                args.ease_out,
                args.start_position,
                args.end_position,
            );

            for &position in stop_positions.iter() {
                let normalized_t = (position - args.start_position) as f64
                    / (args.end_position - args.start_position) as f64;
                let smooth_t = Self::cubic_bezier_ease(normalized_t, args.ease_in, args.ease_out);
                let interpolated_lab = ColorUtils::interpolate_lab(start_lab, end_lab, smooth_t);
                let hex_color = ColorUtils::lab_to_hex(interpolated_lab);
                let rgb_values = ColorUtils::lab_to_rgb(interpolated_lab);
                let wcag_luminance = ColorUtils::wcag_relative_luminance_rgb((
                    rgb_values.0,
                    rgb_values.1,
                    rgb_values.2,
                ));

                gradient_values.push(GradientValue {
                    position: format!("{}%", position),
                    hex: hex_color,
                    rgb: Utils::rgb_to_string(rgb_values.0, rgb_values.1, rgb_values.2),
                    wcag_luminance: format!("{:.3}", wcag_luminance),
                });
            }
        }

        Ok(gradient_values)
    }

}

/// Main gradient generation function
pub fn generate_gradient(args: GradientArgs) -> Result<()> {
    // Validate arguments
    args.validate()?;

    // Parse colors using unified color parser
    use crate::color_parser::ColorParser;
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

    // Print color information with beautiful formatting
    // ColorProcessor::print_color_info_table(start_lab, end_lab);

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

    // Generate gradient values for terminal/file output
    let gradient_values = GradientCalculator::generate_gradient_values(&args, start_lab, end_lab)?;

    // Create structured output data
    let gradient_output =
        create_gradient_analysis_output(&args, start_lab, end_lab, &gradient_values)?;

    // Display structured output to terminal
    let format = args
        .output_format
        .as_ref()
        .unwrap_or(&crate::cli::OutputFormat::Yaml);
    format_gradient_structured_output(&gradient_output, format, args.output_file.as_ref())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubic_bezier_ease() {
        // Test linear case
        assert!((GradientCalculator::cubic_bezier_ease(0.5, 0.0, 1.0) - 0.5).abs() < 0.01);

        // Test boundary conditions
        assert_eq!(GradientCalculator::cubic_bezier_ease(0.0, 0.5, 0.5), 0.0);
        assert_eq!(GradientCalculator::cubic_bezier_ease(1.0, 0.5, 0.5), 1.0);
    }

    #[test]
    fn test_intelligent_stops() {
        let stops = GradientCalculator::calculate_intelligent_stops(5, 0.25, 0.75);
        assert_eq!(stops.len(), 5);
        assert!(stops[0] < stops[1]);
        assert!(stops[stops.len() - 1] <= 1.0);
    }

    #[test]
    fn test_intelligent_stops_integer() {
        let stops = GradientCalculator::calculate_intelligent_stops_integer(5, 0.25, 0.75, 0, 100);
        assert_eq!(stops[0], 0);
        assert_eq!(stops[stops.len() - 1], 100);
        assert!(stops.len() <= 5);
    }

    #[test]
    fn test_unified_color_parsing() {
        use crate::color_parser::ColorParser;
        let parser = ColorParser::new();

        // Test that gradient can parse various color formats
        let (hex_color, _) = parser.parse("#FF0000").unwrap();
        let (rgb_color, _) = parser.parse("rgb(255, 0, 0)").unwrap();
        let (named_color, _) = parser.parse("red").unwrap();
        let (hsl_color, _) = parser.parse("hsl(0, 100%, 50%)").unwrap();

        // All should produce similar LAB values for red
        assert!((hex_color.l - rgb_color.l).abs() < 1.0);
        assert!((hex_color.l - named_color.l).abs() < 1.0);
        assert!((hex_color.l - hsl_color.l).abs() < 1.0);
    }
}

/// Create gradient analysis output for file export
pub fn create_gradient_analysis_output(
    args: &GradientArgs,
    start_lab: Lab,
    end_lab: Lab,
    gradient_values: &[GradientValue],
) -> Result<crate::output_formats::GradientAnalysisOutput> {
    use crate::ColorUtils;
    use crate::output_formats::*;
    use palette::{Srgb};

    // Convert Lab colors to RGB for color info using ColorUtils
    let start_srgb = ColorUtils::lab_to_srgb(start_lab);
    let start_rgb = ColorUtils::lab_to_rgb(start_lab);
    let end_srgb = ColorUtils::lab_to_srgb(end_lab);
    let end_rgb = ColorUtils::lab_to_rgb(end_lab);

    let gradient_output = GradientAnalysisOutput {
        metadata: ProgramMetadata::new(None),
        configuration: GradientConfiguration {
            start_color: args.start_color.clone(),
            end_color: args.end_color.clone(),
            start_position: args.start_position,
            end_position: args.end_position,
            ease_in: args.ease_in,
            ease_out: args.ease_out,
            gradient_steps: gradient_values.len(),
        },
        colors: GradientColors {
            start: ColorInfo {
                hex: format!(
                    "#{:02X}{:02X}{:02X}",
                    start_rgb.0, start_rgb.1, start_rgb.2
                ),
                rgb: format!(
                    "rgb({}, {}, {})",
                    start_rgb.0, start_rgb.1, start_rgb.2
                ),
                lab: format!(
                    "lab({:.2}, {:.2}, {:.2})",
                    start_lab.l, start_lab.a, start_lab.b
                ),
                lch: crate::format_utils::FormatUtils::lab_to_lch(start_lab),
                wcag21_relative_luminance: ColorUtils::wcag_relative_luminance(start_srgb),
            },
            end: ColorInfo {
                hex: format!(
                    "#{:02X}{:02X}{:02X}",
                    end_rgb.0, end_rgb.1, end_rgb.2
                ),
                rgb: Utils::rgb_to_string(end_rgb.0, end_rgb.1, end_rgb.2),
                lab: format!("lab({:.2}, {:.2}, {:.2})", end_lab.l, end_lab.a, end_lab.b),
                lch: crate::format_utils::FormatUtils::lab_to_lch(end_lab),
                wcag21_relative_luminance: ColorUtils::wcag_relative_luminance(end_srgb),
            },
        },
        gradient_stops: gradient_values
            .iter()
            .map(|gv| {
                // Parse the position from the string (like "0%" -> 0.0)
                let position_str = gv.position.trim_end_matches('%');
                let position = position_str.parse::<f64>().unwrap_or(0.0) / 100.0;

                // Parse the hex color to get Lab for luminance calculation
                let hex_clean = gv.hex.trim_start_matches('#');
                let r = u8::from_str_radix(&hex_clean[0..2], 16).unwrap_or(0);
                let g = u8::from_str_radix(&hex_clean[2..4], 16).unwrap_or(0);
                let b = u8::from_str_radix(&hex_clean[4..6], 16).unwrap_or(0);
                let rgb: Srgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
                let lab: Lab = ColorUtils::srgb_to_lab(rgb);

                // Create color parser for name matching
                let parser = crate::color_parser::ColorParser::new();
                let color_name = get_gradient_color_name_info((r, g, b), &parser);

                GradientStop {
                    position,
                    hex: gv.hex.clone(),
                    rgb: gv.rgb.clone(),
                    lab: format!("lab({:.2}, {:.2}, {:.2})", lab.l, lab.a, lab.b),
                    lch: crate::format_utils::FormatUtils::lab_to_lch(lab),
                    wcag21_relative_luminance: ColorUtils::wcag_relative_luminance(rgb),
                    color_name,
                }
            })
            .collect(),
    };

    Ok(gradient_output)
}

/// Get color name information for gradient stops
fn get_gradient_color_name_info(
    rgb: (u8, u8, u8),
    parser: &crate::color_parser::ColorParser,
) -> Option<crate::output_formats::ColorNameInfo> {
    use crate::color_parser::UniversalColor;

    let target = UniversalColor::from_rgb([rgb.0, rgb.1, rgb.2]);
    let target_hex = format!("#{:02X}{:02X}{:02X}", rgb.0, rgb.1, rgb.2);

    // Use the enhanced color matching from all collections
    create_gradient_enhanced_color_name_info(parser, &target_hex, &target)
}

/// Create enhanced color name information for gradient stops with matches from all collections
fn create_gradient_enhanced_color_name_info(
    parser: &crate::color_parser::ColorParser,
    input_hex: &str,
    target: &crate::color_parser::UniversalColor,
) -> Option<crate::output_formats::ColorNameInfo> {
    use crate::color_parser::ColorCollection;
    use crate::output_formats::{
        CollectionColorMatch, ColorNameAllCollections, ColorNameInfo, NearestColorMatch,
    };

    let mut has_any_match = false;

    // Create collection matches for CSS colors
    let css_match = {
        let css_collection = parser.css_collection();
        let exact_match = css_collection.colors().iter().find(|color| {
            let color_hex = format!(
                "#{:02X}{:02X}{:02X}",
                color.color.rgb[0], color.color.rgb[1], color.color.rgb[2]
            );
            color_hex.to_uppercase() == input_hex.to_uppercase()
        });

        let exact = exact_match.map(|color| color.metadata.name.clone());

        let nearest_matches = css_collection.find_closest(target, 1, None);
        let nearest = nearest_matches.first().map(|closest| NearestColorMatch {
            name: closest.entry.metadata.name.clone(),
            collection: "CSS".to_string(),
            distance: closest.distance,
        });

        if exact.is_some() || nearest.is_some() {
            has_any_match = true;
            Some(CollectionColorMatch { exact, nearest })
        } else {
            None
        }
    };

    // Create collection matches for RAL Classic colors
    let ral_classic_match = {
        let ral_classic_matches =
            parser.find_closest_ral_classic([target.rgb[0], target.rgb[1], target.rgb[2]], 1);
        if let Some(closest) = ral_classic_matches.first() {
            // Check for exact match by comparing hex values
            let color_hex = format!(
                "#{:02X}{:02X}{:02X}",
                closest.entry.color.rgb[0], closest.entry.color.rgb[1], closest.entry.color.rgb[2]
            );
            let exact = if color_hex.to_uppercase() == input_hex.to_uppercase() {
                Some(closest.entry.metadata.name.clone())
            } else {
                None
            };

            let nearest = Some(NearestColorMatch {
                name: closest.entry.metadata.name.clone(),
                collection: "RAL Classic".to_string(),
                distance: closest.distance,
            });

            has_any_match = true;
            Some(CollectionColorMatch { exact, nearest })
        } else {
            None
        }
    };

    // Create collection matches for RAL Design colors
    let ral_design_match = {
        let ral_design_matches =
            parser.find_closest_ral_design([target.rgb[0], target.rgb[1], target.rgb[2]], 1);
        if let Some(closest) = ral_design_matches.first() {
            // Check for exact match by comparing hex values
            let color_hex = format!(
                "#{:02X}{:02X}{:02X}",
                closest.entry.color.rgb[0], closest.entry.color.rgb[1], closest.entry.color.rgb[2]
            );
            let exact = if color_hex.to_uppercase() == input_hex.to_uppercase() {
                Some(closest.entry.metadata.name.clone())
            } else {
                None
            };

            let nearest = Some(NearestColorMatch {
                name: closest.entry.metadata.name.clone(),
                collection: "RAL Design".to_string(),
                distance: closest.distance,
            });

            has_any_match = true;
            Some(CollectionColorMatch { exact, nearest })
        } else {
            None
        }
    };

    if !has_any_match {
        return None;
    }

    // For backward compatibility with existing code, use CSS collection for primary exact/nearest
    let primary_exact = css_match.as_ref().and_then(|m| m.exact.clone());
    let primary_nearest = css_match.as_ref().and_then(|m| m.nearest.clone());

    // Create the all_collections data
    let all_collections = ColorNameAllCollections {
        css: css_match,
        ral_classic: ral_classic_match,
        ral_design: ral_design_match,
    };

    Some(ColorNameInfo {
        exact: primary_exact,
        nearest: primary_nearest,
        all_collections: Some(all_collections),
    })
}

/// Format and display gradient structured output to terminal with optional file save
fn format_gradient_structured_output(
    gradient_data: &crate::output_formats::GradientAnalysisOutput,
    format: &crate::cli::OutputFormat,
    file_path: Option<&String>,
) -> crate::error::Result<()> {
    use crate::file_output::FileOutputService;

    // Generate formatted output
    let formatted_output = match format {
        crate::cli::OutputFormat::Toml => toml::to_string_pretty(gradient_data).map_err(|e| {
            crate::error::ColorError::General(format!("TOML serialization failed: {}", e))
        })?,
        crate::cli::OutputFormat::Yaml => serde_yml::to_string(gradient_data).map_err(|e| {
            crate::error::ColorError::General(format!("YAML serialization failed: {}", e))
        })?,
    };

    // Display colorized structured output to terminal
    display_colorized_gradient_output(&formatted_output, format);

    // Save to file if requested
    if let Some(filename) = file_path {
        let extension = match format {
            crate::cli::OutputFormat::Toml => "toml",
            crate::cli::OutputFormat::Yaml => "yaml",
        };

        let full_filename = if filename.contains('.') {
            filename.clone()
        } else {
            format!("{}.{}", filename, extension)
        };

        match format {
            crate::cli::OutputFormat::Toml => {
                FileOutputService::write_gradient_toml(gradient_data, &full_filename)?;
                println!("Gradient analysis saved to TOML file: {}", full_filename);
            }
            crate::cli::OutputFormat::Yaml => {
                FileOutputService::write_gradient_yaml(gradient_data, &full_filename)?;
                println!("Gradient analysis saved to YAML file: {}", full_filename);
            }
        }
    }

    Ok(())
}

/// Display TOML/YAML gradient output to terminal with colorization
fn display_colorized_gradient_output(content: &str, format: &crate::cli::OutputFormat) {
    use colored::*;

    println!(
        "{}",
        format!(
            "# {} OUTPUT",
            match format {
                crate::cli::OutputFormat::Toml => "TOML",
                crate::cli::OutputFormat::Yaml => "YAML",
            }
        )
        .bold()
        .blue()
    );

    for line in content.lines() {
        let colored_line = colorize_gradient_line(line, format);
        println!("{}", colored_line);
    }
}

/// Colorize a single line of gradient TOML/YAML output
fn colorize_gradient_line(line: &str, format: &crate::cli::OutputFormat) -> String {
    use colored::*;

    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];

    match format {
        crate::cli::OutputFormat::Toml => {
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                // Section headers like [metadata], [configuration]
                format!("{}{}", indent, trimmed.bold().cyan())
            } else if trimmed.starts_with("[[") && trimmed.ends_with("]]") {
                // Array section headers like [[gradient_stops]]
                format!("{}{}", indent, trimmed.bold().magenta())
            } else if let Some(eq_pos) = trimmed.find(" = ") {
                // Key = value pairs
                let key = &trimmed[..eq_pos];
                let value = &trimmed[eq_pos + 3..];
                format!(
                    "{}{} = {}",
                    indent,
                    key.green(),
                    crate::output_utils::OutputUtils::colorize_output_value(value)
                )
            } else {
                line.to_string()
            }
        }
        crate::cli::OutputFormat::Yaml => {
            if trimmed.ends_with(':') && !trimmed.contains(' ') {
                // Top-level keys like "metadata:", "configuration:"
                format!("{}{}", indent, trimmed.bold().cyan())
            } else if let Some(colon_pos) = trimmed.find(": ") {
                // Key: value pairs
                let key = &trimmed[..colon_pos + 1];
                let value = &trimmed[colon_pos + 2..];
                format!(
                    "{}{} {}",
                    indent,
                    key.green(),
                    crate::output_utils::OutputUtils::colorize_output_value(value)
                )
            } else if let Some(stripped) = trimmed.strip_prefix("- ") {
                // Array items
                format!(
                    "{}- {}",
                    indent,
                    crate::output_utils::OutputUtils::colorize_output_value(stripped)
                )
            } else {
                line.to_string()
            }
        }
    }
}
