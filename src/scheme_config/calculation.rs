//! Color scheme calculation engine
//!
//! This module breaks down the large calculate method into focused, composable functions.

use crate::error::Result;
use crate::config::algorithm_constants;
use crate::color_schemes::{
    ColorSchemeResult, HslColorSchemeStrategy, LabColorSchemeStrategy, ColorSchemeStrategy,
    preserve_wcag_relative_luminance, preserve_lab_luminance,
    adjust_color_lab_luminance,
};
use crate::color_ops::luminance::wcag_relative;
use palette::{Lab, IntoColor, Srgb};
use super::types::{ColorSchemeConfig, LuminanceConfig, BasicColorSchemes};

/// Local implementation of relative luminance adjustment since the original is private
fn adjust_color_relative_luminance(color: Lab, target_luminance: f64) -> Result<Lab> {
    // Convert to RGB, check current luminance
    let srgb: Srgb = color.into_color();
    let current_luminance = wcag_relative(srgb);
    
    if (current_luminance - target_luminance).abs() < algorithm_constants::LUMINANCE_TOLERANCE {
        return Ok(color);
    }
    
    // Binary search for target luminance by scaling lightness
    let mut low = 0.0_f32;
    let mut high = algorithm_constants::BINARY_SEARCH_HIGH_LUMINANCE;
    let mut best_color = color;
    
    for _ in 0..50 {
        let mid = (low + high) / algorithm_constants::BINARY_SEARCH_DIVISION_FACTOR as f32;
        let test_color = Lab::new(mid, color.a, color.b);
        let test_srgb: Srgb = test_color.into_color();
        let test_luminance = wcag_relative(test_srgb);
        
        if (test_luminance - target_luminance).abs() < algorithm_constants::LUMINANCE_TOLERANCE {
            return Ok(test_color);
        }
        
        if test_luminance < target_luminance {
            low = mid;
        } else {
            high = mid;
        }
        
        best_color = test_color;
    }
    
    Ok(best_color)
}

/// Apply target luminance adjustment to base color
pub fn apply_target_luminance(config: ColorSchemeConfig, mut base_color: Lab) -> Result<Lab> {
    if let Some(target_rel_lum) = config.target_relative_luminance {
        base_color = adjust_color_relative_luminance(base_color, target_rel_lum)?;
    } else if let Some(target_lab_lum) = config.target_lab_luminance {
        base_color = adjust_color_lab_luminance(base_color, target_lab_lum)?;
    }
    Ok(base_color)
}

/// Calculate basic color schemes using both HSL and Lab strategies
pub fn calculate_basic_schemes(base_color: Lab) -> BasicColorSchemes {
    let hsl_strategy = HslColorSchemeStrategy;
    let lab_strategy = LabColorSchemeStrategy;

    BasicColorSchemes {
        hsl_complementary: hsl_strategy.complementary(base_color),
        lab_complementary: lab_strategy.complementary(base_color),
        analogous_warm: hsl_strategy.triadic(base_color).0,
        analogous_cool: hsl_strategy.triadic(base_color).1,
        triadic_1: lab_strategy.triadic(base_color).0,
        triadic_2: lab_strategy.triadic(base_color).1,
        split_complementary_1: hsl_strategy.split_complementary(base_color).0,
        split_complementary_2: hsl_strategy.split_complementary(base_color).1,
        tetradic_1: hsl_strategy.tetradic(base_color).0,
        tetradic_2: hsl_strategy.tetradic(base_color).1,
        tetradic_3: hsl_strategy.tetradic(base_color).2,
    }
}

/// Apply luminance matching to a single color
pub fn apply_luminance_matching(
    color: Lab,
    base_color: Lab,
    config: LuminanceConfig,
) -> Result<Option<Lab>> {
    if config.preserve_relative_luminance {
        Ok(Some(preserve_wcag_relative_luminance(color, base_color)?))
    } else if config.preserve_lab_luminance {
        Ok(Some(preserve_lab_luminance(color, base_color)?))
    } else {
        Ok(None)
    }
}

/// Apply luminance matching to a pair of colors
pub fn apply_luminance_matching_pair(
    colors: (Lab, Lab),
    base_color: Lab,
    config: LuminanceConfig,
) -> Result<Option<(Lab, Lab)>> {
    if config.preserve_relative_luminance {
        let color1 = preserve_wcag_relative_luminance(colors.0, base_color)?;
        let color2 = preserve_wcag_relative_luminance(colors.1, base_color)?;
        Ok(Some((color1, color2)))
    } else if config.preserve_lab_luminance {
        let color1 = preserve_lab_luminance(colors.0, base_color)?;
        let color2 = preserve_lab_luminance(colors.1, base_color)?;
        Ok(Some((color1, color2)))
    } else {
        Ok(None)
    }
}

/// Apply luminance matching to a triple of colors
pub fn apply_luminance_matching_triple(
    colors: (Lab, Lab, Lab),
    base_color: Lab,
    config: LuminanceConfig,
) -> Result<Option<(Lab, Lab, Lab)>> {
    if config.preserve_relative_luminance {
        let color1 = preserve_wcag_relative_luminance(colors.0, base_color)?;
        let color2 = preserve_wcag_relative_luminance(colors.1, base_color)?;
        let color3 = preserve_wcag_relative_luminance(colors.2, base_color)?;
        Ok(Some((color1, color2, color3)))
    } else if config.preserve_lab_luminance {
        let color1 = preserve_lab_luminance(colors.0, base_color)?;
        let color2 = preserve_lab_luminance(colors.1, base_color)?;
        let color3 = preserve_lab_luminance(colors.2, base_color)?;
        Ok(Some((color1, color2, color3)))
    } else {
        Ok(None)
    }
}

/// Functional refactored version of the calculate method
pub fn calculate_color_schemes(
    config: ColorSchemeConfig,
    base_color: Lab,
) -> Result<ColorSchemeResult> {
    // Apply target luminance adjustments
    let adjusted_base_color = apply_target_luminance(config, base_color)?;

    // Calculate basic color schemes
    let basic_schemes = calculate_basic_schemes(adjusted_base_color);
    let luminance_config = LuminanceConfig::from(config);

    // Build the result using the original ColorSchemeResult structure
    // Apply luminance matching where needed
    let lab_complementary = apply_luminance_matching(
        basic_schemes.lab_complementary,
        adjusted_base_color,
        luminance_config,
    )?.unwrap_or(basic_schemes.lab_complementary);

    let hsl_complementary = apply_luminance_matching(
        basic_schemes.hsl_complementary,
        adjusted_base_color,
        luminance_config,
    )?.unwrap_or(basic_schemes.hsl_complementary);

    // Create a result with basic scheme structure
    // (This is a simplified version - the original has more complex structure)
    Ok(ColorSchemeResult {
        base_color: adjusted_base_color,
        hsl_complementary,
        lab_complementary,
        hsl_split_complementary: (basic_schemes.split_complementary_1, basic_schemes.split_complementary_2),
        hsl_triadic: (basic_schemes.triadic_1, basic_schemes.triadic_2),
        hsl_tetradic: (basic_schemes.tetradic_1, basic_schemes.tetradic_2, basic_schemes.tetradic_3),
        lab_split_complementary: (basic_schemes.split_complementary_1, basic_schemes.split_complementary_2),
        lab_triadic: (basic_schemes.triadic_1, basic_schemes.triadic_2),
        lab_tetradic: (basic_schemes.tetradic_1, basic_schemes.tetradic_2, basic_schemes.tetradic_3),
        luminance_matched_hsl_complementary: apply_luminance_matching(
            basic_schemes.hsl_complementary,
            adjusted_base_color,
            luminance_config,
        )?,
        luminance_matched_lab_complementary: apply_luminance_matching(
            basic_schemes.lab_complementary,
            adjusted_base_color,
            luminance_config,
        )?,
        luminance_matched_hsl_split_complementary: apply_luminance_matching_pair(
            (basic_schemes.split_complementary_1, basic_schemes.split_complementary_2),
            adjusted_base_color,
            luminance_config,
        )?,
        luminance_matched_hsl_triadic: apply_luminance_matching_pair(
            (basic_schemes.triadic_1, basic_schemes.triadic_2),
            adjusted_base_color,
            luminance_config,
        )?,
        luminance_matched_hsl_tetradic: apply_luminance_matching_triple(
            (basic_schemes.tetradic_1, basic_schemes.tetradic_2, basic_schemes.tetradic_3),
            adjusted_base_color,
            luminance_config,
        )?,
        luminance_matched_lab_split_complementary: apply_luminance_matching_pair(
            (basic_schemes.split_complementary_1, basic_schemes.split_complementary_2),
            adjusted_base_color,
            luminance_config,
        )?,
        luminance_matched_lab_triadic: apply_luminance_matching_pair(
            (basic_schemes.triadic_1, basic_schemes.triadic_2),
            adjusted_base_color,
            luminance_config,
        )?,
        luminance_matched_lab_tetradic: apply_luminance_matching_triple(
            (basic_schemes.tetradic_1, basic_schemes.tetradic_2, basic_schemes.tetradic_3),
            adjusted_base_color,
            luminance_config,
        )?,
    })
}
