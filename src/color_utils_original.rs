//! Universal color utilities and transformations for color-rs
//!
//! This module contains general-purpose color functions that are used across
//! different parts of the library. It uses the palette crate for reliable
//! color space conversions and calculations.

use crate::error::{ColorError, Result};
use palette::{
    FromColor, Hsl, Hsv, IntoColor, Lab, Mix, Srgb,
    color_difference::{ImprovedCiede2000, Wcag21RelativeContrast},
    color_theory::{Complementary, SplitComplementary, Tetradic, Triadic},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContrastLevel {
    High,
    Medium,
    Marginal,
    Low,
}

impl ContrastLevel {
    pub fn to_string(&self) -> &'static str {
        match self {
            ContrastLevel::High => "High",
            ContrastLevel::Medium => "Medium",
            ContrastLevel::Marginal => "Marginal",
            ContrastLevel::Low => "Low",
        }
    }
}

/// Universal color utilities for calculations and transformations
pub struct ColorUtils;

impl ColorUtils {
    /// Get color contrast assessment based on WCAG guidelines using palette's WCAG methods
    pub fn get_contrast_assessment(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)) -> (f32, ContrastLevel) {
        let srgb1 = Self::rgb_to_srgb(rgb1);
        let srgb2 = Self::rgb_to_srgb(rgb2);
        let contrast = srgb1.relative_contrast(srgb2);

        // Use palette's WCAG methods for assessment
        if srgb1.has_enhanced_contrast_text(srgb2) {
            (contrast, ContrastLevel::High)
        } else if srgb1.has_min_contrast_text(srgb2) {
            (contrast, ContrastLevel::Medium)
        } else if srgb1.has_min_contrast_large_text(srgb2) {
            (contrast, ContrastLevel::Marginal)
        } else {
            (contrast, ContrastLevel::Low)
        }
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
    pub fn lab_distance(lab1: Lab, lab2: Lab) -> f64 {
        lab1.improved_difference(lab2) as f64
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
    /// use palette::Srgb;
    /// let srgb: Srgb = Srgb::new(1.0, 0.341, 0.2); // #FF5733
    /// let luminance = ColorUtils::wcag_relative_luminance(srgb);
    /// // Returns approximately 0.283 for #FF5733
    /// ```
    pub fn wcag_relative_luminance(srgb: Srgb) -> f64 {
        srgb.relative_luminance().luma as f64
    }

    /// Calculate WCAG relative luminance from (r, g, b) u8 values using palette
    ///
    /// # Arguments
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    ///
    /// # Returns
    /// * WCAG relative luminance value (0.0-1.0)
    pub fn wcag_relative_luminance_rgb(rgb: (u8, u8, u8)) -> f64 {
        let srgb = Srgb::new(
            rgb.0 as f64 / 255.0,
            rgb.1 as f64 / 255.0,
            rgb.2 as f64 / 255.0,
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
    /// use palette::Srgb;
    /// let color1: Srgb = Srgb::new(1.0, 0.341, 0.2); // #FF5733
    /// let color2: Srgb = Srgb::new(1.0, 1.0, 1.0); // white
    /// let ratio = ColorUtils::wcag_contrast_ratio(color1, color2);
    /// // Returns approximately 3.15 for #FF5733 vs white
    /// ```
    pub fn wcag_contrast_ratio(srgb1: Srgb, srgb2: Srgb) -> f64 {
        srgb1.relative_contrast(srgb2) as f64
    }

    /// Calculate WCAG contrast ratio from two (r, g, b) u8 tuples
    ///
    /// This is a convenience wrapper that converts the input tuples to Srgb<f64>
    /// and then calls the main wcag_contrast_ratio function.
    ///
    /// # Arguments
    /// * `rgb1` - First color as (r, g, b) tuple (0-255 each)
    /// * `rgb2` - Second color as (r, g, b) tuple (0-255 each)
    ///
    /// # Returns
    /// * Contrast ratio (1.0:1 to 21.0:1)
    pub fn wcag_contrast_ratio_rgb(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)) -> f64 {
        let srgb1 = Self::rgb_to_srgb(rgb1);
        let srgb2 = Self::rgb_to_srgb(rgb2);
        Self::wcag_contrast_ratio(srgb1, srgb2)
    }

    /// Calculate the contrast ratio between two Lab colors using their L (lightness) values.
    ///
    /// This is a perceptual approximation, not a WCAG-compliant contrast ratio.
    /// The ratio is always >= 1.0, with higher values indicating greater contrast.
    ///
    /// # Arguments
    /// * `lab1` - First Lab color
    /// * `lab2` - Second Lab color
    ///
    /// # Returns
    /// * Ratio of the higher L to the lower L (always >= 1.0)
    pub fn lab_contrast_ratio(lab1: Lab, lab2: Lab) -> f64 {
        let l1: f64 = lab1.l.max(0.01) as f64; // Avoid division by zero
        let l2: f64 = lab2.l.max(0.01) as f64;
        if l1 > l2 { l1 / l2 } else { l2 / l1 }
    }

    /// Parse a hex color string into LAB color space
    ///
    /// Supports 3-digit (#RGB), 6-digit (#RRGGBB), and 8-digit (#RRGGBBAA) hex codes.
    /// Alpha channel (if present) is ignored.
    ///
    /// # Arguments
    /// * `hex` - Hex color string (with or without #)
    ///
    /// # Returns
    /// * LAB color space representation
    pub fn parse_hex_color(hex: &str) -> Result<Lab> {
        let hex = hex.trim_start_matches('#');
        let (r, g, b) = match hex.len() {
            3 => {
                // Expand #RGB to #RRGGBB
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)?;
                (r, g, b)
            }
            6 | 8 => {
                // #RRGGBB or #RRGGBBAA (ignore alpha)
                let r = u8::from_str_radix(&hex[0..2], 16)?;
                let g = u8::from_str_radix(&hex[2..4], 16)?;
                let b = u8::from_str_radix(&hex[4..6], 16)?;
                (r, g, b)
            }
            _ => {
                return Err(ColorError::InvalidColor(
                    "Invalid HEX color format. Expected #RGB, #RRGGBB, or #RRGGBBAA".to_string(),
                ));
            }
        };

        let srgb: Srgb = Self::rgb_to_srgb((r, g, b));
        Ok(srgb.into_color())
    }

    /// Convert LAB color to hex string
    ///
    /// # Arguments
    /// * `lab` - LAB color space representation
    ///
    /// # Returns
    /// * Hex color string (with # prefix, always uppercase)
    pub fn lab_to_hex(lab: Lab) -> String {
        // Convert Lab to sRGB, clamp to [0.0, 1.0] to avoid out-of-gamut artifacts
        Self::srgb_to_hex(lab.into_color())
    }

    /// Convert a palette::Srgb<f64> to a hex string (with # prefix, always uppercase)
    ///
    /// # Arguments
    /// * `srgb` - Srgb<f64> color (each channel in 0.0-1.0)
    ///
    /// # Returns
    /// * Hex color string (with # prefix, always uppercase)
    pub fn srgb_to_hex(srgb: Srgb) -> String {
        let (r, g, b) = Self::srgb_to_rgb(srgb);
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    /// Convert LAB color to HSL as a (h, s, l) tuple (f64, f64, f64)
    ///
    /// # Arguments
    /// * `lab` - LAB color space representation
    ///
    /// # Returns
    /// * (h, s, l) tuple where h is in degrees, s and l in 0.0-1.0
    pub fn lab_to_hsl_tuple(lab: Lab) -> (f32, f32, f32) {
        let srgb: Srgb = lab.into_color();
        let hsl: Hsl = srgb.into_color();
        (
            hsl.hue.into_positive_degrees() as f32,
            hsl.saturation as f32,
            hsl.lightness as f32,
        )
    }

    /// Convert LAB color to sRGB (palette::Srgb<f64>)
    ///
    /// # Arguments
    /// * `lab` - LAB color space representation
    ///
    /// # Returns
    /// * Srgb<f64> color (each channel in 0.0-1.0)
    pub fn lab_to_srgb(lab: Lab) -> Srgb {
        lab.into_color()
    }

    /// Convert LAB color to XYZ as a (x, y, z) tuple (f32, f32, f32)
    ///
    /// # Arguments
    /// * `lab` - LAB color space representation
    ///
    /// # Returns
    /// * (x, y, z) tuple where each component is in the D65 reference white space
    pub fn lab_to_xyz_tuple(lab: Lab) -> (f32, f32, f32) {
        let xyz = palette::Xyz::from_color(lab);
        (xyz.x, xyz.y, xyz.z)
    }

    /// Convert LAB color to OKLCH as a (l, c, h) tuple (f32, f32, f32)
    ///
    /// # Arguments
    /// * `lab` - LAB color space representation
    ///
    /// # Returns
    /// * (l, c, h) tuple where l is lightness (0.0-1.0), c is chroma, h is hue in degrees
    pub fn lab_to_oklch_tuple(lab: Lab) -> (f32, f32, f32) {
        let oklch = palette::Oklch::from_color(lab);
        (oklch.l, oklch.chroma, oklch.hue.into_positive_degrees())
    }

    /// Convert a palette::Srgb<f64> to Lab color space
    ///
    /// # Arguments
    /// * `srgb` - Srgb<f64> color (each channel in 0.0-1.0)
    ///
    /// # Returns
    /// * Lab color space representation
    pub fn srgb_to_lab(srgb: Srgb) -> Lab {
        // Ensure input is clamped to [0.0, 1.0] to avoid out-of-gamut artifacts
        let clamped: Srgb = Srgb::new(
            srgb.red.clamp(0.0, 1.0),
            srgb.green.clamp(0.0, 1.0),
            srgb.blue.clamp(0.0, 1.0),
        );
        Lab::from_color(clamped.into_linear())
    }

    /// Convert HSL to Lab color space using the palette library
    ///
    /// # Arguments
    /// * `hsl` - HSL color space representation
    ///
    /// # Returns
    /// * Lab color space representation
    pub fn hsl_to_lab(hsl: Hsl) -> Lab {
        let srgb: Srgb = hsl.into_color();
        Self::srgb_to_lab(srgb)
    }

    /// Convert Lab to HSL color space using the palette library
    ///
    /// # Arguments
    /// * `lab` - Lab color space representation
    ///
    /// # Returns  
    /// * HSL color space representation
    pub fn lab_to_hsl(lab: Lab) -> Hsl {
        let srgb = Self::lab_to_srgb(lab);
        srgb.into_color()
    }

    /// Convert LAB color to LCH (CIE L*C*h) color space
    ///
    /// # Arguments
    /// * `lab` - LAB color space representation
    ///
    /// # Returns
    /// * Lch color space representation
    pub fn lab_to_lch(lab: Lab) -> palette::Lch {
        palette::Lch::from_color(lab)
    }

    /// Convert LAB color to LCH as a (l, c, h) tuple (f32, f32, f32)
    ///
    /// # Arguments
    /// * `lab` - LAB color space representation
    ///
    /// # Returns
    /// * (l, c, h) tuple where l is lightness, c is chroma, h is hue in degrees
    pub fn lab_to_lch_tuple(lab: Lab) -> (f32, f32, f32) {
        let lch: palette::Lch = palette::Lch::from_color(lab);
        (lch.l, lch.chroma, lch.hue.into_positive_degrees())
    }

    /// Convert a "tulip" LAB color (L, a, b as f32) to palette::Lab
    ///
    /// # Arguments
    /// * `lab` - LAB color as (L, a, b) tuple (f32, f32, f32)
    ///
    /// # Returns
    /// * Lab color space representation
    pub fn lab_tulip_to_lab(lab: (f32, f32, f32)) -> Lab {
        Lab::new(lab.0, lab.1, lab.2)
    }

    /// Convert a "tulip" LCH color (L, C, H as f32) to palette::Lab
    ///
    /// # Arguments
    /// * `lch` - LCH color as (L, C, H) tuple (f32, f32, f32), H in degrees
    ///
    /// # Returns
    /// * Lab color space representation
    pub fn lch_tulip_to_lab(lch: (f32, f32, f32)) -> Lab {
        let lch_color = palette::Lch::new(lch.0, lch.1, lch.2);
        Lab::from_color(lch_color)
    }

    /// Convert a "tulip" LAB color (L, a, b as f32) to sRGB (palette::Srgb<f64>)
    ///
    /// # Arguments
    /// * `lab` - LAB color as (L, a, b) tuple (f32, f32, f32)
    ///
    /// # Returns
    /// * Srgb<f64> color (each channel in 0.0-1.0)
    pub fn lab_tulip_to_srgb(lab: (f32, f32, f32)) -> Srgb {
        let lab_color = Lab::new(lab.0, lab.1, lab.2);
        lab_color.into_color()
    }

    /// Convert LCH (CIE L*C*h) color space to LAB color space
    ///
    /// # Arguments
    /// * `lch` - Lch color space representation
    ///
    /// # Returns
    /// * Lab color space representation
    pub fn lch_to_lab(lch: palette::Lch) -> Lab {
        Lab::from_color(lch)
    }

    /// Convert LAB color to HSV as a (h, s, v) tuple (f32, f32, f32)
    ///
    /// # Arguments
    /// * `lab` - LAB color space representation
    ///
    /// # Returns
    /// * (h, s, v) tuple where h is in degrees, s and v in 0.0-1.0
    pub fn lab_to_hsv_tuple(lab: Lab) -> (f32, f32, f32) {
        let srgb: Srgb = lab.into_color();
        let hsv: Hsv = srgb.into_color();
        (
            hsv.hue.into_positive_degrees() as f32,
            hsv.saturation as f32,
            hsv.value as f32,
        )
    }

    /// Convert HSL color to (r, g, b) u8 tuple
    ///
    /// # Arguments
    /// * `hsl` - HSL color space representation
    ///
    /// # Returns
    /// * (r, g, b) tuple where each component is 0-255
    pub fn hsl_to_rgb(hsl: Hsl) -> (u8, u8, u8) {
        let srgb: Srgb = hsl.into_color();
        Self::srgb_to_rgb(srgb)
    }

    /// Convert a palette::Srgb<f64> to HSL as a (h, s, l) tuple (f64, f64, f64)
    ///
    /// # Arguments
    /// * `srgb` - Srgb<f64> color (each channel in 0.0-1.0)
    ///
    /// # Returns
    /// * (h, s, l) tuple where h is in degrees, s and l in 0.0-1.0
    pub fn srgb_to_hsl_tuple(srgb: Srgb) -> (f32, f32, f32) {
        let hsl: Hsl = srgb.into_color();
        (
            hsl.hue.into_positive_degrees(),
            hsl.saturation,
            hsl.lightness,
        )
    }

    /// Convert (r, g, b) u8 tuple to HSL as a (h, s, l) tuple (f32, f32, f32)
    ///
    /// # Arguments
    /// * `rgb` - (r, g, b) tuple where each component is 0-255
    ///
    /// # Returns
    /// * (h, s, l) tuple where h is in degrees, s and l in 0.0-1.0
    pub fn rgb_to_hsl_tuple(rgb: (u8, u8, u8)) -> (f32, f32, f32) {
        let srgb = Self::rgb_to_srgb(rgb);
        let hsl: Hsl = srgb.into_color();
        (
            hsl.hue.into_positive_degrees(),
            hsl.saturation,
            hsl.lightness,
        )
    }

    /// Convert HSL (f64, f64, f64) to (r, g, b) u8 tuple
    ///
    /// # Arguments
    /// * `hsl` - HSL color as (h, s, l) where h in degrees, s and l in 0.0-1.0
    ///
    /// # Returns
    /// * (r, g, b) tuple where each component is 0-255
    pub fn hsl_tuple_to_rgb(hsl: (f64, f64, f64)) -> (u8, u8, u8) {
        let hsl_color = Hsl::new(hsl.0 as f32, hsl.1 as f32, hsl.2 as f32);
        let srgb: Srgb = hsl_color.into_color();
        Self::srgb_to_rgb(srgb)
    }

    /// Convert LAB color to (r, g, b) u8 tuple
    ///
    /// # Arguments
    /// * `lab` - LAB color space representation
    ///
    /// # Returns
    /// * (r, g, b) tuple where each component is 0-255
    pub fn lab_to_rgb(lab: Lab) -> (u8, u8, u8) {
        let srgb: Srgb = lab.into_color();
        Self::srgb_to_rgb(srgb)
    }

    /// Convert LAB color array to RGB array
    ///
    /// # Arguments
    /// * `lab` - LAB values as [L, a, b]
    ///
    /// # Returns
    /// * RGB values as [r, g, b] where each component is 0-255
    pub fn lab_array_to_rgb(lab: [f32; 3]) -> [u8; 3] {
        let lab_color = Lab::new(lab[0], lab[1], lab[2]);
        // Convert Lab to linear RGB, then to sRGB for proper gamma correction
        let srgb: Srgb = lab_color.into_color();
        let (r, g, b) = Self::srgb_to_rgb(srgb);
        [r, g, b]
    }

    /// Convert an (r, g, b) u8 tuple to Lab color space using the palette library
    ///
    /// # Arguments
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    ///
    /// # Returns
    /// * Lab color
    pub fn rgb_to_lab(rgb: (u8, u8, u8)) -> Lab {
        let srgb = Srgb::new(
            rgb.0 as f32 / 255.0,
            rgb.1 as f32 / 255.0,
            rgb.2 as f32 / 255.0,
        )
        .into_linear();
        Lab::from_color(srgb)
    }

    /// Convert RGB array to LAB array
    ///
    /// # Arguments
    /// * `rgb` - RGB values as [r, g, b] where each component is 0-255
    ///
    /// # Returns
    /// * LAB values as [L, a, b]
    pub fn rgb_array_to_lab(rgb: [u8; 3]) -> [f32; 3] {
        let lab: Lab = Self::rgb_to_lab((rgb[0], rgb[1], rgb[2]));
        [lab.l, lab.a, lab.b]
    }

    /// Calculate LAB Delta E distance between two color arrays
    ///
    /// # Arguments
    /// * `lab1` - First LAB color as [L, a, b]
    /// * `lab2` - Second LAB color as [L, a, b]
    ///
    /// # Returns
    /// * Improved Delta E distance (0.0 = identical, higher = more different)
    pub fn lab_array_distance(lab1: [f32; 3], lab2: [f32; 3]) -> f64 {
        let lab1_color = Lab::new(lab1[0], lab1[1], lab1[2]);
        let lab2_color = Lab::new(lab2[0], lab2[1], lab2[2]);
        Self::lab_distance(lab1_color, lab2_color)
    }

    /// Convert (r, g, b) u8 tuple to palette::Srgb<f64>
    ///
    /// # Arguments
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    ///
    /// # Returns
    /// * Srgb<f64> color
    pub fn rgb_to_srgb(rgb: (u8, u8, u8)) -> Srgb {
        Srgb::new(
            rgb.0 as f32 / 255.0,
            rgb.1 as f32 / 255.0,
            rgb.2 as f32 / 255.0,
        )
    }

    /// Convert a palette::Srgb<f64> to an (r, g, b) tuple of u8 values (0-255)
    ///
    /// # Arguments
    /// * `srgb` - Srgb<f64> color (each channel in 0.0-1.0)
    ///
    /// # Returns
    /// * (r, g, b) tuple where each component is clamped and rounded to 0-255
    pub fn srgb_to_rgb(srgb: Srgb) -> (u8, u8, u8) {
        let r = (srgb.red.clamp(0.0, 1.0) * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (srgb.green.clamp(0.0, 1.0) * 255.0)
            .round()
            .clamp(0.0, 255.0) as u8;
        let b = (srgb.blue.clamp(0.0, 1.0) * 255.0)
            .round()
            .clamp(0.0, 255.0) as u8;
        (r, g, b)
    }

    /// Adjust a color to have the specified WCAG relative luminance while preserving hue
    ///
    /// This function works entirely in Lab color space for better perceptual accuracy.
    /// It uses binary search in the Lab L component to find the closest approximation
    /// to the target WCAG relative luminance while preserving the a* and b* components.
    ///
    /// # Arguments
    /// * `color` - The input color in Lab color space
    /// * `target_luminance` - Target WCAG relative luminance (0.0-1.0)
    ///
    /// # Returns
    /// * Adjusted color in Lab color space
    ///
    /// # Example
    /// ```rust
    /// use palette::{Lab, Srgb, IntoColor, FromColor};
    /// use color_rs::color_utils::ColorUtils;
    ///
    /// let red_lab = Lab::from_color(Srgb::new(1.0, 0.0, 0.0));
    /// let adjusted = ColorUtils::adjust_color_relative_luminance(red_lab, 0.5).unwrap();
    /// ```
    pub fn adjust_color_relative_luminance(color: Lab, target_luminance: f64) -> Result<Lab> {
        if !(0.0..=1.0).contains(&target_luminance) {
            return Err(ColorError::InvalidArguments(
                "Relative luminance must be between 0.0 and 1.0".to_string(),
            ));
        }
        let mut low = 0.0f32;
        let mut high = 100.0f32;
        let tolerance = 0.00049;
        let max_iterations = 50;
        let mut best_srgb = Srgb::new(0.0, 0.0, 0.0);

        for _ in 0..max_iterations {
            let mid = (low + high) / 2.0;
            let test_lab = Lab::new(mid, color.a, color.b);
            let test_srgb: Srgb = test_lab.into_color();
            let test_relative_lum: f64 = Self::wcag_relative_luminance(test_srgb) as f64;

            if (test_relative_lum - target_luminance).abs() < tolerance {
                best_srgb = test_srgb;
                break;
            }

            if test_relative_lum < target_luminance {
                low = mid;
            } else {
                high = mid;
            }

            // Keep track of best approximation
            best_srgb = test_srgb;
        }

        // Convert the final sRGB back to Lab to get correct a* and b* components
        Ok(Lab::from_color(best_srgb))
    }

    /// Convert LAB color to CMYK as a (c, m, y, k) tuple (f32, f32, f32, f32)
    ///
    /// # Arguments
    /// * `lab` - LAB color space representation
    ///
    /// # Returns
    /// * (c, m, y, k) tuple where each component is 0.0-1.0
    pub fn lab_to_cmyk_tuple(lab: Lab) -> (f32, f32, f32, f32) {
        let srgb: Srgb = lab.into_color();
        let (r, g, b) = Self::srgb_to_rgb(srgb);
        Self::rgb_to_cmyk_tuple((r, g, b))
    }

    /// Convert RGB to CMYK color space
    ///
    /// Uses the standard CMYK conversion formula for print color representation.
    ///
    /// # Arguments
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    ///
    /// # Returns
    /// * CMYK values as (c, m, y, k) where each component is 0.0-1.0
    ///
    /// # Example
    /// ```rust
    /// use color_rs::color_utils::ColorUtils;
    /// let (c, m, y, k) = ColorUtils::rgb_to_cmyk_tuple((255, 87, 51));
    /// // Returns CMYK values for #FF5733
    /// ```
    pub fn rgb_to_cmyk_tuple(rgb: (u8, u8, u8)) -> (f32, f32, f32, f32) {
        // Convert RGB to 0.0-1.0 range
        let r_norm = rgb.0 as f32 / 255.0;
        let g_norm = rgb.1 as f32 / 255.0;
        let b_norm = rgb.2 as f32 / 255.0;

        // Calculate K (black key)
        let k = 1.0 - r_norm.max(g_norm).max(b_norm);

        // Avoid division by zero and handle black
        if k >= 1.0 - f32::EPSILON {
            return (0.0, 0.0, 0.0, 1.0);
        }

        // Calculate CMY, clamp to [0.0, 1.0] for numerical safety
        let denom = 1.0 - k;
        let c = ((1.0 - r_norm - k) / denom).clamp(0.0, 1.0);
        let m = ((1.0 - g_norm - k) / denom).clamp(0.0, 1.0);
        let y = ((1.0 - b_norm - k) / denom).clamp(0.0, 1.0);

        (c, m, y, k.clamp(0.0, 1.0))
    }

    // Color Scheme Calculation Functions
    // These replace the custom implementations in color_schemes.rs with proper palette library usage

    /// Calculate complementary color using palette's built-in method
    ///
    /// # Arguments
    /// * `color` - Lab color space representation
    ///
    /// # Returns
    /// * Complementary Lab color
    pub fn complementary_lab(color: Lab) -> Lab {
        let lch = Self::lab_to_lch(color);
        let comp_lch = lch.complementary();
        Lab::from_color(comp_lch)
    }

    /// Calculate split complementary colors using palette's built-in method
    ///
    /// # Arguments  
    /// * `color` - Lab color space representation
    ///
    /// # Returns
    /// * Tuple of two split complementary Lab colors
    pub fn split_complementary_lab(color: Lab) -> (Lab, Lab) {
        let lch = Self::lab_to_lch(color);
        let (comp1, comp2) = lch.split_complementary();
        (Lab::from_color(comp1), Lab::from_color(comp2))
    }

    /// Calculate triadic colors using palette's built-in method
    ///
    /// # Arguments
    /// * `color` - Lab color space representation
    ///
    /// # Returns
    /// * Tuple of two triadic Lab colors
    pub fn triadic_lab(color: Lab) -> (Lab, Lab) {
        let lch = Self::lab_to_lch(color);
        let (tri1, tri2) = lch.triadic();
        (Lab::from_color(tri1), Lab::from_color(tri2))
    }

    /// Calculate tetradic colors using palette's built-in method
    ///
    /// # Arguments
    /// * `color` - Lab color space representation
    ///
    /// # Returns
    /// * Tuple of three tetradic Lab colors
    pub fn tetradic_lab(color: Lab) -> (Lab, Lab, Lab) {
        let lch = Self::lab_to_lch(color);
        let (tet1, tet2, tet3) = lch.tetradic();
        (
            Lab::from_color(tet1),
            Lab::from_color(tet2),
            Lab::from_color(tet3),
        )
    }

    /// Calculate complementary color using HSL color space via palette's built-in method
    ///
    /// # Arguments
    /// * `color` - Lab color space representation
    ///
    /// # Returns
    /// * Complementary Lab color calculated via HSL
    pub fn complementary_hsl(color: Lab) -> Lab {
        let hsl = Self::lab_to_hsl(color);
        let comp_hsl = hsl.complementary();
        Self::hsl_to_lab(comp_hsl)
    }

    /// Calculate split complementary colors using HSL color space via palette's built-in method
    ///
    /// # Arguments
    /// * `color` - Lab color space representation
    ///
    /// # Returns
    /// * Tuple of two split complementary Lab colors calculated via HSL
    pub fn split_complementary_hsl(color: Lab) -> (Lab, Lab) {
        let hsl = Self::lab_to_hsl(color);
        let (comp1, comp2) = hsl.split_complementary();
        (Self::hsl_to_lab(comp1), Self::hsl_to_lab(comp2))
    }

    /// Calculate triadic colors using HSL color space via palette's built-in method
    ///
    /// # Arguments
    /// * `color` - Lab color space representation
    ///
    /// # Returns
    /// * Tuple of two triadic Lab colors calculated via HSL
    pub fn triadic_hsl(color: Lab) -> (Lab, Lab) {
        let hsl = Self::lab_to_hsl(color);
        let (tri1, tri2) = hsl.triadic();
        (Self::hsl_to_lab(tri1), Self::hsl_to_lab(tri2))
    }

    /// Calculate tetradic colors using HSL color space via palette's built-in method
    ///
    /// # Arguments
    /// * `color` - Lab color space representation
    ///
    /// # Returns
    /// * Tuple of three tetradic Lab colors calculated via HSL
    pub fn tetradic_hsl(color: Lab) -> (Lab, Lab, Lab) {
        let hsl = Self::lab_to_hsl(color);
        let (tet1, tet2, tet3) = hsl.tetradic();
        (
            Self::hsl_to_lab(tet1),
            Self::hsl_to_lab(tet2),
            Self::hsl_to_lab(tet3),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color() {
        let lab = ColorUtils::parse_hex_color("#FF0000").unwrap();
        assert!(lab.l > 50.0 && lab.l < 55.0); // Red should have lightness around 53
    }

    #[test]
    fn test_invalid_hex_color() {
        assert!(ColorUtils::parse_hex_color("#ZZZZZZ").is_err());
        assert!(ColorUtils::parse_hex_color("#FF00").is_err());
    }

    #[test]
    fn test_lab_delta_e_distance() {
        let red_lab = Lab::new(53.24, 80.09, 67.20);
        let blue_lab = Lab::new(32.30, 79.20, -107.86);

        // Distance should be perceptually meaningful using ImprovedCiede2000
        let distance = ColorUtils::lab_distance(red_lab, blue_lab);
        assert!(distance > 20.0); // Red and blue should be quite different (actual ~23)

        // Test identity (same color should have distance 0)
        let identity_distance = ColorUtils::lab_distance(red_lab, red_lab);
        assert!(identity_distance < 0.001);

        // Test symmetry
        let distance_ab = ColorUtils::lab_distance(red_lab, blue_lab);
        let distance_ba = ColorUtils::lab_distance(blue_lab, red_lab);
        assert!((distance_ab - distance_ba).abs() < 0.001);
    }

    #[test]
    fn test_wcag_luminance() {
        // Test known values
        let red_luminance =
            ColorUtils::wcag_relative_luminance(ColorUtils::rgb_to_srgb((255, 0, 0)));
        assert!((red_luminance - 0.2126).abs() < 0.01);

        let white_luminance =
            ColorUtils::wcag_relative_luminance(ColorUtils::rgb_to_srgb((255, 255, 255)));
        assert!((white_luminance - 1.0).abs() < 0.01);

        let black_luminance =
            ColorUtils::wcag_relative_luminance(ColorUtils::rgb_to_srgb((0, 0, 0)));
        assert!(black_luminance < 0.01);
    }

    #[test]
    fn test_lab_to_hex() {
        let lab = Lab::new(53.2, 80.1, 67.2); // Approximately red
        let hex = ColorUtils::lab_to_hex(lab);
        assert!(hex.starts_with('#'));
        assert_eq!(hex.len(), 7);
    }

    #[test]
    fn test_wcag_contrast_ratio() {
        // Test black vs white (maximum contrast)
        let max_contrast = ColorUtils::wcag_contrast_ratio(
            ColorUtils::rgb_to_srgb((0, 0, 0)),
            ColorUtils::rgb_to_srgb((255, 255, 255)),
        );
        assert!((max_contrast - 21.0).abs() < 0.1);

        // Test identical colors (minimum contrast)
        let min_contrast = ColorUtils::wcag_contrast_ratio(
            ColorUtils::rgb_to_srgb((128, 128, 128)),
            ColorUtils::rgb_to_srgb((128, 128, 128)),
        );
        assert!((min_contrast - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_color_interpolation() {
        let red = Lab::new(53.2, 80.1, 67.2);
        let blue = Lab::new(32.3, 79.2, -107.9);
        let mid = ColorUtils::interpolate_lab(red, blue, 0.5);

        // Middle color should be between red and blue
        assert!(mid.l > blue.l && mid.l < red.l);
    }
}
