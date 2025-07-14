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

    /// Smoothing coefficient for gradient curve (default: 2.0)
    #[arg(long, default_value = "2.0")]
    smoothing: f64,

    /// Additional curve tension parameter (default: 0.5)
    #[arg(long, default_value = "0.5")]
    tension: f64,

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

fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn smooth_interpolate(t: f64, smoothing: f64, tension: f64) -> f64 {
    // Create a smooth curve using a combination of functions
    let smooth_t = smoothstep(0.0, 1.0, t);
    
    // Apply additional smoothing
    let powered_t = smooth_t.powf(smoothing);
    
    // Add tension for a more natural curve
    let tension_factor = 1.0 + tension * (1.0 - 2.0 * (smooth_t - 0.5).abs());
    
    (powered_t * tension_factor).clamp(0.0, 1.0)
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
    
    // Add gradient definition
    svg.push_str("  <defs>\n");
    svg.push_str("    <linearGradient id=\"grad\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"0%\">\n");
    
    // Generate gradient stops
    for position in args.start_position..=args.end_position {
        let normalized_t = (position - args.start_position) as f64 
            / (args.end_position - args.start_position) as f64;
        let smooth_t = smooth_interpolate(normalized_t, args.smoothing, args.tension);
        let interpolated_lab = interpolate_lab(start_lab, end_lab, smooth_t);
        let hex_color = lab_to_hex(interpolated_lab);
        
        let stop_position = ((position - args.start_position) as f64 
            / (args.end_position - args.start_position) as f64) * 100.0;
        
        svg.push_str(&format!("      <stop offset=\"{}%\" stop-color=\"{}\" />\n", 
                             stop_position, hex_color));
    }
    
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
        let smooth_t = smooth_interpolate(normalized_t, args.smoothing, args.tension);
        
        // Interpolate color in LAB space
        let interpolated_lab = interpolate_lab(start_lab, end_lab, smooth_t);
        
        // Convert back to HEX
        let hex_color = lab_to_hex(interpolated_lab);
        
        println!("{}%: {}", position, hex_color);
    }

    Ok(())
}
