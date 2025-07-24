//! Image generation (SVG and PNG) for color-rs

use image::{ImageBuffer, Rgba, RgbaImage};
use palette::Lab;
use resvg;
use std::fs;
use tiny_skia::{Pixmap, Transform};
use usvg::{Options, Tree, fontdb};

use crate::cli::GradientArgs;
use crate::error::{ColorError, Result};
use crate::gradient::GradientCalculator;
use crate::{color_utils::LegacyColorUtils as ColorUtils, config::{HEIGHT_RATIO, DEFAULT_LEGEND_HEIGHT_RATIO, DEFAULT_FONT_SIZE_RATIO, DEFAULT_TEXT_Y_RATIO, FONT_FAMILY}};

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
    #[must_use] pub const fn new() -> Self {
        Self
    }

    /// Generate SVG gradient
    pub fn generate_svg(&self, args: &GradientArgs, start_lab: Lab, end_lab: Lab) -> Result<()> {
        let svg_content = self.create_svg_content(args, start_lab, end_lab)?;
        fs::write(&args.svg_name, svg_content)?;
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
        let gradient_height = (f64::from(width) * HEIGHT_RATIO) as u32;
        let legend_height = if args.no_legend {
            0
        } else {
            (f64::from(gradient_height) * DEFAULT_LEGEND_HEIGHT_RATIO).max(20.0) as u32
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
        img.save(&args.png_name)
            .map_err(|e| ColorError::ImageError(format!("Failed to save PNG: {e}")))?;

        Ok(())
    }

    /// Create SVG content string
    fn create_svg_content(
        &self,
        args: &GradientArgs,
        start_lab: Lab,
        end_lab: Lab,
    ) -> Result<String> {
        let width = args.width;
        let gradient_height = (f64::from(width) * HEIGHT_RATIO) as u32;
        let legend_height = if args.no_legend {
            0
        } else {
            (f64::from(gradient_height) * DEFAULT_LEGEND_HEIGHT_RATIO).max(20.0) as u32
        };
        let total_height = gradient_height + legend_height;

        let start_hex = ColorUtils::lab_to_hex(start_lab);
        let end_hex = ColorUtils::lab_to_hex(end_lab);

        // Calculate positions as pixels
        let start_pixel = (f64::from(args.start_position) / 100.0 * f64::from(width)) as u32;
        let end_pixel = (f64::from(args.end_position) / 100.0 * f64::from(width)) as u32;

        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<svg width="{width}" height="{total_height}" xmlns="http://www.w3.org/2000/svg">"#
        ));
        svg.push('\n');

        // Add gradient definition with properly calculated stops
        svg.push_str("  <defs>\n");
        svg.push_str(
            "    <linearGradient id=\"grad\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"0%\">\n",
        );

        // Calculate dynamic number of stops based on gradient span to prevent banding
        let gradient_span = args.end_position - args.start_position;
        let base_stops = (gradient_span as usize).saturating_mul(2);
        let num_stops = base_stops.clamp(10, 1000);

        for i in 0..=num_stops {
            let t = i as f64 / num_stops as f64;
            let bezier_t = GradientCalculator::cubic_bezier_ease(t, args.ease_in, args.ease_out);
            let interpolated_lab = ColorUtils::interpolate_lab(start_lab, end_lab, bezier_t);
            let hex_color = ColorUtils::lab_to_hex(interpolated_lab);
            let offset = t * 100.0;

            svg.push_str(&format!(
                "      <stop offset=\"{offset}%\" stop-color=\"{hex_color}\" />\n"
            ));
        }

        svg.push_str("    </linearGradient>\n");
        svg.push_str("  </defs>\n");

        // Left fill (0% to start_position) with start color
        if start_pixel > 0 {
            svg.push_str(&format!(
                "  <rect x=\"0\" y=\"0\" width=\"{start_pixel}\" height=\"{gradient_height}\" fill=\"{start_hex}\" />\n"
            ));
        }

        // Gradient section (start_position to end_position)
        if end_pixel > start_pixel {
            svg.push_str(&format!(
                "  <rect x=\"{}\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"url(#grad)\" />\n",
                start_pixel,
                end_pixel - start_pixel,
                gradient_height
            ));
        }

        // Right fill (end_position to 100%) with end color
        if end_pixel < width {
            svg.push_str(&format!(
                "  <rect x=\"{}\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n",
                end_pixel,
                width - end_pixel,
                gradient_height,
                end_hex
            ));
        }

        // Add legend if not disabled
        if !args.no_legend {
            let font_size = (f64::from(legend_height) * DEFAULT_FONT_SIZE_RATIO).max(10.0) as u32;
            let text_y = gradient_height + (f64::from(legend_height) * DEFAULT_TEXT_Y_RATIO) as u32;

            svg.push_str(&format!(
                "  <rect x=\"0\" y=\"{gradient_height}\" width=\"100%\" height=\"{legend_height}\" fill=\"rgb(0,0,0)\" />\n"
            ));
            svg.push_str(&format!(
                "  <text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" fill=\"white\">\n",
                width / 100,
                text_y,
                FONT_FAMILY,
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
        if args.should_generate_svg() && !args.svg_name.ends_with(".svg") {
            return Err(ColorError::InvalidArguments(
                "SVG filename must end with .svg extension".to_string(),
            ));
        }

        if args.should_generate_png() && !args.png_name.ends_with(".png") {
            return Err(ColorError::InvalidArguments(
                "PNG filename must end with .png extension".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for ImageGenerator {
    fn default() -> Self {
        Self::new()
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
            svg: false,
            png: false,
            no_legend: false,
            width: 1000,
            svg_name: "test.svg".to_string(),
            png_name: "test.png".to_string(),
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
        let start_lab = ColorUtils::parse_hex_color(&args.start_color).unwrap();
        let end_lab = ColorUtils::parse_hex_color(&args.end_color).unwrap();

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
