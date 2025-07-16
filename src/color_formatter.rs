//! Color Formatting and Display
//!
//! This module handles formatting colors for display, generating comprehensive reports,
//! and managing color information output. The formatter supports multiple color collections
//! including CSS colors, RAL Classic, and RAL Design System+.
//!
//! # Features
//!
//! - Comprehensive color analysis reports with format conversions
//! - Support for multiple color collections (CSS, RAL Classic, RAL Design)
//! - WCAG accessibility information (contrast ratios, luminance)
//! - Organized output with consistent formatting using shared constants
//!
//! # Usage
//!
//! The main entry point is `ColorFormatter::format_comprehensive_report()` which generates
//! a complete analysis including:
//! - Color format conversions (RGB, Hex, HSL, LAB, XYZ, OKLCH)
//! - Additional information (grayscale, WCAG metrics, brightness)
//! - Color collection matches (CSS names, RAL Classic, RAL Design)

use crate::COLUMN_WIDTH;
use crate::color_utils::ColorUtils;
use crate::error::{ColorError, Result};
use colored::*;
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
        Self::write_format_conversions(&mut output, lab_color)?;
        Self::write_additional_info(&mut output, lab_color)?;
        Self::write_unified_collection_matches(&mut output, lab_color, color_name)?;

        Ok(output.trim_end().to_string())
    }

    /// Format a color into a comprehensive analysis report with collection data
    pub fn format_comprehensive_report_with_collections(
        lab_color: Lab,
        original_input: &str,
        css_name: &str,
        _ral_classic_info: &str,
        _ral_design_info: &str,
    ) -> Result<String> {
        let mut output = String::new();

        Self::write_header(&mut output, original_input)?;
        Self::write_format_conversions(&mut output, lab_color)?;
        Self::write_additional_info(&mut output, lab_color)?;
        Self::write_unified_collection_matches(&mut output, lab_color, css_name)?;

        Ok(output.trim_end().to_string())
    }

    /// Format a color into a comprehensive analysis report with custom distance strategy
    pub fn format_comprehensive_report_with_strategy(
        lab_color: Lab,
        original_input: &str,
        color_name: &str,
        strategy: &dyn crate::color_distance_strategies::ColorDistanceStrategy,
    ) -> Result<String> {
        let mut output = String::new();

        Self::write_header(&mut output, original_input)?;
        Self::write_format_conversions(&mut output, lab_color)?;
        Self::write_additional_info(&mut output, lab_color)?;
        Self::write_unified_collection_matches_with_strategy(
            &mut output,
            lab_color,
            color_name,
            strategy,
        )?;

        Ok(output.trim_end().to_string())
    }

    /// Write the report header
    fn write_header(output: &mut String, color_input: &str) -> Result<()> {
        writeln!(
            output,
            "{:^width$}",
            "Color Analysis".to_uppercase().bold().black().on_white(),
            width = COLUMN_WIDTH * 2
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        writeln!(
            output,
            "{} {}\n",
            format!("{:>width$}", "Color:", width = COLUMN_WIDTH)
                .bold()
                .green(),
            color_input
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        Ok(())
    }

    /// Write all format conversions
    fn write_format_conversions(output: &mut String, lab_color: Lab) -> Result<()> {
        writeln!(
            output,
            "{:^width$}",
            "Format Conversions"
                .to_uppercase()
                .bold()
                .black()
                .on_white(),
            width = COLUMN_WIDTH * 2
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // Convert LAB to sRGB for RGB/Hex display
        let srgb: Srgb = lab_color.into_color();
        let r = (srgb.red * 255.0).round() as u8;
        let g = (srgb.green * 255.0).round() as u8;
        let b = (srgb.blue * 255.0).round() as u8;

        // RGB
        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", "RGB:", width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!("rgb({}, {}, {})", r, g, b).white()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // Hex
        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", "Hex:", width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!("#{:02x}{:02x}{:02x}", r, g, b)
                .to_uppercase()
                .white()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // HSL
        let hsl: Hsl = lab_color.into_color();
        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", "HSL:", width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!(
                "hsl({:.0}, {:.1}%, {:.1}%)",
                hsl.hue.into_positive_degrees(),
                hsl.saturation * 100.0,
                hsl.lightness * 100.0
            )
            .white()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // LAB
        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", "LAB:", width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!(
                "lab({:.2}, {:.2}, {:.2})",
                lab_color.l, lab_color.a, lab_color.b
            )
            .white()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // XYZ
        let xyz: Xyz = lab_color.into_color();
        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", "XYZ:", width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!("xyz({:.3}, {:.3}, {:.3})", xyz.x, xyz.y, xyz.z).white()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // OKLCH
        let oklch: Oklch = lab_color.into_color();
        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", "OKLCH:", width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!(
                "oklch({:.3}, {:.3}, {:.1})",
                oklch.l,
                oklch.chroma,
                oklch.hue.into_positive_degrees()
            )
            .white()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        writeln!(output, "").map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        Ok(())
    }

    /// Write additional color information
    fn write_additional_info(output: &mut String, lab_color: Lab) -> Result<()> {
        writeln!(
            output,
            "{:^width$}",
            "Additional Information"
                .to_uppercase()
                .bold()
                .black()
                .on_white(),
            width = COLUMN_WIDTH * 2
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // Convert LAB to sRGB for calculations
        let srgb: Srgb = lab_color.into_color();
        let r = (srgb.red * 255.0).round() as u8;
        let g = (srgb.green * 255.0).round() as u8;
        let b = (srgb.blue * 255.0).round() as u8;

        // Grayscale equivalent using LAB L* component
        let grayscale_l = lab_color.l;
        let grayscale_rgb = (grayscale_l / 100.0 * 255.0).round() as u8;

        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", "Grayscale:", width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!(
                "rgb({}, {}, {})",
                grayscale_rgb, grayscale_rgb, grayscale_rgb
            )
            .white()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", "", width = COLUMN_WIDTH),
            format!(
                "#{:02x}{:02x}{:02x}",
                grayscale_rgb, grayscale_rgb, grayscale_rgb
            )
            .to_uppercase()
            .white()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // WCAG calculations
        let wcag_luminance = ColorUtils::wcag_relative_luminance(r, g, b);
        writeln!(
            output,
            "{} {}",
            format!(
                "{:>width$}",
                "WCAG Relative Luminance:",
                width = COLUMN_WIDTH
            )
            .bold()
            .green(),
            format!("{:.3}", wcag_luminance).white()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        let contrast_white = ColorUtils::wcag_contrast_ratio((r, g, b), (255, 255, 255));
        let contrast_black = ColorUtils::wcag_contrast_ratio((r, g, b), (0, 0, 0));

        // Color-coded contrast ratios
        let white_contrast_color = if contrast_white >= 7.0 {
            "✓".bold().green()
        } else if contrast_white >= 4.5 {
            "~".bold().yellow()
        } else {
            "✗".bold().red()
        };

        let black_contrast_color = if contrast_black >= 7.0 {
            "✓".bold().green()
        } else if contrast_black >= 4.5 {
            "~".bold().yellow()
        } else {
            "✗".bold().red()
        };

        writeln!(
            output,
            "{} {} [{}]",
            format!("{:>width$}", "Contrast vs White:", width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!("{:.2}:1", contrast_white).white(),
            white_contrast_color
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        writeln!(
            output,
            "{} {} [{}]",
            format!("{:>width$}", "Contrast vs Black:", width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!("{:.2}:1", contrast_black).white(),
            black_contrast_color
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        let brightness = if wcag_luminance > 0.18 {
            "Light".bold()
        } else {
            "Dark".bold()
        };
        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", "Brightness:", width = COLUMN_WIDTH)
                .bold()
                .green(),
            brightness
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        writeln!(output, "").map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        Ok(())
    }

    /// Write all collection matches together with closest colors
    fn write_unified_collection_matches(
        output: &mut String,
        lab_color: Lab,
        _css_name: &str,
    ) -> Result<()> {
        use crate::color_parser::{RgbColor, find_closest_ral_classic, find_closest_ral_design};
        use palette::IntoColor;

        writeln!(
            output,
            "{:^width$}",
            "Color Collections".to_uppercase().bold().on_white().black(),
            width = COLUMN_WIDTH * 2
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // Convert LAB to RGB for collection matching
        let srgb: Srgb = lab_color.into_color();
        let r = (srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;
        let rgb = RgbColor::new(r, g, b);
        let rgb_array = [r, g, b];

        // CSS Color Collection
        let css_matches = Self::find_closest_css_colors(rgb_array, 2)?;
        Self::write_css_collection(output, &css_matches)?;

        // RAL Classic Collection
        let classic_matches = find_closest_ral_classic(&rgb, 2);
        Self::write_ral_classic_collection(output, &classic_matches)?;

        // RAL Design Collection
        let design_matches = find_closest_ral_design(&rgb, 2);
        Self::write_ral_design_collection(output, &design_matches)?;

        writeln!(output, "").map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        Ok(())
    }

    /// Write all collection matches together with closest colors using a custom strategy
    fn write_unified_collection_matches_with_strategy(
        output: &mut String,
        lab_color: Lab,
        _css_name: &str,
        strategy: &dyn crate::color_distance_strategies::ColorDistanceStrategy,
    ) -> Result<()> {
        use palette::IntoColor;

        writeln!(
            output,
            "{:^width$}",
            "Color Collections".to_uppercase().bold().on_white().black(),
            width = COLUMN_WIDTH * 2
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // Convert LAB to RGB for collection matching
        let srgb: Srgb = lab_color.into_color();
        let r = (srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;
        let rgb_array = [r, g, b];

        // Use UnifiedColorManager with strategy
        let manager = crate::color_parser::UnifiedColorManager::new()?;

        // CSS Color Collection with strategy
        let css_matches = manager.find_closest_css_colors_with_strategy(rgb_array, 2, strategy);
        Self::write_color_matches("CSS Colors", output, &css_matches)?;

        // RAL Classic Collection with strategy
        let classic_matches =
            manager.find_closest_ral_classic_with_strategy(rgb_array, 2, strategy);
        Self::write_color_matches("RAL Classic", output, &classic_matches)?;

        // RAL Design Collection with strategy
        let design_matches = manager.find_closest_ral_design_with_strategy(rgb_array, 2, strategy);
        Self::write_color_matches("RAL Design System+", output, &design_matches)?;

        writeln!(output, "").map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        Ok(())
    }

    /// Write CSS color collection with closest matches
    fn write_css_collection(
        output: &mut String,
        css_matches: &[crate::color_parser::RalMatch],
    ) -> Result<()> {
        Self::write_collection_search_results("CSS Colors", output, css_matches)
    }

    /// Write collection search results with closest matches
    fn write_collection_search_results(
        collection_name: &str,
        output: &mut String,
        matches: &[crate::color_parser::RalMatch],
    ) -> Result<()> {
        writeln!(
            output,
            "{}",
            format!("{:^width$}", collection_name, width = COLUMN_WIDTH * 2)
                .bold()
                .on_bright_black()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        if matches.is_empty() {
            writeln!(
                output,
                "{:>width$}",
                "No close matches".bold(),
                width = COLUMN_WIDTH * 2
            )
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        } else {
            for ral_match in matches.iter() {
                writeln!(
                    output,
                    "{} {}",
                    format!(
                        "{:>width$}",
                        format!("{}:", ral_match.name),
                        width = COLUMN_WIDTH
                    )
                    .bold()
                    .green(),
                    ral_match.code.white()
                )
                .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
                writeln!(
                    output,
                    "{:>width$} {}",
                    format!("[ΔE {:.2}] ", ral_match.distance),
                    ral_match.hex.to_uppercase().yellow(),
                    width = COLUMN_WIDTH
                )
                .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
            }
        }
        Ok(())
    }

    /// Write RAL Classic collection with closest matches
    fn write_ral_classic_collection(
        output: &mut String,
        matches: &[crate::color_parser::RalMatch],
    ) -> Result<()> {
        Self::write_collection_search_results("RAL Classic", output, matches)
    }

    /// Write RAL Design collection with closest matches
    fn write_ral_design_collection(
        output: &mut String,
        matches: &[crate::color_parser::RalMatch],
    ) -> Result<()> {
        Self::write_collection_search_results("RAL Design System+", output, matches)
    }

    /// Write color matches for any collection type
    fn write_color_matches(
        collection_name: &str,
        output: &mut String,
        matches: &[crate::color_parser::ColorMatch],
    ) -> Result<()> {
        writeln!(
            output,
            "{}",
            format!("{:^width$}", collection_name, width = COLUMN_WIDTH * 2)
                .bold()
                .on_bright_black()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        for color_match in matches {
            // Convert RGB to hex for display
            let [r, g, b] = color_match.entry.color.rgb;
            let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);

            writeln!(
                output,
                "{} {}",
                format!(
                    "{:>width$}",
                    format!("{}:", color_match.entry.metadata.name),
                    width = COLUMN_WIDTH
                )
                .bold()
                .green(),
                color_match
                    .entry
                    .metadata
                    .code
                    .as_ref()
                    .unwrap_or(&"".to_string())
                    .white()
            )
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

            writeln!(
                output,
                "{:>width$} {}",
                format!("[ΔE {:.2}] ", color_match.distance),
                hex.to_uppercase().yellow(),
                width = COLUMN_WIDTH
            )
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        }

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
            lab: format!(
                "lab({:.2}, {:.2}, {:.2})",
                lab_color.l, lab_color.a, lab_color.b
            ),
        }
    }

    /// Find closest CSS colors and convert to RalMatch format for unified display
    fn find_closest_css_colors(
        rgb: [u8; 3],
        max_results: usize,
    ) -> anyhow::Result<Vec<crate::color_parser::RalMatch>> {
        use crate::color_parser::{RalClassification, RalMatch, UnifiedColorManager};

        let manager = UnifiedColorManager::new()?;
        let css_matches = manager.find_closest_css_colors(rgb, max_results);

        let matches: Vec<_> = css_matches
            .into_iter()
            .map(|color_match| {
                let entry = &color_match.entry;
                let code = entry
                    .metadata
                    .code
                    .clone()
                    .unwrap_or_else(|| "CSS".to_string());
                let hex = format!(
                    "#{:02x}{:02x}{:02x}",
                    entry.color.rgb[0], entry.color.rgb[1], entry.color.rgb[2]
                );

                RalMatch {
                    code,
                    name: entry.metadata.name.clone(),
                    hex,
                    distance: color_match.distance,
                    classification: RalClassification::Classic, // CSS doesn't have classification, using Classic as default
                }
            })
            .collect();

        Ok(matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Lab;

    #[test]
    fn test_format_comprehensive_report() {
        let lab_color = Lab::new(50.0, 20.0, -30.0);
        let result = ColorFormatter::format_comprehensive_report(lab_color, "#008080", "teal");

        assert!(result.is_ok());
        let output = result.unwrap();
        // Check if the output contains the essential text parts with new format
        assert!(output.contains("#008080"));
        // Note: "teal" should appear in the CSS Colors section as closest match
        assert!(output.contains("FORMAT CONVERSIONS"));
        assert!(output.contains("ADDITIONAL INFORMATION"));
        assert!(output.contains("COLOR COLLECTIONS"));
        assert!(output.contains("CSS Colors"));
        assert!(output.contains("RAL Classic"));
        assert!(output.contains("RAL Design System+"));
        assert!(output.contains("RGB"));
        assert!(output.contains("HSL"));
        assert!(output.contains("LAB"));
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
