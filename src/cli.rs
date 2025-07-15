//! Command-line interface for color-rs

use clap::{Args, Parser, Subcommand};
use crate::config::*;
use crate::error::{ColorError, Result};

/// Parse percentage values for CLI arguments
fn parse_percentage(s: &str) -> std::result::Result<u8, String> {
    let trimmed = s.trim_end_matches('%');
    trimmed
        .parse::<u8>()
        .map_err(|_| format!("Invalid percentage value: {}", s))
}

/// Main CLI structure
#[derive(Parser)]
#[command(name = APP_NAME)]
#[command(about = APP_ABOUT)]
#[command(author = APP_AUTHOR)]
#[command(version = APP_VERSION)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands
#[derive(Subcommand)]
pub enum Commands {
    /// Generate a gradient between two colors using LAB color space with cubic-bezier timing
    Gradient(GradientArgs),
    /// Match and convert a color between different color spaces
    ColorMatch(ColorMatchArgs),
}

/// Arguments for gradient generation
#[derive(Args, Clone)]
pub struct GradientArgs {
    /// Starting color (HEX, RGB, HSL, or named color, e.g., #FF0000, rgb(255,0,0), red)
    #[arg(long, value_name = "COLOR")]
    pub start_color: String,

    /// Starting position as percentage (e.g., 20 or 20%, default: 0%)
    #[arg(long, value_name = "PERCENT", value_parser = parse_percentage, default_value = DEFAULT_START_POSITION)]
    pub start_position: u8,

    /// Ending color (HEX, RGB, HSL, or named color, e.g., #0000FF, rgb(0,0,255), blue)
    #[arg(long, value_name = "COLOR")]
    pub end_color: String,

    /// Ending position as percentage (e.g., 80 or 80%, default: 100%)
    #[arg(long, value_name = "PERCENT", value_parser = parse_percentage, default_value = DEFAULT_END_POSITION)]
    pub end_position: u8,

    /// Ease-in control point for cubic-bezier (0.0-1.0, default: 0.65)
    #[arg(long, default_value = DEFAULT_EASE_IN)]
    pub ease_in: f64,

    /// Ease-out control point for cubic-bezier (0.0-1.0, default: 0.35)
    #[arg(long, default_value = DEFAULT_EASE_OUT)]
    pub ease_out: f64,

    /// Generate SVG image of the gradient
    #[arg(long)]
    pub svg: bool,

    /// Generate PNG image of the gradient
    #[arg(long)]
    pub png: bool,

    /// Disable legend/caption on gradient images (only valid with --svg or --png)
    #[arg(long)]
    pub no_legend: bool,

    /// Width of the image in pixels (default: 1000)
    #[arg(long, default_value = DEFAULT_WIDTH)]
    pub width: u32,

    /// Output filename for SVG image (default: gradient.svg)
    #[arg(long, default_value = DEFAULT_SVG_NAME)]
    pub svg_name: String,

    /// Output filename for PNG image (default: gradient.png)
    #[arg(long, default_value = DEFAULT_PNG_NAME)]
    pub png_name: String,

    /// Output gradient values every X percent (default: 5%)
    #[arg(long, default_value = DEFAULT_GRAD_STEP, conflicts_with_all = ["grad_stops", "grad_stops_simple"], help = "Output gradient values every X percent (default: 5%)")]
    pub grad_step: u8,

    /// Number of intelligently placed gradient stops to output
    #[arg(long, conflicts_with_all = ["grad_step", "grad_stops_simple"], help = "Number of intelligently placed gradient stops using curve derivatives")]
    pub grad_stops: Option<usize>,

    /// Number of equally spaced gradient stops to output
    #[arg(long, conflicts_with_all = ["grad_step", "grad_stops"], help = "Number of equally spaced gradient stops")]
    pub grad_stops_simple: Option<usize>,
}

impl GradientArgs {
    /// Validate the gradient arguments
    pub fn validate(&self) -> Result<()> {
        // Validate position bounds
        if self.start_position > MAX_PERCENTAGE || self.end_position > MAX_PERCENTAGE {
            return Err(ColorError::InvalidArguments(
                "Positions must be between 0 and 100".to_string(),
            ));
        }

        // Validate position order
        if self.start_position >= self.end_position {
            return Err(ColorError::InvalidArguments(
                "Start position must be less than end position".to_string(),
            ));
        }

        // Validate ease values
        if self.ease_in < BEZIER_MIN || self.ease_in > BEZIER_MAX {
            return Err(ColorError::InvalidArguments(
                "Ease-in value must be between 0.0 and 1.0".to_string(),
            ));
        }

        if self.ease_out < BEZIER_MIN || self.ease_out > BEZIER_MAX {
            return Err(ColorError::InvalidArguments(
                "Ease-out value must be between 0.0 and 1.0".to_string(),
            ));
        }

        // Validate --no-legend usage
        if self.no_legend && !self.svg && !self.png {
            return Err(ColorError::InvalidArguments(
                "--no-legend can only be used with --svg or --png".to_string(),
            ));
        }

        // Validate width
        if self.width == 0 {
            return Err(ColorError::InvalidArguments(
                "Width must be greater than 0".to_string(),
            ));
        }

        // Validate grad_step
        if self.grad_step == 0 {
            return Err(ColorError::InvalidArguments(
                "Gradient step must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }
}

/// Arguments for color matching and conversion
#[derive(Args, Clone)]
pub struct ColorMatchArgs {
    /// Input color value (any format: hex, rgb(), rgba(), hsl(), hsla(), or color name)
    #[arg(value_name = "COLOR")]
    pub color: String,
}

impl ColorMatchArgs {
    /// Validate the color match arguments
    pub fn validate(&self) -> Result<()> {
        // The color validation will be done by the color parser
        // No specific validation needed for CLI args
        Ok(())
    }
}

/// Print application information in cargo style
pub fn print_app_info() {
    use colored::*;
    
    println!(
        "{:>13} {} v{}",
        "Application:".green().bold(),
        APP_NAME,
        APP_VERSION
    );
    println!("{:>13} {}", "About:".green().bold(), APP_ABOUT);
    println!("{:>13} {}", "Author:".green().bold(), APP_AUTHOR);
    println!();
}
