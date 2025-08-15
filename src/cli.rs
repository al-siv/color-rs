//! Command-line interface for color-rs

use crate::config::{
    APP_AUTHOR, APP_DESCRIPTION, APP_NAME, APP_VERSION, BEZIER_MAX, BEZIER_MIN,
    DEFAULT_BORDER_COLOR, DEFAULT_BORDER_WIDTH, DEFAULT_EASE_IN, DEFAULT_EASE_OUT,
    DEFAULT_END_POSITION, DEFAULT_FONT_SIZE, DEFAULT_START_POSITION, DEFAULT_WIDTH, MAX_PERCENTAGE,
};
use crate::error::{ColorError, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::default::Default;

/// Output format for file export
#[derive(Debug, Clone, ValueEnum, Default, PartialEq, Eq)]
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
    /// Global log level (trace, debug, info, warn, error, none)
    #[arg(long = "log-level", value_enum, global = true, value_name = "LEVEL")]
    pub log_level: Option<LogLevelCli>,
    #[command(subcommand)]
    pub command: Commands,
}

/// CLI-facing log level (includes 'none') mapped to internal logger capability
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum LogLevelCli {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    None,
}

/// Available commands
#[derive(Subcommand)]
pub enum Commands {
    /// Generate a gradient between two colors using LAB color space with cubic-bezier timing
    Gradient(GradientArgs),
    /// Analyze and convert a color between different color spaces
    Color(ColorArgs),
    /// Analyze hue relationships and color harmony patterns
    Hue(HueArgs),
}

/// Arguments for gradient generation
#[derive(Args, Clone, Debug)]
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
    #[arg(short = 'S', long, value_name = "FILENAME")]
    pub svg: Option<String>,

    /// Generate PNG image of the gradient with specified filename
    #[arg(short = 'P', long, value_name = "FILENAME")]
    pub png: Option<String>,

    /// Convert text elements to vector paths in SVG output (default: enabled)
    #[arg(
        long,
        default_value_t = true,
        action = clap::ArgAction::Set,
        help = "Convert text to vector paths for better design tool compatibility (default: enabled)"
    )]
    pub vectorized_text: bool,

    /// Disable legend/caption on gradient images (only valid with --svg or --png)
    #[arg(long)]
    pub no_legend: bool,

    /// Width of the image in pixels (default: 1000)
    #[arg(short = 'w', long, default_value = DEFAULT_WIDTH)]
    pub width: u32,

    /// Output gradient values every X percent
    #[arg(short = 't', long = "step", conflicts_with_all = ["stops"], help = "Output gradient values every X percent")]
    pub step: Option<u8>,

    /// Number of gradient stops to output (default: 5)
    #[arg(short = 'T', long = "stops", default_value = "5", conflicts_with_all = ["step"], help = "Number of gradient stops using curve derivatives (default: 5)")]
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
    pub const fn should_generate_svg(&self) -> bool {
        self.svg.is_some()
    }

    /// Check if PNG generation should be enabled
    #[must_use]
    pub const fn should_generate_png(&self) -> bool {
        self.png.is_some()
    }

    /// Get SVG filename
    #[must_use]
    pub fn svg_name(&self) -> String {
        self.svg
            .clone()
            .unwrap_or_else(|| "gradient.svg".to_string())
    }

    /// Get PNG filename
    #[must_use]
    pub fn png_name(&self) -> String {
        self.png
            .clone()
            .unwrap_or_else(|| "gradient.png".to_string())
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
        default_value = "lch",
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

/// Arguments for hue mode - display entire color collections sorted by hue
#[derive(Debug, Clone, Args)]
#[allow(clippy::struct_excessive_bools)]
pub struct HueArgs {
    /// Color collection to display (css, ralc, rald)
    #[arg(value_name = "COLLECTION")]
    pub collection: String,

    /// Hue range filter [min...max] (degrees, can be negative for wraparound)
    #[arg(
        short = 'H',
        long = "h-range",
        value_name = "[MIN...MAX]",
        help = "Filter by hue range [min...max] degrees, e.g., [300...360] or [-25...25]"
    )]
    pub hue_range: Option<String>,

    /// Lightness range filter [min...max] (0-100%)
    #[arg(
        short = 'L',
        long = "l-range",
        value_name = "[MIN...MAX]",
        help = "Filter by lightness range [min...max] percent, e.g., [50...80]"
    )]
    pub lightness_range: Option<String>,

    /// Chroma range filter [min...max]
    #[arg(
        short = 'C',
        long = "c-range",
        value_name = "[MIN...MAX]",
        help = "Filter by chroma range [min...max], e.g., [30...70]"
    )]
    pub chroma_range: Option<String>,

    /// Generate horizontal gradient layout
    #[arg(
        short = 'g',
        long,
        conflicts_with = "pal",
        help = "Generate horizontal gradient layout"
    )]
    pub grad: bool,

    /// Generate vertical palette layout  
    #[arg(
        short = 'p',
        long,
        conflicts_with = "grad",
        help = "Generate vertical palette layout"
    )]
    pub pal: bool,

    /// SVG output filename (requires --grad or --pal)
    #[arg(
        short = 'G',
        long,
        value_name = "FILENAME",
        help = "SVG output filename"
    )]
    pub svg: Option<String>,

    /// Generate PNG version of visual output (requires --grad or --pal and --svg)
    #[arg(
        short = 'P',
        long,
        value_name = "FILENAME",
        help = "Generate PNG version"
    )]
    pub png: Option<String>,

    /// Convert text elements to vector paths in SVG output (default: enabled)
    #[arg(
        long,
        default_value_t = true,
        action = clap::ArgAction::Set,
        help = "Convert text to vector paths for better design tool compatibility (default: enabled)"
    )]
    pub vectorized_text: bool,

    /// Width of visual output in pixels (default: 1000)
    #[arg(short = 'w', long, default_value = DEFAULT_WIDTH, help = "Visual output width")]
    pub width: u32,

    /// Disable labels on visual output
    #[arg(long, help = "Disable color labels on visual output")]
    pub no_labels: bool,

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

    /// Height of each color block in palette layout (requires --pal)
    #[arg(
        short = 'z',
        long = "color-height",
        value_name = "PIXELS",
        help = "Height of each color block in pixels for palette layout"
    )]
    pub color_height: Option<u32>,

    /// Font size for palette text in points
    #[arg(
        short = 's',
        long = "font-size",
        value_name = "SIZE",
        default_value = DEFAULT_FONT_SIZE,
        help = "Font size for palette text in points"
    )]
    pub font_size: u32,

    /// Border width for palette elements in pixels
    #[arg(
        short = 'b',
        long = "border-width",
        value_name = "PIXELS",
        default_value = DEFAULT_BORDER_WIDTH,
        help = "Border width for palette elements in pixels"
    )]
    pub border_width: u32,

    /// Border color for palette elements
    #[arg(
        long = "border-color",
        value_name = "COLOR",
        default_value = DEFAULT_BORDER_COLOR,
        help = "Border color for palette elements (color name or hex)"
    )]
    pub border_color: String,

    /// Custom header text for palette layout (requires --pal)
    #[arg(
        long = "header-text",
        value_name = "TEXT",
        help = "Custom header text for palette layout (replaces default collection title)"
    )]
    pub header_text: Option<String>,
}
// Range utilities moved to `cli_range.rs` (Milestone 4 Phase 4.1 module size reduction)
pub use crate::cli_range::Range; // Re-exported for external users/tests

impl HueArgs {
    /// Validate the hue arguments
    ///
    /// # Errors
    /// Returns error if visual output parameters are inconsistent or invalid
    pub fn validate(&self) -> Result<()> {
        validate_collection(&self.collection)?;
        validate_hue_range(&self.hue_range)?;
        validate_lightness_range(&self.lightness_range)?;
        validate_chroma_range(&self.chroma_range)?;
        validate_visual_output_params(self)?;
        validate_output_dependency_params(self)?;
        validate_palette_specific_params(self)?;
        validate_text_and_border_params(self)?;
        Ok(())
    }

    /// Parse hue range if provided
    ///
    /// # Errors
    /// Returns error if range parsing fails
    pub fn get_hue_range(&self) -> Result<Option<Range>> {
        if let Some(ref range_str) = self.hue_range {
            Ok(Some(Range::parse(range_str)?))
        } else {
            Ok(None)
        }
    }

    /// Parse lightness range if provided
    ///
    /// # Errors
    /// Returns error if range parsing fails
    pub fn get_lightness_range(&self) -> Result<Option<Range>> {
        if let Some(ref range_str) = self.lightness_range {
            Ok(Some(Range::parse(range_str)?))
        } else {
            Ok(None)
        }
    }

    /// Parse chroma range if provided
    ///
    /// # Errors
    /// Returns error if range parsing fails
    pub fn get_chroma_range(&self) -> Result<Option<Range>> {
        if let Some(ref range_str) = self.chroma_range {
            Ok(Some(Range::parse(range_str)?))
        } else {
            Ok(None)
        }
    }

    /// Check if horizontal gradient generation should be enabled
    #[must_use]
    pub const fn should_generate_gradient(&self) -> bool {
        self.grad
    }

    /// Check if vertical palette generation should be enabled
    #[must_use]
    pub const fn should_generate_palette(&self) -> bool {
        self.pal
    }

    /// Check if any visual output should be generated
    #[must_use]
    pub const fn should_generate_visual(&self) -> bool {
        self.should_generate_gradient() || self.should_generate_palette()
    }

    /// Check if SVG generation should be enabled
    #[must_use]
    pub const fn should_generate_svg(&self) -> bool {
        self.svg.is_some() && self.should_generate_visual()
    }

    /// Check if PNG generation should be enabled
    #[must_use]
    pub const fn should_generate_png(&self) -> bool {
        self.png.is_some() && self.should_generate_visual()
    }

    /// Get SVG filename
    #[must_use]
    pub fn svg_name(&self) -> String {
        self.svg.clone().unwrap_or_else(|| {
            if self.should_generate_gradient() {
                "hue_gradient.svg".to_string()
            } else {
                "hue_palette.svg".to_string()
            }
        })
    }

    /// Get PNG filename
    #[must_use]
    pub fn png_name(&self) -> String {
        self.png.clone().unwrap_or_else(|| {
            if self.should_generate_gradient() {
                "hue_gradient.png".to_string()
            } else {
                "hue_palette.png".to_string()
            }
        })
    }
}

// ---- HueArgs validation helpers (extracted to reduce HueArgs::validate size) ----

fn validate_collection(collection: &str) -> Result<()> {
    match collection {
        "css" | "ralc" | "rald" => Ok(()),
        other => Err(ColorError::InvalidArguments(format!(
            "Invalid collection '{other}'. Must be: css, ralc, or rald"
        ))),
    }
}

fn validate_hue_range(hue_range: &Option<String>) -> Result<()> {
    if let Some(range_str) = hue_range {
        let range = Range::parse(range_str)?; // reuse existing parser (may wrap values)
        if range.min < -360.0 || range.max > 720.0 {
            return Err(ColorError::InvalidArguments(
                "Hue range values should be between -360 and 720 degrees".to_string(),
            ));
        }
    }
    Ok(())
}

fn validate_lightness_range(lightness_range: &Option<String>) -> Result<()> {
    if let Some(range_str) = lightness_range {
        let range = Range::parse(range_str)?;
        if range.min < 0.0 || range.max > 100.0 || range.min > range.max {
            return Err(ColorError::InvalidArguments(
                "Lightness range must be 0-100% with min <= max".to_string(),
            ));
        }
    }
    Ok(())
}

fn validate_chroma_range(chroma_range: &Option<String>) -> Result<()> {
    if let Some(range_str) = chroma_range {
        let range = Range::parse(range_str)?;
        if range.min < 0.0 || range.max > 200.0 || range.min > range.max {
            return Err(ColorError::InvalidArguments(
                "Chroma range must be 0-200 with min <= max".to_string(),
            ));
        }
    }
    Ok(())
}

fn validate_visual_output_params(args: &HueArgs) -> Result<()> {
    if args.should_generate_visual() {
        if args.svg.is_none() {
            return Err(ColorError::InvalidArguments(
                "SVG filename (--svg) is required when using --grad or --pal".to_string(),
            ));
        }
        if args.width == 0 {
            return Err(ColorError::InvalidArguments(
                "Width must be greater than 0".to_string(),
            ));
        }
        if args.width > 10000 {
            return Err(ColorError::InvalidArguments(
                "Width should not exceed 10000 pixels for performance reasons".to_string(),
            ));
        }
        if args.should_generate_svg()
            && !std::path::Path::new(&args.svg_name())
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("svg"))
        {
            return Err(ColorError::InvalidArguments(
                "SVG filename must end with .svg extension".to_string(),
            ));
        }
        if args.should_generate_png()
            && !std::path::Path::new(&args.png_name())
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("png"))
        {
            return Err(ColorError::InvalidArguments(
                "PNG filename must end with .png extension".to_string(),
            ));
        }
    }
    Ok(())
}

fn validate_output_dependency_params(args: &HueArgs) -> Result<()> {
    if args.png.is_some() && !args.should_generate_visual() {
        return Err(ColorError::InvalidArguments(
            "PNG output (--png) requires --grad or --pal".to_string(),
        ));
    }
    if args.svg.is_some() && !args.should_generate_visual() {
        return Err(ColorError::InvalidArguments(
            "SVG output (--svg) requires --grad or --pal".to_string(),
        ));
    }
    if args.no_labels && !args.should_generate_visual() {
        return Err(ColorError::InvalidArguments(
            "--no-labels can only be used with --grad or --pal".to_string(),
        ));
    }
    Ok(())
}

fn validate_palette_specific_params(args: &HueArgs) -> Result<()> {
    if let Some(color_height) = args.color_height {
        if !args.should_generate_palette() {
            return Err(ColorError::InvalidArguments(
                "--color-height can only be used with --pal".to_string(),
            ));
        }
        if color_height == 0 {
            return Err(ColorError::InvalidArguments(
                "Color height must be greater than 0".to_string(),
            ));
        }
        if color_height > 500 {
            return Err(ColorError::InvalidArguments(
                "Color height should not exceed 500 pixels for reasonable layout".to_string(),
            ));
        }
    }
    Ok(())
}

fn validate_text_and_border_params(args: &HueArgs) -> Result<()> {
    if args.font_size == 0 {
        return Err(ColorError::InvalidArguments(
            "Font size must be greater than 0".to_string(),
        ));
    }
    if args.font_size > 72 {
        return Err(ColorError::InvalidArguments(
            "Font size should not exceed 72 points for reasonable layout".to_string(),
        ));
    }
    if args.border_width > 0 && !args.should_generate_palette() {
        return Err(ColorError::InvalidArguments(
            "--border-width can only be used with --pal (gradients don't have borders)"
                .to_string(),
        ));
    }
    if args.border_color != "white" && !args.should_generate_palette() {
        return Err(ColorError::InvalidArguments(
            "--border-color can only be used with --pal (gradients don't have borders)"
                .to_string(),
        ));
    }
    if args.border_width > 50 {
        return Err(ColorError::InvalidArguments(
            "Border width should not exceed 50 pixels for reasonable layout".to_string(),
        ));
    }
    Ok(())
}
