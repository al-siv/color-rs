//! Color Formatting and Display
//!
//! This module handles formatting colors for display, generating comprehensive reports,
//! and managing color information output. The formatter supports multiple color collections
//! including CSS colors, RAL Classic, and RAL Design System+.
//!
//! # Features
//!
//! - Comprehensive color analysis reports with format conversions using Strategy Pattern
//! - Support for multiple color collections (CSS, RAL Classic, RAL Design)
//! - WCAG accessibility information (contrast ratios, luminance)
//! - Organized output with consistent formatting using shared constants
//! - Pluggable formatting strategies for different output types
//!
//! # Usage
//!
//! The main entry point is `ColorFormatter::format_with_strategy()` which generates
//! a complete analysis using the specified formatting strategy:
//! - Color format conversions (RGB, Hex, HSL, LAB, XYZ, OKLCH)
//! - Additional information (grayscale, WCAG metrics, brightness)
//! - Color collection matches (CSS names, RAL Classic, RAL Design)

/// Safely convert a clamped f32 color component to u8
#[inline]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn f32_to_u8_clamped(value: f32) -> u8 {
    (value * 255.0).round().clamp(0.0, 255.0) as u8
}

use crate::error::Result;
use crate::output_formats::{
    BrightnessInfo, ColorAnalysisOutput, ColorCollections, ColorFormats, ColorMatch, ContrastData,
    ContrastInfo, GrayscaleData,
};
use crate::utils::Utils;
use palette::{IntoColor, Lab, Srgb, Hsl, Lch};

/// Color formatter for generating comprehensive color reports
pub struct ColorFormatter;

impl ColorFormatter {
    /// Format a color into a comprehensive analysis report (deprecated)
    pub const fn format_comprehensive_report(
        lab_color: Lab,
        original_input: &str,
        color_name: &str,
    ) {
        // This function is deprecated in favor of collect_color_analysis_data
        // Keep for backward compatibility but doesn't actually format anything
        let _ = (lab_color, original_input, color_name);
    }

    /// Format a simple color info for table display
    #[must_use]
    pub fn format_color_info(lab_color: Lab, label: &str) -> crate::color::ColorInfo {
        // Convert LAB to RGB using functional conversion
        let srgb: Srgb = lab_color.into_color();
        let red = (srgb.red * 255.0).round() as u8;
        let green = (srgb.green * 255.0).round() as u8;
        let blue = (srgb.blue * 255.0).round() as u8;

        // Convert LAB to HSL using functional conversion
        let hsl: Hsl = srgb.into_color();
        let hue = hsl.hue.into_inner() as f64;
        let saturation = hsl.saturation as f64;
        let lightness = hsl.lightness as f64;

        crate::color::ColorInfo {
            label: label.to_string(),
            hex: format!("#{red:02X}{green:02X}{blue:02X}"),
            rgb: Utils::rgb_to_string(red, green, blue),
            hsl: format!(
                "hsl({:.0}, {:.1}%, {:.1}%)",
                hue,
                saturation * 100.0,
                lightness * 100.0
            ),
            lab: format!(
                "lab({:.2}, {:.2}, {:.2})",
                lab_color.l, lab_color.a, lab_color.b
            ),
        }
    }

    /// Collect color analysis data for file output instead of printing
    ///
    /// # Errors
    ///
    /// Returns an error if color conversion or analysis fails
    pub fn collect_color_analysis_data(
        lab_color: Lab,
        original_input: &str,
        color_name: &str,
        algorithm: crate::color_distance_strategies::DistanceAlgorithm,
    ) -> Result<ColorAnalysisOutput> {
        let conversion = Self::collect_format_conversions(lab_color);
        let contrast = Self::collect_contrast_data(lab_color);
        let grayscale = Self::collect_grayscale_data(lab_color);
        let color_collections = Self::collect_color_collections(lab_color, color_name, algorithm);

        let mut output = ColorAnalysisOutput::new();
        // Update metadata with distance algorithm
        output.metadata = crate::output_formats::ProgramMetadata::new(Some(algorithm.name()));

        Ok(output
            .with_input(
                original_input.to_string(),
                crate::color_ops::conversion::srgb_to_hex(lab_color.into_color()),
            )
            .with_conversion(conversion)
            .with_contrast(contrast)
            .with_grayscale(grayscale)
            .with_color_collections(color_collections))
    }

    /// Collect format conversion data
    /// Collect format conversion data using centralized formatting utilities
    fn collect_format_conversions(lab_color: Lab) -> ColorFormats {
        crate::format_utils::FormatUtils::get_all_formats(lab_color)
    }

    /// Collect contrast and luminance data
    fn collect_contrast_data(lab_color: Lab) -> ContrastData {
        let srgb: Srgb = lab_color.into_color();
        let relative_luminance = crate::color_ops::luminance::wcag_relative(srgb);
        let white_luminance = 1.0;
        let black_luminance = 0.0;

        let white_contrast = if relative_luminance > white_luminance {
            (relative_luminance + 0.05) / (white_luminance + 0.05)
        } else {
            (white_luminance + 0.05) / (relative_luminance + 0.05)
        };

        let black_contrast = if relative_luminance > black_luminance {
            (relative_luminance + 0.05) / (black_luminance + 0.05)
        } else {
            (black_luminance + 0.05) / (relative_luminance + 0.05)
        };

        ContrastData {
            wcag21_relative_luminance: relative_luminance,
            contrast_vs_white: ContrastInfo {
                ratio: white_contrast,
                assessment: Self::assess_contrast_level(white_contrast),
            },
            contrast_vs_black: ContrastInfo {
                ratio: black_contrast,
                assessment: Self::assess_contrast_level(black_contrast),
            },
            brightness: BrightnessInfo {
                lab_assessment: Self::assess_lab_brightness(lab_color.l),
                wcag_assessment: Self::assess_wcag_brightness(relative_luminance),
            },
        }
    }

    /// Assess contrast level
    fn assess_contrast_level(ratio: f64) -> String {
        if ratio >= 7.0 {
            "High".to_string()
        } else if ratio >= 4.5 {
            "Medium".to_string()
        } else if ratio >= 3.0 {
            "Low".to_string()
        } else {
            "Very Low".to_string()
        }
    }

    /// Assess LAB brightness
    fn assess_lab_brightness(l: f32) -> String {
        if l >= 70.0 {
            "Light".to_string()
        } else if l >= 50.0 {
            "Medium".to_string()
        } else {
            "Dark".to_string()
        }
    }

    /// Assess WCAG brightness
    fn assess_wcag_brightness(luminance: f64) -> String {
        if luminance >= 0.18 {
            "Light".to_string()
        } else {
            "Dark".to_string()
        }
    }

    /// Collect grayscale variations data
    fn collect_grayscale_data(lab_color: Lab) -> GrayscaleData {
        let lch: Lch = lab_color.into_color();

        // Create LCH variations with different chroma levels and convert to LAB
        let lch0_lab: Lab = Lch::new(lch.l, 0.0, lch.hue).into_color();
        let lch2_lab: Lab = Lch::new(lch.l, lch.chroma * 0.02, lch.hue).into_color();
        let lch4_lab: Lab = Lch::new(lch.l, lch.chroma * 0.04, lch.hue).into_color();
        let lch6_lab: Lab = Lch::new(lch.l, lch.chroma * 0.06, lch.hue).into_color();
        
        // Convert to hex using functional conversion
        let lch0_hex = crate::color_ops::conversion::srgb_to_hex(lch0_lab.into_color());
        let lch2_hex = crate::color_ops::conversion::srgb_to_hex(lch2_lab.into_color());
        let lch4_hex = crate::color_ops::conversion::srgb_to_hex(lch4_lab.into_color());
        let lch6_hex = crate::color_ops::conversion::srgb_to_hex(lch6_lab.into_color());

        GrayscaleData {
            lch0_hex,
            lch0: format!(
                "lch({:.2}, 0.000, {:.1})",
                lch.l,
                lch.hue.into_positive_degrees()
            ),
            lch2_hex,
            lch2: format!(
                "lch({:.2}, 2.000, {:.1})",
                lch.l,
                lch.hue.into_positive_degrees()
            ),
            lch4_hex,
            lch4: format!(
                "lch({:.2}, 4.000, {:.1})",
                lch.l,
                lch.hue.into_positive_degrees()
            ),
            lch6_hex,
            lch6: format!(
                "lch({:.2}, 6.000, {:.1})",
                lch.l,
                lch.hue.into_positive_degrees()
            ),
        }
    }

    /// Collect color collection matches with up to 4 colors and relative luminance
    fn collect_color_collections(
        lab_color: Lab,
        _color_name: &str,
        algorithm: crate::color_distance_strategies::DistanceAlgorithm,
    ) -> ColorCollections {
        use crate::color_parser::unified_manager::UnifiedColorManager;

        let manager = UnifiedColorManager::new().unwrap_or_default();
        let srgb: Srgb = lab_color.into_color();
        let rgb = [
            f32_to_u8_clamped(srgb.red),
            f32_to_u8_clamped(srgb.green),
            f32_to_u8_clamped(srgb.blue),
        ];

        // Get CSS colors
        let css_matches = manager.find_closest_css_colors_with_algorithm(rgb, 4, algorithm);
        let css_colors = css_matches
            .into_iter()
            .map(|m| {
                let match_lab = Lab::from(m.entry.color.lab);
                let match_srgb: Srgb = match_lab.into_color();
                ColorMatch {
                    name: m.entry.metadata.name.clone(),
                    hex: crate::color_ops::conversion::srgb_to_hex(match_srgb),
                    lch: crate::format_utils::FormatUtils::lab_to_lch(match_lab),
                    code: m.entry.metadata.code.clone(),
                    distance: m.distance,
                    wcag21_relative_luminance: crate::color_ops::luminance::wcag_relative(match_srgb),
                }
            })
            .collect();

        // Get RAL Classic colors
        let ral_classic_matches = manager.find_closest_ral_classic_with_algorithm(rgb, 4, algorithm);
        let ral_classic = ral_classic_matches
            .into_iter()
            .map(|m| {
                let match_lab = Lab::from(m.entry.color.lab);
                let match_srgb: Srgb = match_lab.into_color();
                ColorMatch {
                    name: m.entry.metadata.name.clone(),
                    hex: crate::color_ops::conversion::srgb_to_hex(match_srgb),
                    lch: crate::format_utils::FormatUtils::lab_to_lch(match_lab),
                    code: m.entry.metadata.code.clone(),
                    distance: m.distance,
                    wcag21_relative_luminance: crate::color_ops::luminance::wcag_relative(match_srgb),
                }
            })
            .collect();

        // Get RAL Design colors
        let ral_design_matches = manager.find_closest_ral_design_with_algorithm(rgb, 4, algorithm);
        let ral_design = ral_design_matches
            .into_iter()
            .map(|m| {
                let match_lab = Lab::from(m.entry.color.lab);
                let match_srgb: Srgb = match_lab.into_color();
                ColorMatch {
                    name: m.entry.metadata.name.clone(),
                    hex: crate::color_ops::conversion::srgb_to_hex(match_srgb),
                    lch: crate::format_utils::FormatUtils::lab_to_lch(match_lab),
                    code: m.entry.metadata.code.clone(),
                    distance: m.distance,
                    wcag21_relative_luminance: crate::color_ops::luminance::wcag_relative(match_srgb),
                }
            })
            .collect();

        ColorCollections {
            css_colors,
            ral_classic,
            ral_design,
        }
    }

    // ...existing code...
}
