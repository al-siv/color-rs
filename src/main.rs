use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use palette::{Lab, Srgb, FromColor, IntoColor};
use std::fs;

fn parse_percentage(s: &str) -> Result<u8, String> {
    let trimmed = s.trim_end_matches('%');
    trimmed.parse::<u8>().map_err(|_| format!("Invalid percentage value: {}", s))
}

#[derive(Parser)]
#[command(name = "color-rs")]
#[command(about = "A CLI tool for color gradient calculations using LAB color space")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a gradient between two colors using LAB color space
    Gradient(GradientArgs),
}

#[derive(Args)]
struct GradientArgs {
    /// Starting color in HEX format (e.g., #FF0000 or FF0000)
    #[arg(long, value_name = "HEX")]
    start_color: String,

    /// Starting position as percentage (e.g., 20 or 20%, default: 0%)
    #[arg(long, value_name = "PERCENT", value_parser = parse_percentage, default_value = "0")]
    start_position: u8,

    /// Ending color in HEX format (e.g., #0000FF or 0000FF)
    #[arg(long, value_name = "HEX")]
    end_color: String,

    /// Ending position as percentage (e.g., 80 or 80%, default: 100%)
    #[arg(long, value_name = "PERCENT", value_parser = parse_percentage, default_value = "100")]
    end_position: u8,

    /// Ease-in control point for cubic-bezier (0.0-1.0, default: 0.42)
    #[arg(long, default_value = "0.42")]
    ease_in: f64,

    /// Ease-out control point for cubic-bezier (0.0-1.0, default: 0.58)
    #[arg(long, default_value = "0.58")]
    ease_out: f64,

    /// Generate SVG image of the gradient
    #[arg(long)]
    img: bool,

    /// Width of the SVG image in pixels (default: 1000)
    #[arg(long, default_value = "1000")]
    width: u32,

    /// Output filename for SVG image (default: gradient.svg)
    #[arg(long, default_value = "gradient.svg")]
    img_name: String,
}

fn main() -> Result<()> {
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

/// Cubic Bezier easing function (CSS timing function)
/// Implements cubic-bezier(x1, 0, x2, 1) for t in [0, 1]
/// This matches CSS timing functions like ease-in-out: cubic-bezier(0.42, 0, 0.58, 1)
fn cubic_bezier_ease(t: f64, x1: f64, x2: f64) -> f64 {
    if t <= 0.0 { return 0.0; }
    if t >= 1.0 { return 1.0; }
    
    // Clamp control points to valid range
    let x1 = x1.clamp(0.0, 1.0);
    let x2 = x2.clamp(0.0, 1.0);
    
    // For cubic bezier with control points (0,0), (x1,0), (x2,1), (1,1)
    // We need to solve for t value where x-coordinate equals input t
    // Then return the corresponding y-coordinate
    
    // Use Newton-Raphson method to find parameter value
    cubic_bezier_solve(t, x1, x2)
}

/// Solve cubic bezier equation to find y-value for given x using Newton-Raphson
fn cubic_bezier_solve(x: f64, x1: f64, x2: f64) -> f64 {
    // For cubic bezier (0,0), (x1,0), (x2,1), (1,1)
    // x(t) = 3(1-t)²t*x1 + 3(1-t)t²*x2 + t³
    // y(t) = 3(1-t)t²*1 + t³ = 3(1-t)t² + t³ = t²(3-3t+t) = t²(3-2t)
    
    let mut t = x; // Initial guess
    
    // Newton-Raphson iteration
    for _ in 0..8 {
        let current_x = cubic_bezier_x(t, x1, x2);
        let error = current_x - x;
        
        if error.abs() < 1e-7 {
            break;
        }
        
        let derivative = cubic_bezier_x_derivative(t, x1, x2);
        if derivative.abs() < 1e-7 {
            break;
        }
        
        t = t - error / derivative;
        t = t.clamp(0.0, 1.0);
    }
    
    // Return y-coordinate for the solved t
    cubic_bezier_y(t)
}

/// Calculate x-coordinate of cubic bezier curve
fn cubic_bezier_x(t: f64, x1: f64, x2: f64) -> f64 {
    let u = 1.0 - t;
    3.0 * u * u * t * x1 + 3.0 * u * t * t * x2 + t * t * t
}

/// Calculate derivative of x-coordinate 
fn cubic_bezier_x_derivative(t: f64, x1: f64, x2: f64) -> f64 {
    let u = 1.0 - t;
    3.0 * u * u * x1 + 6.0 * u * t * (x2 - x1) + 3.0 * t * t * (1.0 - x2)
}

/// Calculate y-coordinate of cubic bezier curve with control points (0,0), (x1,0), (x2,1), (1,1)
fn cubic_bezier_y(t: f64) -> f64 {
    // y(t) = 3(1-t)t² + t³ = t²(3-2t)
    t * t * (3.0 - 2.0 * t)
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
    let height = 100; // Fixed height for the gradient bar
    
    let start_hex = lab_to_hex(start_lab);
    let end_hex = lab_to_hex(end_lab);
    
    // Calculate positions as pixels
    let start_pixel = (args.start_position as f64 / 100.0 * width as f64) as u32;
    let end_pixel = (args.end_position as f64 / 100.0 * width as f64) as u32;
    
    let mut svg = String::new();
    svg.push_str(&format!(r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#, width, height));
    svg.push('\n');
    
    // Add CSS animation with cubic-bezier timing
    svg.push_str("  <style>\n");
    svg.push_str("    .gradient-rect {\n");
    svg.push_str(&format!("      animation: colorTransition 1s ease-out forwards;\n"));
    svg.push_str(&format!("      animation-timing-function: cubic-bezier({}, 0, {}, 1);\n", args.ease_in, args.ease_out));
    svg.push_str("    }\n");
    svg.push_str("    @keyframes colorTransition {\n");
    svg.push_str(&format!("      from {{ stop-color: {}; }}\n", start_hex));
    svg.push_str(&format!("      to {{ stop-color: {}; }}\n", end_hex));
    svg.push_str("    }\n");
    svg.push_str("  </style>\n");
    
    // Add gradient definition using cubic-bezier directly
    svg.push_str("  <defs>\n");
    svg.push_str("    <linearGradient id=\"grad\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"0%\">\n");
    
    // Use only start and end colors - let CSS cubic-bezier handle the interpolation
    svg.push_str(&format!("      <stop offset=\"0%\" stop-color=\"{}\" />\n", start_hex));
    svg.push_str(&format!("      <stop offset=\"100%\" stop-color=\"{}\" />\n", end_hex));
    svg.push_str(&format!("        <animate attributeName=\"offset\" values=\"0%;100%\" dur=\"1s\" \n"));
    svg.push_str(&format!("                 calcMode=\"spline\" keySplines=\"{} 0 {} 1\" keyTimes=\"0;1\" />\n", args.ease_in, args.ease_out));
    
    svg.push_str("    </linearGradient>\n");
    svg.push_str("  </defs>\n");
    
    // Left fill (0% to start_position) with start color
    if start_pixel > 0 {
        svg.push_str(&format!("  <rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n",
                             start_pixel, height, start_hex));
    }
    
    // Gradient section (start_position to end_position)
    if end_pixel > start_pixel {
        svg.push_str(&format!("  <rect x=\"{}\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"url(#grad)\" />\n",
                             start_pixel, end_pixel - start_pixel, height));
    }
    
    // Right fill (end_position to 100%) with end color
    if end_pixel < width {
        svg.push_str(&format!("  <rect x=\"{}\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n",
                             end_pixel, width - end_pixel, height, end_hex));
    }
    
    // Add information text
    svg.push_str(&format!("  <text x=\"10\" y=\"{}\" font-family=\"monospace\" font-size=\"12\" fill=\"white\">\n", height + 20));
    svg.push_str(&format!("    cubic-bezier({}, 0, {}, 1)\n", args.ease_in, args.ease_out));
    svg.push_str("  </text>\n");
    
    svg.push_str("</svg>");
    
    Ok(svg)
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
