//! Color Operations Facade for simplified color processing
//!
//! This module implements the Facade pattern to provide a simplified interface
//! for complex color operations, making the library easier to use for common tasks.

use crate::color_utils::ColorUtils;
use crate::error::Result;
use palette::{Hsl, IntoColor, Lab, Srgb};

/// Facade for complex color operations
///
/// Implements the Facade pattern to provide a simplified interface for
/// color space conversions, calculations, and operations without exposing
/// the complexity of the underlying systems.
///
/// # Example
/// ```rust
/// use color_rs::ColorOperationsFacade;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let facade = ColorOperationsFacade::new();
///
/// // Simple color conversion
/// let rgb = facade.hex_to_rgb("#FF5733")?;
/// let lab = facade.rgb_to_lab(rgb)?;
/// let contrast = facade.calculate_contrast(rgb, (255, 255, 255))?;
/// # Ok(())
/// # }
/// ```
pub struct ColorOperationsFacade;

impl ColorOperationsFacade {
    /// Create a new color operations facade
    pub fn new() -> Self {
        Self
    }

    /// Convert hex color to RGB values
    ///
    /// # Arguments
    /// * `hex` - Hex color string (with or without #)
    ///
    /// # Returns
    /// * RGB values as (r, g, b) tuple
    pub fn hex_to_rgb(&self, hex: &str) -> Result<(u8, u8, u8)> {
        let lab = ColorUtils::parse_hex_color(hex)?;
        Ok(ColorUtils::lab_to_rgb(lab))
    }

    /// Convert RGB values to LAB color space
    ///
    /// # Arguments
    /// * `rgb` - RGB values as (r, g, b) tuple
    ///
    /// # Returns
    /// * LAB color space representation
    pub fn rgb_to_lab(&self, rgb: (u8, u8, u8)) -> Result<Lab> {
        Ok(ColorUtils::rgb_to_lab([rgb.0, rgb.1, rgb.2]))
    }

    /// Convert LAB to hex color string
    ///
    /// # Arguments
    /// * `lab` - LAB color space representation
    ///
    /// # Returns
    /// * Hex color string with # prefix
    pub fn lab_to_hex(&self, lab: Lab) -> Result<String> {
        Ok(ColorUtils::lab_to_hex(lab))
    }

    /// Convert RGB to HSL values
    ///
    /// # Arguments
    /// * `rgb` - RGB values as (r, g, b) tuple
    ///
    /// # Returns
    /// * HSL values as (hue, saturation, lightness) where hue is in degrees
    pub fn rgb_to_hsl(&self, rgb: (u8, u8, u8)) -> Result<(f32, f32, f32)> {
        let srgb = Srgb::new(
            rgb.0 as f32 / 255.0,
            rgb.1 as f32 / 255.0,
            rgb.2 as f32 / 255.0,
        );
        let hsl: Hsl = srgb.into_color();
        Ok((
            hsl.hue.into_positive_degrees(),
            hsl.saturation,
            hsl.lightness,
        ))
    }

    /// Convert HSL to RGB values
    ///
    /// # Arguments
    /// * `hue` - Hue in degrees (0-360)
    /// * `saturation` - Saturation (0.0-1.0)
    /// * `lightness` - Lightness (0.0-1.0)
    ///
    /// # Returns
    /// * RGB values as (r, g, b) tuple
    pub fn hsl_to_rgb(&self, hue: f32, saturation: f32, lightness: f32) -> Result<(u8, u8, u8)> {
        let h_normalized = hue / 360.0;
        Ok(ColorUtils::hsl_to_rgb(h_normalized, saturation, lightness))
    }

    /// Calculate WCAG contrast ratio between two colors
    ///
    /// # Arguments
    /// * `color1` - First color as RGB tuple
    /// * `color2` - Second color as RGB tuple
    ///
    /// # Returns
    /// * Contrast ratio (1.0:1 to 21.0:1)
    pub fn calculate_contrast(&self, color1: (u8, u8, u8), color2: (u8, u8, u8)) -> Result<f32> {
        Ok(ColorUtils::wcag_contrast_ratio(color1, color2))
    }

    /// Calculate relative luminance for WCAG compliance
    ///
    /// # Arguments
    /// * `rgb` - RGB values as (r, g, b) tuple
    ///
    /// # Returns
    /// * Relative luminance (0.0-1.0)
    pub fn calculate_luminance(&self, rgb: (u8, u8, u8)) -> Result<f32> {
        Ok(ColorUtils::wcag_relative_luminance(rgb.0, rgb.1, rgb.2))
    }

    /// Calculate perceptual distance between two colors using Delta E
    ///
    /// # Arguments
    /// * `color1` - First color as RGB tuple
    /// * `color2` - Second color as RGB tuple
    ///
    /// # Returns
    /// * Delta E distance (0.0 = identical, higher = more different)
    pub fn calculate_distance(&self, color1: (u8, u8, u8), color2: (u8, u8, u8)) -> Result<f32> {
        let lab1 = self.rgb_to_lab(color1)?;
        let lab2 = self.rgb_to_lab(color2)?;
        Ok(ColorUtils::lab_distance(lab1, lab2))
    }

    /// Create a grayscale version of a color using LAB L* component
    ///
    /// # Arguments
    /// * `rgb` - Source color as RGB tuple
    ///
    /// # Returns
    /// * Grayscale color as RGB tuple
    pub fn to_grayscale(&self, rgb: (u8, u8, u8)) -> Result<(u8, u8, u8)> {
        let lab = self.rgb_to_lab(rgb)?;
        // Create grayscale using LAB L* component (a=0, b=0)
        let gray_lab = Lab::new(lab.l, 0.0, 0.0);
        Ok(ColorUtils::lab_to_rgb(gray_lab))
    }

    /// Mix two colors in LAB color space
    ///
    /// # Arguments
    /// * `color1` - First color as RGB tuple
    /// * `color2` - Second color as RGB tuple
    /// * `ratio` - Mix ratio (0.0 = color1, 1.0 = color2)
    ///
    /// # Returns
    /// * Mixed color as RGB tuple
    pub fn mix_colors(
        &self,
        color1: (u8, u8, u8),
        color2: (u8, u8, u8),
        ratio: f32,
    ) -> Result<(u8, u8, u8)> {
        let lab1 = self.rgb_to_lab(color1)?;
        let lab2 = self.rgb_to_lab(color2)?;
        let mixed = ColorUtils::interpolate_lab(lab1, lab2, ratio as f64);
        Ok(ColorUtils::lab_to_rgb(mixed))
    }

    /// Perform complete color analysis for a hex color
    ///
    /// # Arguments
    /// * `hex` - Hex color string
    ///
    /// # Returns
    /// * ColorAnalysis struct with all calculated values
    pub fn analyze_color(&self, hex: &str) -> Result<ColorAnalysis> {
        let rgb = self.hex_to_rgb(hex)?;
        let lab = self.rgb_to_lab(rgb)?;
        let hsl = self.rgb_to_hsl(rgb)?;
        let luminance = self.calculate_luminance(rgb)?;
        let contrast_white = self.calculate_contrast(rgb, (255, 255, 255))?;
        let contrast_black = self.calculate_contrast(rgb, (0, 0, 0))?;
        let grayscale = self.to_grayscale(rgb)?;

        Ok(ColorAnalysis {
            rgb,
            hex: self.lab_to_hex(lab)?,
            lab,
            hsl,
            luminance,
            contrast_white,
            contrast_black,
            grayscale,
        })
    }
}

impl Default for ColorOperationsFacade {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete color analysis result
#[derive(Debug, Clone)]
pub struct ColorAnalysis {
    /// RGB values
    pub rgb: (u8, u8, u8),
    /// Hex color string
    pub hex: String,
    /// LAB color space values
    pub lab: Lab,
    /// HSL values (hue in degrees, saturation and lightness 0.0-1.0)
    pub hsl: (f32, f32, f32),
    /// WCAG relative luminance
    pub luminance: f32,
    /// Contrast ratio against white
    pub contrast_white: f32,
    /// Contrast ratio against black
    pub contrast_black: f32,
    /// Grayscale equivalent
    pub grayscale: (u8, u8, u8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_facade_hex_to_rgb() {
        let facade = ColorOperationsFacade::new();
        let rgb = facade.hex_to_rgb("#FF5733").unwrap();
        assert_eq!(rgb, (255, 87, 51));
    }

    #[test]
    fn test_facade_rgb_to_lab() {
        let facade = ColorOperationsFacade::new();
        let lab = facade.rgb_to_lab((255, 87, 51)).unwrap();
        assert!(lab.l > 50.0 && lab.l < 70.0);
        assert!(lab.a > 40.0);
        assert!(lab.b > 40.0);
    }

    #[test]
    fn test_facade_contrast_calculation() {
        let facade = ColorOperationsFacade::new();
        let contrast = facade
            .calculate_contrast((255, 87, 51), (255, 255, 255))
            .unwrap();
        assert!(contrast > 3.0 && contrast < 4.0); // Should be around 3.15
    }

    #[test]
    fn test_facade_color_analysis() {
        let facade = ColorOperationsFacade::new();
        let analysis = facade.analyze_color("#FF5733").unwrap();

        assert_eq!(analysis.rgb, (255, 87, 51));
        assert!(analysis.hex.to_uppercase().contains("FF5733"));
        assert!(analysis.luminance > 0.2 && analysis.luminance < 0.4);
        assert!(analysis.contrast_white > 3.0);
        assert!(analysis.contrast_black > 6.0);
    }

    #[test]
    fn test_facade_grayscale() {
        let facade = ColorOperationsFacade::new();
        let gray = facade.to_grayscale((255, 87, 51)).unwrap();

        // Grayscale should have similar R, G, B values
        let (r, g, b) = gray;
        assert!((r as i16 - g as i16).abs() < 5);
        assert!((g as i16 - b as i16).abs() < 5);
    }

    #[test]
    fn test_facade_color_mixing() {
        let facade = ColorOperationsFacade::new();
        let red = (255, 0, 0);
        let blue = (0, 0, 255);

        let purple = facade.mix_colors(red, blue, 0.5).unwrap();

        // Mixed color should be somewhere between red and blue
        assert!(purple.0 > 0 && purple.0 < 255);
        assert!(purple.2 > 0 && purple.2 < 255);
    }
}
