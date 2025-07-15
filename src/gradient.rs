//! Gradient calculations and easing functions for color-rs

use kurbo::{CubicBez, ParamCurve, Point};
use palette::Lab;
use tabled::Tabled;
use crate::cli::GradientArgs;
use crate::color::ColorProcessor;
use crate::config::*;
use crate::error::{Result, ColorError};

/// Gradient value for display in tables
#[derive(Tabled)]
pub struct GradientValue {
    #[tabled(rename = "Position")]
    pub position: String,
    #[tabled(rename = "Hex")]
    pub hex: String,
    #[tabled(rename = "RGB")]
    pub rgb: String,
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
        let mut low = 0.0;
        let mut high = 1.0;

        for _ in 0..MAX_ITERATIONS {
            let mid = (low + high) * 0.5;
            let point = curve.eval(mid);
            let current_x = point.x;

            if (current_x - target_x).abs() < EPSILON {
                return point.y.clamp(0.0, 1.0);
            }

            if current_x < target_x {
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

        if let Some(num_stops) = args.grad_stops {
            // Intelligent stop placement with integer percentages
            let stop_positions = Self::calculate_intelligent_stops_integer(
                num_stops,
                args.ease_in,
                args.ease_out,
                args.start_position,
                args.end_position,
            );

            for &position in stop_positions.iter() {
                let normalized_t = (position - args.start_position) as f64
                    / (args.end_position - args.start_position) as f64;
                let smooth_t = Self::cubic_bezier_ease(normalized_t, args.ease_in, args.ease_out);
                let interpolated_lab = ColorProcessor::interpolate_lab(start_lab, end_lab, smooth_t);
                let hex_color = ColorProcessor::lab_to_hex(interpolated_lab);
                let rgb_values = ColorProcessor::lab_to_rgb_values(interpolated_lab);

                gradient_values.push(GradientValue {
                    position: format!("{}%", position),
                    hex: hex_color,
                    rgb: format!("rgb({}, {}, {})", rgb_values.0, rgb_values.1, rgb_values.2),
                });
            }
        } else if let Some(num_stops) = args.grad_stops_simple {
            // Simple equal spacing with integer percentages
            for i in 0..num_stops {
                let t = if num_stops == 1 {
                    0.5
                } else {
                    i as f64 / (num_stops - 1) as f64
                };

                let position_float =
                    args.start_position as f64 + t * (args.end_position - args.start_position) as f64;
                let position = position_float.round() as u8;
                let normalized_t = (position - args.start_position) as f64
                    / (args.end_position - args.start_position) as f64;
                let smooth_t = Self::cubic_bezier_ease(normalized_t, args.ease_in, args.ease_out);
                let interpolated_lab = ColorProcessor::interpolate_lab(start_lab, end_lab, smooth_t);
                let hex_color = ColorProcessor::lab_to_hex(interpolated_lab);
                let rgb_values = ColorProcessor::lab_to_rgb_values(interpolated_lab);

                gradient_values.push(GradientValue {
                    position: format!("{}%", position),
                    hex: hex_color,
                    rgb: format!("rgb({}, {}, {})", rgb_values.0, rgb_values.1, rgb_values.2),
                });
            }

            // Remove duplicates based on position
            gradient_values.dedup_by(|a, b| a.position == b.position);
        } else {
            // Default behavior: every grad_step percent (already integer)
            let mut position = args.start_position;
            while position <= args.end_position {
                let normalized_t = (position - args.start_position) as f64
                    / (args.end_position - args.start_position) as f64;

                let smooth_t = Self::cubic_bezier_ease(normalized_t, args.ease_in, args.ease_out);
                let interpolated_lab = ColorProcessor::interpolate_lab(start_lab, end_lab, smooth_t);
                let hex_color = ColorProcessor::lab_to_hex(interpolated_lab);
                let rgb_values = ColorProcessor::lab_to_rgb_values(interpolated_lab);

                gradient_values.push(GradientValue {
                    position: format!("{}%", position),
                    hex: hex_color,
                    rgb: format!("rgb({}, {}, {})", rgb_values.0, rgb_values.1, rgb_values.2),
                });

                position += args.grad_step;
                if position > args.end_position && position - args.grad_step < args.end_position {
                    // Ensure we always include the end position
                    position = args.end_position;
                } else if position > args.end_position {
                    break;
                }
            }
        }

        Ok(gradient_values)
    }

    /// Print gradient table
    pub fn print_gradient_table(values: Vec<GradientValue>) {
        if values.is_empty() {
            return;
        }

        use colored::*;
        use tabled::{Table, settings::{Alignment, Style, object::Columns}};

        println!(
            "{}",
            " Gradient Values: "
                .bold()
                .to_uppercase()
                .black()
                .on_bright_white()
        );
        let mut table = Table::new(values);
        table.with(Style::rounded());
        table.modify(Columns::first(), Alignment::right()); // Right-align Position column
        println!("{}", table);
    }
}

/// Main gradient generation function
pub fn generate_gradient(args: GradientArgs) -> Result<()> {
    // Validate arguments
    args.validate()?;

    // Parse colors using unified color parser
    use crate::color_parser::ColorParser;
    let parser = ColorParser::new();
    
    let (start_lab, _) = parser.parse(&args.start_color)
        .map_err(|e| ColorError::InvalidColor(format!("Failed to parse start color '{}': {}", args.start_color, e)))?;
    let (end_lab, _) = parser.parse(&args.end_color)
        .map_err(|e| ColorError::InvalidColor(format!("Failed to parse end color '{}': {}", args.end_color, e)))?;

    // Print color information with beautiful formatting
    ColorProcessor::print_color_info_table(start_lab, end_lab);

    // Generate images if requested
    if args.svg || args.png {
        use crate::image::ImageGenerator;
        let generator = ImageGenerator::new();
        
        if args.svg {
            generator.generate_svg(&args, start_lab, end_lab)?;
            use colored::*;
            println!(
                "{} {}\n",
                "SVG gradient saved to:".green().bold(),
                args.svg_name.bright_white()
            );
        }

        if args.png {
            generator.generate_png(&args, start_lab, end_lab)?;
            use colored::*;
            println!(
                "{} {}\n",
                "PNG gradient saved to:".green().bold(),
                args.png_name.bright_white()
            );
        }
    }

    // Generate gradient values for console output
    let gradient_values = GradientCalculator::generate_gradient_values(&args, start_lab, end_lab)?;
    GradientCalculator::print_gradient_table(gradient_values);

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
        assert!(stops[stops.len()-1] <= 1.0);
    }

    #[test]
    fn test_intelligent_stops_integer() {
        let stops = GradientCalculator::calculate_intelligent_stops_integer(5, 0.25, 0.75, 0, 100);
        assert_eq!(stops[0], 0);
        assert_eq!(stops[stops.len()-1], 100);
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
