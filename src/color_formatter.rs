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

use crate::color_utils::*;
use crate::config::*;
use crate::error::Result;
use crate::output_utils::*;
use colored::*;
use palette::{Hsl, IntoColor, Lab, Lch, Srgb};

/// Color formatter for generating comprehensive color reports
pub struct ColorFormatter;

impl ColorFormatter {
    /// Format a color into a comprehensive analysis report with optional strategy (defaults to DeltaE2000)
    pub fn format_comprehensive_report(
        lab_color: Lab, 
        original_input: &str, 
        color_name: &str
    ) {
        // Use default DeltaE2000 strategy
        let default_strategy = crate::color_distance_strategies::DeltaE2000Strategy::default();
        let _ = Self::format_comprehensive_report_with_strategy(
            lab_color, 
            original_input, 
            color_name, 
            &default_strategy
        );
    }

    /// Format a color into a comprehensive analysis report with custom distance strategy
    pub fn format_comprehensive_report_with_strategy(
        lab_color: Lab,
        original_input: &str,
        color_name: &str,
        strategy: &dyn crate::color_distance_strategies::ColorDistanceStrategy,
    ) -> Result<String> {
        Self::write_header(original_input, lab_color);
        Self::write_format_conversions(lab_color);
        Self::write_additional_info(lab_color);
        Self::write_unified_collection_matches_impl(lab_color, color_name, strategy);

        Ok(String::new())
    }

    /// Write the report header
    fn write_header(color_input: &str, lab_color: Lab) {
        OutputUtils::print_header_ln(HEADER_COLOR_ANALYSIS);
        OutputUtils::print_pair_ln(LABEL_INPUT_COLOR, color_input);
        OutputUtils::print_hex_ln(LABEL_BASE_COLOR, lab_color, ColorFormat::Hex);

        println!();
    }

    /// Write all format conversions
    fn write_format_conversions(lab_color: Lab) {
        OutputUtils::print_header_ln(HEADER_FORMAT_CONVERSIONS);
        OutputUtils::print_hex_ln(LABEL_HEX, lab_color, ColorFormat::Rgb);
        OutputUtils::print_color_ln(LABEL_HSB, lab_color, ColorFormat::Hsv);
        OutputUtils::print_color_ln(LABEL_HSL, lab_color, ColorFormat::Hsl);
        OutputUtils::print_color_ln(LABEL_LAB, lab_color, ColorFormat::Lab);
        OutputUtils::print_color_ln(LABEL_LCH, lab_color, ColorFormat::Lch);
        OutputUtils::print_color_ln(LABEL_CMYK, lab_color, ColorFormat::Cmyk);
        OutputUtils::print_color_ln(LABEL_XYZ, lab_color, ColorFormat::Xyz);
        OutputUtils::print_color_ln(LABEL_OKLCH, lab_color, ColorFormat::Oklch);

        println!();
    }

    /// Write additional color information
    fn write_additional_info(lab_color: Lab) {
        OutputUtils::print_header_ln(HEADER_ADDITIONAL_INFO);
        OutputUtils::print_hex_ln(
            LABEL_GRAYSCALE_LAB,
            ColorUtils::lab_tulip_to_lab((lab_color.l, 0.0, 0.0)),
            ColorFormat::Lab,
        );
        let lch: Lch = ColorUtils::lab_to_lch(lab_color);
        OutputUtils::print_hex_ln(
            LABEL_GRAYSCALE_LCH_0,
            ColorUtils::lch_tulip_to_lab((lch.l, 0.0, lch.hue.into_degrees())),
            ColorFormat::Lch,
        );
        OutputUtils::print_hex_ln(
            LABEL_GRAYSCALE_LCH_2,
            ColorUtils::lch_tulip_to_lab((lch.l, lch.chroma * 0.02, lch.hue.into_degrees())),
            ColorFormat::Lch,
        );
        OutputUtils::print_hex_ln(
            LABEL_GRAYSCALE_LCH_4,
            ColorUtils::lch_tulip_to_lab((lch.l, lch.chroma * 0.04, lch.hue.into_degrees())),
            ColorFormat::Lch,
        );
        OutputUtils::print_hex_ln(
            LABEL_GRAYSCALE_LCH_6,
            ColorUtils::lch_tulip_to_lab((lch.l, lch.chroma * 0.06, lch.hue.into_degrees())),
            ColorFormat::Lch,
        );
        OutputUtils::print_f64_ln(
            LABEL_WCAG_LUMINANCE,
            ColorUtils::wcag_relative_luminance(ColorUtils::lab_to_srgb(lab_color)),
        );

        OutputUtils::print_contrast_white_ln(lab_color);
        OutputUtils::print_contrast_black_ln(lab_color);
        OutputUtils::print_brightness(lab_color);

        println!();
    }

    /// Implementation for writing unified collection matches with strategy
    fn write_unified_collection_matches_impl(
        lab_color: Lab,
        _css_name: &str,
        strategy: &dyn crate::color_distance_strategies::ColorDistanceStrategy,
    ) {
        use palette::IntoColor;

        println!(
            "{:#<width$}",
            HEADER_COLOR_COLLECTIONS.to_uppercase().bold(),
            width = COLUMN_HEADER_WIDTH
        );

        // Convert LAB to RGB for collection matching
        let srgb: Srgb = lab_color.into_color();
        let r = (srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;
        let rgb_array = [r, g, b];

        let manager = match crate::color_parser::UnifiedColorManager::new() {
            Ok(manager) => manager,
            Err(e) => {
                println!("Error creating color manager: {}", e);
                return;
            }
        };

        // Use strategy-based matching (always uses strategy now)
        let css_matches = manager.find_closest_css_colors_with_strategy(rgb_array, 2, strategy);
        Self::write_unified_collection_results(COLLECTION_CSS, &css_matches);

        let classic_matches =
            manager.find_closest_ral_classic_with_strategy(rgb_array, 2, strategy);
        Self::write_unified_collection_results(COLLECTION_RAL_CLASSIC, &classic_matches);

        let design_matches =
            manager.find_closest_ral_design_with_strategy(rgb_array, 2, strategy);
        Self::write_unified_collection_results(COLLECTION_RAL_DESIGN, &design_matches);

        println!();
    }

    /// Write unified collection search results that works with both ColorMatch and RalMatch
    fn write_unified_collection_results(
        collection_name: &str,
        matches: &[crate::color_parser::ColorMatch],
    ) {
        println!("{}", collection_name.bold());

        if matches.is_empty() {
            println!("{}", NO_MATCHES_MESSAGE.bold());
        } else {
            for color_match in matches.iter() {
                let [r, g, b] = color_match.entry.color.rgb;
                let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
                let code_default = "CSS".to_string();
                let code = color_match
                    .entry
                    .metadata
                    .code
                    .as_ref()
                    .unwrap_or(&code_default);

                println!(
                    "{} {} | {} [ΔE {:.2}]",
                    format!(
                        "{:>width$}",
                        format!("{}:", color_match.entry.metadata.name),
                        width = COLUMN_WIDTH
                    )
                    .bold()
                    .green(),
                    hex.to_uppercase().yellow(),
                    code,
                    color_match.distance
                );
            }
        }

        println!();
    }

    /// Write header for color schemes section
    fn write_color_scheme_header(section_title: &str) {
        // Header for color schemes section
        println!(
            "{:#<width$}",
            format!(
                "{} | {} ",
                HEADER_COLOR_SCHEMES.to_uppercase(),
                section_title
            )
            .bold(),
            width = COLUMN_HEADER_WIDTH
        );
    }

    /// Format color schemes section for comprehensive reports with both HSL and Lab strategies
    pub fn format_color_schemes(schemes: &crate::color_schemes::ColorSchemeResult) {
        Self::write_color_scheme_header("HSL Color Space Strategy");

        // HSL Complementary color section
        Self::write_color_scheme_section(
            HEADER_SCHEMA_COMPLIMENTARY,
            &[schemes.hsl_complementary],
            None,
            false, // Use HSL format
            "HSL",
        );

        // HSL Split-complementary colors section
        Self::write_color_scheme_section(
            HEADER_SCHEMA_SPLIT_COMPLIMENTARY,
            &[
                schemes.hsl_split_complementary.0,
                schemes.hsl_split_complementary.1,
            ],
            None,
            false, // Use HSL format
            "HSL",
        );

        // HSL Triadic colors section
        Self::write_color_scheme_section(
            HEADER_SCHEMA_TRIADIC,
            &[schemes.hsl_triadic.0, schemes.hsl_triadic.1],
            None,
            false, // Use HSL format
            "HSL",
        );

        println!();

        Self::write_color_scheme_header("Lab Color Space Strategy");

        // Lab Complementary color section
        Self::write_color_scheme_section(
            HEADER_SCHEMA_COMPLIMENTARY,
            &[schemes.lab_complementary],
            None,
            true, // Use Lab format
            "Lab",
        );

        // Lab Split-complementary colors section
        Self::write_color_scheme_section(
            HEADER_SCHEMA_SPLIT_COMPLIMENTARY,
            &[
                schemes.lab_split_complementary.0,
                schemes.lab_split_complementary.1,
            ],
            None,
            true, // Use Lab format
            "Lab",
        );

        // Lab Triadic colors section
        Self::write_color_scheme_section(
            HEADER_SCHEMA_TRIADIC,
            &[schemes.lab_triadic.0, schemes.lab_triadic.1],
            None,
            true, // Use Lab format
            "Lab",
        );
    }

    /// Write a specific color scheme section
    fn write_color_scheme_section(
        title: &str,
        colors: &[Lab],
        luminance_matched: Option<&[Lab]>,
        use_lab_output: bool,
        strategy_name: &str,
    ) {
        use colored::Colorize;

        // Basic colors with names like "Complementary", "Split 1", etc.
        for (i, color) in colors.iter().enumerate() {
            let color_name = match title {
                HEADER_SCHEMA_COMPLIMENTARY => LABEL_SCHEMA_COMPLIMENTARY_COLOR.to_string(),
                HEADER_SCHEMA_SPLIT_COMPLIMENTARY => {
                    format!("{} {}", LABEL_SCHEMA_SPLIT_COMPLIMENTARY_COLOR, i + 1)
                }
                HEADER_SCHEMA_TRIADIC => {
                    format!("{} {}", LABEL_SCHEMA_TRIADIC_COLOR, i + 1)
                }
                _ => format!("{} {}", LABEL_SCHEMA_OTHER_COLOR, i + 1),
            };

            let (color_value, hex) = Self::format_color_for_output(*color, use_lab_output);

            println!(
                "{} {} | {}",
                format!(
                    "{:>width$}",
                    format!("{}:", color_name),
                    width = COLUMN_WIDTH
                )
                .bold()
                .green(),
                hex.yellow(),
                color_value
            );
        }

        // Luminance-matched variations if available
        if let Some(matched_colors) = luminance_matched {
            let header_text = format!("Luminance-matched variations ({})", strategy_name);
            println!("{}", header_text.bold());

            for (i, color) in matched_colors.iter().enumerate() {
                let color_name = match title {
                    HEADER_SCHEMA_COMPLIMENTARY => HEADER_SCHEMA_COMPLIMENTARY.to_string(),
                    HEADER_SCHEMA_SPLIT_COMPLIMENTARY => {
                        format!("{} {}", LABEL_SCHEMA_SPLIT_COMPLIMENTARY_COLOR, i + 1)
                    }
                    HEADER_SCHEMA_TRIADIC => format!("{} {}", LABEL_SCHEMA_TRIADIC_COLOR, i + 1),
                    _ => format!("{} {}", LABEL_SCHEMA_OTHER_COLOR, i + 1),
                };

                let (color_value, hex) = Self::format_color_for_output(*color, use_lab_output);

                println!(
                    "{} {}",
                    format!(
                        "{:>width$}",
                        format!("{}:", color_name),
                        width = COLUMN_WIDTH
                    )
                    .bold()
                    .green(),
                    color_value
                );

                println!("{:>width$} {}", "", hex.yellow(), width = COLUMN_WIDTH);
            }
        }
    }

    /// Format a color for output as either HSL or Lab based on flag
    fn format_color_for_output(color: Lab, use_lab_output: bool) -> (String, String) {
        use palette::IntoColor;

        // Always generate hex
        let srgb: Srgb = color.into_color();
        let r = (srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;
        let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);

        // Generate either HSL or Lab format
        let color_value = if use_lab_output {
            format!("lab({:.2}, {:.2}, {:.2})", color.l, color.a, color.b)
        } else {
            let hsl: palette::Hsl = color.into_color();
            format!(
                "hsl({:.0}, {:.1}%, {:.1}%)",
                hsl.hue.into_positive_degrees(),
                hsl.saturation * 100.0,
                hsl.lightness * 100.0
            )
        };

        (color_value, hex)
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
            hex: format!("#{:02X}{:02X}{:02X}", r, g, b),
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

    // ...existing code...
}
