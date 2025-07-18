//! Color Operations Facade for simplified color processing
//!
//! This module implements the Facade pattern to provide a simplified interface
//! for complex color operations, making the library easier to use for common tasks.

use crate::color_utils::*;
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

    pub fn hex_to_srgb(&self, hex: &str) -> Result<Srgb> {
        let lab = ColorUtils::parse_hex_color(hex)?;
        let srgb: Srgb = lab.into_color();
        Ok(srgb)
    }

    /// Convert RGB values to LAB color space
    ///
    /// # Arguments
    /// * `rgb` - RGB values as (r, g, b) tuple
    ///
    /// # Returns
    /// * LAB color space representation
    pub fn srgb_to_lab(&self, srgb: Srgb) -> Result<Lab> {
        Ok(srgb.into_color())
    }

    /// Convert RGB values to LAB color space
    ///
    /// # Arguments
    /// * `rgb` - RGB values as (r, g, b) tuple
    ///
    /// # Returns
    /// * LAB color space representation
    pub fn rgb_to_lab(&self, rgb: (u8, u8, u8)) -> Result<Lab> {
        Ok(ColorUtils::rgb_to_lab(rgb))
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
        Ok(ColorUtils::rgb_to_hsl_tuple(rgb))
    }

    /// Convert Srgb to HSL values as (hue, saturation, lightness)
    ///
    /// # Arguments
    /// * `srgb` - Srgb color
    ///
    /// # Returns
    /// * HSL values as (hue, saturation, lightness) where hue is in degrees
    pub fn srgb_to_hsl_tuple(&self, srgb: Srgb) -> Result<(f32, f32, f32)> {
        Ok(ColorUtils::srgb_to_hsl_tuple(srgb))
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
    pub fn hsl_to_rgb(&self, hsl: (f64, f64, f64)) -> Result<(u8, u8, u8)> {
        Ok(ColorUtils::hsl_tuple_to_rgb(hsl))
    }

    /// Calculate WCAG contrast ratio between two colors
    ///
    /// # Arguments
    /// * `color1` - First color as RGB tuple
    /// * `color2` - Second color as RGB tuple
    ///
    /// # Returns
    /// * Contrast ratio (1.0:1 to 21.0:1)
    pub fn calculate_contrast(&self, srgb1: Srgb, srgb2: Srgb) -> Result<f64> {
        Ok(ColorUtils::wcag_contrast_ratio(srgb1, srgb2))
    }

    /// Calculate WCAG contrast ratio between two colors given as RGB tuples
    ///
    /// # Arguments
    /// * `color1` - First color as RGB tuple
    /// * `color2` - Second color as RGB tuple
    ///
    /// # Returns
    /// * Contrast ratio (1.0:1 to 21.0:1)
    pub fn calculate_contrast_rgb(&self, rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)) -> Result<f64> {
        let srgb1 = ColorUtils::rgb_to_srgb(rgb1);
        let srgb2 = ColorUtils::rgb_to_srgb(rgb2);
        Ok(ColorUtils::wcag_contrast_ratio(srgb1, srgb2))
    }

    /// Calculate relative luminance for WCAG compliance
    ///
    /// # Arguments
    /// * `rgb` - RGB values as (r, g, b) tuple
    ///
    /// # Returns
    /// * Relative luminance (0.0-1.0)
    pub fn calculate_luminance_rgb(&self, rgb: (u8, u8, u8)) -> Result<f64> {
        Ok(ColorUtils::wcag_relative_luminance_rgb(rgb))
    }

    /// Convert Srgb to RGB tuple (u8, u8, u8)
    ///
    /// # Arguments
    /// * `srgb` - Srgb color
    ///
    /// # Returns
    /// * RGB values as (r, g, b) tuple
    pub fn srgb_to_rgb(&self, srgb: Srgb) -> Result<(u8, u8, u8)> {
        Ok(ColorUtils::srgb_to_rgb(srgb))
    }

    /// Convert RGB tuple (u8, u8, u8) to Srgb
    ///
    /// # Arguments
    /// * `rgb` - RGB values as (r, g, b) tuple
    ///
    /// # Returns
    /// * Srgb color
    pub fn rgb_to_srgb(&self, rgb: (u8, u8, u8)) -> Result<Srgb> {
        Ok(ColorUtils::rgb_to_srgb(rgb))
    }

    /// Calculate relative luminance for WCAG compliance
    ///
    /// # Arguments
    /// * `srgb` - Srgb color
    ///
    /// # Returns
    /// * Relative luminance (0.0-1.0)
    pub fn calculate_luminance(&self, srgb: Srgb) -> Result<f64> {
        Ok(ColorUtils::wcag_relative_luminance(srgb))
    }

    /// Calculate perceptual distance between two colors using Delta E
    ///
    /// # Arguments
    /// * `color1` - First color as RGB tuple
    /// * `color2` - Second color as RGB tuple
    ///
    /// # Returns
    /// * Delta E distance (0.0 = identical, higher = more different)
    pub fn calculate_distance(&self, srgb1: Srgb, srgb2: Srgb) -> Result<f64> {
        let lab1: Lab = srgb1.into_color();
        let lab2: Lab = srgb2.into_color();
        Ok(ColorUtils::lab_distance(lab1, lab2))
    }

    /// Create a grayscale version of a color using LAB L* component
    ///
    /// # Arguments
    /// * `rgb` - Source color as RGB tuple
    ///
    /// # Returns
    /// * Grayscale color as RGB tuple
    pub fn to_grayscale(&self, srgb: Srgb) -> Result<Srgb> {
        let lab: Lab = srgb.into_color();
        // Create grayscale using LAB L* component (a=0, b=0)
        let gray_lab = Lab::new(lab.l, 0.0, 0.0);
        Ok(gray_lab.into_color())
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
        ratio: f64,
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
        let srgb1: Srgb = self.hex_to_srgb(hex)?;
        let lab = self.srgb_to_lab(srgb1)?;
        let hsl = self.srgb_to_hsl_tuple(srgb1)?;
        let luminance = self.calculate_luminance(srgb1)?;
        let contrast_white =
            self.calculate_contrast_rgb(self.srgb_to_rgb(srgb1)?, (255, 255, 255))?;
        let contrast_black = self.calculate_contrast_rgb(self.srgb_to_rgb(srgb1)?, (0, 0, 0))?;
        let grayscale = self.to_grayscale(srgb1)?;

        Ok(ColorAnalysis {
            srgb: srgb1,
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
    pub srgb: Srgb,
    /// Hex color string
    pub hex: String,
    /// LAB color space values
    pub lab: Lab,
    /// HSL values (hue in degrees, saturation and lightness 0.0-1.0)
    pub hsl: (f32, f32, f32),
    /// WCAG relative luminance
    pub luminance: f64,
    /// Contrast ratio against white
    pub contrast_white: f64,
    /// Contrast ratio against black
    pub contrast_black: f64,
    /// Grayscale equivalent
    pub grayscale: Srgb,
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
        let srgb1 = Srgb::new(255.0 / 255.0, 87.0 / 255.0, 51.0 / 255.0);
        let srgb2 = Srgb::new(1.0, 1.0, 1.0);
        let contrast = facade.calculate_contrast(srgb1, srgb2).unwrap();
        assert!(contrast > 3.0 && contrast < 4.0); // Should be around 3.15
    }

    #[test]
    fn test_facade_color_analysis() {
        let facade = ColorOperationsFacade::new();
        let analysis = facade.analyze_color("#FF5733").unwrap();

        assert_eq!(analysis.srgb, facade.rgb_to_srgb((255, 87, 51)).unwrap());
        assert!(analysis.hex.to_uppercase().contains("FF5733"));
        assert!(analysis.luminance > 0.2 && analysis.luminance < 0.4);
        assert!(analysis.contrast_white > 3.0);
        assert!(analysis.contrast_black > 6.0);
    }

    #[test]
    fn test_facade_grayscale() {
        let facade = ColorOperationsFacade::new();
        let gray: Srgb = facade
            .to_grayscale(facade.rgb_to_srgb((255, 87, 51)).unwrap())
            .unwrap();

        // Grayscale should have similar R, G, B values
        let (r, g, b) = facade.srgb_to_rgb(gray).unwrap();
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
