//! Universal color utilities and transformations for color-rs
//!
//! This module contains general-purpose color functions that are used across
//! different parts of the library. It uses the palette crate for reliable
//! color space conversions and calculations.

use crate::error::{ColorError, Result};
use palette::{FromColor, Hsl, IntoColor, Lab, Srgb};

/// Universal color utilities for calculations and transformations
pub struct ColorUtils;

impl ColorUtils {
    /// Convert RGB to LAB color space for perceptually accurate comparisons
    /// 
    /// Uses the palette crate for reliable sRGB to LAB conversion
    /// 
    /// # Arguments
    /// * `rgb` - RGB values as [r, g, b] where each component is 0-255
    /// 
    /// # Returns
    /// * LAB color space representation
    pub fn rgb_to_lab(rgb: [u8; 3]) -> Lab {
        let srgb = Srgb::new(
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0,
        );
        Lab::from_color(srgb)
    }

    /// Convert LAB color to RGB values
    /// 
    /// # Arguments
    /// * `lab` - LAB color space representation
    /// 
    /// # Returns
    /// * RGB values as (r, g, b) where each component is 0-255
    pub fn lab_to_rgb(lab: Lab) -> (u8, u8, u8) {
        let srgb: Srgb = lab.into_color();
        let r = (srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;
        (r, g, b)
    }

    /// Convert HSL to RGB using the palette crate
    /// 
    /// This replaces manual HSL->RGB conversion with the reliable palette implementation
    /// 
    /// # Arguments
    /// * `h` - Hue (0.0-1.0, will be wrapped if outside range)
    /// * `s` - Saturation (0.0-1.0, will be clamped)
    /// * `l` - Lightness (0.0-1.0, will be clamped)
    /// 
    /// # Returns
    /// * RGB values as (r, g, b) where each component is 0-255
    pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
        use palette::RgbHue;
        
        // Create HSL color using palette
        let hsl: Hsl = Hsl::new(
            RgbHue::from_degrees(h * 360.0),
            s.clamp(0.0, 1.0),
            l.clamp(0.0, 1.0),
        );
        
        // Convert directly to sRGB
        let srgb: Srgb = hsl.into_color();
        
        // Convert to 0-255 range
        let r = (srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;
        
        (r, g, b)
    }

    /// Calculate Delta E 1976 (CIE76) color difference in LAB space
    /// 
    /// This is a perceptually uniform color difference metric where
    /// equal numerical differences correspond to equal perceived differences
    /// 
    /// # Arguments
    /// * `lab1` - First LAB color
    /// * `lab2` - Second LAB color
    /// 
    /// # Returns
    /// * Delta E distance (0.0 = identical, higher = more different)
    pub fn lab_delta_e_distance(lab1: Lab, lab2: Lab) -> f32 {
        let dl = lab1.l - lab2.l;
        let da = lab1.a - lab2.a;
        let db = lab1.b - lab2.b;
        (dl * dl + da * da + db * db).sqrt()
    }

    /// Interpolate between two LAB colors
    /// 
    /// # Arguments
    /// * `start` - Starting LAB color
    /// * `end` - Ending LAB color
    /// * `t` - Interpolation factor (0.0 = start, 1.0 = end)
    /// 
    /// # Returns
    /// * Interpolated LAB color
    pub fn interpolate_lab(start: Lab, end: Lab, t: f64) -> Lab {
        let t = t as f32;
        Lab::new(
            start.l + (end.l - start.l) * t,
            start.a + (end.a - start.a) * t,
            start.b + (end.b - start.b) * t,
        )
    }

    /// Calculate WCAG relative luminance with proper gamma correction
    /// 
    /// This function implements the official WCAG 2.1 relative luminance calculation
    /// which differs from simple weighted averages in several important ways:
    /// 
    /// 1. **Gamma Correction**: sRGB values are linearized before calculation
    /// 2. **Modern Coefficients**: Uses sRGB/BT.709 coefficients (0.2126, 0.7152, 0.0722)
    ///    instead of older ITU-R BT.601 coefficients (0.299, 0.587, 0.114)
    /// 3. **Standards Compliance**: Follows W3C WCAG 2.1 specification exactly
    /// 
    /// The result is used for calculating contrast ratios that comply with
    /// WCAG accessibility guidelines. This is NOT the same as:
    /// - LRV (Light Reflectance Value) used in architecture
    /// - Simple weighted RGB averages
    /// - Perceptual lightness measures like LAB L*
    /// 
    /// # Arguments
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255) 
    /// * `b` - Blue component (0-255)
    /// 
    /// # Returns
    /// * WCAG relative luminance value (0.0-1.0)
    /// 
    /// # Example
    /// ```rust
    /// use color_rs::color_utils::ColorUtils;
    /// let luminance = ColorUtils::wcag_relative_luminance(255, 87, 51);
    /// // Returns approximately 0.283 for #FF5733
    /// ```
    pub fn wcag_relative_luminance(r: u8, g: u8, b: u8) -> f32 {
        fn linearize_srgb(value: f32) -> f32 {
            if value <= 0.03928 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        }

        let r_linear = linearize_srgb(r as f32 / 255.0);
        let g_linear = linearize_srgb(g as f32 / 255.0);
        let b_linear = linearize_srgb(b as f32 / 255.0);

        // WCAG relative luminance using sRGB/BT.709 coefficients
        0.2126 * r_linear + 0.7152 * g_linear + 0.0722 * b_linear
    }

    /// Calculate WCAG contrast ratio between two colors
    /// 
    /// Implements the WCAG 2.1 contrast ratio formula:
    /// (L1 + 0.05) / (L2 + 0.05) where L1 is the lighter color
    /// 
    /// WCAG compliance levels:
    /// - AA Normal Text: 4.5:1 minimum
    /// - AA Large Text: 3.0:1 minimum  
    /// - AAA Normal Text: 7.0:1 minimum
    /// - AAA Large Text: 4.5:1 minimum
    /// 
    /// # Arguments
    /// * `color1_rgb` - First color as (r, g, b) tuple (0-255 each)
    /// * `color2_rgb` - Second color as (r, g, b) tuple (0-255 each)
    /// 
    /// # Returns
    /// * Contrast ratio (1.0:1 to 21.0:1)
    /// 
    /// # Example
    /// ```rust
    /// use color_rs::color_utils::ColorUtils;
    /// let ratio = ColorUtils::wcag_contrast_ratio((255, 87, 51), (255, 255, 255));
    /// // Returns approximately 3.15 for #FF5733 vs white
    /// ```
    pub fn wcag_contrast_ratio(color1_rgb: (u8, u8, u8), color2_rgb: (u8, u8, u8)) -> f32 {
        let l1 = Self::wcag_relative_luminance(color1_rgb.0, color1_rgb.1, color1_rgb.2);
        let l2 = Self::wcag_relative_luminance(color2_rgb.0, color2_rgb.1, color2_rgb.2);

        let lighter = l1.max(l2);
        let darker = l1.min(l2);

        (lighter + 0.05) / (darker + 0.05)
    }

    /// Parse a hex color string into LAB color space
    /// 
    /// # Arguments
    /// * `hex` - Hex color string (with or without #)
    /// 
    /// # Returns
    /// * LAB color space representation
    pub fn parse_hex_color(hex: &str) -> Result<Lab> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err(ColorError::InvalidColor(
                "Invalid HEX color format. Expected format: #RRGGBB".to_string(),
            ));
        }

        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;

        Ok(Self::rgb_to_lab([r, g, b]))
    }

    /// Convert LAB color to hex string
    /// 
    /// # Arguments
    /// * `lab` - LAB color space representation
    /// 
    /// # Returns
    /// * Hex color string (with # prefix)
    pub fn lab_to_hex(lab: Lab) -> String {
        let (r, g, b) = Self::lab_to_rgb(lab);
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_lab_conversion() {
        // Test known RGB to LAB conversion
        let red_lab = ColorUtils::rgb_to_lab([255, 0, 0]);
        assert!((red_lab.l - 53.24).abs() < 1.0); // Red lightness around 53
        assert!(red_lab.a > 70.0); // Positive a (green-red axis)
        assert!(red_lab.b > 60.0); // Positive b (blue-yellow axis)

        let black_lab = ColorUtils::rgb_to_lab([0, 0, 0]);
        assert!(black_lab.l < 1.0); // Black should have very low lightness

        let white_lab = ColorUtils::rgb_to_lab([255, 255, 255]);
        assert!(white_lab.l > 95.0); // White should have high lightness
    }

    #[test]
    fn test_hsl_to_rgb_conversion() {
        // Test pure red (H=0, S=1, L=0.5)
        let (r, g, b) = ColorUtils::hsl_to_rgb(0.0, 1.0, 0.5);
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);

        // Test pure blue (H=240/360, S=1, L=0.5)
        let (r, g, b) = ColorUtils::hsl_to_rgb(240.0 / 360.0, 1.0, 0.5);
        assert_eq!(r, 0);
        assert_eq!(g, 0);
        assert_eq!(b, 255);
    }

    #[test]
    fn test_lab_delta_e_distance() {
        let red_lab = Lab::new(53.24, 80.09, 67.20);
        let blue_lab = Lab::new(32.30, 79.20, -107.86);

        // Distance should be perceptually meaningful
        let distance = ColorUtils::lab_delta_e_distance(red_lab, blue_lab);
        assert!(distance > 100.0); // Red and blue should be quite different

        // Test identity (same color should have distance 0)
        let identity_distance = ColorUtils::lab_delta_e_distance(red_lab, red_lab);
        assert!(identity_distance < 0.001);

        // Test symmetry
        let distance_ab = ColorUtils::lab_delta_e_distance(red_lab, blue_lab);
        let distance_ba = ColorUtils::lab_delta_e_distance(blue_lab, red_lab);
        assert!((distance_ab - distance_ba).abs() < 0.001);
    }

    #[test]
    fn test_wcag_luminance() {
        // Test known values
        let red_luminance = ColorUtils::wcag_relative_luminance(255, 0, 0);
        assert!((red_luminance - 0.2126).abs() < 0.01);

        let white_luminance = ColorUtils::wcag_relative_luminance(255, 255, 255);
        assert!((white_luminance - 1.0).abs() < 0.01);

        let black_luminance = ColorUtils::wcag_relative_luminance(0, 0, 0);
        assert!(black_luminance < 0.01);
    }

    #[test]
    fn test_wcag_contrast_ratio() {
        // Test black vs white (maximum contrast)
        let max_contrast = ColorUtils::wcag_contrast_ratio((0, 0, 0), (255, 255, 255));
        assert!((max_contrast - 21.0).abs() < 0.1);

        // Test identical colors (minimum contrast)
        let min_contrast = ColorUtils::wcag_contrast_ratio((128, 128, 128), (128, 128, 128));
        assert!((min_contrast - 1.0).abs() < 0.01);
    }
}
