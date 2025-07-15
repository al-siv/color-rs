//! Universal color utilities and transformations for color-rs
//!
//! This module contains general-purpose color functions that are used across
//! different parts of the library. It uses the palette crate for reliable
//! color space conversions and calculations.

use crate::error::{ColorError, Result};
use palette::{
    color_difference::{ImprovedCiede2000, Wcag21RelativeContrast}, 
    FromColor, Hsl, IntoColor, Lab, Mix, Srgb, Xyz
};

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

    /// Convert HSL to RGB via LAB color space using palette library
    /// 
    /// This provides an alternative conversion path: HSL → XYZ → LAB → RGB
    /// which may have different characteristics than direct HSL → RGB conversion
    /// 
    /// # Arguments
    /// * `h` - Hue (0.0-1.0, will be wrapped if outside range)
    /// * `s` - Saturation (0.0-1.0, will be clamped)
    /// * `l` - Lightness (0.0-1.0, will be clamped)
    /// 
    /// # Returns
    /// * RGB values as (r, g, b) where each component is 0-255
    pub fn hsl_to_rgb_via_lab(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
        use palette::RgbHue;
        
        // Create HSL color using palette
        let hsl: Hsl = Hsl::new(
            RgbHue::from_degrees(h * 360.0),
            s.clamp(0.0, 1.0),
            l.clamp(0.0, 1.0),
        );
        
        // Convert HSL → XYZ → LAB → RGB
        let xyz: Xyz = hsl.into_color();
        let lab: Lab = xyz.into_color();
        let srgb: Srgb = lab.into_color();
        
        // Convert to 0-255 range
        let r = (srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;
        
        (r, g, b)
    }

    /// Calculate Delta E using improved CIEDE2000 algorithm from palette library
    /// 
    /// This uses the ImprovedCiede2000 implementation from palette which provides
    /// more accurate perceptual color difference measurements than the basic formula
    /// 
    /// # Arguments
    /// * `lab1` - First LAB color
    /// * `lab2` - Second LAB color
    /// 
    /// # Returns
    /// * Improved Delta E distance (0.0 = identical, higher = more different)
    pub fn lab_delta_e_distance(lab1: Lab, lab2: Lab) -> f32 {
        lab1.improved_difference(lab2)
    }

    /// Interpolate between two LAB colors using palette's Mix trait
    /// 
    /// # Arguments
    /// * `start` - Starting LAB color
    /// * `end` - Ending LAB color
    /// * `t` - Interpolation factor (0.0 = start, 1.0 = end)
    /// 
    /// # Returns
    /// * Interpolated LAB color
    pub fn interpolate_lab(start: Lab, end: Lab, t: f64) -> Lab {
        start.mix(end, t as f32)
    }

    /// Calculate WCAG relative luminance using palette's implementation
    /// 
    /// Uses the official WCAG 2.1 implementation from the palette library
    /// which provides proper gamma correction and standards compliance
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
        let srgb = Srgb::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        );
        srgb.relative_luminance().luma
    }

    /// Calculate WCAG contrast ratio using palette's implementation
    /// 
    /// Uses the official WCAG 2.1 contrast ratio implementation from palette
    /// which automatically handles the (L1 + 0.05) / (L2 + 0.05) formula
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
        let srgb1 = Srgb::new(
            color1_rgb.0 as f32 / 255.0,
            color1_rgb.1 as f32 / 255.0,
            color1_rgb.2 as f32 / 255.0,
        );
        let srgb2 = Srgb::new(
            color2_rgb.0 as f32 / 255.0,
            color2_rgb.1 as f32 / 255.0,
            color2_rgb.2 as f32 / 255.0,
        );

        srgb1.relative_contrast(srgb2)
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
    fn test_hsl_to_rgb_via_lab_conversion() {
        // Test pure red (H=0, S=1, L=0.5)
        let (r, g, b) = ColorUtils::hsl_to_rgb_via_lab(0.0, 1.0, 0.5);
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);

        // Test pure blue (H=240/360, S=1, L=0.5)
        let (r, g, b) = ColorUtils::hsl_to_rgb_via_lab(240.0 / 360.0, 1.0, 0.5);
        assert_eq!(r, 0);
        assert_eq!(g, 0);
        assert_eq!(b, 255);
    }

    #[test]
    fn test_hsl_conversion_methods_comparison() {
        // Test 10 randomly chosen colors across HSL space
        let test_colors = [
            (0.0, 1.0, 0.5),      // Pure red
            (120.0/360.0, 1.0, 0.5), // Pure green  
            (240.0/360.0, 1.0, 0.5), // Pure blue
            (60.0/360.0, 1.0, 0.5),  // Yellow
            (300.0/360.0, 1.0, 0.5), // Magenta
            (180.0/360.0, 1.0, 0.5), // Cyan
            (30.0/360.0, 0.8, 0.6),  // Orange-ish
            (210.0/360.0, 0.6, 0.4), // Blue-ish
            (270.0/360.0, 0.7, 0.3), // Purple-ish
            (150.0/360.0, 0.5, 0.7), // Green-ish
        ];

        for (h, s, l) in test_colors.iter() {
            let direct = ColorUtils::hsl_to_rgb(*h, *s, *l);
            let via_lab = ColorUtils::hsl_to_rgb_via_lab(*h, *s, *l);
            
            // Calculate difference between the two methods
            let diff_r = (direct.0 as i16 - via_lab.0 as i16).abs();
            let diff_g = (direct.1 as i16 - via_lab.1 as i16).abs();
            let diff_b = (direct.2 as i16 - via_lab.2 as i16).abs();
            
            // Print comparison for analysis
            println!("HSL({:.3}, {:.3}, {:.3}) -> Direct: RGB({}, {}, {}) vs Via LAB: RGB({}, {}, {}) | Diff: ({}, {}, {})",
                h * 360.0, s, l, 
                direct.0, direct.1, direct.2,
                via_lab.0, via_lab.1, via_lab.2,
                diff_r, diff_g, diff_b
            );
            
            // The differences should generally be small for most colors
            // but may be larger in some edge cases due to different conversion paths
            // We'll allow up to 5 units of difference per channel as acceptable
            assert!(diff_r <= 5, "Red difference too large: {} for HSL({:.3}, {:.3}, {:.3})", diff_r, h * 360.0, s, l);
            assert!(diff_g <= 5, "Green difference too large: {} for HSL({:.3}, {:.3}, {:.3})", diff_g, h * 360.0, s, l);
            assert!(diff_b <= 5, "Blue difference too large: {} for HSL({:.3}, {:.3}, {:.3})", diff_b, h * 360.0, s, l);
        }
    }

    #[test]
    fn test_lab_delta_e_distance() {
        let red_lab = Lab::new(53.24, 80.09, 67.20);
        let blue_lab = Lab::new(32.30, 79.20, -107.86);

        // Distance should be perceptually meaningful using ImprovedCiede2000
        let distance = ColorUtils::lab_delta_e_distance(red_lab, blue_lab);
        assert!(distance > 20.0); // Red and blue should be quite different (actual ~23)

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
