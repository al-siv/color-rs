// Using kurbo library for robust 2D curve operations
// kurbo is part of the Rust graphics ecosystem and provides:
// - Well-tested cubic Bezier implementations
// - Optimized mathematical operations
// - Industry-standard curve handling used by xi-editor, Runebender, etc.
use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use palette::{Lab, Srgb, Hsl, FromColor, IntoColor};
use std::fs;
use kurbo::{CubicBez, Point, ParamCurve};
use image::{ImageBuffer, Rgba, RgbaImage};
use tiny_skia::*;
use usvg::{Options, Tree, fontdb};
use resvg;
use colored::*;
use tabled::{Table, Tabled, settings::Style};

// Application metadata
const APP_NAME: &str = "Color-rs";
const APP_ABOUT: &str = "A CLI tool for color gradient calculations using LAB color space with cubic-bezier easing functions";
const APP_AUTHOR: &str = "https://github.com/al-siv";
const APP_VERSION: &str = "0.6.0";

// Height ratio: gradient height = width * HEIGHT_RATIO
const HEIGHT_RATIO: f64 = 0.2; // 1/5 of width

// Default values for GradientArgs
const DEFAULT_START_POSITION: &str = "0";
const DEFAULT_END_POSITION: &str = "100";
const DEFAULT_EASE_IN: &str = "0.65";
const DEFAULT_EASE_OUT: &str = "0.35";
const DEFAULT_WIDTH: &str = "1000";
const DEFAULT_SVG_NAME: &str = "gradient.svg";
const DEFAULT_PNG_NAME: &str = "gradient.png";
const DEFAULT_GRAD_STEP: &str = "5";

// Number of sample points for intelligent stop calculation
const INTELLIGENT_STOP_SAMPLE_POINTS: usize = 10000;

fn parse_percentage(s: &str) -> Result<u8, String> {
    let trimmed = s.trim_end_matches('%');
    trimmed.parse::<u8>().map_err(|_| format!("Invalid percentage value: {}", s))
}

#[derive(Parser)]
#[command(name = APP_NAME)]
#[command(about = APP_ABOUT)]
#[command(author = APP_AUTHOR)]
#[command(version = APP_VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a gradient between two colors using LAB color space with cubic-bezier timing
    Gradient(GradientArgs),
}

#[derive(Args)]
struct GradientArgs {
    /// Starting color in HEX format (e.g., #FF0000 or FF0000)
    #[arg(long, value_name = "HEX")]
    start_color: String,

    /// Starting position as percentage (e.g., 20 or 20%, default: 0%)
    #[arg(long, value_name = "PERCENT", value_parser = parse_percentage, default_value = DEFAULT_START_POSITION)]
    start_position: u8,

    /// Ending color in HEX format (e.g., #0000FF or 0000FF)
    #[arg(long, value_name = "HEX")]
    end_color: String,

    /// Ending position as percentage (e.g., 80 or 80%, default: 100%)
    #[arg(long, value_name = "PERCENT", value_parser = parse_percentage, default_value = DEFAULT_END_POSITION)]
    end_position: u8,

    /// Ease-in control point for cubic-bezier (0.0-1.0, default: 0.65)
    #[arg(long, default_value = DEFAULT_EASE_IN)]
    ease_in: f64,

    /// Ease-out control point for cubic-bezier (0.0-1.0, default: 0.35)
    #[arg(long, default_value = DEFAULT_EASE_OUT)]
    ease_out: f64,

    /// Generate SVG image of the gradient
    #[arg(long)]
    svg: bool,

    /// Generate PNG image of the gradient
    #[arg(long)]
    png: bool,

    /// Disable legend/caption on gradient images (only valid with --svg or --png)
    #[arg(long)]
    no_legend: bool,

    /// Width of the image in pixels (default: 1000)
    #[arg(long, default_value = DEFAULT_WIDTH)]
    width: u32,

    /// Output filename for SVG image (default: gradient.svg)
    #[arg(long, default_value = DEFAULT_SVG_NAME)]
    svg_name: String,

    /// Output filename for PNG image (default: gradient.png)
    #[arg(long, default_value = DEFAULT_PNG_NAME)]
    png_name: String,

    /// Output gradient values every X percent (default: 5%)
    #[arg(long, default_value = DEFAULT_GRAD_STEP, conflicts_with_all = ["grad_stops", "grad_stops_simple"], help = "Output gradient values every X percent (default: 5%)")]
    grad_step: u8,

    /// Number of intelligently placed gradient stops to output
    #[arg(long, conflicts_with_all = ["grad_step", "grad_stops_simple"], help = "Number of intelligently placed gradient stops using curve derivatives")]
    grad_stops: Option<usize>,

    /// Number of equally spaced gradient stops to output
    #[arg(long, conflicts_with_all = ["grad_step", "grad_stops"], help = "Number of equally spaced gradient stops")]
    grad_stops_simple: Option<usize>,
}

#[derive(Tabled)]
struct ColorInfo {
    #[tabled(rename = "Color")]
    label: String,
    #[tabled(rename = "Hex")]
    hex: String,
    #[tabled(rename = "RGB")]
    rgb: String,
    #[tabled(rename = "HSL")]
    hsl: String,
    #[tabled(rename = "Lab")]
    lab: String,
}

#[derive(Tabled)]
struct GradientValue {
    #[tabled(rename = "Position")]
    position: String,
    #[tabled(rename = "Hex")]
    hex: String,
    #[tabled(rename = "RGB")]
    rgb: String,
}

fn main() -> Result<()> {
    // Print program information with cargo-style formatting
    println!("{:>12} {} v{}", "Application:".green().bold(), APP_NAME, APP_VERSION);
    println!("{:>12} {}", "About:".green().bold(), APP_ABOUT);
    println!("{:>12} {}", "Author:".green().bold(), APP_AUTHOR);
    println!();

    let cli = Cli::parse();

    match cli.command {
        Commands::Gradient(args) => generate_gradient(args)?,
    }

    Ok(())
}

fn parse_hex_color(hex: &str) -> Result<Lab> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(anyhow!("Invalid HEX color format. Expected format: #RRGGBB"));
    }

    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    let rgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
    Ok(Lab::from_color(rgb))
}

fn lab_to_hex(lab: Lab) -> String {
    let rgb: Srgb = lab.into_color();
    let r = (rgb.red * 255.0).round() as u8;
    let g = (rgb.green * 255.0).round() as u8;
    let b = (rgb.blue * 255.0).round() as u8;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn lab_to_rgb_values(lab: Lab) -> (u8, u8, u8) {
    let rgb: Srgb = lab.into_color();
    let r = (rgb.red * 255.0).round() as u8;
    let g = (rgb.green * 255.0).round() as u8;
    let b = (rgb.blue * 255.0).round() as u8;
    (r, g, b)
}

fn lab_to_hsl_values(lab: Lab) -> (f32, f32, f32) {
    let hsl: Hsl = lab.into_color();
    (hsl.hue.into_positive_degrees(), hsl.saturation, hsl.lightness)
}

fn print_color_info_table(start_lab: Lab, end_lab: Lab) {
    let start_hex = lab_to_hex(start_lab);
    let start_rgb = lab_to_rgb_values(start_lab);
    let start_hsl = lab_to_hsl_values(start_lab);
    
    let end_hex = lab_to_hex(end_lab);
    let end_rgb = lab_to_rgb_values(end_lab);
    let end_hsl = lab_to_hsl_values(end_lab);
    
    let color_data = vec![
        ColorInfo {
            label: "Start Color".to_string(),
            hex: start_hex,
            rgb: format!("RGB({}, {}, {})", start_rgb.0, start_rgb.1, start_rgb.2),
            hsl: format!("HSL({:.1}°, {:.1}%, {:.1}%)", start_hsl.0, start_hsl.1 * 100.0, start_hsl.2 * 100.0),
            lab: format!("Lab({:.1}, {:.1}, {:.1})", start_lab.l, start_lab.a, start_lab.b),
        },
        ColorInfo {
            label: "End Color".to_string(),
            hex: end_hex,
            rgb: format!("RGB({}, {}, {})", end_rgb.0, end_rgb.1, end_rgb.2),
            hsl: format!("HSL({:.1}°, {:.1}%, {:.1}%)", end_hsl.0, end_hsl.1 * 100.0, end_hsl.2 * 100.0),
            lab: format!("Lab({:.1}, {:.1}, {:.1})", end_lab.l, end_lab.a, end_lab.b),
        },
    ];
    
    println!("{}", " Color Information: ".bold().to_uppercase().bright_yellow().on_blue());
    let mut table = Table::new(color_data);
    table.with(Style::rounded());
    println!("{}", table);
    println!();
}

/// Cubic Bezier easing function using kurbo library
/// Implements cubic-bezier(x1, 0, x2, 1) easing functions
/// This matches CSS timing functions like ease-in-out: cubic-bezier(0.42, 0, 0.58, 1)
fn cubic_bezier_ease(t: f64, x1: f64, x2: f64) -> f64 {
    if t <= 0.0 { return 0.0; }
    if t >= 1.0 { return 1.0; }
    
    // Clamp control points to valid range
    let x1 = x1.clamp(0.0, 1.0);
    let x2 = x2.clamp(0.0, 1.0);
    
    // Create cubic bezier curve with control points (0,0), (x1,0), (x2,1), (1,1)
    // This matches cubic-bezier specification
    let curve = CubicBez::new(
        Point::new(0.0, 0.0),      // Start point
        Point::new(x1, 0.0),       // First control point (x1, 0)
        Point::new(x2, 1.0),       // Second control point (x2, 1)
        Point::new(1.0, 1.0),      // End point
    );
    
    // Use binary search to find parameter where x-coordinate equals target
    solve_cubic_bezier_for_x(&curve, t)
}

/// Binary search to find parameter t where curve.eval(t).x == target_x
/// This replaces our custom Newton-Raphson implementation with a robust binary search
fn solve_cubic_bezier_for_x(curve: &CubicBez, target_x: f64) -> f64 {
    const EPSILON: f64 = 1e-7;
    const MAX_ITERATIONS: usize = 50;
    
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
fn calculate_intelligent_stops(num_stops: usize, ease_in: f64, ease_out: f64) -> Vec<f64> {
    if num_stops == 0 {
        return vec![];
    }
    if num_stops == 1 {
        return vec![0.5];
    }
    
    let x1 = ease_in.clamp(0.0, 1.0);
    let x2 = ease_out.clamp(0.0, 1.0);
    
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
fn calculate_intelligent_stops_integer(num_stops: usize, ease_in: f64, ease_out: f64, start_pos: u8, end_pos: u8) -> Vec<u8> {
    if num_stops == 0 {
        return vec![];
    }
    if num_stops == 1 {
        return vec![(start_pos + end_pos) / 2];
    }
    
    // Get floating point positions first
    let float_positions = calculate_intelligent_stops(num_stops, ease_in, ease_out);
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

fn interpolate_lab(start: Lab, end: Lab, t: f64) -> Lab {
    let t = t as f32;
    Lab::new(
        start.l + (end.l - start.l) * t,
        start.a + (end.a - start.a) * t,
        start.b + (end.b - start.b) * t,
    )
}

fn generate_svg_gradient(args: &GradientArgs, start_lab: Lab, end_lab: Lab) -> Result<String> {
    let width = args.width;
    let gradient_height = (width as f64 * HEIGHT_RATIO) as u32;
    let legend_height = if args.no_legend { 0 } else { (gradient_height as f64 * 0.2).max(20.0) as u32 };
    let total_height = gradient_height + legend_height;
    
    let start_hex = lab_to_hex(start_lab);
    let end_hex = lab_to_hex(end_lab);
    
    // Calculate positions as pixels
    let start_pixel = (args.start_position as f64 / 100.0 * width as f64) as u32;
    let end_pixel = (args.end_position as f64 / 100.0 * width as f64) as u32;
    
    let mut svg = String::new();
    svg.push_str(&format!(r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#, width, total_height));
    svg.push('\n');
    
    // Add gradient definition with properly calculated stops
    svg.push_str("  <defs>\n");
    svg.push_str("    <linearGradient id=\"grad\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"0%\">\n");
    
    // Calculate dynamic number of stops based on gradient span to prevent banding
    let gradient_span = args.end_position - args.start_position;
    let base_stops = (gradient_span as usize).saturating_mul(2);
    let num_stops = base_stops.max(10).min(1000);
    
    for i in 0..=num_stops {
        let t = i as f64 / num_stops as f64;
        let bezier_t = cubic_bezier_ease(t, args.ease_in, args.ease_out);
        let interpolated_lab = interpolate_lab(start_lab, end_lab, bezier_t);
        let hex_color = lab_to_hex(interpolated_lab);
        let offset = t * 100.0;
        
        svg.push_str(&format!("      <stop offset=\"{}%\" stop-color=\"{}\" />\n", 
                             offset, hex_color));
    }
    
    svg.push_str("    </linearGradient>\n");
    svg.push_str("  </defs>\n");
    
    // Left fill (0% to start_position) with start color
    if start_pixel > 0 {
        svg.push_str(&format!("  <rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n",
                             start_pixel, gradient_height, start_hex));
    }
    
    // Gradient section (start_position to end_position)
    if end_pixel > start_pixel {
        svg.push_str(&format!("  <rect x=\"{}\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"url(#grad)\" />\n",
                             start_pixel, end_pixel - start_pixel, gradient_height));
    }
    
    // Right fill (end_position to 100%) with end color
    if end_pixel < width {
        svg.push_str(&format!("  <rect x=\"{}\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n",
                             end_pixel, width - end_pixel, gradient_height, end_hex));
    }
    
    // Add legend if not disabled
    if !args.no_legend {
        let font_size = (legend_height as f64 * 0.6).max(10.0) as u32;
        let text_y = gradient_height + (legend_height as f64 * 0.75) as u32;
        
        svg.push_str(&format!("  <rect x=\"0\" y=\"{}\" width=\"100%\" height=\"{}\" fill=\"rgb(0,0,0)\" />\n", 
                             gradient_height, legend_height));
        svg.push_str(&format!("  <text x=\"{}\" y=\"{}\" font-family=\"'Montserrat', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif\" font-size=\"{}\" fill=\"white\">\n", 
                             width / 100, text_y, font_size));
        svg.push_str(&format!("    cubic-bezier({}, 0, {}, 1) | positions: {}%-{}% | colors: {}-{}\n", 
                             args.ease_in, args.ease_out, args.start_position, args.end_position, start_hex, end_hex));
        svg.push_str("  </text>\n");
    }
    
    svg.push_str("</svg>");
    
    Ok(svg)
}


fn generate_png_gradient(args: &GradientArgs, start_lab: Lab, end_lab: Lab) -> Result<()> {
    let width = args.width;
    let gradient_height = (width as f64 * HEIGHT_RATIO) as u32;
    let legend_height = if args.no_legend { 0 } else { (gradient_height as f64 * 0.2).max(20.0) as u32 };
    let total_height = gradient_height + legend_height;
    
    // Create SVG content using the same function as SVG generation
    let svg_content = generate_svg_gradient(args, start_lab, end_lab)?;
    
    // Configure usvg options with font database for text-to-paths conversion
    let mut options = Options::default();
    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();
    options.fontdb = std::sync::Arc::new(fontdb);
    
    // Parse SVG with font resolution
    let tree = Tree::from_str(&svg_content, &options)
        .map_err(|e| anyhow!("Failed to parse SVG: {}", e))?;
    
    // Create pixmap
    let mut pixmap = Pixmap::new(width, total_height)
        .ok_or_else(|| anyhow!("Failed to create pixmap"))?;
    
    // Render SVG to pixmap (this converts text to paths automatically)
    resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());
    
    // Convert to image crate format
    let img: RgbaImage = ImageBuffer::from_fn(width, total_height, |x, y| {
        let pixel = pixmap.pixel(x, y).unwrap();
        Rgba([pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()])
    });
    
    // Save PNG
    img.save(&args.png_name)
        .map_err(|e| anyhow!("Failed to save PNG: {}", e))?;
    
    Ok(())
}

fn print_gradient_table(values: Vec<GradientValue>) {
    if values.is_empty() {
        return;
    }
    
    println!("{}", " Gradient Values: ".bold().to_uppercase().bright_yellow().on_blue());
    let mut table = Table::new(values);
    table.with(Style::rounded());
    println!("{}", table);
}

fn generate_gradient(args: GradientArgs) -> Result<()> {
    // Validate position bounds first
    if args.start_position > 100 || args.end_position > 100 {
        return Err(anyhow!("Positions must be between 0 and 100"));
    }
    
    // Then validate position order
    if args.start_position >= args.end_position {
        return Err(anyhow!("Start position must be less than end position"));
    }

    // Validate --no-legend usage
    if args.no_legend && !args.svg && !args.png {
        return Err(anyhow!("--no-legend can only be used with --svg or --png"));
    }

    // Parse colors
    let start_lab = parse_hex_color(&args.start_color)?;
    let end_lab = parse_hex_color(&args.end_color)?;

    // Print color information with beautiful formatting
    print_color_info_table(start_lab, end_lab);

    // Generate SVG if requested
    if args.svg {
        let svg_content = generate_svg_gradient(&args, start_lab, end_lab)?;
        fs::write(&args.svg_name, svg_content)?;
        println!("{} {}\n", "SVG gradient saved to:".green().bold(), args.svg_name.bright_white());
    }

    // Generate PNG if requested
    if args.png {
        generate_png_gradient(&args, start_lab, end_lab)?;
        println!("{} {}\n", "PNG gradient saved to:".green().bold(), args.png_name.bright_white());
    }

    // Generate gradient (console output)
    let mut gradient_values = Vec::new();
    
    if let Some(num_stops) = args.grad_stops {
        // Intelligent stop placement with integer percentages
        let stop_positions = calculate_intelligent_stops_integer(num_stops, args.ease_in, args.ease_out, args.start_position, args.end_position);
        
        for &position in stop_positions.iter() {
            let normalized_t = (position - args.start_position) as f64 
                / (args.end_position - args.start_position) as f64;
            let smooth_t = cubic_bezier_ease(normalized_t, args.ease_in, args.ease_out);
            let interpolated_lab = interpolate_lab(start_lab, end_lab, smooth_t);
            let hex_color = lab_to_hex(interpolated_lab);
            let rgb_values = lab_to_rgb_values(interpolated_lab);
            
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
            
            let position_float = args.start_position as f64 + t * (args.end_position - args.start_position) as f64;
            let position = position_float.round() as u8;
            let normalized_t = (position - args.start_position) as f64 
                / (args.end_position - args.start_position) as f64;
            let smooth_t = cubic_bezier_ease(normalized_t, args.ease_in, args.ease_out);
            let interpolated_lab = interpolate_lab(start_lab, end_lab, smooth_t);
            let hex_color = lab_to_hex(interpolated_lab);
            let rgb_values = lab_to_rgb_values(interpolated_lab);
            
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
            
            let smooth_t = cubic_bezier_ease(normalized_t, args.ease_in, args.ease_out);
            let interpolated_lab = interpolate_lab(start_lab, end_lab, smooth_t);
            let hex_color = lab_to_hex(interpolated_lab);
            let rgb_values = lab_to_rgb_values(interpolated_lab);
            
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
    
    print_gradient_table(gradient_values);

    Ok(())
}
