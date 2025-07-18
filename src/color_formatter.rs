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

use crate::color_utils::ColorUtils;
use crate::config::*;
use crate::error::{ColorError, Result, IoResultExt, Utf8ResultExt};
use crate::formatter_strategies::FormattingStrategyFactory;
use colored::*;
use palette::{Hsl, Hsv, IntoColor, Lab, Oklch, Srgb, Xyz};
use std::fmt::Write;
use crate::output_utils::*;

/// Color formatter for generating comprehensive color reports
pub struct ColorFormatter;

impl ColorFormatter {
    /// Format a color using the specified formatting strategy (NEW STRATEGY PATTERN APPROACH)
    pub fn format_with_strategy(
        lab_color: Lab,
        original_input: &str,
        color_name: &str,
        strategy_name: &str,
    ) -> Result<String> {
        let strategy = FormattingStrategyFactory::create_strategy(strategy_name);
        strategy.format_color(lab_color, original_input, color_name)
    }

    /// Format a color into a comprehensive analysis report
    pub fn format_comprehensive_report(
        lab_color: Lab,
        original_input: &str,
        color_name: &str,
    ) -> Result<String> {
        let mut output = String::new();

        Self::write_header(&mut output, original_input, lab_color)?;
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

        Self::write_header(&mut output, original_input, lab_color)?;
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

        Self::write_header(&mut output, original_input, lab_color)?;
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
    fn write_header(output: &mut String, color_input: &str, lab_color: Lab) -> Result<()> {
        writeln!(
            output,
            "{:#<width$}",
            HEADER_COLOR_ANALYSIS.to_uppercase().bold(),
            width = COLUMN_HEADER_WIDTH
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", LABEL_INPUT_COLOR, width = COLUMN_WIDTH)
                .bold()
                .green(),
            color_input
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        let srgb: Srgb = lab_color.into_color();
        writeln!(
            output,
            "{} {}\n",
            format!("{:>width$}", LABEL_BASE_COLOR, width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!(
                "#{:02X}{:02X}{:02X}",
                (srgb.red * 255.0).round() as u8,
                (srgb.green * 255.0).round() as u8,
                (srgb.blue * 255.0).round() as u8
            )
            .yellow()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        Ok(())
    }

    /// Write all format conversions
    fn write_format_conversions(output: &mut String, lab_color: Lab) -> Result<()> {
        writeln!(
            output,
            "{:#<width$}",
            HEADER_FORMAT_CONVERSIONS.to_uppercase().bold(),
            width = COLUMN_HEADER_WIDTH
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // Convert LAB to sRGB for RGB/Hex display
        let srgb: Srgb = lab_color.into_color();
        let r = (srgb.red * 255.0).round() as u8;
        let g = (srgb.green * 255.0).round() as u8;
        let b = (srgb.blue * 255.0).round() as u8;

        // Hex
        writeln!(
            output,
            "{} {} | {}",
            format!("{:>width$}", LABEL_HEX, width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!("#{:02X}{:02X}{:02X}", r, g, b).yellow(),
            format!("rgb({}, {}, {})", r, g, b)
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // HSL - convert through sRGB for better accuracy
        let hsl: Hsl = srgb.into_color();

        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", LABEL_HSL, width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!(
                "hsl({:.0}, {:.1}%, {:.1}%)",
                hsl.hue.into_positive_degrees(),
                hsl.saturation * 100.0,
                hsl.lightness * 100.0
            )
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // HSB (HSV) - Hue, Saturation, Brightness
        let hsv: Hsv = srgb.into_color();

        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", LABEL_HSB, width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!(
                "hsb({:.0}, {:.1}%, {:.1}%)",
                hsv.hue.into_positive_degrees(),
                hsv.saturation * 100.0,
                hsv.value * 100.0
            )
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // CMYK - Cyan, Magenta, Yellow, Key (Black)
        let (c, m, y, k) = crate::color_utils::ColorUtils::rgb_to_cmyk((r, g, b));

        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", LABEL_CMYK, width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!(
                "cmyk({:.1}%, {:.1}%, {:.1}%, {:.1}%)",
                c * 100.0,
                m * 100.0,
                y * 100.0,
                k * 100.0
            )
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // LAB
        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", LABEL_LAB, width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!(
                "lab({:.2}, {:.2}, {:.2})",
                lab_color.l, lab_color.a, lab_color.b
            )
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // XYZ
        let xyz: Xyz = lab_color.into_color();
        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", LABEL_XYZ, width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!("xyz({:.3}, {:.3}, {:.3})", xyz.x, xyz.y, xyz.z)
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // OKLCH
        let oklch: Oklch = lab_color.into_color();
        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", LABEL_OKLCH, width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!(
                "oklch({:.3}, {:.3}, {:.1})",
                oklch.l,
                oklch.chroma,
                oklch.hue.into_positive_degrees()
            )
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        writeln!(output, "").map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        Ok(())
    }

    /// Get color contrast assessment based on WCAG guidelines
    fn get_contrast_assessment(
        color1_rgb: (u8, u8, u8),
        color2_rgb: (u8, u8, u8),
    ) -> (f64, ColoredString) {
        let contrast = ColorUtils::wcag_contrast_ratio_rgb(color1_rgb, color2_rgb);
        if contrast >= 7.0 {
            (contrast, crate::config::STATUS_PASS.bold().green())
        } else if contrast >= 4.5 {
            (contrast, crate::config::STATUS_WARNING.bold().yellow())
        } else {
            (contrast, crate::config::STATUS_FAIL.bold().red())
        }
    }

    /// Write additional color information
    fn write_additional_info(output: &mut String, lab_color: Lab) -> Result<()> {
        writeln!(
            output,
            "{:#<width$}",
            HEADER_ADDITIONAL_INFO.to_uppercase().bold(),
            width = COLUMN_HEADER_WIDTH
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // Convert LAB to sRGB for calculations
        let srgb: Srgb = lab_color.into_color();
        let r = (srgb.red * 255.0).round() as u8;
        let g = (srgb.green * 255.0).round() as u8;
        let b = (srgb.blue * 255.0).round() as u8;

        // Grayscale equivalent using LAB L* component
        let grayscale_lab = Lab::new(
            lab_color.l,
            0.0, // a component is not used for grayscale
            0.0, // b component is not used for grayscale
        );
        let grayscale_rgb: Srgb = grayscale_lab.into_color();

        writeln!(
            output,
            "{} {}",
            format!("{:>width$}", LABEL_GRAYSCALE, width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!(
                "#{:02X}{:02X}{:02X}",
                (grayscale_rgb.red * 255.0).round() as u8,
                (grayscale_rgb.green * 255.0).round() as u8,
                (grayscale_rgb.blue * 255.0).round() as u8
            )
            .yellow()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // WCAG calculations
        let wcag_luminance = ColorUtils::wcag_relative_luminance_rgb((r, g, b));
        writeln!(
            output,
            "{} {} {}",
            format!("{:>width$}", LABEL_WCAG_LUMINANCE, width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!("{:<width$.3}", wcag_luminance, width = COLUMN_WIDTH),
            format!("{:<width$}", LABEL_WCAG_COMPARTIBLE, width = COLUMN_WIDTH)
                .bold()
                .green()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // Color-coded contrast ratios
        let (contrast_white, white_contrast_color) =
            Self::get_contrast_assessment((r, g, b), (255, 255, 255));

        writeln!(
            output,
            "{} {} [{}]",
            format!("{:>width$}", LABEL_CONTRAST_WHITE, width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!("{:.2}:1", contrast_white),
            white_contrast_color
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        let (contrast_black, black_contrast_color) =
            Self::get_contrast_assessment((r, g, b), (0, 0, 0));

        writeln!(
            output,
            "{} {} [{}]",
            format!("{:>width$}", LABEL_CONTRAST_BLACK, width = COLUMN_WIDTH)
                .bold()
                .green(),
            format!("{:.2}:1", contrast_black),
            black_contrast_color
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        writeln!(
            output,
            "{} {} [{}] | {} [{}]",
            format!("{:>width$}", LABEL_BRIGHTNESS, width = COLUMN_WIDTH)
                .bold()
                .green(),
            Self::get_brightness_asssessment_lab(lab_color),
            "Lab".bold().green(),
            Self::get_brightness_asssessment_wcag(wcag_luminance),
            "WCAG".bold().green()
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        writeln!(output, "").map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        Ok(())
    }

    fn get_brightness_asssessment_lab(lab_color: Lab) -> String {
        // Calculate brightness based on L* value
        if lab_color.l > 50.0 {
            "Light".to_string()
        } else {
            "Dark".to_string()
        }
    }

    fn get_brightness_asssessment_wcag(wcag_luminance: f64) -> String {
        // Calculate brightness based on WCAG luminance
        if wcag_luminance > 0.18 {
            "Light".to_string()
        } else {
            "Dark".to_string()
        }
    }

    /// Write all collection matches together with closest colors
    fn write_unified_collection_matches(
        output: &mut String,
        lab_color: Lab,
        css_name: &str,
    ) -> Result<()> {
        Self::write_unified_collection_matches_impl(output, lab_color, css_name, None)
    }

    /// Write all collection matches together with closest colors using a custom strategy
    fn write_unified_collection_matches_with_strategy(
        output: &mut String,
        lab_color: Lab,
        css_name: &str,
        strategy: &dyn crate::color_distance_strategies::ColorDistanceStrategy,
    ) -> Result<()> {
        Self::write_unified_collection_matches_impl(output, lab_color, css_name, Some(strategy))
    }

    /// Implementation for writing unified collection matches with optional strategy
    fn write_unified_collection_matches_impl(
        output: &mut String,
        lab_color: Lab,
        _css_name: &str,
        strategy: Option<&dyn crate::color_distance_strategies::ColorDistanceStrategy>,
    ) -> Result<()> {
        use palette::IntoColor;

        writeln!(
            output,
            "{:#<width$}",
            HEADER_COLOR_COLLECTIONS.to_uppercase().bold(),
            width = COLUMN_HEADER_WIDTH
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        // Convert LAB to RGB for collection matching
        let srgb: Srgb = lab_color.into_color();
        let r = (srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;
        let rgb_array = [r, g, b];

        let manager = crate::color_parser::UnifiedColorManager::new()?;

        // Choose appropriate matching method based on strategy
        if let Some(strategy) = strategy {
            // Use strategy-based matching
            let css_matches = manager.find_closest_css_colors_with_strategy(rgb_array, 2, strategy);
            Self::write_unified_collection_results(COLLECTION_CSS, output, &css_matches)?;

            let classic_matches =
                manager.find_closest_ral_classic_with_strategy(rgb_array, 2, strategy);
            Self::write_unified_collection_results(
                COLLECTION_RAL_CLASSIC,
                output,
                &classic_matches,
            )?;

            let design_matches =
                manager.find_closest_ral_design_with_strategy(rgb_array, 2, strategy);
            Self::write_unified_collection_results(COLLECTION_RAL_DESIGN, output, &design_matches)?;
        } else {
            // Use default matching (backward compatibility)
            use crate::color_parser::{
                RgbColor, find_closest_ral_classic, find_closest_ral_design,
            };

            let css_matches = manager.find_closest_css_colors(rgb_array, 2);
            Self::write_css_collection(output, &css_matches)?;

            let rgb = RgbColor::new(r, g, b);
            let classic_matches = find_closest_ral_classic(&rgb, 2);
            Self::write_ral_classic_collection(output, &classic_matches)?;

            let design_matches = find_closest_ral_design(&rgb, 2);
            Self::write_ral_design_collection(output, &design_matches)?;
        }

        writeln!(output, "").map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        Ok(())
    }

    /// Write CSS color collection with closest matches (unified with other collections)
    fn write_css_collection(
        output: &mut String,
        css_matches: &[crate::color_parser::ColorMatch],
    ) -> Result<()> {
        Self::write_unified_collection_results(COLLECTION_CSS, output, css_matches)
    }

    /// Write unified collection search results that works with both ColorMatch and RalMatch
    fn write_unified_collection_results(
        collection_name: &str,
        output: &mut String,
        matches: &[crate::color_parser::ColorMatch],
    ) -> Result<()> {
        writeln!(output, "{}", format!("{}", collection_name).bold())
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        if matches.is_empty() {
            writeln!(output, "{}", NO_MATCHES_MESSAGE.bold())
                .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
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

                writeln!(
                    output,
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
                )
                .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
            }
        }

        writeln!(output, "").map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        Ok(())
    }

    /// Write collection search results with closest matches
    fn write_collection_search_results(
        collection_name: &str,
        output: &mut String,
        matches: &[crate::color_parser::RalMatch],
    ) -> Result<()> {
        writeln!(output, "{}", format!("{}", collection_name).bold())
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

        if matches.is_empty() {
            writeln!(output, "{}", NO_MATCHES_MESSAGE.bold())
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
                    ral_match.code
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
        Self::write_collection_search_results(COLLECTION_RAL_CLASSIC, output, matches)
    }

    /// Write RAL Design collection with closest matches
    fn write_ral_design_collection(
        output: &mut String,
        matches: &[crate::color_parser::RalMatch],
    ) -> Result<()> {
        Self::write_collection_search_results(COLLECTION_RAL_DESIGN, output, matches)
    }

    /// Write header for color schemes section
    fn write_color_scheme_header(output: &mut String, section_title: &str) -> Result<()> {
        // Header for color schemes section
        writeln!(
            output,
            "\n{:#<width$}",
            format!(
                "{} | {} ",
                HEADER_COLOR_SCHEMES.to_uppercase(),
                section_title
            )
            .bold(),
            width = COLUMN_HEADER_WIDTH
        )
        .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        Ok(())
    }

    /// Format color schemes section for comprehensive reports with both HSL and Lab strategies
    pub fn format_color_schemes(
        schemes: &crate::color_schemes::ColorSchemeResult,
    ) -> Result<String> {
        let mut output = String::new();

        Self::write_color_scheme_header(&mut output, "HSL Color Space Strategy")?;

        // HSL Complementary color section
        Self::write_color_scheme_section(
            &mut output,
            HEADER_SCHEMA_COMPLIMENTARY,
            &[schemes.hsl_complementary],
            None,
            false, // Use HSL format
            "HSL",
        )?;

        // HSL Split-complementary colors section
        Self::write_color_scheme_section(
            &mut output,
            HEADER_SCHEMA_SPLIT_COMPLIMENTARY,
            &[
                schemes.hsl_split_complementary.0,
                schemes.hsl_split_complementary.1,
            ],
            None,
            false, // Use HSL format
            "HSL",
        )?;

        // HSL Triadic colors section
        Self::write_color_scheme_section(
            &mut output,
            HEADER_SCHEMA_TRIADIC,
            &[schemes.hsl_triadic.0, schemes.hsl_triadic.1],
            None,
            false, // Use HSL format
            "HSL",
        )?;

        Self::write_color_scheme_header(&mut output, "Lab Color Space Strategy")?;

        // Lab Complementary color section
        Self::write_color_scheme_section(
            &mut output,
            HEADER_SCHEMA_COMPLIMENTARY,
            &[schemes.lab_complementary],
            None,
            true, // Use Lab format
            "Lab",
        )?;

        // Lab Split-complementary colors section
        Self::write_color_scheme_section(
            &mut output,
            HEADER_SCHEMA_SPLIT_COMPLIMENTARY,
            &[
                schemes.lab_split_complementary.0,
                schemes.lab_split_complementary.1,
            ],
            None,
            true, // Use Lab format
            "Lab",
        )?;

        // Lab Triadic colors section
        Self::write_color_scheme_section(
            &mut output,
            HEADER_SCHEMA_TRIADIC,
            &[schemes.lab_triadic.0, schemes.lab_triadic.1],
            None,
            true, // Use Lab format
            "Lab",
        )?;

        Ok(output)
    }

    /// Write a specific color scheme section
    fn write_color_scheme_section(
        output: &mut String,
        title: &str,
        colors: &[Lab],
        luminance_matched: Option<&[Lab]>,
        use_lab_output: bool,
        strategy_name: &str,
    ) -> Result<()> {
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

            writeln!(
                output,
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
            )
            .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
        }

        // Luminance-matched variations if available
        if let Some(matched_colors) = luminance_matched {
            let header_text = format!("Luminance-matched variations ({})", strategy_name);
            writeln!(output, "{}", format!("{}", header_text).bold())
                .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

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

                writeln!(
                    output,
                    "{} {}",
                    format!(
                        "{:>width$}",
                        format!("{}:", color_name),
                        width = COLUMN_WIDTH
                    )
                    .bold()
                    .green(),
                    color_value
                )
                .map_err(|e| ColorError::InvalidColor(e.to_string()))?;

                writeln!(
                    output,
                    "{:>width$} {}",
                    "",
                    hex.yellow(),
                    width = COLUMN_WIDTH
                )
                .map_err(|e| ColorError::InvalidColor(e.to_string()))?;
            }
        }
        Ok(())
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

    /// Example function showing how to use OutputUtils with compact error handling
    /// 
    /// This demonstrates the integration between color_formatter and output_utils
    /// using the IoResultExt and Utf8ResultExt traits for clean error handling.
    /// 
    /// Before:
    /// ```rust
    /// OutputUtils::write_header(&mut output, "Color Information", 40)
    ///     .map_err(|e| ColorError::InvalidColor(format!("IO error: {}", e)))?;
    /// ```
    /// 
    /// After:
    /// ```rust
    /// OutputUtils::write_header(&mut output, "Color Information", 40).to_err()?;
    /// ```
    pub fn format_color_with_output_utils(
        lab_color: Lab,
        color_name: &str,
    ) -> Result<String> {
        let mut output = Vec::new();
        
        // Using OutputUtils functions with compact error handling
        OutputUtils::write_header(&mut output, "Color Information", 40).to_err()?;
        
        // Convert Lab to RGB for display
        let srgb: Srgb = lab_color.into_color();
        let r = (srgb.red * 255.0).round() as u8;
        let g = (srgb.green * 255.0).round() as u8;
        let b = (srgb.blue * 255.0).round() as u8;
        
        // Format color values
        let hex_value = format!("#{:02X}{:02X}{:02X}", r, g, b);
        let rgb_value = format!("rgb({}, {}, {})", r, g, b);
        
        OutputUtils::write_label_value(&mut output, "Color Name", color_name, 15).to_err()?;
        OutputUtils::write_label_value(&mut output, "Hex", &hex_value, 15).to_err()?;
        OutputUtils::write_label_value(&mut output, "RGB", &rgb_value, 15).to_err()?;
        
        OutputUtils::write_separator(&mut output, 40).to_err()?;
        
        // Convert Vec<u8> to String
        String::from_utf8(output).to_err()
    }

    /// Another example showing the power of extension traits for error handling
    /// This function demonstrates multiple types of error conversions
    pub fn format_color_report_compact(
        lab_color: Lab,
        title: &str,
    ) -> Result<String> {
        let mut output = Vec::new();
        
        // All these calls are now very clean!
        OutputUtils::write_header(&mut output, title, 50).to_err()?;
        
        let srgb: Srgb = lab_color.into_color();
        let r = (srgb.red * 255.0).round() as u8;
        let g = (srgb.green * 255.0).round() as u8;
        let b = (srgb.blue * 255.0).round() as u8;
        
        OutputUtils::write_label_value(&mut output, "RGB", &format!("({}, {}, {})", r, g, b), 10).to_err()?;
        OutputUtils::write_label_value(&mut output, "Lab", &format!("({:.1}, {:.1}, {:.1})", lab_color.l, lab_color.a, lab_color.b), 10).to_err()?;
        OutputUtils::write_separator(&mut output, 50).to_err()?;
        
        // Even the UTF-8 conversion is clean!
        String::from_utf8(output).to_err()
    }

    // ...existing code...
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
        assert!(output.contains("rgb(")); // RGB is now embedded in HEX line as "rgb(r, g, b)"
        assert!(output.contains("HSL"));
        assert!(output.contains("LAB"));
    }

    #[test]
    fn test_format_with_strategy() {
        let lab_color = Lab::new(50.0, 20.0, -30.0);

        // Test comprehensive strategy (default)
        let result =
            ColorFormatter::format_with_strategy(lab_color, "#008080", "teal", "comprehensive");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("FORMAT CONVERSIONS"));

        // Test minimal strategy
        let result = ColorFormatter::format_with_strategy(lab_color, "#008080", "teal", "minimal");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("RGB"));
        assert!(output.contains("HEX"));

        // Test JSON strategy
        let result = ColorFormatter::format_with_strategy(lab_color, "#008080", "teal", "json");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("{"));
        assert!(output.contains("\"rgb\""));
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

    #[test]
    fn test_format_color_with_output_utils() {
        let lab_color = Lab::new(50.0, 20.0, -30.0);
        let result = ColorFormatter::format_color_with_output_utils(lab_color, "Test Color");
        
        assert!(result.is_ok());
        let output = result.unwrap();
        
        // Print the actual output for demonstration
        println!("\n=== OutputUtils Integration Demo ===");
        println!("{}", output);
        
        assert!(output.contains("COLOR INFORMATION"));
        assert!(output.contains("Test Color"));
        assert!(output.contains("Hex"));
        assert!(output.contains("RGB"));
        assert!(output.contains("#"));
    }

    #[test]
    fn test_format_color_report_compact() {
        let lab_color = Lab::new(60.0, 25.0, -40.0);
        let result = ColorFormatter::format_color_report_compact(lab_color, "Compact Report");
        
        assert!(result.is_ok());
        let output = result.unwrap();
        
        // Print the actual output for demonstration
        println!("\n=== Compact Report Demo ===");
        println!("{}", output);
        
        assert!(output.contains("COMPACT REPORT"));
        assert!(output.contains("RGB"));
        assert!(output.contains("Lab"));
        assert!(output.contains("60.0"));
    }
}
