//! Color Operations Facade for simplified color processing
//!
//! This module implements the Facade pattern to provide a simplified interface
//! for complex color operations, making the library easier to use for common tasks.

use crate::color_utils::*;
use crate::error::Result;
use palette::{Lab, Srgb};

/// Facade for complex color operations
///
/// Implements the Facade pattern to provide a simplified interface for
/// color space conversions, calculations, and operations without exposing
/// the complexity of the underlying systems.
///
/// # Example
/// ```rust
/// use color_rs::ColorOperationsFacade;
/// use palette::Srgb;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let facade = ColorOperationsFacade::new();
///
/// // Color analysis
/// let analysis = facade.analyze_color("#FF5733")?;
///
/// // Calculate luminance
/// let srgb = Srgb::new(1.0, 0.341, 0.2);
/// let luminance = facade.calculate_luminance(srgb)?;
///
/// // Calculate distance between colors
/// let srgb1 = Srgb::new(1.0, 0.0, 0.0);
/// let srgb2 = Srgb::new(0.0, 1.0, 0.0);
/// let distance = facade.calculate_distance(srgb1, srgb2)?;
/// # Ok(())
/// # }
/// ```
pub struct ColorOperationsFacade;

impl ColorOperationsFacade {
    /// Create a new color operations facade
    pub fn new() -> Self {
        Self
    }

    // CRITICAL DUPLICATION ELIMINATION: ColorOperationsFacade removed all wrapper functions
    // These were duplicating ColorUtils functions with unnecessary Result<> wrappers
    // Direct usage of ColorUtils is preferred - this facade adds no value and creates confusion

    // Use ColorUtils::parse_hex_color() directly instead of hex_to_rgb()
    // Use ColorUtils::rgb_to_lab() directly instead of rgb_to_lab()
    // Use ColorUtils::lab_to_hex() directly instead of lab_to_hex()
    // Use ColorUtils::rgb_to_hsl_tuple() directly instead of rgb_to_hsl()
    // Use ColorUtils::wcag_contrast_ratio() directly instead of calculate_contrast()
    // Use ColorUtils::wcag_relative_luminance_rgb() directly instead of calculate_luminance_rgb()

    // This facade pattern was creating massive code duplication without adding functionality

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
        let lab1 = ColorUtils::srgb_to_lab(srgb1);
        let lab2 = ColorUtils::srgb_to_lab(srgb2);
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
        let lab = ColorUtils::srgb_to_lab(srgb);
        // Create grayscale using LAB L* component (a=0, b=0)
        let gray_lab = Lab::new(lab.l, 0.0, 0.0);
        Ok(ColorUtils::lab_to_srgb(gray_lab))
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
        let lab1 = ColorUtils::rgb_to_lab(color1);
        let lab2 = ColorUtils::rgb_to_lab(color2);
        let mixed = ColorUtils::interpolate_lab(lab1, lab2, ratio);
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
        let lab = ColorUtils::parse_hex_color(hex)?;
        let srgb1 = ColorUtils::lab_to_srgb(lab);
        let hsl = ColorUtils::srgb_to_hsl_tuple(srgb1);
        let luminance = ColorUtils::wcag_relative_luminance(srgb1);
        let white_srgb = ColorUtils::rgb_to_srgb((255, 255, 255));
        let black_srgb = ColorUtils::rgb_to_srgb((0, 0, 0));
        let contrast_white = ColorUtils::wcag_contrast_ratio(srgb1, white_srgb);
        let contrast_black = ColorUtils::wcag_contrast_ratio(srgb1, black_srgb);
        let grayscale = self.to_grayscale(srgb1)?;

        Ok(ColorAnalysis {
            srgb: srgb1,
            hex: ColorUtils::lab_to_hex(lab),
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
        let lab = ColorUtils::parse_hex_color("#FF5733").unwrap();
        let rgb = ColorUtils::lab_to_rgb(lab);
        assert_eq!(rgb, (255, 87, 51));
    }

    #[test]
    fn test_facade_rgb_to_lab() {
        let lab = ColorUtils::rgb_to_lab((255, 87, 51));
        assert!(lab.l > 50.0 && lab.l < 70.0);
        assert!(lab.a > 40.0);
        assert!(lab.b > 40.0);
    }

    #[test]
    fn test_facade_contrast_calculation() {
        let srgb1 = Srgb::new(255.0 / 255.0, 87.0 / 255.0, 51.0 / 255.0);
        let srgb2 = Srgb::new(1.0, 1.0, 1.0);
        let contrast = ColorUtils::wcag_contrast_ratio(srgb1, srgb2);
        assert!(contrast > 3.0 && contrast < 4.0); // Should be around 3.15
    }

    #[test]
    fn test_facade_color_analysis() {
        let facade = ColorOperationsFacade::new();
        let analysis = facade.analyze_color("#FF5733").unwrap();

        let expected_srgb = ColorUtils::rgb_to_srgb((255, 87, 51));
        // Use epsilon comparison for floating point values
        assert!((analysis.srgb.red - expected_srgb.red).abs() < 1e-6);
        assert!((analysis.srgb.green - expected_srgb.green).abs() < 1e-6);
        assert!((analysis.srgb.blue - expected_srgb.blue).abs() < 1e-6);
        assert!(analysis.hex.to_uppercase().contains("FF5733"));
        assert!(analysis.luminance > 0.2 && analysis.luminance < 0.4);
        assert!(analysis.contrast_white > 3.0);
        assert!(analysis.contrast_black > 6.0);
    }

    #[test]
    fn test_facade_grayscale() {
        let facade = ColorOperationsFacade::new();
        let input_srgb = ColorUtils::rgb_to_srgb((255, 87, 51));
        let gray: Srgb = facade.to_grayscale(input_srgb).unwrap();

        // Grayscale should have similar R, G, B values
        let (r, g, b) = ColorUtils::srgb_to_rgb(gray);
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
