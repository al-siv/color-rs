//! Centralized color format conversion utilities
//!
//! Provides consistent color format conversions with standardized precision
//! for both console output and file export, eliminating code duplication.

use crate::color_utils::LegacyColorUtils as ColorUtils;
use crate::precision_utils::PrecisionUtils;
use crate::utils::Utils;
use palette::Lab;

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

    /// Convert LAB to hex format string - DIRECT DELEGATION to `ColorUtils`
    #[must_use] pub fn lab_to_hex(lab: Lab) -> String {
        ColorUtils::lab_to_hex(lab)
    }

    /// Convert LAB to RGB format string
    #[must_use] pub fn lab_to_rgb(lab: Lab) -> String {
        let (r, g, b) = ColorUtils::lab_to_rgb(lab);
        Utils::rgb_to_string(r, g, b)
    }

    /// Convert LAB to HSL format string with standardized precision
    #[must_use] pub fn lab_to_hsl(lab: Lab) -> String {
        let (h, s, l) = ColorUtils::lab_to_hsl_tuple(lab);
        PrecisionUtils::format_hsl(h, s, l)
    }

    /// Convert LAB to HSV/HSB format string with standardized precision
    #[must_use] pub fn lab_to_hsv(lab: Lab) -> String {
        let (h, s, v) = ColorUtils::lab_to_hsv_tuple(lab);
        PrecisionUtils::format_hsv(h, s, v)
    }

    /// Convert LAB to CMYK format string with standardized precision
    #[must_use] pub fn lab_to_cmyk(lab: Lab) -> String {
        let (c, m, y, k) = ColorUtils::lab_to_cmyk_tuple(lab);
        PrecisionUtils::format_cmyk(c, m, y, k)
    }

    /// Convert LAB to XYZ format string with standardized precision
    #[must_use] pub fn lab_to_xyz(lab: Lab) -> String {
        let (x, y, z) = ColorUtils::lab_to_xyz_tuple(lab);
        PrecisionUtils::format_xyz(x, y, z)
    }

    /// Convert LAB to LAB format string with standardized precision
    #[must_use] pub fn lab_to_lab(lab: Lab) -> String {
        PrecisionUtils::format_lab(f64::from(lab.l), f64::from(lab.a), f64::from(lab.b))
    }

    /// Convert LAB to LCH format string with standardized precision
    #[must_use] pub fn lab_to_lch(lab: Lab) -> String {
        let (l, c, h) = ColorUtils::lab_to_lch_tuple(lab);
        PrecisionUtils::format_lch(l, c, h)
    }

    /// Convert LAB to OKLCH format string with standardized precision
    #[must_use] pub fn lab_to_oklch(lab: Lab) -> String {
        let (l, c, h) = ColorUtils::lab_to_oklch_tuple(lab);
        PrecisionUtils::format_oklch(l, c, h)
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
    #[must_use] pub fn format_color(lab: Lab, color_format: &ColorFormat) -> String {
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
