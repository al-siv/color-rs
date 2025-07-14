// Using kurbo library for robust 2D curve operations
// kurbo is part of the Rust graphics ecosystem and provides:
// - Well-tested cubic Bezier implementations
// - Optimized mathematical operations
// - Industry-standard curve handling used by xi-editor, Runebender, etc.
use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use palette::{Lab, Srgb, FromColor, IntoColor};
use std::fs;
use kurbo::{CubicBez, Point, ParamCurve};
use image::{ImageBuffer, Rgba, RgbaImage};
use tiny_skia::*;
use usvg::{Options, Tree};
use resvg;


const APP_NAME: &str = "color-rs";
const APP_ABOUT: &str = "A CLI tool for color gradient calculations using LAB color space with cubic-bezier easing functions";
const APP_AUTHOR: &str = "https://github.com/al-siv";
const APP_VERSION: &str = "0.5.0";

// Default values for GradientArgs
const DEFAULT_START_POSITION: &str = "0";
const DEFAULT_END_POSITION: &str = "100";
const DEFAULT_EASE_IN: &str = "0.65";
const DEFAULT_EASE_OUT: &str = "0.35";
const DEFAULT_WIDTH: &str = "1000";
const DEFAULT_SVG_NAME: &str = "gradient.svg";
const DEFAULT_PNG_NAME: &str = "gradient.png";

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
    img: bool,

    /// Generate PNG image of the gradient
    #[arg(long)]
    png: bool,

    /// Width of the image in pixels (default: 1000)
    #[arg(long, default_value = DEFAULT_WIDTH)]
    width: u32,

    /// Output filename for SVG image (default: gradient.svg)
    #[arg(long, default_value = DEFAULT_SVG_NAME)]
    img_name: String,

    /// Output filename for PNG image (default: gradient.png)
    #[arg(long, default_value = DEFAULT_PNG_NAME)]
    png_name: String,
}

fn main() -> Result<()> {
    // Print program information
    println!("{} v{} - {}", APP_NAME, APP_VERSION, APP_ABOUT);
    println!("Author: {}", APP_AUTHOR);
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
    let height = 120; // Increased height for text
    
    let start_hex = lab_to_hex(start_lab);
    let end_hex = lab_to_hex(end_lab);
    
    // Calculate positions as pixels
    let start_pixel = (args.start_position as f64 / 100.0 * width as f64) as u32;
    let end_pixel = (args.end_position as f64 / 100.0 * width as f64) as u32;
    
    let mut svg = String::new();
    svg.push_str(&format!(r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#, width, height));
    svg.push('\n');
    
    // Add gradient definition with properly calculated stops
    svg.push_str("  <defs>\n");
    svg.push_str("    <linearGradient id=\"grad\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"0%\">\n");
    
    // Calculate dynamic number of stops based on gradient span to prevent banding
    // Use quality factor of 2x the percentage span, with reasonable min/max bounds
    let gradient_span = args.end_position - args.start_position;
    let base_stops = (gradient_span as usize).saturating_mul(2); // Quality factor of 2
    let num_stops = base_stops.max(10).min(1000); // Min 10 stops, max 1000 for performance
    
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
        svg.push_str(&format!("  <rect x=\"0\" y=\"0\" width=\"{}\" height=\"100\" fill=\"{}\" />\n",
                             start_pixel, start_hex));
    }
    
    // Gradient section (start_position to end_position)
    if end_pixel > start_pixel {
        svg.push_str(&format!("  <rect x=\"{}\" y=\"0\" width=\"{}\" height=\"100\" fill=\"url(#grad)\" />\n",
                             start_pixel, end_pixel - start_pixel));
    }
    
    // Right fill (end_position to 100%) with end color
    if end_pixel < width {
        svg.push_str(&format!("  <rect x=\"{}\" y=\"0\" width=\"{}\" height=\"100\" fill=\"{}\" />\n",
                             end_pixel, width - end_pixel, end_hex));
    }
    
    // Add information text with dark background for readability
    svg.push_str("  <rect x=\"0\" y=\"100\" width=\"100%\" height=\"20\" fill=\"rgba(0,0,0,0.8)\" />\n");
    svg.push_str(&format!("  <text x=\"10\" y=\"115\" font-family=\"'Montserrat', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif\" font-size=\"12\" fill=\"white\">\n"));
    svg.push_str(&format!("    cubic-bezier({}, 0, {}, 1) | positions: {}%-{}% | colors: {}-{}\n", 
                         args.ease_in, args.ease_out, args.start_position, args.end_position, start_hex, end_hex));
    svg.push_str("  </text>\n");
    
    svg.push_str("</svg>");
    
    Ok(svg)
}

fn generate_png_gradient(args: &GradientArgs, start_lab: Lab, end_lab: Lab) -> Result<()> {
    let width = args.width;
    let height = 120;
    
    // Create SVG first
    let svg_content = generate_svg_gradient(args, start_lab, end_lab)?;
    
    // Parse SVG
    let options = Options::default();
    let tree = Tree::from_str(&svg_content, &options)
        .map_err(|e| anyhow!("Failed to parse SVG: {}", e))?;
    
    // Create pixmap
    let mut pixmap = Pixmap::new(width, height)
        .ok_or_else(|| anyhow!("Failed to create pixmap"))?;
    
    // Render SVG to pixmap
    resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());
    
    // Convert to image crate format
    let img: RgbaImage = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = pixmap.pixel(x, y).unwrap();
        Rgba([pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()])
    });
    
    // Save PNG
    img.save(&args.png_name)
        .map_err(|e| anyhow!("Failed to save PNG: {}", e))?;
    
    Ok(())
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

    // Parse colors
    let start_lab = parse_hex_color(&args.start_color)?;
    let end_lab = parse_hex_color(&args.end_color)?;

    // Generate SVG if requested
    if args.img {
        let svg_content = generate_svg_gradient(&args, start_lab, end_lab)?;
        fs::write(&args.img_name, svg_content)?;
        println!("SVG gradient saved to: {}", args.img_name);
    }

    // Generate PNG if requested
    if args.png {
        generate_png_gradient(&args, start_lab, end_lab)?;
        println!("PNG gradient saved to: {}", args.png_name);
    }

    // Generate gradient (console output)
    for position in args.start_position..=args.end_position {
        // Normalize position to range [0, 1]
        let normalized_t = (position - args.start_position) as f64 
            / (args.end_position - args.start_position) as f64;
        
        // Apply smoothing
        let smooth_t = cubic_bezier_ease(normalized_t, args.ease_in, args.ease_out);
        
        // Interpolate color in LAB space
        let interpolated_lab = interpolate_lab(start_lab, end_lab, smooth_t);
        
        // Convert back to HEX
        let hex_color = lab_to_hex(interpolated_lab);
        
        println!("{}%: {}", position, hex_color);
    }

    Ok(())
}
