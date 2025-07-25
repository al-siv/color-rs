//! Command-line interface for color-rs

use crate::config::{
    APP_AUTHOR, APP_DESCRIPTION, APP_NAME, APP_VERSION, BEZIER_MAX, BEZIER_MIN, DEFAULT_EASE_IN,
    DEFAULT_EASE_OUT, DEFAULT_END_POSITION, DEFAULT_START_POSITION,
    DEFAULT_WIDTH, MAX_PERCENTAGE,
};
use crate::error::{ColorError, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};

/// Output format for file export
#[derive(Debug, Clone, ValueEnum, Default)]
pub enum OutputFormat {
    /// TOML format output
    #[clap(alias = "t")]
    Toml,
    /// YAML format output  
    #[clap(alias = "y")]
    #[default]
    Yaml,
}

/// Parse percentage values for CLI arguments
fn parse_percentage(s: &str) -> std::result::Result<u8, String> {
    let trimmed = s.trim_end_matches('%');
    trimmed
        .parse::<u8>()
        .map_err(|_| format!("Invalid percentage value: {s}"))
}

/// Main CLI structure
#[derive(Parser)]
#[command(name = APP_NAME)]
#[command(about = APP_DESCRIPTION)]
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
    /// Analyze and convert a color between different color spaces
    Color(ColorArgs),
}

/// Arguments for gradient generation
#[derive(Args, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct GradientArgs {
    /// Starting color (HEX, RGB, HSL, or named color, e.g., #FF0000, rgb(255,0,0), red)
    #[arg(value_name = "START_COLOR")]
    pub start_color: String,

    /// Ending color (HEX, RGB, HSL, or named color, e.g., #0000FF, rgb(0,0,255), blue)
    #[arg(value_name = "END_COLOR")]
    pub end_color: String,

    /// Starting position as percentage (e.g., 20 or 20%, default: 0%)
    #[arg(short = 's', long, value_name = "PERCENT", value_parser = parse_percentage, default_value = DEFAULT_START_POSITION)]
    pub start_position: u8,

    /// Ending position as percentage (e.g., 80 or 80%, default: 100%)
    #[arg(short = 'e', long, value_name = "PERCENT", value_parser = parse_percentage, default_value = DEFAULT_END_POSITION)]
    pub end_position: u8,

    /// Ease-in control point for cubic-bezier (0.0-1.0, default: 0.65)
    #[arg(long, default_value = DEFAULT_EASE_IN)]
    pub ease_in: f64,

    /// Ease-out control point for cubic-bezier (0.0-1.0, default: 0.35)
    #[arg(long, default_value = DEFAULT_EASE_OUT)]
    pub ease_out: f64,

    /// Generate SVG image of the gradient with specified filename
    #[arg(long, value_name = "FILENAME")]
    pub svg: Option<String>,

    /// Generate PNG image of the gradient with specified filename
    #[arg(long, value_name = "FILENAME")]
    pub png: Option<String>,

    /// Disable legend/caption on gradient images (only valid with --svg or --png)
    #[arg(long)]
    pub no_legend: bool,

    /// Width of the image in pixels (default: 1000)
    #[arg(long, default_value = DEFAULT_WIDTH)]
    pub width: u32,

    /// Output gradient values every X percent
    #[arg(short = 't', long = "step", conflicts_with_all = ["stops"], help = "Output gradient values every X percent")]
    pub step: Option<u8>,

    /// Number of gradient stops to output (default: 5)
    #[arg(short = 'g', long = "stops", default_value = "5", conflicts_with_all = ["step"], help = "Number of gradient stops using curve derivatives (default: 5)")]
    pub stops: usize,

    /// Use equally spaced gradient stops instead of intelligent placement
    #[arg(
        long = "stops-simple",
        requires = "stops",
        help = "Use equally spaced gradient stops instead of intelligent placement"
    )]
    pub stops_simple: bool,

    /// Output format for file export (toml/t or yaml/y, default: yaml)
    #[arg(
        short = 'o',
        long = "output",
        value_enum,
        help = "Output format: toml (t) or yaml (y), default: yaml"
    )]
    pub output_format: Option<OutputFormat>,

    /// Output filename (extension will be added based on format)
    #[arg(
        short = 'f',
        long = "file",
        value_name = "FILENAME",
        help = "Output filename (extension added automatically based on format)"
    )]
    pub output_file: Option<String>,

    /// Filter functionality blocks and fields to display (default: all)
    #[arg(
        long = "func",
        value_name = "FILTERS",
        help = "Filter blocks/fields: [all], [input], [conversion], [contrast], [grayscale], [color_collections], [color_schemes], [block.field], [!exclude]. Simple format: hex,rgb,hsl,lab. Examples: [input,conversion], hex,rgb, [contrast.wcag21_relative_luminance], [all,!color_collections.css_colors]"
    )]
    pub func_filter: Option<String>,
}

impl GradientArgs {
    /// Validate the gradient arguments
    ///
    /// # Errors
    ///
    /// Returns `ColorError::InvalidArguments` if:
    /// - Start or end positions are outside 0-100 range
    /// - Start position is greater than or equal to end position
    /// - Ease-in or ease-out values are outside 0.0-1.0 range
    /// - Width or steps values are zero or negative
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

        // Validate --no-legend usage (check both explicit flags and implied flags)
        if self.no_legend && !self.should_generate_svg() && !self.should_generate_png() {
            return Err(ColorError::InvalidArguments(
                "--no-legend can only be used with --svg, --png, --svg-name, or --png-name"
                    .to_string(),
            ));
        }

        // Validate width
        if self.width == 0 {
            return Err(ColorError::InvalidArguments(
                "Width must be greater than 0".to_string(),
            ));
        }

        // Validate step (if provided)
        if let Some(step) = self.step {
            if step == 0 {
                return Err(ColorError::InvalidArguments(
                    "Gradient step must be greater than 0".to_string(),
                ));
            }
        }

        // Validate stops
        if self.stops == 0 {
            return Err(ColorError::InvalidArguments(
                "Number of gradient stops must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Check if SVG generation should be enabled
    #[must_use]
    pub fn should_generate_svg(&self) -> bool {
        self.svg.is_some()
    }

    /// Check if PNG generation should be enabled
    #[must_use]
    pub fn should_generate_png(&self) -> bool {
        self.png.is_some()
    }

    /// Get SVG filename
    #[must_use]
    pub fn svg_name(&self) -> String {
        self.svg.clone().unwrap_or_else(|| "gradient.svg".to_string())
    }

    /// Get PNG filename
    #[must_use]
    pub fn png_name(&self) -> String {
        self.png.clone().unwrap_or_else(|| "gradient.png".to_string())
    }
}

/// Arguments for color analysis and conversion
#[derive(Args, Clone)]
pub struct ColorArgs {
    /// Input color value (any format: hex, `rgb()`, `rgba()`, `hsl()`, `hsla()`, or color name)
    #[arg(value_name = "COLOR")]
    pub color: String,

    /// Distance calculation method for color matching
    #[arg(
        long,
        value_name = "METHOD",
        default_value = "delta-e-2000",
        help = "Distance calculation method: delta-e-76, delta-e-2000, euclidean-lab, lch"
    )]
    pub distance_method: String,

    /// Color scheme strategy to use
    #[arg(
        long = "schemes",
        value_name = "STRATEGY",
        default_value = "lab",
        help = "Color scheme strategy: hsl or lab (default: lab)"
    )]
    pub scheme_strategy: String,

    /// Replace input color with same hue but specified WCAG relative luminance
    /// If used without value, color schemes will use luminance-matched variations
    #[arg(
        short = 'r',
        long,
        value_name = "LUM_VALUE",
        help = "Replace color with specified WCAG relative luminance (0.0-1.0)"
    )]
    pub relative_luminance: Option<f64>,

    /// Replace input color with same hue but specified Lab luminance
    /// If used without value, color schemes will use luminance-matched variations
    #[arg(
        short = 'l',
        long,
        value_name = "LUM_VALUE",
        help = "Replace color with specified Lab luminance value"
    )]
    pub luminance: Option<f64>,

    /// Output format for file export (toml/t or yaml/y, default: yaml)
    #[arg(
        short = 'o',
        long = "output",
        value_enum,
        help = "Output format: toml (t) or yaml (y), default: yaml"
    )]
    pub output_format: Option<OutputFormat>,

    /// Output filename (extension will be added based on format)
    #[arg(
        short = 'f',
        long = "file",
        value_name = "FILENAME",
        help = "Output filename (extension added automatically based on format)"
    )]
    pub output_file: Option<String>,

    /// Filter functionality blocks and fields to display (default: all)
    #[arg(
        long = "func",
        value_name = "FILTERS",
        help = "Filter blocks/fields: [all], [input], [conversion], [contrast], [grayscale], [color_collections], [color_schemes], [block.field], [!exclude]. Examples: [input,conversion], [contrast.wcag21_relative_luminance], [all,!color_collections.css_colors]"
    )]
    pub func_filter: Option<String>,
}

impl ColorArgs {
    /// Validate the color arguments
    ///
    /// # Errors
    ///
    /// Returns `ColorError::InvalidArguments` if:
    /// - Scheme strategy is not 'hsl' or 'lab'
    /// - Relative luminance is outside 0.0-100.0 range
    /// - Limit per collection is zero or negative
    pub fn validate(&self) -> Result<()> {
        // Validate scheme strategy
        if !matches!(self.scheme_strategy.as_str(), "hsl" | "lab") {
            return Err(ColorError::InvalidArguments(
                "Scheme strategy must be either 'hsl' or 'lab'".to_string(),
            ));
        }

        // Validate relative luminance range
        if let Some(relative_lum) = self.relative_luminance {
            if !(0.0..=1.0).contains(&relative_lum) {
                return Err(ColorError::InvalidArguments(
                    "Relative luminance must be between 0.0 and 1.0".to_string(),
                ));
            }
        }

        // Validate Lab luminance range (typical range is 0-100, but can extend beyond)
        if let Some(lab_lum) = self.luminance {
            if !(0.0..=100.0).contains(&lab_lum) {
                return Err(ColorError::InvalidArguments(
                    "Lab luminance should typically be between 0.0 and 100.0".to_string(),
                ));
            }
        }

        // Ensure both luminance arguments are not provided simultaneously
        if self.relative_luminance.is_some() && self.luminance.is_some() {
            return Err(ColorError::InvalidArguments(
                "Cannot specify both --relative-luminance and --luminance simultaneously"
                    .to_string(),
            ));
        }

        Ok(())
    }
}
