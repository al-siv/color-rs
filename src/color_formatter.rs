//! Color Formatting and Display
//!
//! This module handles formatting colors for display, generating reports,
//! and managing color information output.

use crate::color_utils::ColorUtils;
use crate::error::{ColorError, Result};
use palette::{Hsl, IntoColor, Lab, Oklch, Srgb, Xyz};
use std::fmt::Write;

/// Color formatter for generating comprehensive color reports
pub struct ColorFormatter;

impl ColorFormatter {
    /// Format a color into a comprehensive analysis report
    pub fn format_comprehensive_report(
        lab_color: Lab,
        original_input: &str,
        color_name: &str,
    ) -> Result<String> {
        let mut output = String::new();

        Self::write_header(&mut output, original_input)?;
        Self::write_basic_info(&mut output, original_input, color_name)?;
        Self::write_format_conversions(&mut output, lab_color)?;
        Self::write_additional_info(&mut output, lab_color)?;

        Ok(output.trim_end().to_string())
    }

    /// Write the report header
    fn write_header(output: &mut String, color_input: &str) -> Result<()> {
        write!(output, "Color Analysis for: {}\n", color_input)
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        write!(
            output,
            "{}\n",
            "──────────────────────────────────────────────────"
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        Ok(())
    }

    /// Write basic color information
    fn write_basic_info(
        output: &mut String,
        color_input: &str,
        color_name: &str,
    ) -> Result<()> {
        write!(output, "Input: {}\n", color_input)
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        write!(output, "Name: {}\n", color_name)
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        write!(output, "\n").map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        Ok(())
    }

    /// Write all format conversions
    fn write_format_conversions(output: &mut String, lab_color: Lab) -> Result<()> {
        write!(output, "Format Conversions:\n")
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // Convert LAB to sRGB for RGB/Hex display
        let srgb: Srgb = lab_color.into_color();
        let r = (srgb.red * 255.0).round() as u8;
        let g = (srgb.green * 255.0).round() as u8;
        let b = (srgb.blue * 255.0).round() as u8;

        // RGB
        write!(output, "• RGB:    rgb({}, {}, {})\n", r, g, b)
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // Hex
        write!(output, "• Hex:    #{:02x}{:02x}{:02x}\n", r, g, b)
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // HSL
        let hsl: Hsl = lab_color.into_color();
        write!(
            output,
            "• HSL:    hsl({:.0}, {:.1}%, {:.1}%)\n",
            hsl.hue.into_positive_degrees(),
            hsl.saturation * 100.0,
            hsl.lightness * 100.0
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // LAB
        write!(
            output,
            "• LAB:    lab({:.2}, {:.2}, {:.2})\n",
            lab_color.l, lab_color.a, lab_color.b
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // XYZ
        let xyz: Xyz = lab_color.into_color();
        write!(
            output,
            "• XYZ:    xyz({:.3}, {:.3}, {:.3})\n",
            xyz.x, xyz.y, xyz.z
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // OKLCH
        let oklch: Oklch = lab_color.into_color();
        write!(
            output,
            "• OKLCH:  oklch({:.3}, {:.3}, {:.1}°)\n",
            oklch.l,
            oklch.chroma,
            oklch.hue.into_positive_degrees()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        write!(output, "\n").map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        Ok(())
    }

    /// Write additional color information
    fn write_additional_info(output: &mut String, lab_color: Lab) -> Result<()> {
        write!(output, "Additional Information:\n")
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // Convert LAB to sRGB for calculations
        let srgb: Srgb = lab_color.into_color();
        let r = (srgb.red * 255.0).round() as u8;
        let g = (srgb.green * 255.0).round() as u8;
        let b = (srgb.blue * 255.0).round() as u8;

        // Grayscale equivalent using LAB L* component
        let grayscale_l = lab_color.l;
        let grayscale_rgb = (grayscale_l / 100.0 * 255.0).round() as u8;

        write!(
            output,
            "• Grayscale: rgb({}, {}, {}) #{:02x}{:02x}{:02x} (LAB L* = {:.1})\n",
            grayscale_rgb,
            grayscale_rgb,
            grayscale_rgb,
            grayscale_rgb,
            grayscale_rgb,
            grayscale_rgb,
            grayscale_l
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // WCAG calculations
        let wcag_luminance = ColorUtils::wcag_relative_luminance(r, g, b);
        write!(output, "• WCAG Relative Luminance: {:.3}\n", wcag_luminance)
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        let contrast_white = ColorUtils::wcag_contrast_ratio((r, g, b), (255, 255, 255));
        let contrast_black = ColorUtils::wcag_contrast_ratio((r, g, b), (0, 0, 0));
        write!(output, "• Contrast vs White: {:.2}:1\n", contrast_white)
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        write!(output, "• Contrast vs Black: {:.2}:1\n", contrast_black)
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        let brightness = if wcag_luminance > 0.18 {
            "Light"
        } else {
            "Dark"
        };
        write!(output, "• Brightness: {}", brightness)
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        Ok(())
    }

    /// Format a simple color info for table display
    pub fn format_color_info(lab_color: Lab, label: &str) -> crate::color::ColorInfo {
        let srgb: Srgb = lab_color.into_color();
        let r = (srgb.red * 255.0).round() as u8;
        let g = (srgb.green * 255.0).round() as u8;
        let b = (srgb.blue * 255.0).round() as u8;

        let hsl: Hsl = lab_color.into_color();

        crate::color::ColorInfo {
            label: label.to_string(),
            hex: format!("#{:02x}{:02x}{:02x}", r, g, b),
            rgb: format!("rgb({}, {}, {})", r, g, b),
            hsl: format!(
                "hsl({:.0}, {:.1}%, {:.1}%)",
                hsl.hue.into_positive_degrees(),
                hsl.saturation * 100.0,
                hsl.lightness * 100.0
            ),
            lab: format!("lab({:.2}, {:.2}, {:.2})", lab_color.l, lab_color.a, lab_color.b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Lab;

    #[test]
    fn test_format_comprehensive_report() {
        let lab_color = Lab::new(50.0, 20.0, -30.0);
        let result = ColorFormatter::format_comprehensive_report(
            lab_color,
            "#008080",
            "teal",
        );

        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Color Analysis for: #008080"));
        assert!(output.contains("Name: teal"));
        assert!(output.contains("Format Conversions:"));
        assert!(output.contains("Additional Information:"));
    }

    #[test]
    fn test_format_color_info() {
        let lab_color = Lab::new(50.0, 0.0, 0.0);
        let info = ColorFormatter::format_color_info(lab_color, "Test Color");

        assert_eq!(info.label, "Test Color");
        assert!(info.hex.starts_with('#'));
        assert!(info.rgb.starts_with("rgb("));
        assert!(info.hsl.starts_with("hsl("));
        assert!(info.lab.starts_with("lab("));
    }
}
