//! Centralized color format conversion utilities
//!
//! Provides consistent color format conversions with standardized precision
//! for both console output and file export, eliminating code duplication.

use crate::color_ops::conversion;
use crate::precision_utils::PrecisionUtils;
use crate::utils::Utils;
use palette::{Lab, Srgb, Hsl, Hsv, Lch, Xyz, IntoColor};

/// Consolidated color format utilities
pub struct FormatUtils;

impl FormatUtils {
    /// Parse hex color string to RGB values
    #[must_use]
    pub fn parse_hex_color(hex: &str) -> Option<palette::Srgb> {
        let hex_clean = hex.trim_start_matches('#');
        if hex_clean.len() != 6 {
            return None;
        }

        let r = u8::from_str_radix(&hex_clean[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex_clean[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex_clean[4..6], 16).ok()?;

        Some(palette::Srgb::new(
            f32::from(r) / 255.0,
            f32::from(g) / 255.0,
            f32::from(b) / 255.0,
        ))
    }

    /// Convert LAB to hex format string using functional conversion
    #[must_use]
    pub fn lab_to_hex(lab: Lab) -> String {
        let srgb: Srgb = lab.into_color();
        conversion::srgb_to_hex(srgb)
    }

    /// Convert LAB to RGB format string using functional conversion
    #[must_use]
    pub fn lab_to_rgb(lab: Lab) -> String {
        let srgb: Srgb = lab.into_color();
        let r = (srgb.red * 255.0).round() as u8;
        let g = (srgb.green * 255.0).round() as u8;
        let b = (srgb.blue * 255.0).round() as u8;
        Utils::rgb_to_string(r, g, b)
    }

    /// Convert LAB to HSL format string with standardized precision using functional conversion
    #[must_use]
    pub fn lab_to_hsl(lab: Lab) -> String {
        let srgb: Srgb = lab.into_color();
        let hsl: Hsl = srgb.into_color();
        PrecisionUtils::format_hsl(
            hsl.hue.into_inner() as f64,
            (hsl.saturation * 100.0) as f64,
            (hsl.lightness * 100.0) as f64,
        )
    }

    /// Convert LAB to HSV/HSB format string with standardized precision using functional conversion
    #[must_use]
    pub fn lab_to_hsv(lab: Lab) -> String {
        let srgb: Srgb = lab.into_color();
        let hsv: Hsv = srgb.into_color();
        PrecisionUtils::format_hsv(
            hsv.hue.into_inner() as f64,
            (hsv.saturation * 100.0) as f64,
            (hsv.value * 100.0) as f64,
        )
    }

    /// Convert LAB to CMYK format string with standardized precision using functional conversion
    #[must_use]
    pub fn lab_to_cmyk(lab: Lab) -> String {
        let srgb: Srgb = lab.into_color();
        // Simple CMYK conversion formula
        let r = srgb.red;
        let g = srgb.green;
        let b = srgb.blue;
        
        let k = 1.0 - r.max(g).max(b);
        if k >= 1.0 {
            PrecisionUtils::format_cmyk(0.0, 0.0, 0.0, 100.0)
        } else {
            let c = (1.0 - r - k) / (1.0 - k);
            let m = (1.0 - g - k) / (1.0 - k);
            let y = (1.0 - b - k) / (1.0 - k);
            PrecisionUtils::format_cmyk(
                (c * 100.0) as f64,
                (m * 100.0) as f64,
                (y * 100.0) as f64,
                (k * 100.0) as f64,
            )
        }
    }

    /// Convert LAB to XYZ format string with standardized precision using functional conversion
    #[must_use]
    pub fn lab_to_xyz(lab: Lab) -> String {
        let srgb: Srgb = lab.into_color();
        let xyz: Xyz = srgb.into_color();
        PrecisionUtils::format_xyz(xyz.x as f64, xyz.y as f64, xyz.z as f64)
    }

    /// Convert LAB to LAB format string with standardized precision
    #[must_use]
    pub fn lab_to_lab(lab: Lab) -> String {
        PrecisionUtils::format_lab(f64::from(lab.l), f64::from(lab.a), f64::from(lab.b))
    }

    /// Convert LAB to LCH format string with standardized precision using functional conversion
    #[must_use]
    pub fn lab_to_lch(lab: Lab) -> String {
        let lch: Lch = lab.into_color();
        PrecisionUtils::format_lch(
            lch.l as f64,
            lch.chroma as f64,
            lch.hue.into_inner() as f64,
        )
    }

    /// Convert LAB to OKLCH format string with standardized precision using functional conversion
    #[must_use]
    pub fn lab_to_oklch(lab: Lab) -> String {
        // For now, use LCH as approximation for OKLCH since palette doesn't have native OKLCH
        // This is a simplified conversion - for true OKLCH, more complex color space conversion would be needed
        let lch: Lch = lab.into_color();
        PrecisionUtils::format_oklch(
            lch.l as f64,
            lch.chroma as f64,
            lch.hue.into_inner() as f64,
        )
    }

    /// Get all color format strings - this is the ONLY non-duplicate function in `FormatUtils`
    /// It actually adds value by collecting all formats into a structured output
    #[must_use]
    pub fn get_all_formats(lab: Lab) -> crate::output_formats::ColorFormats {
        crate::output_formats::ColorFormats {
            hex: Self::lab_to_hex(lab),
            rgb: Self::lab_to_rgb(lab),
            hsl: Self::lab_to_hsl(lab),
            hsb: Self::lab_to_hsv(lab),
            cmyk: Self::lab_to_cmyk(lab),
            xyz: Self::lab_to_xyz(lab),
            lab: Self::lab_to_lab(lab),
            lch: Self::lab_to_lch(lab),
            oklch: Self::lab_to_oklch(lab),
        }
    }
}

/// Enum for selecting the color output type.
#[derive(PartialEq, Eq)]
pub enum ColorFormat {
    Hex,
    Lab,
    Rgb,
    Hsl,
    Hsv,
    Cmyk,
    Xyz,
    Oklch,
    Lch,
}

impl FormatUtils {
    /// Format a color according to the specified format type
    #[must_use]
    pub fn format_color(lab: Lab, color_format: &ColorFormat) -> String {
        match color_format {
            ColorFormat::Lab => Self::lab_to_lab(lab),
            ColorFormat::Rgb => Self::lab_to_rgb(lab),
            ColorFormat::Hsl => Self::lab_to_hsl(lab),
            ColorFormat::Hsv => Self::lab_to_hsv(lab),
            ColorFormat::Cmyk => Self::lab_to_cmyk(lab),
            ColorFormat::Hex => Self::lab_to_hex(lab),
            ColorFormat::Xyz => Self::lab_to_xyz(lab),
            ColorFormat::Oklch => Self::lab_to_oklch(lab),
            ColorFormat::Lch => Self::lab_to_lch(lab),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Lab;

    #[test]
    fn test_lab_to_formats() {
        let lab = Lab::new(50.0, 25.0, -15.0);

        // Test that formats are consistent and have proper precision
        let hex = FormatUtils::lab_to_hex(lab);
        assert!(hex.starts_with('#'));
        assert_eq!(hex.len(), 7);

        let rgb = FormatUtils::lab_to_rgb(lab);
        assert!(rgb.starts_with("rgb("));
        assert!(rgb.ends_with(')'));

        let hsl = FormatUtils::lab_to_hsl(lab);
        assert!(hsl.starts_with("hsl("));
        assert!(hsl.contains('%'));

        let lab_str = FormatUtils::lab_to_lab(lab);
        assert!(lab_str.starts_with("lab("));
        assert!(lab_str.contains("50.00")); // Check precision
    }

    #[test]
    fn test_get_all_formats() {
        let lab = Lab::new(50.0, 25.0, -15.0);
        let formats = FormatUtils::get_all_formats(lab);

        assert!(!formats.hex.is_empty());
        assert!(!formats.rgb.is_empty());
        assert!(!formats.hsl.is_empty());
        assert!(!formats.lab.is_empty());
        assert!(!formats.lch.is_empty());
    }
}
