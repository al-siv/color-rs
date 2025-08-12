//! Image generation (SVG and PNG) for color-rs

use image::{ImageBuffer, Rgba, RgbaImage};
use palette::Lab;
use resvg;
use std::fs;
use tiny_skia::{Pixmap, Transform};
use usvg::{Options, Tree, fontdb};

use crate::cli::{GradientArgs, HueArgs};
use crate::color_ops::analysis::hue::HueAnalysisResult;
use crate::config::display_constants;
use crate::error::{ColorError, Result};
use crate::image_core::{create_hue_gradient_svg, create_hue_palette_svg, create_svg_content};

// Extraction marker to force rebuild and document pure core delegation
const _IMAGE_CORE_EXTRACTED: bool = true;

// component_to_u8 moved to image_core (pure helper)

// lab_to_hex, lch_to_hex now provided by image_core (pure core)

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

        // Generate vectorized SVG if requested
        if args.vectorized_text {
            self.generate_vectorized_svg_gradient(args, start_lab, end_lab)?;
        }

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
            if let Some(pixel) = pixmap.pixel(x, y) {
                Rgba([pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()])
            } else {
                Rgba([0, 0, 0, 0]) // Fallback to transparent if out of bounds
            }
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
        create_svg_content(args, start_lab, end_lab)
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

        // Generate vectorized SVG if requested
        if args.vectorized_text {
            self.generate_vectorized_svg_hue_gradient(args, colors)?;
        }

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

        // Generate vectorized SVG if requested
        if args.vectorized_text {
            self.generate_vectorized_svg_hue_palette(args, colors)?;
        }

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
        create_hue_gradient_svg(args, colors)
    }

    /// Create vertical palette SVG from hue analysis results
    fn create_hue_palette_svg(
        &self,
        args: &HueArgs,
        colors: &[HueAnalysisResult],
    ) -> Result<String> {
        create_hue_palette_svg(args, colors)
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
            if let Some(pixel) = pixmap.pixel(x, y) {
                Rgba([pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()])
            } else {
                Rgba([0, 0, 0, 0])
            }
        });

        // Save PNG
        img.save(png_path)
            .map_err(|e| ColorError::ImageError(format!("Failed to save PNG: {e}")))?;

        Ok(())
    }

    /// Generate vectorized SVG for gradient (text as paths for designers)
    fn generate_vectorized_svg_gradient(
        &self,
        args: &GradientArgs,
        start_lab: Lab,
        end_lab: Lab,
    ) -> Result<()> {
        // Create regular SVG content first
        let svg_content = self.create_svg_content(args, start_lab, end_lab)?;

        // Convert text to paths using usvg
        let vectorized_svg = self.convert_text_to_paths(&svg_content)?;

        // Save with _vectorized suffix
        let vectorized_filename = args.svg_name().replace(".svg", "_vectorized.svg");
        fs::write(&vectorized_filename, vectorized_svg)?;

        Ok(())
    }

    /// Generate vectorized SVG for hue gradient (text as paths for designers)
    fn generate_vectorized_svg_hue_gradient(
        &self,
        args: &HueArgs,
        colors: &[HueAnalysisResult],
    ) -> Result<()> {
        // Create regular SVG content first
        let svg_content = self.create_hue_gradient_svg(args, colors)?;

        // Convert text to paths using usvg
        let vectorized_svg = self.convert_text_to_paths(&svg_content)?;

        // Save with _vectorized suffix
        let vectorized_filename = args.svg_name().replace(".svg", "_vectorized.svg");
        fs::write(vectorized_filename, vectorized_svg)?;

        Ok(())
    }

    /// Generate vectorized SVG for hue palette (text as paths for designers)
    fn generate_vectorized_svg_hue_palette(
        &self,
        args: &HueArgs,
        colors: &[HueAnalysisResult],
    ) -> Result<()> {
        // Create regular SVG content first
        let svg_content = self.create_hue_palette_svg(args, colors)?;

        // Convert text to paths using usvg
        let vectorized_svg = self.convert_text_to_paths(&svg_content)?;

        // Save with _vectorized suffix
        let vectorized_filename = args.svg_name().replace(".svg", "_vectorized.svg");
        fs::write(vectorized_filename, vectorized_svg)?;

        Ok(())
    }

    /// Convert text elements in SVG to vector paths using usvg
    fn convert_text_to_paths(&self, svg_content: &str) -> Result<String> {
        // Configure usvg options with font loading
        let mut options = Options::default();
        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();
        options.fontdb = std::sync::Arc::new(fontdb);

        // Parse SVG with text-to-path conversion (this automatically converts text to paths)
        let tree = Tree::from_str(svg_content, &options).map_err(|e| {
            ColorError::SvgError(format!("Failed to parse SVG for vectorization: {e}"))
        })?;

        // Convert tree back to SVG string with WriteOptions for proper formatting
        let write_options = usvg::WriteOptions::default();
        let vectorized_svg = tree.to_string(&write_options);

        Ok(vectorized_svg)
    }
}

impl Default for ImageGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// is_dark_color now provided by image_core

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
            vectorized_text: false,
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
