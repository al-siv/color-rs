//! Image generation (SVG and PNG) for color-rs

use image::{ImageBuffer, Rgba, RgbaImage};
use palette::{IntoColor, Lab, Lch, Srgb};
use resvg;
use std::fs;
use tiny_skia::{Pixmap, Transform};
use usvg::{Options, Tree, fontdb};

use crate::cli::{GradientArgs, HueArgs};
use crate::color_ops::analysis::hue::HueAnalysisResult;
use crate::config::{algorithm_constants, display_constants, math_constants};
use crate::error::{ColorError, Result};
use crate::gradient::GradientCalculator;

/// Convert a color component from 0.0-1.0 range to 0-255 u8
///
/// # Arguments
/// * `component` - Color component value in 0.0-1.0 range
///
/// # Returns
/// u8 value in 0-255 range
#[must_use]
fn component_to_u8(component: f32) -> u8 {
    // Since RGB_MAX_VALUE is 255.0 and we clamp to this range,
    // the cast to u8 is safe and will never wrap
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    {
        (component * math_constants::RGB_MAX_VALUE)
            .round()
            .clamp(0.0, math_constants::RGB_MAX_VALUE) as u8
    }
}

/// Convert LAB color to hex string for image generation
/// RGB values are clamped to [0,255] range before casting to u8 for safety
#[must_use]
pub fn lab_to_hex(lab: Lab) -> String {
    let srgb: Srgb = lab.into_color();
    format!(
        "#{:02X}{:02X}{:02X}",
        component_to_u8(srgb.red),
        component_to_u8(srgb.green),
        component_to_u8(srgb.blue),
    )
}

/// Convert LCH color to hex string for image generation
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn lch_to_hex(lch: Lch) -> String {
    let lab: Lab = lch.into_color();
    lab_to_hex(lab)
}

/// Supported image formats
#[derive(Debug, Clone, Copy)]
pub enum ImageFormat {
    Svg,
    Png,
}

/// Image generation and processing
pub struct ImageGenerator;

impl ImageGenerator {
    /// Create a new image generator
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Generate SVG gradient
    pub fn generate_svg(&self, args: &GradientArgs, start_lab: Lab, end_lab: Lab) -> Result<()> {
        let svg_content = self.create_svg_content(args, start_lab, end_lab)?;
        fs::write(args.svg_name(), svg_content)?;
        Ok(())
    }

    /// Generate PNG gradient
    pub fn generate_png(&self, args: &GradientArgs, start_lab: Lab, end_lab: Lab) -> Result<()> {
        // Create SVG content first
        let svg_content = self.create_svg_content(args, start_lab, end_lab)?;

        // Configure usvg options with font database for text-to-paths conversion
        let mut options = Options::default();
        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();
        options.fontdb = std::sync::Arc::new(fontdb);

        // Parse SVG with font resolution
        let tree = Tree::from_str(&svg_content, &options)
            .map_err(|e| ColorError::SvgError(format!("Failed to parse SVG: {e}")))?;

        let width = args.width;
        let gradient_height = (f64::from(width) * display_constants::HEIGHT_RATIO) as u32;
        let legend_height = if args.no_legend {
            0
        } else {
            (f64::from(gradient_height) * display_constants::DEFAULT_LEGEND_HEIGHT_RATIO)
                .max(display_constants::MIN_LEGEND_HEIGHT) as u32
        };
        let total_height = gradient_height + legend_height;

        // Create pixmap
        let mut pixmap = Pixmap::new(width, total_height)
            .ok_or_else(|| ColorError::ImageError("Failed to create pixmap".to_string()))?;

        // Render SVG to pixmap (this converts text to paths automatically)
        resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());

        // Convert to image crate format
        let img: RgbaImage = ImageBuffer::from_fn(width, total_height, |x, y| {
            let pixel = pixmap.pixel(x, y).unwrap();
            Rgba([pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()])
        });

        // Save PNG
        img.save(&args.png_name())
            .map_err(|e| ColorError::ImageError(format!("Failed to save PNG: {e}")))?;

        Ok(())
    }

    /// Create SVG content string
    ///
    /// # Errors
    /// This function currently cannot fail but returns Result for future extensibility
    /// when error conditions may be added (e.g., invalid color spaces, malformed arguments).
    fn create_svg_content(
        &self,
        args: &GradientArgs,
        start_lab: Lab,
        end_lab: Lab,
    ) -> Result<String> {
        let width = args.width;
        let gradient_height = (f64::from(width) * display_constants::HEIGHT_RATIO) as u32;
        let legend_height = if args.no_legend {
            0
        } else {
            (f64::from(gradient_height) * display_constants::DEFAULT_LEGEND_HEIGHT_RATIO)
                .max(display_constants::MIN_LEGEND_HEIGHT) as u32
        };
        let total_height = gradient_height + legend_height;

        let start_hex = lab_to_hex(start_lab);
        let end_hex = lab_to_hex(end_lab);

        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<svg width="{width}" height="{total_height}" xmlns="http://www.w3.org/2000/svg">"#
        ));
        svg.push('\n');

        // Add gradient definition that maps start_position to end_position
        svg.push_str("  <defs>\n");
        svg.push_str(&format!(
            "    <linearGradient id=\"grad\" x1=\"{}%\" y1=\"0%\" x2=\"{}%\" y2=\"0%\">\n",
            args.start_position, args.end_position
        ));

        // Use unified gradient calculation for consistent results with YAML output
        // Generate many stops (400) for smooth bezier rendering in SVG
        let svg_steps = 400; // High resolution for smooth gradients
        let unified_stops = GradientCalculator::calculate_unified_gradient(
            start_lab,
            end_lab,
            args.start_position,
            args.end_position,
            args.ease_in,
            args.ease_out,
            svg_steps,
            args.stops_simple, // Use same mode as YAML output
        );

        // Convert unified stops to SVG stops with proper offset mapping
        // Map stop positions from [start_position, end_position] to [0%, 100%]
        let position_range = args.end_position - args.start_position;
        let mut last_offset: Option<f64> = None;

        for stop in unified_stops {
            let hex_color = lab_to_hex(stop.lab_color);
            // Convert absolute position to relative position within the gradient with 0.5% precision
            let relative_offset_precise =
                (stop.position - args.start_position) as f64 / position_range as f64 * 100.0;
            let relative_offset =
                (relative_offset_precise * algorithm_constants::GRADIENT_OFFSET_PRECISION).round()
                    / algorithm_constants::GRADIENT_OFFSET_PRECISION; // Round to nearest 0.5%

            // Skip duplicates - only add if offset changed by at least 0.5%
            if let Some(last) = last_offset {
                if (relative_offset - last).abs() < 0.5 {
                    continue;
                }
            }

            last_offset = Some(relative_offset);

            // Format offset with proper precision (show .5 when needed, hide .0)
            let offset_str = if relative_offset.fract() == 0.0 {
                format!("{}%", relative_offset.round() as u8)
            } else {
                format!("{relative_offset:.1}%")
            };

            svg.push_str(&format!(
                "      <stop offset=\"{offset_str}\" stop-color=\"{hex_color}\" />\n"
            ));
        }

        svg.push_str("    </linearGradient>\n");
        svg.push_str("  </defs>\n");

        // Create full-width gradient rectangle
        svg.push_str(&format!(
            "  <rect x=\"0\" y=\"0\" width=\"{width}\" height=\"{gradient_height}\" fill=\"url(#grad)\" />\n"
        ));

        // Add legend if not disabled
        if !args.no_legend {
            let font_size = (f64::from(legend_height) * display_constants::DEFAULT_FONT_SIZE_RATIO)
                .max(display_constants::MIN_FONT_SIZE) as u32;
            let text_y = gradient_height
                + (f64::from(legend_height) * display_constants::DEFAULT_TEXT_Y_RATIO) as u32;

            svg.push_str(&format!(
                "  <rect x=\"0\" y=\"{gradient_height}\" width=\"100%\" height=\"{legend_height}\" fill=\"rgb(0,0,0)\" />\n"
            ));
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" fill=\"white\">\n",
                width / 100,
                text_y,
                display_constants::FONT_FAMILY,
                font_size
            ));
            svg.push_str(&format!(
                "    cubic-bezier({}, 0, {}, 1) | positions: {}%-{}% | colors: {}-{}\n",
                args.ease_in,
                args.ease_out,
                args.start_position,
                args.end_position,
                start_hex,
                end_hex
            ));
            svg.push_str("  </text>\n");
        }

        svg.push_str("</svg>");

        Ok(svg)
    }

    /// Validate image generation parameters
    /// Validate image generation parameters
    ///
    /// # Errors
    /// Returns error if filenames don't have proper extensions or if validation fails
    /// for any required parameter values.
    pub fn validate_image_params(&self, args: &GradientArgs) -> Result<()> {
        if args.width == 0 {
            return Err(ColorError::InvalidArguments(
                "Image width must be greater than 0".to_string(),
            ));
        }

        if args.width > 10000 {
            return Err(ColorError::InvalidArguments(
                "Image width should not exceed 10000 pixels for performance reasons".to_string(),
            ));
        }

        // Validate filename extensions
        if args.should_generate_svg() && !args.svg_name().ends_with(".svg") {
            return Err(ColorError::InvalidArguments(
                "SVG filename must end with .svg extension".to_string(),
            ));
        }

        if args.should_generate_png() && !args.png_name().ends_with(".png") {
            return Err(ColorError::InvalidArguments(
                "PNG filename must end with .png extension".to_string(),
            ));
        }

        Ok(())
    }

    /// Generate horizontal gradient SVG from hue analysis results
    pub fn generate_hue_gradient(
        &self,
        args: &HueArgs,
        colors: &[HueAnalysisResult],
    ) -> Result<()> {
        let svg_content = self.create_hue_gradient_svg(args, colors)?;
        fs::write(args.svg_name(), svg_content)?;

        // Generate PNG if requested
        if args.should_generate_png() {
            self.svg_to_png(&args.svg_name(), &args.png_name(), args.width)?;
        }

        Ok(())
    }

    /// Generate vertical palette SVG from hue analysis results  
    pub fn generate_hue_palette(&self, args: &HueArgs, colors: &[HueAnalysisResult]) -> Result<()> {
        let svg_content = self.create_hue_palette_svg(args, colors)?;
        fs::write(args.svg_name(), svg_content)?;

        // Generate PNG if requested
        if args.should_generate_png() {
            self.svg_to_png(&args.svg_name(), &args.png_name(), args.width)?;
        }

        Ok(())
    }

    /// Create horizontal gradient SVG from hue analysis results
    fn create_hue_gradient_svg(
        &self,
        args: &HueArgs,
        colors: &[HueAnalysisResult],
    ) -> Result<String> {
        if colors.is_empty() {
            return Err(ColorError::InvalidArguments(
                "Cannot create gradient from empty color collection".to_string(),
            ));
        }

        let width = args.width;
        let height = (f64::from(width) * display_constants::HEIGHT_RATIO) as u32;

        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<svg width="{width}" height="{height}" xmlns="http://www.w3.org/2000/svg">"#
        ));
        svg.push('\n');

        // Add white background
        svg.push_str(&format!(
            "  <rect x=\"0\" y=\"0\" width=\"{width}\" height=\"{height}\" fill=\"white\" />\n"
        ));

        // Add gradient definition
        svg.push_str("  <defs>\n");
        svg.push_str(
            "    <linearGradient id=\"huegrad\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"0%\">\n",
        );

        // Create gradient stops from colors with banding behavior (+1% offset for hard transitions)
        let step = 100.0 / (colors.len() - 1).max(1) as f64;
        for (i, color) in colors.iter().enumerate() {
            let hex_color = lch_to_hex(color.color);

            if i == 0 {
                // First color starts at 0%
                svg.push_str(&format!(
                    "      <stop offset=\"0%\" stop-color=\"{hex_color}\" />\n"
                ));
            } else if i == colors.len() - 1 {
                // Last color: create a hard edge 1% before the end, then extend to 100%
                let end_position = (i as f64 * step) - 0.5;
                let prev_hex = lch_to_hex(colors[i - 1].color);

                // End the previous color 1% before the transition
                svg.push_str(&format!(
                    "      <stop offset=\"{end_position:.1}%\" stop-color=\"{prev_hex}\" />\n"
                ));
                // Start the final color immediately after
                svg.push_str(&format!(
                    "      <stop offset=\"{:.1}%\" stop-color=\"{hex_color}\" />\n",
                    end_position + 0.1
                ));
                // Extend final color to 100%
                svg.push_str(&format!(
                    "      <stop offset=\"100%\" stop-color=\"{hex_color}\" />\n"
                ));
            } else {
                // Middle colors: create hard transitions with +1% offset behavior
                let start_position = (i as f64 * step) - 0.5;
                let end_position = (i as f64 * step) + 0.5;
                let prev_hex = lch_to_hex(colors[i - 1].color);

                // End previous color just before this one
                svg.push_str(&format!(
                    "      <stop offset=\"{start_position:.1}%\" stop-color=\"{prev_hex}\" />\n"
                ));
                // Start current color immediately after
                svg.push_str(&format!(
                    "      <stop offset=\"{:.1}%\" stop-color=\"{hex_color}\" />\n",
                    start_position + 0.1
                ));
                // Extend current color until next transition
                svg.push_str(&format!(
                    "      <stop offset=\"{end_position:.1}%\" stop-color=\"{hex_color}\" />\n"
                ));
            }
        }

        svg.push_str("    </linearGradient>\n");
        svg.push_str("  </defs>\n");

        // Create gradient rectangle
        svg.push_str(&format!(
            "  <rect x=\"0\" y=\"0\" width=\"{width}\" height=\"{height}\" fill=\"url(#huegrad)\" />\n"
        ));

        // Add title if labels are enabled
        if !args.no_labels {
            let font_size = 24;
            let title = format!(
                "{} Collection Hue Gradient ({} colors)",
                args.collection.to_uppercase(),
                colors.len()
            );
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" fill=\"white\" text-anchor=\"middle\" stroke=\"black\" stroke-width=\"1\">\n",
                width / 2,
                font_size + 10,
                display_constants::FONT_FAMILY,
                font_size
            ));
            svg.push_str(&format!("    {title}\n"));
            svg.push_str("  </text>\n");
        }

        svg.push_str("</svg>");
        Ok(svg)
    }

    /// Create vertical palette SVG from hue analysis results
    fn create_hue_palette_svg(
        &self,
        args: &HueArgs,
        colors: &[HueAnalysisResult],
    ) -> Result<String> {
        if colors.is_empty() {
            return Err(ColorError::InvalidArguments(
                "Cannot create palette from empty color collection".to_string(),
            ));
        }

        let width = args.width;
        let swatch_height = args.color_height.unwrap_or(80u32); // Use color_height parameter or default to 80
        let header_height = if args.no_labels { 0 } else { 60 }; // Increased header from 50 to 60
        let total_height = header_height + (colors.len() as u32 * swatch_height);

        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<svg width="{width}" height="{total_height}" xmlns="http://www.w3.org/2000/svg">"#
        ));
        svg.push('\n');

        // Add white background
        svg.push_str(&format!(
            "  <rect x=\"0\" y=\"0\" width=\"{width}\" height=\"{total_height}\" fill=\"white\" />\n"
        ));

        // Add header if labels are enabled
        let mut y_offset = 0;
        if !args.no_labels {
            let font_size = 28; // Increased from 24
            let title = format!(
                "{} Collection Color Palette ({} colors)",
                args.collection.to_uppercase(),
                colors.len()
            );

            // Calculate left padding as 1/2 of (color-height + border-width)
            let header_padding = (args.color_height.unwrap_or(50) + args.border_width) / 2;

            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" fill=\"black\" text-anchor=\"start\">\n",
                header_padding,
                font_size + 15, // Increased spacing
                display_constants::HEADER_FONT_FAMILY,
                font_size
            ));
            svg.push_str(&format!("    {title}\n"));
            svg.push_str("  </text>\n");
            y_offset = header_height;
        }

        // Create color swatches - full width blocks with text inside
        for (i, color) in colors.iter().enumerate() {
            let y = y_offset + (i as u32 * swatch_height);
            let hex_color = lch_to_hex(color.color);

            // Full-width color block with borders from args
            svg.push_str(&format!(
                "  <rect x=\"0\" y=\"{y}\" width=\"{width}\" height=\"{swatch_height}\" fill=\"{hex_color}\" stroke=\"{}\" stroke-width=\"{}\" />\n",
                args.border_color, args.border_width
            ));

            // Text inside the color block if labels are enabled
            if !args.no_labels {
                let font_size = args.font_size; // Use font size from args
                let text_y = y + swatch_height / 2 + font_size / 2;

                // Extract LCH components from the color
                let lch = color.color;
                let hue_str = format!("{:.0}", lch.hue.into_positive_degrees());
                let hex_str = hex_color.to_uppercase();
                let lch_str = format!(
                    "lch({:.1}, {:.1}, {:.1})",
                    lch.l,
                    lch.chroma,
                    lch.hue.into_positive_degrees()
                );
                let code_str = color.code.as_deref().unwrap_or("Unknown");
                let name_str = color.name.as_deref().unwrap_or("Unknown");

                // Calculate hue delta from previous color (for palette mode)
                let hue_delta_str = if i == 0 {
                    "—".to_string() // First color has no previous hue
                } else {
                    let prev_hue = colors[i - 1].color.hue.into_positive_degrees();
                    let current_hue = lch.hue.into_positive_degrees();
                    let delta = current_hue - prev_hue;
                    format!("{:+.2}", delta)
                };

                // Create LCH format: {H} | {HEX} | {lch(ll.l, cc.c, hhh.h)} | {code} | {color_name} | {hue_delta}
                let display_text = format!(
                    "{} | {} | {} | {} | {} | {}",
                    hue_str, hex_str, lch_str, code_str, name_str, hue_delta_str
                );

                // Calculate contrast color for text (white or black)
                let text_color = if is_dark_color(&hex_color) {
                    "white"
                } else {
                    "black"
                };

                // Calculate left padding as 1/2 of (color-height + border-width) minus half the text height
                let text_padding = (args.color_height.unwrap_or(50) + args.border_width) / 2 - font_size / 2;

                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"{text_y}\" font-family=\"{}\" font-size=\"{font_size}\" fill=\"{text_color}\" text-anchor=\"start\">\n",
                    text_padding,
                    display_constants::FONT_FAMILY
                ));
                svg.push_str(&format!("{display_text}\n"));
                svg.push_str("  </text>\n");
            }
        }

        svg.push_str("</svg>\n");
        Ok(svg)
    }

    /// Convert SVG file to PNG
    fn svg_to_png(&self, svg_path: &str, png_path: &str, _width: u32) -> Result<()> {
        // Read SVG content
        let svg_content = fs::read_to_string(svg_path)
            .map_err(|e| ColorError::SvgError(format!("Failed to read SVG file: {e}")))?;

        // Configure usvg options
        let mut options = Options::default();
        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();
        options.fontdb = std::sync::Arc::new(fontdb);

        // Parse SVG
        let tree = Tree::from_str(&svg_content, &options)
            .map_err(|e| ColorError::SvgError(format!("Failed to parse SVG: {e}")))?;

        // Get actual dimensions from the tree or use width with calculated height
        let svg_size = tree.size();
        let actual_width = svg_size.width() as u32;
        let actual_height = svg_size.height() as u32;

        // Create pixmap
        let mut pixmap = Pixmap::new(actual_width, actual_height)
            .ok_or_else(|| ColorError::ImageError("Failed to create pixmap".to_string()))?;

        // Render SVG to pixmap
        resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());

        // Convert to image crate format
        let img: RgbaImage = ImageBuffer::from_fn(actual_width, actual_height, |x, y| {
            let pixel = pixmap.pixel(x, y).unwrap();
            Rgba([pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()])
        });

        // Save PNG
        img.save(png_path)
            .map_err(|e| ColorError::ImageError(format!("Failed to save PNG: {e}")))?;

        Ok(())
    }
}

impl Default for ImageGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to determine if a color is dark (for text contrast)
fn is_dark_color(hex_color: &str) -> bool {
    // Remove # if present
    let hex = hex_color.trim_start_matches('#');

    if hex.len() != 6 {
        return false; // Default to dark if can't parse
    }

    // Parse RGB values
    if let (Ok(r), Ok(g), Ok(b)) = (
        u8::from_str_radix(&hex[0..2], 16),
        u8::from_str_radix(&hex[2..4], 16),
        u8::from_str_radix(&hex[4..6], 16),
    ) {
        // Calculate relative luminance using sRGB coefficients
        let luminance = 0.299 * f64::from(r) + 0.587 * f64::from(g) + 0.114 * f64::from(b);
        luminance < 128.0 // Dark if luminance < 50%
    } else {
        false // Default to light if parsing fails
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::GradientArgs;

    fn create_test_args() -> GradientArgs {
        GradientArgs {
            start_color: "FF0000".to_string(),
            end_color: "0000FF".to_string(),
            start_position: 0,
            end_position: 100,
            ease_in: 0.25,
            ease_out: 0.75,
            svg: Some("test.svg".to_string()),
            png: None,
            no_legend: false,
            width: 1000,
            step: None,
            stops: 5,
            stops_simple: false,
            output_format: None,
            output_file: None,
            func_filter: None,
        }
    }

    #[test]
    fn test_svg_content_creation() {
        let generator = ImageGenerator::new();
        let args = create_test_args();

        // Parse hex colors using color_parser modern approach
        use crate::color_parser::ColorParser;
        let parser = ColorParser::new();
        let (start_lab, _) = parser.parse(&args.start_color).unwrap();
        let (end_lab, _) = parser.parse(&args.end_color).unwrap();

        let svg_content = generator
            .create_svg_content(&args, start_lab, end_lab)
            .unwrap();
        assert!(svg_content.contains("<svg"));
        assert!(svg_content.contains("linearGradient"));
        assert!(svg_content.contains("</svg>"));
    }

    #[test]
    fn test_image_params_validation() {
        let generator = ImageGenerator::new();
        let mut args = create_test_args();

        // Valid case
        assert!(generator.validate_image_params(&args).is_ok());

        // Invalid width
        args.width = 0;
        assert!(generator.validate_image_params(&args).is_err());

        // Too large width
        args.width = 15000;
        assert!(generator.validate_image_params(&args).is_err());
    }
}
